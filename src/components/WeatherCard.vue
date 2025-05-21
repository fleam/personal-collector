<template>
  <div class="weather-container">
    <v-progress-linear v-if="loading" indeterminate color="primary"></v-progress-linear>

    <v-container v-else>
      <!-- 每个城市单独一行 -->
      <v-row v-for="[city, data] in Object.entries(weatherData)" :key="city" class="weather-line">
        <v-col cols="12">
          <div class="weather-text">

            {{ city }}
            {{ data.data.wendu }}°C
            <!-- {{ data.data.forecast?.[0]?.type || '未知' }} -->
            <i :class="getIconClass(data.data.forecast?.[0]?.type || '')"></i>

            最高温 {{ formatTemp(data.data.forecast?.[0]?.high) }}°C
            最低温 {{ formatTemp(data.data.forecast?.[0]?.low) }}°C
            湿度 {{ data.data.shidu }}
          </div>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Forecast {
  date: string
  high: string
  low: string
  type: string
}

interface WeatherResponseFromRust {
  status: number
  message: string
  date: string
  time: string
  city_info: {
    city: string
    citykey: string
    parent: string
    updateTime: string
  }
  data: {
    shidu: string
    pm25: number | null
    pm10: number | null
    quality: string
    wendu: string
    ganmao: string
    forecast: Forecast[]
  }
  yesterday: {
    date: string
    high: string
    low: string
    type: string
  }
}

const weatherData = ref<Record<string, WeatherResponseFromRust>>({})
const loading = ref(true)

onMounted(async () => {
  const cityCodes: Record<string, string> = {
    北京: '101010100',
    上海: '101020100',
    深圳: '101280601',
    杭州: '101210101',
    鹤壁: '101180701',
    郑州: '101180101',
  }

  const result: Record<string, WeatherResponseFromRust> = {}

  for (const [cityName, cityCode] of Object.entries(cityCodes)) {
    try {
      console.log(`开始获取${cityName}天气数据....`)
      const data = await invoke<WeatherResponseFromRust>('get_weather_for_city', { cityCode })
      console.log(`获取${cityName}天气数据成功:`, data)
      result[cityName] = data
    } catch (error) {
      console.error(`获取${cityName}天气数据失败:`, error)
      result[cityName] = {
        status: 500,
        message: '请求失败',
        date: '--',
        time: '--',
        city_info: { city: cityName, citykey: cityCode, parent: '', updateTime: '--' },
        data: {
          shidu: '--',
          pm25: null,
          pm10: null,
          quality: '--',
          wendu: '--',
          ganmao: '',
          forecast: []
        },
        yesterday: {
          date: '--',
          high: '--',
          low: '--',
          type: '--'
        }
      }
    }
  }

  weatherData.value = result
  loading.value = false
})

const formatTemp = (tempStr: string | undefined): string => {
  if (!tempStr) return '--'
  const match = tempStr.match(/[-+]?\d+(\.\d+)?/g)
  return match ? match[0] : '--'
}


const getIconClass = (type: string): string => {
  if (!type) return 'fa fa-question-circle'

  switch (type) {
    case '晴':
      return 'fa fa-sun-o'
    case '多云':
    case '阴':
      return 'fa fa-cloud'
    case '小雨':
    case '中雨':
    case '大雨':
    case '暴雨':
      return 'fa fa-tint'
    case '雪':
    case '小雪':
    case '中雪':
    case '大雪':
      return 'fa fa-snowflake-o'
    case '雾':
    case '霾':
      return 'fa fa-exclamation-triangle'
    default:
      return 'fa fa-question-circle'
  }
}

</script>

<style scoped>
.weather-container {
  font-size: 14px;
}

.weather-line {
  border-bottom: 1px solid #eee;
  padding: 8px 0;
}

.weather-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.weather-text i {
  margin-left: 4px;
  margin-right: 4px;
  color: #42b983;
  /* 可自定义颜色 */
}
</style>