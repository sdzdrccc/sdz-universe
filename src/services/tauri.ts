import { invoke } from '@tauri-apps/api/core'
import type { CatalogItem, CategoryKey, CategoryStats } from '../types'

export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window
}

export interface SearchFilter {
  keyword?: string
  category?: CategoryKey
  tag?: string
  favoritesOnly?: boolean
}

export const tauriData = {
  kind: 'tauri' as const,
  editable: true,
  /** Rust 侧 CatalogItem 序列化即前端 CatalogItem 形状（camelCase，tags 为数组），无需映射 */
  listItems(filter?: SearchFilter): Promise<CatalogItem[]> {
    return invoke('list_items', filter ? { filter } : undefined)
  },
  stats(): Promise<CategoryStats> {
    return invoke('stats')
  },
  allTags(category?: CategoryKey): Promise<string[]> {
    return invoke<string[]>('all_tags', category ? { category } : undefined)
  },
  toggleFavorite(id: string, favorite: boolean): Promise<void> {
    return invoke('toggle_favorite', { id, favorite })
  },
  upsertItem(item: CatalogItem): Promise<void> {
    return invoke('upsert_item', { item })
  },
  deleteItem(id: string): Promise<void> {
    return invoke('delete_item', { id })
  },
  exportJson(): Promise<string> {
    return invoke<string>('export_json')
  },
  importJson(payload: string): Promise<number> {
    return invoke<number>('import_json', { payload })
  },
}
