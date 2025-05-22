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

          <!-- 本地音乐 -->
          <v-col cols="4" class="pa-2">
            <v-card class="card">
              <v-card-title class="card-header">本地音乐</v-card-title>
              <v-card-text class="card-content">
                <MusicCard />
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
import MusicCard from './components/MusicCard.vue'
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