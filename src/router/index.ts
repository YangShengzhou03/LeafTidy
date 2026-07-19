import { createRouter, createWebHistory } from 'vue-router'
import TaskManagement from '@/pages/TaskManagement.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: TaskManagement
    }
  ]
})

export default router
