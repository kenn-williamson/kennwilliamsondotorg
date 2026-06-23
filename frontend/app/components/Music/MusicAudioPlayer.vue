<template>
  <div class="audio-player" :class="{ 'audio-player--disabled': !src }">
    <button
      type="button"
      class="play-btn"
      :disabled="!src"
      :aria-label="isPlaying ? `Pause ${title}` : `Play ${title}`"
      @click="togglePlay"
    >
      <svg v-if="!isPlaying" class="play-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
        <path d="M8 5v14l11-7z" />
      </svg>
      <svg v-else class="play-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
        <path d="M6 5h4v14H6zM14 5h4v14h-4z" />
      </svg>
    </button>

    <div class="player-body">
      <div class="player-meta">
        <span class="player-time">{{ formatTime(currentTime) }}</span>
        <span class="player-time player-time--total">{{ formatTime(duration) }}</span>
      </div>
      <input
        type="range"
        class="seek"
        :class="{ 'seek--disabled': !src }"
        min="0"
        :max="duration || 0"
        step="0.1"
        :value="currentTime"
        :disabled="!src"
        :style="{ '--progress': `${progressPercent}%` }"
        aria-label="Seek"
        @input="onSeek"
      />
      <p v-if="!src" class="player-unavailable">Audio coming soon</p>
    </div>

    <button
      v-if="src"
      type="button"
      class="download-btn"
      :disabled="downloading"
      :aria-label="`Download ${title}`"
      :title="`Download ${title}`"
      @click="downloadTrack"
    >
      <svg v-if="!downloading" class="download-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2M7 10l5 5 5-5M12 15V3" />
      </svg>
      <svg v-else class="download-icon download-spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" aria-hidden="true">
        <circle cx="12" cy="12" r="9" stroke-width="2" stroke-dasharray="42" stroke-linecap="round" />
      </svg>
    </button>

    <audio
      v-if="src"
      ref="audioRef"
      :src="src"
      preload="metadata"
      @loadedmetadata="onLoaded"
      @timeupdate="onTimeUpdate"
      @play="onPlay"
      @pause="onPause"
      @ended="onEnded"
    />
  </div>
</template>

<script setup lang="ts">
import { useNowPlaying } from '~/composables/useNowPlaying'

const props = defineProps<{
  id: string
  src: string | null
  title: string
}>()

const audioRef = ref<HTMLAudioElement | null>(null)
const isPlaying = ref(false)
const currentTime = ref(0)
const duration = ref(0)

const { currentId, setCurrent } = useNowPlaying()

const progressPercent = computed(() => {
  if (!duration.value) return 0
  return Math.min(100, (currentTime.value / duration.value) * 100)
})

const togglePlay = () => {
  const el = audioRef.value
  if (!el) return
  if (el.paused) {
    setCurrent(props.id)
    void el.play()
  } else {
    el.pause()
  }
}

const downloading = ref(false)

const downloadTrack = async () => {
  const src = props.src
  if (!src || downloading.value) return
  downloading.value = true
  try {
    // Fetch (allowed by the bucket CORS) so we can save with a clean filename
    // instead of opening the raw S3 object inline.
    const res = await fetch(src)
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    const blob = await res.blob()
    const objectUrl = URL.createObjectURL(blob)
    const clean = src.split('?')[0] ?? src
    const ext = clean.split('.').pop() || 'mp3'
    const a = document.createElement('a')
    a.href = objectUrl
    a.download = `${props.title}.${ext}`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(objectUrl)
  } catch (err) {
    // Fallback: open the file so the user can still save it manually
    console.error('Download failed, opening in a new tab:', err)
    window.open(src, '_blank', 'noopener')
  } finally {
    downloading.value = false
  }
}

const onSeek = (e: Event) => {
  const el = audioRef.value
  if (!el) return
  const value = parseFloat((e.target as HTMLInputElement).value)
  el.currentTime = value
  currentTime.value = value
}

const onLoaded = () => {
  duration.value = audioRef.value?.duration || 0
}
const onTimeUpdate = () => {
  currentTime.value = audioRef.value?.currentTime || 0
}
const onPlay = () => {
  isPlaying.value = true
}
const onPause = () => {
  isPlaying.value = false
}
const onEnded = () => {
  isPlaying.value = false
  currentTime.value = 0
}

// Pause this player when another track starts playing
watch(currentId, (id) => {
  if (id !== props.id && audioRef.value && !audioRef.value.paused) {
    audioRef.value.pause()
  }
})

const formatTime = (seconds: number): string => {
  if (!seconds || !isFinite(seconds)) return '0:00'
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}
</script>

<style scoped>
.audio-player {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: linear-gradient(135deg, #1e293b, #0f172a);
  border: 1px solid rgba(148, 163, 184, 0.3);
  border-radius: 0.5rem;
  box-shadow: inset 0 1px 0 rgba(148, 163, 184, 0.1);
}

.play-btn {
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 3rem;
  height: 3rem;
  border-radius: 9999px;
  border: 2px solid rgba(103, 232, 249, 0.4);
  background: radial-gradient(circle at 30% 30%, #475569, #1e293b);
  color: #67e8f9; /* accent-300 */
  transition: all 0.2s ease;
}

.play-btn:hover:not(:disabled) {
  color: #a5f3fc;
  border-color: rgba(103, 232, 249, 0.8);
  box-shadow: 0 0 12px rgba(103, 232, 249, 0.4);
  transform: scale(1.05);
}

.play-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.play-icon {
  width: 1.5rem;
  height: 1.5rem;
}

.download-btn {
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.25rem;
  height: 2.25rem;
  border-radius: 9999px;
  border: 1px solid rgba(148, 163, 184, 0.4);
  background: transparent;
  color: #94a3b8;
  transition: all 0.2s ease;
}

.download-btn:hover:not(:disabled) {
  color: #67e8f9;
  border-color: rgba(103, 232, 249, 0.6);
  background: rgba(103, 232, 249, 0.08);
}

.download-btn:disabled {
  opacity: 0.6;
  cursor: progress;
}

.download-icon {
  width: 1.1rem;
  height: 1.1rem;
}

.download-spinner {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.player-body {
  flex: 1;
  min-width: 0;
}

.player-meta {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  font-variant-numeric: tabular-nums;
  color: #94a3b8; /* nautical-400 */
  margin-bottom: 0.25rem;
}

.player-time--total {
  color: #cbd5e1; /* nautical-300 */
}

/* Seek bar with brass/cyan fill */
.seek {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 6px;
  border-radius: 9999px;
  background: linear-gradient(
    to right,
    #67e8f9 0%,
    #22d3ee var(--progress, 0%),
    #334155 var(--progress, 0%),
    #334155 100%
  );
  cursor: pointer;
  outline: none;
}

.seek::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 9999px;
  background: #f59e0b; /* gold-500 - brass knob */
  border: 2px solid #fef3c7;
  box-shadow: 0 0 6px rgba(245, 158, 11, 0.6);
}

.seek::-moz-range-thumb {
  width: 14px;
  height: 14px;
  border-radius: 9999px;
  background: #f59e0b;
  border: 2px solid #fef3c7;
  box-shadow: 0 0 6px rgba(245, 158, 11, 0.6);
}

.seek--disabled {
  cursor: not-allowed;
}

.player-unavailable {
  font-size: 0.75rem;
  color: #64748b;
  margin-top: 0.25rem;
}
</style>
