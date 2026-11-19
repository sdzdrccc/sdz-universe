<script setup lang="ts">
import { computed } from 'vue'
import { CATEGORY_MAP } from '../types'
import type { CatalogItem } from '../types'

const props = defineProps<{
  item: CatalogItem
  index?: number
}>()

defineEmits<{ select: [item: CatalogItem] }>()

const accent = computed(() => CATEGORY_MAP[props.item.category]?.accent ?? '#c2543e')
const initial = computed(() => (props.item.name ?? props.item.id ?? '?').trim().charAt(0).toUpperCase())
const badge = computed(() => {
  if (typeof props.item.rating === 'number') return `★ ${props.item.rating.toFixed(1)}`
  if (props.item.year) return String(props.item.year)
  return ''
})
const metaLine = computed(() =>
  [props.item.year ? String(props.item.year) : '', props.item.meta, (props.item.tags || []).slice(0, 3).join(' / ')]
    .filter(Boolean)
    .join(' · '),
)
</script>

<template>
  <button
    type="button"
    class="poster-card"
    :style="{ '--card-accent': accent, animationDelay: `${Math.min(index ?? 0, 12) * 40}ms` }"
    @click="$emit('select', item)"
  >
    <span class="poster-card-media">
      <img v-if="item.poster" :src="item.poster" :alt="item.name" loading="lazy" decoding="async" draggable="false" />
      <span v-else class="poster-card-fallback" aria-hidden="true">{{ initial }}</span>
      <span v-if="badge" class="poster-card-badge">{{ badge }}</span>
      <span v-if="item.favorite === 1" class="poster-card-star" aria-label="已收藏">★</span>
    </span>
    <span class="poster-card-info">
      <span class="poster-card-title">{{ item.name }}</span>
      <span v-if="item.original" class="poster-card-subtitle">{{ item.original }}</span>
      <span v-if="metaLine" class="poster-card-meta">{{ metaLine }}</span>
    </span>
  </button>
</template>
