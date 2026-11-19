<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { CATEGORIES } from './types'

const route = useRoute()
const catKey = computed(() => (route.path === '/' ? '' : (String(route.path).slice(1) as string)))
const activeLabel = computed(() =>
  CATEGORIES.find((c) => c.key === catKey.value)?.label ?? '总览',
)
</script>

<template>
  <div class="app">
    <header class="app-header">
      <div class="wrap">
        <RouterLink to="/" class="brand">
          <span class="brand-mark">SDZ</span>
          <span class="brand-name">SDZ Universe</span>
          <span class="brand-handle">/ sdz-universe</span>
        </RouterLink>
        <nav class="app-nav" aria-label="主导航">
          <RouterLink to="/" :class="{ active: catKey === '' }">总览</RouterLink>
          <RouterLink
            v-for="c in CATEGORIES"
            :key="c.key"
            :to="`/${c.key}`"
            :class="{ active: catKey === c.key }"
          >
            <span class="nav-icon" aria-hidden="true">{{ c.icon }}</span>{{ c.label }}
          </RouterLink>
        </nav>
      </div>
    </header>

    <main class="app-main">
      <div class="wrap">
        <p class="page-subtitle">{{ activeLabel }}</p>
        <RouterView />
      </div>
    </main>

    <footer class="app-footer">
      <div class="wrap">
        <span>SDZ Universe · Tauri + Vue 3 + Rust + SQLite</span>
        <span class="footer-sep">·</span>
        <span>桌面版：数据存本地 SQLite；网页版：静态 JSON 基线 + 浏览器本地收藏</span>
      </div>
    </footer>
  </div>
</template>
