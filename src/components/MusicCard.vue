<template>
  <div class="music-list">
    <h2>本地音乐列表</h2>

    <ul>
      <li v-for="(music, index) in musicList" :key="index" @click="playMusic(music)">
        {{ music.title || music.name }}
      </li>
    </ul>

    <div v-if="currentMusic" class="player-controls">
      <button @click="togglePlay">{{ isPlaying ? '暂停' : '播放' }}</button>
      <input type="range" min="0" :max="duration" v-model.number="currentTime" @change="seekTo" class="progress-bar" />
      <div class="time-display">
        {{ formatTime(currentTime) }} / {{ formatTime(duration) }}
      </div>
    </div>

    <!-- 隐藏的音频播放器 -->
    <audio ref="audioPlayer" :src="currentMusic?.path" @timeupdate="onTimeUpdate"
      @loadedmetadata="onLoadedMetadata"></audio>
  </div>
</template>

<script>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export default {
  data() {
    return {
      musicList: [],
      currentMusic: null,
      isPlaying: false,
      currentTime: 0,
      duration: 0,
      audioLoading: false // 防止重复加载
    };
  },
  mounted() {
    this.loadMusicList();
  },
  methods: {
    async loadMusicList() {
      try {
        const result = await invoke('get_local_music_list');
        console.log("result", result);
        this.musicList = result;
      } catch (error) {
        console.error('加载音乐失败:', error);
      }
    },
    async playMusic(music) {
      if (this.audioLoading) return; // 如果正在加载，直接返回

      this.currentMusic = music;
      const audio = this.$refs.audioPlayer;

      this.audioLoading = true;
      audio.load(); // 加载新音频

      try {
        await audio.play();
        this.isPlaying = true;
      } catch (err) {
        if (err.name === 'AbortError') {
          console.warn('播放被中断', err);
        } else {
          console.error('播放出错:', err);
        }
      } finally {
        this.audioLoading = false;
      }
    },
    togglePlay() {
      const audio = this.$refs.audioPlayer;
      if (this.isPlaying) {
        audio.pause();
      } else {
        audio.play().catch(err => {
          if (err.name === 'AbortError') {
            console.warn('播放被中断', err);
          } else {
            console.error('播放出错:', err);
          }
        });
      }
      this.isPlaying = !this.isPlaying;
    },
    onTimeUpdate() {
      const audio = this.$refs.audioPlayer;
      this.currentTime = audio.currentTime;
    },
    onLoadedMetadata() {
      const audio = this.$refs.audioPlayer;
      this.duration = audio.duration;
    },
    seekTo() {
      const audio = this.$refs.audioPlayer;
      audio.currentTime = this.currentTime;
    },
    formatTime(seconds) {
      const mins = Math.floor(seconds / 60).toString().padStart(2, '0');
      const secs = Math.floor(seconds % 60).toString().padStart(2, '0');
      return `${mins}:${secs}`;
    }
  }
};
</script>

<style scoped>
.music-list {
  padding: 20px;
}

ul {
  list-style: none;
  padding-left: 0;
}

li {
  cursor: pointer;
  padding: 8px;
  border-bottom: 1px solid #ccc;
}

.player-controls {
  margin-top: 20px;
}

.progress-bar {
  width: 100%;
  margin: 10px 0;
}

.time-display {
  text-align: center;
  font-size: 14px;
  color: #555;
}
</style>