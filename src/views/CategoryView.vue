<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { CATEGORIES, CATEGORY_MAP } from '../types'
import type { CategoryKey } from '../types'
import { useData } from '../composables/useData'
import PosterWall from '../components/PosterWall.vue'

const route = useRoute()
const router = useRouter()
const key = computed<CategoryKey>(() => {
  const k = String(route.params.category || '') as CategoryKey
  return CATEGORIES.some((c) => c.key === k) ? k : 'games'
})
const meta = computed(() => CATEGORY_MAP[key.value])

const { api, items, loading, error, mode, load, refresh } = useData()
const keyword = ref('')
const tagFilter = ref('')
const favOnly = ref(false)
const tags = ref<string[]>([])

watch(key, () => {
  keyword.value = ''
  tagFilter.value = ''
  favOnly.value = false
  reload()
})

async function reload() {
  tags.value = await api.allTags(key.value)
  await load({ keyword: keyword.value, category: key.value, tag: tagFilter.value || undefined, favoritesOnly: favOnly.value })
  await refresh()
}

onMounted(reload)

function applySearch() {
  reload()
}

function toggleTag(t: string) {
  tagFilter.value = tagFilter.value === t ? '' : t
  reload()
}

function toggleFav() {
  favOnly.value = !favOnly.value
  reload()
}

function exportData() {
  api.exportJson().then((text) => {
    const blob = new Blob([text], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'sdz-universe-export.json'
    a.click()
    URL.revokeObjectURL(url)
  })
}

function importData(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    api.importJson(String(reader.result)).then(() => reload())
  }
  reader.readAsText(file)
}

async function onFavorite(id: string, favorite: boolean) {
  await api.toggleFavorite(id, favorite)
  await reload()
}

// 非法路径（如 /xyz）→ 默认 games
watch(key, (k) => {
  if (!CATEGORIES.some((c) => c.key === k)) {
    router.replace('/games')
  }
}, { immediate: true })
</script>

<template>
  <div>
    <div class="cat-toolbar">
      <h1 class="toolbar-title" :style="{ color: meta.accent }">{{ meta.icon }} {{ meta.label }}</h1>
      <span class="toolbar-count">{{ items.length }} 条</span>
      <span class="grow"></span>
      <input
        v-model="keyword"
        class="toolbar-input"
        :placeholder="`搜索${meta.label}…`"
        @keyup.enter="applySearch"
      />
      <button class="toolbar-btn" :class="{ 'is-on': favOnly }" @click="toggleFav">★ 只看收藏</button>
      <template v-if="mode === 'tauri'">
        <button class="toolbar-btn" @click="exportData">导出 JSON</button>
        <label class="toolbar-btn" style="display: inline-flex; align-items: center">
          导入 JSON
          <input type="file" accept=".json,application/json" hidden @change="importData" />
        </label>
      </template>
      <span class="mode-badge">{{ mode === 'tauri' ? '桌面 · SQLite' : '网页 · JSON + 本地' }}</span>
    </div>

    <div v-if="tags.length" class="tag-chips">
      <button class="chip" :class="{ active: tagFilter === '' }" @click="toggleTag(tagFilter)">全部</button>
      <button
        v-for="t in tags"
        :key="t"
        class="chip"
        :class="{ active: tagFilter === t }"
        @click="toggleTag(t)"
      >{{ t }}</button>
    </div>

    <p v-if="error" class="empty-state">加载失败：{{ error }}</p>
    <p v-else-if="loading" class="empty-state">加载中…</p>
    <PosterWall v-else :items="items" @favorite="onFavorite" />
    <p v-if="!loading && !items.length && !error" class="empty-state">该分类暂无条目</p>
  </div>
</template>

<style scoped>
.toolbar-btn.is-on {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}
</style>
