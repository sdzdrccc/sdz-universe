import type { CatalogItem } from '../types'

/**
 * 内置种子数据（当前为空）。
 * - 网页版（GitHub Pages）：作为无远端 JSON 时的兜底基线
 * - 桌面版（Tauri）：SQLite 为空时由 Rust 侧导入 seed.json
 * 在 public/data/catalog.json 里维护你自己的条目即可（桌面端可通过 导入 JSON 同步进库）。
 */
export function seedItems(): CatalogItem[] {
  return []
}
