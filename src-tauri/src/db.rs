//! Rust 侧：SQLite 数据库封装 + Tauri 命令
//! 错误统一用 String，便于前端捕获并展示。
use rusqlite::{params, Connection, ToSql, types::Type};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

type CmdResult<T> = Result<T, String>;

const CATEGORIES: [&str; 6] = ["books", "anime", "novels", "games", "apps", "llm"];

// ===================== 数据模型 =====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub id: String,
    pub category: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_label: Option<String>,
    #[serde(default)]
    pub favorite: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorites_only: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CategoryStats {
    pub total: i64,
    pub favorites: i64,
    #[serde(rename = "byCategory")]
    pub by_category: serde_json::Value,
}

// ===================== 数据库 =====================

pub struct AppDatabase {
    conn: Mutex<Connection>,
}

impl AppDatabase {
    pub fn new() -> CmdResult<Self> {
        let path = db_path();
        let conn = Connection::open(&path).map_err(|e| format!("open db: {e}"))?;
        Self::init_schema(&conn).map_err(|e| format!("init schema: {e}"))?;
        // 首次启动：若空库则导入种子数据
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM items", [], |r| r.get(0))
            .map_err(|e| format!("count: {e}"))?;
        if count == 0 {
            let seed_json: &str = include_str!("seed.json");
            let items: Vec<CatalogItem> =
                serde_json::from_str(seed_json).map_err(|e| format!("bad seed: {e}"))?;
            for item in &items {
                upsert_item_inner(&conn, item).map_err(|e| format!("seed upsert: {e}"))?;
            }
        }
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn init_schema(conn: &Connection) -> Result<(), String> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS items (
                id          TEXT PRIMARY KEY,
                category    TEXT NOT NULL,
                name        TEXT NOT NULL,
                original    TEXT,
                year        INTEGER,
                meta        TEXT,
                rating      REAL,
                tags        TEXT,
                description TEXT,
                poster      TEXT,
                link        TEXT,
                link_label  TEXT,
                favorite    INTEGER NOT NULL DEFAULT 0,
                note        TEXT,
                created_at  TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_items_category ON items(category);
            CREATE TABLE IF NOT EXISTS check_logs (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                item_id    TEXT,
                result     TEXT,
                checked_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            "#,
        )
        .map_err(|e| e.to_string())
    }

    fn map_row(row: &rusqlite::Row) -> Result<CatalogItem, String> {
        // 列序: 0 id, 1 category, 2 name, 3 original, 4 year, 5 meta,
        //       6 rating, 7 tags, 8 description, 9 poster, 10 link,
        //       11 link_label, 12 favorite, 13 note
        let tags_raw: String = row.get(7).map_err(|e| e.to_string())?;
        Ok(CatalogItem {
            id: row.get(0).map_err(|e| e.to_string())?,
            category: row.get(1).map_err(|e| e.to_string())?,
            name: row.get(2).map_err(|e| e.to_string())?,
            original: row.get(3).map_err(|e| e.to_string())?,
            year: row.get(4).map_err(|e| e.to_string())?,
            meta: row.get(5).map_err(|e| e.to_string())?,
            rating: row.get(6).map_err(|e| e.to_string())?,
            tags: serde_json::from_str(&tags_raw).unwrap_or_default(),
            description: row.get(8).map_err(|e| e.to_string())?,
            poster: row.get(9).map_err(|e| e.to_string())?,
            link: row.get(10).map_err(|e| e.to_string())?,
            link_label: row.get(11).map_err(|e| e.to_string())?,
            favorite: row.get(12).map_err(|e| e.to_string())?,
            note: row.get(13).map_err(|e| e.to_string())?,
        })
    }
}

/// 数据文件位置：Windows 放 %APPDATA%\<identifier>，否则退回当前目录。
fn db_path() -> std::path::PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let dir = std::path::PathBuf::from(appdata).join("com.sdz.universe");
        let _ = std::fs::create_dir_all(&dir);
        dir.join("sdz-universe.db")
    } else {
        std::path::PathBuf::from("sdz-universe.db")
    }
}

// ===================== Tauri 命令 =====================

