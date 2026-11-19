export type CategoryKey = 'books' | 'anime' | 'novels' | 'games' | 'apps' | 'llm'

export interface CatalogItem {
  id: string
  category: CategoryKey
  name: string
  /** 原名 / 副名 / 作者等展示名 */
  original?: string
  year?: number
  /** 开发方 / 作者 / 出版方等一行元信息 */
  meta?: string
  rating?: number | null
  tags?: string[]
  description?: string
  /** 海报图 URL；为空时卡片渲染占位封面 */
  poster?: string
  link?: string
  linkLabel?: string
  favorite?: number
  note?: string
}

export interface CategoryStats {
  total: number
  favorites: number
  /** Rust 侧 camelCase 序列化为 byCategory */
  byCategory: Record<CategoryKey, number>
}

export interface CategoryMeta {
  key: CategoryKey
  label: string
  icon: string
  accent: string
  blurb: string
}

export const CATEGORIES: CategoryMeta[] = [
  { key: 'books', label: '书籍', icon: '📚', accent: '#c2543e', blurb: '读过、收藏中、还想重读的书' },
  { key: 'anime', label: '动漫', icon: '🎌', accent: '#3e7cb2', blurb: '追过的番剧与经典' },
  { key: 'novels', label: '小说', icon: '📖', accent: '#7a5cc2', blurb: '网文与长篇连载' },
  { key: 'games', label: '游戏', icon: '🎮', accent: '#2e8b57', blurb: '主机、PC 与独立游戏' },
  { key: 'apps', label: '应用', icon: '🧰', accent: '#c2883e', blurb: '常用工具与应用收藏' },
  { key: 'llm', label: '大模型', icon: '🤖', accent: '#5c8a7e', blurb: 'LLM 与 AI 模型跟踪' },
]

export const CATEGORY_MAP: Record<CategoryKey, CategoryMeta> = Object.fromEntries(
  CATEGORIES.map((c) => [c.key, c]),
) as Record<CategoryKey, CategoryMeta>
