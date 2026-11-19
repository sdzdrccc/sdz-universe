import type { CatalogItem, CategoryKey, CategoryStats } from '../types'
import type { SearchFilter } from './tauri'

export interface DataAPI {
  readonly kind: 'tauri' | 'web'
  readonly editable: boolean
  listItems(filter?: SearchFilter): Promise<CatalogItem[]>
  stats(): Promise<CategoryStats>
  allTags(category?: CategoryKey): Promise<string[]>
  toggleFavorite(id: string, favorite: boolean): Promise<void>
  upsertItem(item: CatalogItem): Promise<void>
  deleteItem(id: string): Promise<void>
  exportJson(): Promise<string>
  importJson(payload: string): Promise<number>
}
