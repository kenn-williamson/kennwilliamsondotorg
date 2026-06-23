<template>
  <article class="song-card">
    <NuxtLink :to="`/music/${song.slug}`" class="song-art-link" :aria-label="`Open ${song.title}`">
      <img
        v-if="song.artwork_url"
        :src="song.artwork_url"
        :alt="`${song.title} cover art`"
        class="song-art"
      />
      <div v-else class="song-art song-art--placeholder">
        <svg class="w-12 h-12" viewBox="0 0 24 24" fill="none" stroke="currentColor" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 18V5l12-2v13M9 18a3 3 0 11-6 0 3 3 0 016 0zm12-2a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </div>
    </NuxtLink>

    <div class="song-info">
      <div class="song-header">
        <NuxtLink :to="`/music/${song.slug}`" class="song-title">{{ song.title }}</NuxtLink>
        <span v-if="song.duration_seconds" class="song-duration">{{ formatDuration(song.duration_seconds) }}</span>
      </div>

      <p v-if="song.description" class="song-desc">{{ song.description }}</p>

      <MusicAudioPlayer :id="song.id" :src="song.audio_url" :title="song.title" />
    </div>
  </article>
</template>

<script setup lang="ts">
import type { Song } from '#shared/types'

defineProps<{
  song: Song
}>()

const formatDuration = (seconds: number): string => {
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}
</script>

<style scoped>
.song-card {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: #ffffff;
  border: 1px solid #e2e8f0; /* nautical-200 */
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.song-card:hover {
  border-color: #60a5fa; /* primary-400 */
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
}

.song-art-link {
  flex-shrink: 0;
}

.song-art {
  width: 6rem;
  height: 6rem;
  object-fit: cover;
  border-radius: 0.375rem;
  border: 1px solid #cbd5e1;
}

.song-art--placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #1e293b, #334155);
  color: #67e8f9;
}

.song-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.song-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.5rem;
}

.song-title {
  font-size: 1.25rem;
  font-weight: 700;
  color: #1e3a8a; /* primary-900 */
  transition: color 0.15s ease;
  text-decoration: none;
}

.song-title:hover {
  color: #1d4ed8; /* primary-700 */
}

.song-duration {
  flex-shrink: 0;
  font-size: 0.8125rem;
  font-variant-numeric: tabular-nums;
  color: #64748b;
}

.song-desc {
  color: #334155; /* nautical-700 */
  font-size: 0.9375rem;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
