import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// GitHub Pages 部署时用 `npm run build:pages`（CLI 传入 --base=/sdz-universe/）
// 桌面端与本地开发保持默认 '/'。
export default defineConfig({
  base: process.env.VITE_BASE_URL || '/',
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
  },
  build: {
    target: 'es2021',
  },
})
