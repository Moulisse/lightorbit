import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { VueQueryPlugin } from '@tanstack/vue-query'

const app = createApp(App)

app.use(VueQueryPlugin)
app.use(createPinia())

app.mount('#app')
