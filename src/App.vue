<template>
  <v-app>
    <v-main>
      <v-container class="container pa-0">
        <v-row no-gutters justify="center">
          <!-- 待办事项 -->
          <v-col cols="4" class="pa-2">
            <v-card class="card">
              <v-card-title class="card-header">待办事项</v-card-title>
              <v-card-text class="card-content">
                <TodoList />
              </v-card-text>
            </v-card>
          </v-col>

          <!-- 天气卡片 -->
          <v-col cols="4" class="pa-2">
            <v-card class="card">
              <v-card-title class="card-header">城市天气预报</v-card-title>
              <v-card-text class="card-content">
                <WeatherCard />
              </v-card-text>
            </v-card>
          </v-col>

          <!-- 示例模块卡片 -->
          <v-col cols="4" class="pa-2" v-for="module in modules" :key="module.title">
            <v-card class="card">
              <v-card-title class="card-header">{{ module.title }}</v-card-title>
              <v-card-text class="card-content">
                <p>{{ module.description }}</p>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import TodoList from './components/TodoList.vue'
import WeatherCard from './components/WeatherCard.vue'

const modules = [
  { title: '备忘录', description: '功能开发中...' },
  { title: '日历', description: '功能开发中...' },
  { title: '便签', description: '功能开发中...' },
  { title: '音乐播放器', description: '功能开发中...' },
  { title: '时钟', description: '功能开发中...' },
  { title: '笔记', description: '功能开发中...' },
  { title: '设置', description: '功能开发中...' }
]
</script>

<style scoped>
.card {
  border-radius: 2px;
  overflow: hidden;
}

.card-header {
  background-color: #f5f5f5;
}

.card-content {
  padding: 1rem;
}
</style>