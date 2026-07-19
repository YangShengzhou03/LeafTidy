import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/pages/Home.vue'
import AutoMessage from '@/pages/AutoMessage.vue'
import Settings from '@/pages/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Home
    },
    {
      path: '/auto-message',
      component: AutoMessage
    },
    {
      path: '/settings',
      component: Settings
    }
  ]
})

export default router
