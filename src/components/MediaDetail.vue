<script setup lang="ts">
import { computed } from 'vue'
import { CATEGORY_MAP } from '../types'
import type { CatalogItem } from '../types'

const props = defineProps<{ item: CatalogItem | null }>()
const emit = defineEmits<{
  close: []
  favorite: [id: string, favorite: boolean]
}>()

const cat = computed(() =>
  props.item ? CATEGORY_MAP[props.item.category] : undefined,
)
const metaLine = computed(() => {
  if (!props.item) return ''
  const i = props.item
  return [i.year ? String(i.year) : '', i.meta, (i.tags || []).join(' / ')]
    .filter(Boolean)
    .join(' · ')
})
</script>

<template>
  <Teleport to="body">
    <div v-if="item" class="modal-backdrop" @click.self="emit('close')" @keydown.escape="emit('close')">
      <div class="modal-panel" role="dialog" aria-modal="true">
        <div class="modal-head">
          <span class="modal-title">{{ cat?.label }} · {{ item.name }}</span>
          <button class="modal-close" @click="emit('close')" aria-label="关闭">✕</button>
        </div>
        <div class="media-detail">
          <img
            v-if="item.poster"
            class="media-detail-poster"
            :src="item.poster"
            :alt="item.name"
          />
          <div v-else
            class="media-detail-poster"
            :style="{ background: `color-mix(in srgb, ${cat?.accent ?? '#c2543e'} 14%, #f5efe4)`, display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '72px', fontWeight: 800, color: `color-mix(in srgb, ${cat?.accent ?? '#c2543e'} 60%, #fff)` }">
            {{ item.name.charAt(0).toUpperCase() }}
          </div>
          <div class="media-detail-info">
            <p class="media-detail-meta">{{ metaLine }}</p>
            <p v-if="item.description" class="media-detail-desc">{{ item.description }}</p>
            <div v-if="item.tags?.length" class="media-detail-tags">
              <span v-for="t in item.tags" :key="t" class="media-detail-tag">{{ t }}</span>
            </div>
            <p v-if="item.note" class="media-detail-desc" style="font-size: 13px; color: var(--ink-faint)">备注：{{ item.note }}</p>
            <div class="detail-actions">
              <button
                class="toolbar-btn"
                :class="{ 'is-on': item.favorite === 1 }"
                @click="emit('favorite', item.id, item.favorite !== 1)"
              >
                {{ item.favorite === 1 ? '★ 已收藏' : '☆ 收藏' }}
              </button>
              <a
                v-if="item.link"
                class="media-detail-link"
                :href="item.link"
                target="_blank"
                rel="noopener"
              >
                {{ item.linkLabel || '了解更多 ↗' }}
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.toolbar-btn.is-on {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}
</style>
