<template>
  <v-app>
    <v-main>
      <v-container class="container pa-0">
        <!-- 使用自定义瀑布流容器 -->
        <div class="masonry-grid">
          <!-- 待办事项 -->
          <div class="masonry-item">
            <v-card class="card">
              <v-card-title class="card-header">待办事项</v-card-title>
              <v-card-text class="card-content">
                <TodoList />
              </v-card-text>
            </v-card>
          </div>

          <!-- 天气卡片 -->
          <div class="masonry-item">
            <v-card class="card">
              <v-card-title class="card-header">城市天气预报</v-card-title>
              <v-card-text class="card-content">
                <WeatherCard />
              </v-card-text>
            </v-card>
          </div>

          <!-- 本地音乐 -->
          <div class="masonry-item">
            <v-card class="card">
              <v-card-title class="card-header">本地音乐</v-card-title>
              <v-card-text class="card-content">
                <MusicCard />
              </v-card-text>
            </v-card>
          </div>

          <!-- 日历 -->
          <div class="masonry-item">
            <v-card class="card">
              <v-card-title class="card-header">日历</v-card-title>
              <v-card-text class="card-content">
                <CalendarCard />
              </v-card-text>
            </v-card>
          </div>
        </div>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import TodoList from './components/TodoList.vue'
import WeatherCard from './components/WeatherCard.vue'
import MusicCard from './components/MusicCard.vue'
import CalendarCard from './components/CalendarCard.vue'
</script>

<style scoped>
.container {
  max-width: 1200px;
  margin: auto;
}

.masonry-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.masonry-item {
  break-inside: avoid;
  page-break-inside: avoid;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  transition: transform 0.2s ease;
}

.masonry-item:hover {
  transform: translateY(-4px);
}

.card {
  border-radius: 2px;
  overflow: hidden;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.card-header {
  background-color: #f5f5f5;
  font-size: 1rem;
  padding: 12px 16px;
}

.card-content {
  padding: 1rem;
  flex-grow: 1;
}
</style>