<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { CATEGORIES } from '../types'
import type { CatalogItem } from '../types'
import { useData } from '../composables/useData'
import PosterCard from '../components/PosterCard.vue'
import MediaDetail from '../components/MediaDetail.vue'

const { api, stats, mode } = useData()
const selected = ref<CatalogItem | null>(null)
const latest = ref<CatalogItem[]>([])

onMounted(async () => {
  latest.value = (await api.listItems()).slice(0, 8)
  Object.assign(stats.value, await api.stats())
})

const topByCategory = () =>
  CATEGORIES.map((c) => ({ ...c, count: stats.value.byCategory[c.key] ?? 0 }))

async function onFavorite(id: string, favorite: boolean) {
  await api.toggleFavorite(id, favorite)
  selected.value = null
  const s = await api.stats()
  Object.assign(stats.value, s)
  latest.value = (await api.listItems()).slice(0, 8)
}
</script>

<template>
  <div>
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-num">{{ stats.total }}</div>
        <div class="stat-label">收录条目</div>
      </div>
      <div class="stat-card">
        <div class="stat-num">{{ stats.favorites }}</div>
        <div class="stat-label">已收藏 ★</div>
      </div>
      <div class="stat-card">
        <div class="stat-num">{{ CATEGORIES.length }}</div>
        <div class="stat-label">分类</div>
      </div>
      <div class="stat-card">
        <div class="stat-num" style="font-size: 15px; padding-top: 7px">
          {{ mode === 'tauri' ? '桌面版 · 本地 SQLite' : '网页版 · JSON + 本地收藏' }}
        </div>
        <div class="stat-label">当前数据源</div>
      </div>
    </div>

    <div class="section-block">
      <div class="section-head">
        <h2>分类总览</h2>
        <span class="hint">点击进入各分类卡片墙</span>
      </div>
      <div class="stats-grid" style="margin-bottom: 0">
        <RouterLink
          v-for="c in topByCategory()"
          :key="c.key"
          :to="`/${c.key}`"
          class="stat-card cat-link"
        >
          <div class="stat-num" :style="{ color: c.accent }">{{ c.count }}</div>
          <div class="stat-label">{{ c.icon }} {{ c.label }}</div>
          <div class="stat-label" style="margin-top: 6px">{{ c.blurb }}</div>
        </RouterLink>
      </div>
    </div>

    <div class="section-block">
      <div class="section-head">
        <h2>最近收录</h2>
        <span class="hint">点击查看详情</span>
      </div>
      <div v-if="latest.length" class="poster-wall-grid">
        <PosterCard
          v-for="(it, i) in latest"
          :key="it.id"
          :item="it"
          :index="i"
          @select="(x) => (selected = x)"
        />
      </div>
      <p v-else class="empty-state">暂无条目</p>
    </div>

    <MediaDetail :item="selected" @close="() => (selected = null)" @favorite="onFavorite" />
  </div>
</template>