#[tauri::command]
pub fn list_items(db: State<AppDatabase>, filter: Option<SearchFilter>) -> CmdResult<Vec<CatalogItem>> {
    let conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    let mut sql = String::from(
        "SELECT id, category, name, original, year, meta, rating, tags, description, poster, link, link_label, favorite, note FROM items",
    );
    let mut conds: Vec<String> = Vec::new();
    let mut args: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(f) = &filter {
        if let Some(cat) = &f.category {
            conds.push("category = ?".into());
            args.push(Box::new(cat.clone()));
        }
        if let Some(kw) = &f.keyword {
            let like = format!("%{kw}%");
            conds.push(
                "(name LIKE ? OR original LIKE ? OR meta LIKE ? OR description LIKE ? OR tags LIKE ?)"
                    .into(),
            );
            for _ in 0..5 {
                args.push(Box::new(like.clone()));
            }
        }
        if let Some(tag) = &f.tag {
            conds.push("tags LIKE ?".into());
            args.push(Box::new(format!("%\"{tag}\"%")));
        }
        if f.favorites_only.unwrap_or(false) {
            conds.push("favorite = 1".into());
        }
    }
    if !conds.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conds.join(" AND "));
    }
    sql.push_str(" ORDER BY updated_at DESC, id");

    let params_ref: Vec<&dyn ToSql> = args.iter().map(|b| b.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params_ref.as_slice(), |row| {
            AppDatabase::map_row(row)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, e.into()))
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn stats(db: State<AppDatabase>) -> CmdResult<CategoryStats> {
    let conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM items", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let favorites: i64 = conn
        .query_row("SELECT COUNT(*) FROM items WHERE favorite = 1", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let mut by = serde_json::Map::new();
    for c in CATEGORIES {
        let n: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM items WHERE category = ?1",
                params![c],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;
        by.insert(c.to_string(), serde_json::json!(n));
    }
    Ok(CategoryStats {
        total,
        favorites,
        by_category: serde_json::Value::Object(by),
    })
}

#[tauri::command]
pub fn all_tags(db: State<AppDatabase>, category: Option<String>) -> CmdResult<Vec<String>> {
    let conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    let mut sql = String::from("SELECT tags FROM items");
    let mut args: Vec<Box<dyn ToSql>> = Vec::new();
    if let Some(cat) = &category {
        sql.push_str(" WHERE category = ?");
        args.push(Box::new(cat.clone()));
    }
    let params_ref: Vec<&dyn ToSql> = args.iter().map(|b| b.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params_ref.as_slice(), |r| r.get::<_, String>(0))
        .map_err(|e| e.to_string())?;
    let mut set = std::collections::BTreeSet::new();
    for row in rows {
        let raw = row.map_err(|e| e.to_string())?;
        if let Ok(tags) = serde_json::from_str::<Vec<String>>(&raw) {
            for t in tags {
                set.insert(t);
            }
        }
    }
    Ok(set.into_iter().collect())
}

#[tauri::command]
pub fn toggle_favorite(db: State<AppDatabase>, id: String, favorite: bool) -> CmdResult<()> {
    let conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    conn.execute(
        "UPDATE items SET favorite = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![if favorite { 1 } else { 0 }, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn upsert_item(db: State<AppDatabase>, item: CatalogItem) -> CmdResult<()> {
    let mut conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    upsert_item_inner(&mut conn, &item)?;
    Ok(())
}

#[tauri::command]
pub fn delete_item(db: State<AppDatabase>, id: String) -> CmdResult<()> {
    let conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    conn.execute("DELETE FROM items WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn export_json(db: State<AppDatabase>) -> CmdResult<String> {
    let items = list_items(db, None)?;
    serde_json::to_string_pretty(&items).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_json(db: State<AppDatabase>, payload: String) -> CmdResult<i32> {
    let items: Vec<CatalogItem> = serde_json::from_str(&payload).map_err(|e| e.to_string())?;
    let mut conn = db.conn.lock().map_err(|e| format!("lock: {e}"))?;
    let mut n = 0i32;
    for item in items {
        if item.id.is_empty() || item.name.is_empty() {
            continue;
        }
        upsert_item_inner(&mut conn, &item).map_err(|e| e.to_string())?;
        n += 1;
    }
    Ok(n)
}

fn upsert_item_inner(conn: &Connection, item: &CatalogItem) -> CmdResult<()> {
    let tags_json = serde_json::to_string(&item.tags).map_err(|e| e.to_string())?;
    conn.execute(
        r#"INSERT INTO items (id, category, name, original, year, meta, rating, tags, description,
                              poster, link, link_label, favorite, note, updated_at)
           VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, datetime('now'))
           ON CONFLICT(id) DO UPDATE SET
             category=excluded.category, name=excluded.name, original=excluded.original,
             year=excluded.year, meta=excluded.meta, rating=excluded.rating,
             tags=excluded.tags, description=excluded.description, poster=excluded.poster,
             link=excluded.link, link_label=excluded.link_label, favorite=excluded.favorite,
             note=excluded.note, updated_at=datetime('now')"#,
        params![
            item.id,
            item.category,
            item.name,
            item.original,
            item.year,
            item.meta,
            item.rating,
            tags_json,
            item.description,
            item.poster,
            item.link,
            item.link_label,
            item.favorite,
            item.note,
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
