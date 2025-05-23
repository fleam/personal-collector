<template>
    <div class="calendar-container">
        <v-progress-linear v-if="loading" indeterminate color="primary"></v-progress-linear>

        <div v-else class="calendar">
            <!-- 当前月份 -->
            <h3>{{ currentMonth }}</h3>

            <!-- 日历表头 -->
            <div class="calendar-header">
                <div v-for="day in ['日', '一', '二', '三', '四', '五', '六']" :key="day">{{ day }}</div>
            </div>

            <!-- 日历内容 -->
            <div class="calendar-days">
                <!-- 补全上个月的空白 -->
                <div v-for="n in firstDayOfMonth" :key="'empty' + n" class="day empty"></div>

                <!-- 生成当月的天数 -->
                <div v-for="day in daysInMonth" :key="day" :class="{
                    'day': true,
                    'today': isToday(day),
                    'holiday': isHoliday(day),
                    'has-event': hasEvent(day)
                }" @click="selectDate(day)">
                    {{ day }}

                    <!-- 天气图标 -->
                    <div v-if="getForecastForDate(day)" class="weather-icon">
                        <i :class="getIconClass(getForecastForDate(day)?.type || '')"></i>
                    </div>

                    <!-- 待办事项数量 -->
                    <div v-if="events[formatDate(currentYear, currentMonthNumber, day)]" class="event-count">
                        {{ events[formatDate(currentYear, currentMonthNumber, day)].length }}
                    </div>
                </div>
            </div>

            <!-- 显示所选日期详情 -->
            <div v-if="selectedDate" class="date-details">
                <h4>选定日期: {{ selectedDate }}</h4>

                <p v-if="weatherData[selectedDate]">
                    温度: {{ weatherData[selectedDate].data.wendu }}°C<br />
                    天气: {{ weatherData[selectedDate].data.type }}
                </p>
                <!-- <p v-else>加载天气中...</p> -->

                <ul class="event-list">
                    <li v-for="event in events[selectedDate]" :key="event.id" class="event-item">
                        <input type="checkbox" :checked="event.is_done" @change="toggleEventStatus(event)" />
                        <span :class="{ done: event.is_done }">{{ event.title }}</span>
                        <button @click="removeEvent(event.id)">删除</button>
                    </li>
                </ul>

                <div class="event-form">
                    <input style="width: 83%;height: 20px;" v-model="newEventTitle" placeholder="输入待办事项" />
                    <button @click="addEvent">添加</button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// === 接口定义 ===
interface Forecast {
    date: string
    high: string
    low: string
    ymd: string
    type: string
    notice?: string
}

interface WeatherResponseFromRust {
    status: number
    message: string
    data: {
        wendu: string
        type: string
        forecast: Forecast[]
    }
}

interface Event {
    id: number
    date: string
    title: string
    description?: string
    is_done: boolean
}

// === 响应式数据 ===
const loading = ref(true)
const currentYear = ref(new Date().getFullYear())
const currentMonthNumber = ref(new Date().getMonth() + 1)
const currentMonth = ref(getMonthName(new Date().getMonth()))
const daysInMonth = ref(getDaysInMonth(currentYear.value, currentMonthNumber.value))
const firstDayOfMonth = ref(getFirstDayOfMonth(currentYear.value, currentMonthNumber.value))

const holidays = ref<Record<string, string>>({})
const weatherData = ref<Record<string, WeatherResponseFromRust>>({})
const events = ref<Record<string, Event[]>>({})
const selectedDate = ref<string | null>(null)
const newEventTitle = ref('')

// === 方法定义 ===
function getMonthName(monthIndex: number): string {
    const months = [
        '一月',
        '二月',
        '三月',
        '四月',
        '五月',
        '六月',
        '七月',
        '八月',
        '九月',
        '十月',
        '十一月',
        '十二月'
    ]
    return months[monthIndex]
}

function getDaysInMonth(year: number, month: number): number {
    return new Date(year, month, 0).getDate()
}

function getFirstDayOfMonth(year: number, month: number): number {
    return new Date(year, month - 1, 1).getDay()
}

function isToday(day: number): boolean {
    const today = new Date()
    return (
        today.getDate() === day &&
        today.getMonth() + 1 === currentMonthNumber.value &&
        today.getFullYear() === currentYear.value
    )
}

function isHoliday(day: number): boolean {
    const dateStr = formatDate(currentYear.value, currentMonthNumber.value, day)
    return !!holidays.value[dateStr]
}

function hasEvent(day: number): boolean {
    const dateStr = formatDate(currentYear.value, currentMonthNumber.value, day)
    return Array.isArray(events.value[dateStr]) && events.value[dateStr].length > 0
}

