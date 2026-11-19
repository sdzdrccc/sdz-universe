# SDZ Universe — 个人收藏工作台

> 书籍 · 动漫 · 小说 · 游戏 · 应用 · 大模型，卡片墙式浏览与个人收藏管理。
> 参考 [catyi.top/games](https://catyi.top/games) 的海报墙卡片风格。

## 技术栈

| 层 | 技术 | 说明 |
|---|---|---|
| 前端 | Vue 3 + TypeScript + Vite + vue-router | 卡片墙 UI、总览统计 |
| 桌面端 | Tauri 2 + Rust | `cargo tauri build` 出 5~20MB 安装包 |
| 数据库 | SQLite（rusqlite bundled） | 单文件数据，备份 = 复制文件 |
| 网页版 | 纯静态 | 部署 GitHub Pages，静态 JSON + localStorage |

## 数据源切换

同一套前端代码，两种运行形态，**自动切换**（`src/composables/useData.ts`）：

| 形态 | 运行环境 | 数据源 | 能力 |
|---|---|---|---|
| 桌面版 | Tauri（`window.__TAURI__` 存在） | Rust + SQLite（`%APPDATA%\com.sdz.universe\sdz-universe.db`） | 完整可写：CRUD、搜索、收藏、导出/导入 JSON、首次启动自动导入种子数据 |
| 网页版 | GitHub Pages / 任意静态托管 | 远端静态 JSON（`data/catalog.json`）+ localStorage 叠加层 | 可读可收藏（收藏/备注存浏览器）；无 Rust 功能 |

远端 JSON 基线地址：构建时设置 `VITE_DATA_BASE_URL`（默认读部署产物内的 `data/catalog.json`；未配置则回落到内置种子数据）。

## 目录结构

```
sdz-universe/
├── src/                        # 前端（Vue 3 + TS）
│   ├── types.ts                # CatalogItem / 分类元数据
│   ├── data/seed.ts            # 内置种子数据（断网兜底）
│   ├── services/
│   │   ├── dataApi.ts          # 统一 DataAPI 接口（切换层）
│   │   ├── tauri.ts            # 桌面端：invoke Rust 命令
│   │   └── web.ts             # 网页版：JSON 基线 + localStorage
│   ├── composables/useData.ts  # isTauri() 自动选择数据源
│   ├── components/
│   │   ├── PosterCard.vue      # 卡片（海报/占位封面/角标/星标）
│   │   ├── PosterWall.vue      # 卡片网格 + 详情弹窗
│   │   └── MediaDetail.vue     # 详情弹窗（海报/标签/收藏/外链）
│   └── views/
│       ├── HomeView.vue        # 总览：统计 + 分类入口 + 最近收录
│       └── CategoryView.vue    # 分类页：搜索/标签筛选/只看收藏/导出导入
├── public/data/catalog.json    # 网页版基线数据（随产物发布）
├── src-tauri/                  # Rust 桌面端
│   ├── icons/                  # 打包图标（ico + png，占位纯色图，建议替换）
│   └── src/
│       ├── lib.rs              # 命令注册 + 数据库初始化
│       ├── db.rs               # SQLite CRUD/搜索/统计/导入导出
│       └── seed.json           # 首次启动种子数据（与 public/data/catalog.json 同步）
└── .github/workflows/
    ├── pages.yml               # GitHub Pages 自动部署（网页版）
    └── tauri.yml               # 桌面安装包构建（Releases）
```

## 开发

```bash
# 前端 + Tauri 桌面端（热更新）
npm install
npm run tauri dev

# 仅前端（网页版开发）
npm run dev

# 类型检查
npm run typecheck
```

首次启动时 Rust 侧会把 `src-tauri/src/seed.json`（与 `public/data/catalog.json` 保持同步）导入空库。

## 构建与发布

```bash
# 网页版（GitHub Pages）
npm run build:pages        # 产物 dist/，base=/sdz-universe/
# → 推送到 main，.github/workflows/pages.yml 自动部署

# 桌面版
npm run tauri build        # 出 NSIS 安装包 → GitHub Releases
```

## 数据模型（SQLite）

```sql
items(
  id TEXT PRIMARY KEY,
  category TEXT NOT NULL,     -- books/anime/novels/games/apps/llm
  name TEXT NOT NULL,
  original TEXT, year INTEGER, meta TEXT,
  rating REAL, tags TEXT,     -- tags: JSON 数组字符串
  description TEXT, poster TEXT,
  link TEXT, link_label TEXT,
  favorite INTEGER NOT NULL DEFAULT 0,
  note TEXT,
  created_at TEXT, updated_at TEXT
)
```

## 参考

- 交互风格参考：[catyi.top/games](https://catyi.top/games)（海报卡片 + 详情弹窗）
- 技术路线参考：`应用收藏工作台-技术方案.md` 中方案 C（Tauri）+ 4.1（Pages 只读版）
