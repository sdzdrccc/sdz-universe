import type { CatalogItem, CategoryKey, CategoryStats } from '../types'
import type { SearchFilter } from './tauri'
import type { DataAPI } from './dataApi'
import { seedItems } from '../data/seed'

/**
 * 网页版数据源（GitHub Pages 只读场景）：
 * - 基线数据：远端静态 JSON（部署产物内 data/catalog.json 或 CDN/自定义 URL）
 * - 用户自定义（新增/删除/收藏/备注）：localStorage 叠加层
 * 无远端 URL 时退化为「内置种子 + localStorage」。
 */

const LS_KEY = 'sdz-universe:overrides:v1'
// 部署站点前缀：dev 为 '/'，GitHub Pages 构建为 '/sdz-universe/'（由 vite --base 决定）
const BASE = (import.meta as any).env?.BASE_URL || '/'
// 可选的自定义远端数据源（默认用站点相对路径，无需配置）
const REMOTE_BASE = (import.meta as any).env?.VITE_DATA_BASE_URL as string | undefined

export const webData: DataAPI = {
  kind: 'web',
  editable: true,
  async listItems(filter) {
    const items = await fetchAllItems()
    return applyFilter(items, filter)
  },
  async stats() {
    const items = await fetchAllItems()
    const byCategory: Record<CategoryKey, number> = {
      books: 0, anime: 0, novels: 0, games: 0, apps: 0, llm: 0,
    }
    for (const it of items) byCategory[it.category] = (byCategory[it.category] || 0) + 1
    return {
      total: items.length,
      favorites: items.filter((i) => i.favorite === 1).length,
      byCategory,
    }
  },
  async allTags(category) {
    const items = (await fetchAllItems()).filter((i) => !category || i.category === category)
    const set = new Set<string>()
    for (const it of items) for (const t of it.tags || []) set.add(t)
    return [...set].sort()
  },
  async toggleFavorite(id, favorite) {
    const overrides = readOverrides()
    overrides.favorite[id] = favorite ? 1 : 0
    writeOverrides(overrides)
  },
  async upsertItem(item) {
    const overrides = readOverrides()
    overrides.custom[item.id] = { ...item }
    overrides.favorite[item.id] = item.favorite ? 1 : 0
    if (item.note) overrides.note[item.id] = item.note
    writeOverrides(overrides)
  },
  async deleteItem(id) {
    const overrides = readOverrides()
    overrides.removed.push(id)
    delete overrides.custom[id]
    writeOverrides(overrides)
  },
  async exportJson() {
    return JSON.stringify(await fetchAllItems(), null, 2)
  },
  async importJson(payload) {
    const parsed = JSON.parse(payload) as CatalogItem[]
    if (!Array.isArray(parsed)) throw new Error('Invalid catalog JSON')
    const overrides = readOverrides()
    for (const item of parsed) {
      if (!item?.id || !item?.name) continue
      overrides.custom[item.id] = { ...item, favorite: item.favorite ? 1 : 0 }
      overrides.removed = overrides.removed.filter((r) => r !== item.id)
    }
    writeOverrides(overrides)
    return parsed.length
  },
}

interface Overrides {
  favorite: Record<string, number>
  note: Record<string, string>
  custom: Record<string, CatalogItem>
  removed: string[]
}

function readOverrides(): Overrides {
  try {
    const raw = localStorage.getItem(LS_KEY)
    if (raw) return { favorite: {}, note: {}, custom: {}, removed: [], ...JSON.parse(raw) }
  } catch {
    /* corrupted storage: start clean */
  }
  return { favorite: {}, note: {}, custom: {}, removed: [] }
}

function writeOverrides(o: Overrides) {
  try {
    localStorage.setItem(LS_KEY, JSON.stringify(o))
  } catch {
    /* storage full / unavailable: silently ignore in web mode */
  }
}

let cached: CatalogItem[] | null = null

async function fetchAllItems(): Promise<CatalogItem[]> {
  if (cached) return cached
  let base: CatalogItem[] = []
  try {
    // 优先自定义远端源；否则按站点部署前缀取相对路径（dev=/data，Pages=/sdz-universe/data）
    const url = REMOTE_BASE
      ? `${REMOTE_BASE.replace(/\/$/, '')}/data/catalog.json`
      : `${BASE.replace(/\/$/, '')}/data/catalog.json`
    const res = await fetch(url, { cache: 'no-cache' })
    if (res.ok) {
      const json = (await res.json()) as CatalogItem[]
      if (Array.isArray(json)) base = json
    }
  } catch {
    /* remote unavailable: fall back to inlined seed */
  }
  if (base.length === 0) base = seedItems()
  const o = readOverrides()
  const merged: CatalogItem[] = base
    .filter((i) => !o.removed.includes(i.id))
    .map((i) => {
      const custom = o.custom[i.id]
      const item: CatalogItem = custom ? { ...i, ...custom } : { ...i }
      if (o.favorite[i.id] !== undefined) item.favorite = o.favorite[i.id]
      if (o.note[i.id] !== undefined) item.note = o.note[i.id]
      return item
    })
  // 用户新增的条目（基线里不存在）
  for (const id of Object.keys(o.custom)) {
    if (!base.some((i) => i.id === id) && !o.removed.includes(id)) {
      merged.push({ ...o.custom[id], favorite: o.favorite[id] || 0 })
    }
  }
  cached = merged
  return merged
}

function applyFilter(items: CatalogItem[], filter?: SearchFilter): CatalogItem[] {
  if (!filter) return items
  const kw = filter.keyword?.trim().toLowerCase()
  return items.filter((i) => {
    if (filter.category && i.category !== filter.category) return false
    if (filter.tag && !(i.tags || []).includes(filter.tag)) return false
    if (filter.favoritesOnly && i.favorite !== 1) return false
    if (kw) {
      const hay = [i.name, i.original, i.meta, i.description, ...(i.tags || [])]
        .filter(Boolean)
        .join(' ')
        .toLowerCase()
      if (!hay.includes(kw)) return false
    }
    return true
  })
}