function formatDate(year: number, month: number, day: number): string {
    return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`
}

// ========== 🚀 优化：一次性加载整月天气和事件 ==========

async function loadAllWeather(): Promise<void> {
    try {
        const data = await invoke<WeatherResponseFromRust>('get_weather_for_city', {
            cityCode: '101020100' // 上海城市代码
        })

        console.log("data.data.forecast=====>", data.data.forecast);
        // 直接缓存所有天气数据到对应的 ymd 字段
        data.data.forecast.forEach(forecast => {
            console.log("forecast.ymd=====>", forecast.ymd);
            weatherData.value[forecast.ymd] = {
                status: data.status,
                message: data.message,
                data: {
                    wendu: forecast.date,
                    type: forecast.type,
                    forecast: [forecast] // 当前日期的天气信息作为数组
                }
            }
        })
        console.log("weatherData=====>", weatherData.value);

    } catch (error) {
        console.error('获取天气失败:', error)
    }
}

async function loadAllEvents(): Promise<void> {
    try {
        const year = currentYear.value
        const month = String(currentMonthNumber.value).padStart(2, '0')

        // 遍历这个月的所有日期，依次调用 get_events(date)
        for (let day = 1; day <= daysInMonth.value; day++) {
            const dateStr = `${year}-${month}-${String(day).padStart(2, '0')}`
            const eventList = await invoke<Event[]>('get_events', { date: dateStr })

            if (eventList.length > 0) {
                events.value[dateStr] = eventList
            }
        }
    } catch (error) {
        console.error('获取事件失败:', error)
    }
}

// ========== 📅 点击选择日期 ==========
function getForecastForDate(day: number): Forecast | undefined {
    const dateStr = formatDate(currentYear.value, currentMonthNumber.value, day)
    const forecastList = weatherData.value[dateStr]?.data?.forecast
    if (!forecastList) return undefined
    return forecastList.find(f => f.ymd === dateStr)
}

function selectDate(day: number): void {
    const dateStr = formatDate(currentYear.value, currentMonthNumber.value, day)
    selectedDate.value = dateStr
}

// ========== 📝 待办事项操作 ==========
async function addEvent(): Promise<void> {
    if (!selectedDate.value || !newEventTitle.value.trim()) return

    await invoke('add_event', {
        title: newEventTitle.value,
        description: null,
        date: selectedDate.value
    })
    newEventTitle.value = ''
    await loadSingleEvent(selectedDate.value)
}

async function loadSingleEvent(dateStr: string): Promise<void> {
    const eventList = await invoke<Event[]>('get_events', { date: dateStr })
    events.value[dateStr] = eventList
}

async function toggleEventStatus(event: Event): Promise<void> {
    await invoke('update_event_status', {
        id: event.id,
        is_done: !event.is_done
    })
    event.is_done = !event.is_done
}

async function removeEvent(id: number): Promise<void> {
    await invoke('delete_event', { id })
    if (selectedDate.value && events.value[selectedDate.value]) {
        events.value[selectedDate.value] = events.value[selectedDate.value].filter(e => e.id !== id)
    }
}

function getIconClass(type: string): string {
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

onMounted(async () => {
    try {
        const currentMonthStr = `${currentYear.value}-${String(currentMonthNumber.value).padStart(2, '0')}`
        const holidayData = await invoke<[string, string][]>('get_holidays', { month: currentMonthStr })
        // 将二维数组转换为对象
        const holidayMap: Record<string, string> = {}
        for (const [date, name] of holidayData) {
            holidayMap[date] = name
        }
        holidays.value = holidayMap

        await loadAllWeather()
        await loadAllEvents()

        loading.value = false
    } catch (error) {
        console.error('初始化日历失败:', error)
        loading.value = false
    }
})
</script>

<style scoped>
.calendar-container {
    font-family: Arial, sans-serif;
    padding: 16px;
}

.calendar {
    max-width: 100%;
    margin: auto;
    width: 100%;
}

.calendar-header,
.calendar-days {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(40px, 1fr));
    /* gap: 8px; */
    text-align: center;
}

.day {
    padding: 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.3s ease;
}

.day.today {
    background-color: #e0f7fa;
}

.day.holiday {
    color: red;
    font-weight: bold;
}

.day.has-event {
    background-color: #fffde7;
}

.weather-icon i {
    margin-top: 4px;
    color: #42b983;
}

.event-count {
    font-size: 12px;
    color: #333;
}

.date-details {
    margin-top: 16px;
    padding: 12px;
    background-color: #f9f9f9;
    border-radius: 4px;
}

/* 移动端优化 */
@media (max-width: 600px) {

    .calendar-header,
    .calendar-days {
        grid-template-columns: repeat(auto-fit, minmax(32px, 1fr));
        gap: 4px;
    }

    .day {
        padding: 6px;
        font-size: 12px;
    }

    .event-count {
        font-size: 10px;
    }
}
</style>