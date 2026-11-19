import { ref } from 'vue'
import type { CatalogItem, CategoryStats } from '../types'
import { tauriData, isTauri, type SearchFilter } from '../services/tauri'
import { webData } from '../services/web'
import type { DataAPI } from '../services/dataApi'

/**
 * 数据源切换：
 * - Tauri 桌面端 → Rust + SQLite（完整可写，本地数据文件）
 * - 网页（GitHub Pages）→ 静态 JSON 基线 + localStorage 叠加层
 */
export function createDataApi(): DataAPI {
  return isTauri() ? tauriData : webData
}

export function useData() {
  const api = createDataApi()
  const items = ref<CatalogItem[]>([])
  const stats = ref<CategoryStats>({
    total: 0,
    favorites: 0,
    byCategory: { books: 0, anime: 0, novels: 0, games: 0, apps: 0, llm: 0 },
  })
  const loading = ref(false)
  const error = ref('')

  async function load(filter?: SearchFilter) {
    loading.value = true
    error.value = ''
    try {
      items.value = await api.listItems(filter)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function refresh() {
    try {
      stats.value = await api.stats()
    } catch {
      /* keep previous */
    }
  }

  return { api, mode: api.kind, items, stats, loading, error, load, refresh }
}
