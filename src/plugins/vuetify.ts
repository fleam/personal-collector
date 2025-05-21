import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg' // 引入 Material Design 图标

export default createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi', // 默认图标集
    aliases,
    sets: {
      mdi,
    },
  },
  
  theme: {
    defaultTheme: 'light', // 主题模式（light/dark）
    themes: {
      light: {
        colors: {
          primary: '#1976D2', // 主色（蓝色）
          secondary: '#FF4081', // 副色（粉色）
          background: '#FFFFFF', // 背景色
        },
      },
      dark: {
        colors: {
          primary: '#2196F3',
          secondary: '#F50057',
          background: '#121212',
        },
      },
    },
  },


})