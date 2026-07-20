import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/pages/Home.vue'
import AutoMessage from '@/pages/AutoMessage.vue'
import Settings from '@/pages/Settings.vue'
import DataAnalysis from '@/pages/DataAnalysis.vue'
import LogFiles from '@/pages/LogFiles.vue'
import LogViewer from '@/pages/LogViewer.vue'

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
      path: '/data-analysis',
      component: DataAnalysis
    },
    {
      path: '/log-files',
      component: LogFiles
    },
    {
      path: '/log-viewer',
      component: LogViewer
    },
    {
      path: '/settings',
      component: Settings
    }
  ]
})

export default router
