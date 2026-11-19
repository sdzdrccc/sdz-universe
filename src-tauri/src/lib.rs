//! Rust 后端入口：注册 Tauri 命令与 SQLite 状态。
pub mod db;

use db::AppDatabase;

pub fn run() {
    let db = AppDatabase::new().expect("failed to initialize SQLite database");
    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            db::list_items,
            db::stats,
            db::all_tags,
            db::toggle_favorite,
            db::upsert_item,
            db::delete_item,
            db::export_json,
            db::import_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
