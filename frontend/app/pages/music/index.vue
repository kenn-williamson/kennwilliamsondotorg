<template>
  <div class="min-h-screen nautical-background">
    <SteampunkBackground />

    <div class="relative z-10 max-w-4xl mx-auto px-4 py-8">
      <article class="bg-gradient-to-br from-nautical-50 via-primary-50 to-sky-50 border-2 border-primary-700 rounded-lg shadow-xl overflow-hidden">
        <!-- Decorative Header Border -->
        <div class="h-2 bg-gradient-to-r from-primary-600 via-indigo-600 to-primary-700"></div>

        <div class="p-6 sm:p-8 lg:p-12">
          <!-- Header -->
          <header class="mb-8">
            <h1 class="text-4xl sm:text-5xl font-bold text-primary-900 mb-4">Music</h1>
            <p class="text-lg text-nautical-700">
              Songs I've been making. Written and directed by me, brought to life with AI, with my own vocals on a few.
            </p>
          </header>

          <!-- Artist project -->
          <section class="mb-10 flex flex-col sm:flex-row gap-6 items-center sm:items-start">
            <img
              src="~/assets/images/zosimas-pupil-eye.jpg"
              alt="Zosima's Pupil logo: a grey eye with a neon ZP in the pupil"
              class="w-36 h-36 sm:w-40 sm:h-40 rounded-full object-cover border-2 border-primary-700 shadow-md flex-shrink-0"
            />
            <div class="flex-1 min-w-0">
              <h2 class="text-2xl sm:text-3xl font-bold text-primary-900 mb-3">Zosima's Pupil</h2>
              <p class="text-lg text-nautical-700">
                I release these songs as <strong>Zosima's Pupil</strong>. The debut EP, <em>Vigil and Thirst</em>, is out December 4, 2026 on Skandalon Records, with a new single every Friday from October 30.
              </p>
              <p class="mt-4 flex flex-wrap gap-x-6 gap-y-2">
                <a
                  href="https://open.spotify.com/artist/2fSLvbg3aLEubTzxbFjG5n"
                  target="_blank"
                  rel="noopener"
                  class="text-primary-700 hover:text-primary-900 font-medium underline underline-offset-2"
                >Zosima's Pupil on Spotify</a>
                <a
                  href="https://music.apple.com/artist/6817109497"
                  target="_blank"
                  rel="noopener"
                  class="text-primary-700 hover:text-primary-900 font-medium underline underline-offset-2"
                >Zosima's Pupil on Apple Music</a>
              </p>
            </div>
          </section>

          <!-- Loading State -->
          <div v-if="pending" class="space-y-4">
            <div v-for="i in 3" :key="i" class="flex gap-4 bg-white rounded-lg shadow-md p-4 animate-pulse border border-nautical-200">
              <div class="w-24 h-24 bg-nautical-200 rounded-md"></div>
              <div class="flex-1 space-y-3 py-2">
                <div class="h-5 bg-nautical-200 rounded w-1/2"></div>
                <div class="h-4 bg-nautical-200 rounded w-3/4"></div>
                <div class="h-8 bg-nautical-200 rounded w-full"></div>
              </div>
            </div>
          </div>

          <!-- Error State -->
          <div v-else-if="error" class="bg-red-50 border-2 border-red-300 rounded-lg p-6 text-center">
            <svg class="w-12 h-12 text-red-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <h2 class="text-xl font-bold text-red-900 mb-2">Failed to load music</h2>
            <p class="text-red-700">{{ error.message }}</p>
          </div>

          <!-- No Results -->
          <div v-else-if="songs.length === 0" class="text-center py-12">
            <svg class="w-16 h-16 text-nautical-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 18V5l12-2v13M9 18a3 3 0 11-6 0 3 3 0 016 0zm12-2a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <h2 class="text-2xl font-bold text-nautical-800 mb-2">No tracks yet</h2>
            <p class="text-nautical-600">New songs are on the way. Check back soon!</p>
          </div>

          <!-- Songs List -->
          <div v-else class="space-y-4">
            <MusicSongCard
              v-for="song in songs"
              :key="song.id"
              :song="song"
            />
          </div>
        </div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMusicStore } from '~/stores/music'

const musicStore = useMusicStore()

// SSR data fetching - published songs only
await useAsyncData('music-songs', async () => {
  await musicStore.loadSongs({ page: 1, limit: 50, status: 'published' })
})

const songs = computed(() => musicStore.songs)
const pending = computed(() => musicStore.isLoading)
const error = computed(() => (musicStore.error ? { message: musicStore.error } : null))

// SEO Meta Tags
useHead({
  title: 'Music',
  meta: [
    {
      name: 'description',
      content: "Original songs by Kenn Williamson, released as Zosima's Pupil. Written and directed by Kenn, produced with AI."
    }
  ]
})

// Social Share Meta Tags (uses default share image)
const { useSocialShare } = await import('~/composables/useSocialShare')
useSocialShare({
  title: 'Music - Kenn Williamson',
  description: 'Original songs written and directed by Kenn, produced with AI.'
})
</script>

<style scoped>
.nautical-background {
  background: linear-gradient(
    135deg,
    #0f172a 0%,
    #1e293b 25%,
    #334155 50%,
    #1e293b 75%,
    #0f172a 100%
  );
}
</style>
