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
      </div>

      <p v-if="song.description" class="song-desc">{{ song.description }}</p>
    </div>

    <MusicAudioPlayer class="song-player" :id="song.id" :src="song.audio_url" :title="song.title" />
  </article>
</template>

<script setup lang="ts">
import type { Song } from '#shared/types'

defineProps<{
  song: Song
}>()
</script>

<style scoped>
/* Mobile-first: a compact thumbnail + title/description sit on top, with the
   audio player on its own full-width row below so its controls never get
   crushed. At sm and up the artwork becomes a square spanning the full card
   height beside the stacked title/desc/player (see the media query). */
.song-card {
  display: grid;
  grid-template-columns: 4.5rem 1fr;
  grid-template-areas:
    'art info'
    'player player';
  column-gap: 0.875rem;
  row-gap: 0.875rem;
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

/* Square cover. On mobile the grid column fixes its width; on sm+ the art
   spans both rows so the cell height is definite and aspect-ratio yields a
   real square (flush, no crop). */
.song-art-link {
  grid-area: art;
  position: relative;
  width: 100%;
  height: auto;
  aspect-ratio: 1 / 1;
  overflow: hidden;
  border-radius: 0.375rem;
  border: 1px solid #cbd5e1;
}

.song-art {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.song-art--placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #1e293b, #334155);
  color: #67e8f9;
}

.song-info {
  grid-area: info;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.song-player {
  grid-area: player;
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

.song-desc {
  color: #334155; /* nautical-700 */
  font-size: 0.9375rem;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@media (min-width: 640px) {
  .song-card {
    grid-template-columns: auto 1fr;
    grid-template-areas:
      'art info'
      'art player';
    column-gap: 1rem;
    row-gap: 0.5rem;
  }

  /* Art spans both rows, so its height is the full card content height;
     aspect-ratio derives the matching width for a flush square. */
  .song-art-link {
    width: auto;
    height: 100%;
  }
}
</style>
