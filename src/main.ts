import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify' // 引入 Vuetify 实例
// import '@/assets/styles/main.css' // 全局样式（可选）
import 'font-awesome/css/font-awesome.min.css'

createApp(App)
  .use(vuetify) // 安装 Vuetify 插件
  .mount('#app')