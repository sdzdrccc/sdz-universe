<script setup lang="ts">
import { ref } from 'vue'
import PosterCard from './PosterCard.vue'
import MediaDetail from './MediaDetail.vue'
import type { CatalogItem } from '../types'

defineProps<{ items: CatalogItem[] }>()

const emit = defineEmits<{
  favorite: [id: string, favorite: boolean]
}>()

const selected = ref<CatalogItem | null>(null)
</script>

<template>
  <div class="poster-wall">
    <div class="poster-wall-grid">
      <template v-for="(item, i) in items" :key="item.id">
        <PosterCard :item="item" :index="i" @select="(it) => (selected = it)" />
      </template>
    </div>
    <MediaDetail
      :item="selected"
      @close="() => (selected = null)"
      @favorite="(id, fav) => emit('favorite', id, fav)"
    />
  </div>
</template>
