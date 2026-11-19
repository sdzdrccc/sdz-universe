import { createRouter, createWebHashHistory } from 'vue-router'

// 网页版（GitHub Pages）用 hash 路由，无需服务端 rewrite，/sdz-universe/ 下即可直接部署。
const routes = [
  { path: '/', name: 'home', component: () => import('../views/HomeView.vue'), meta: { title: '总览' } },
  { path: '/:category', name: 'category', component: () => import('../views/CategoryView.vue'),
    props: true, meta: { title: '收藏' } },
]

export default createRouter({
  history: createWebHashHistory(),
  routes,
})
