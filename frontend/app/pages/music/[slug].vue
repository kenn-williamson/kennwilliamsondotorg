<template>
  <div class="min-h-screen nautical-background">
    <SteampunkBackground />

    <div class="relative z-10 max-w-3xl mx-auto px-4 py-8">
      <article class="bg-gradient-to-br from-nautical-50 via-primary-50 to-sky-50 border-2 border-primary-700 rounded-lg shadow-xl overflow-hidden">
        <div class="h-2 bg-gradient-to-r from-primary-600 via-indigo-600 to-primary-700"></div>

        <div class="p-6 sm:p-8 lg:p-10">
          <!-- Back link -->
          <NuxtLink to="/music" class="inline-flex items-center gap-1 text-primary-700 hover:text-primary-900 font-medium mb-6">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
            All music
          </NuxtLink>

          <div v-if="song" class="space-y-6">
            <!-- Artwork + title -->
            <div class="flex flex-col sm:flex-row gap-6 items-start">
              <img
                v-if="song.artwork_url"
                :src="song.artwork_url"
                :alt="`${song.title} cover art`"
                class="w-48 h-48 object-cover rounded-lg border border-nautical-300 shadow-md mx-auto sm:mx-0"
              />
              <div
                v-else
                class="w-48 h-48 rounded-lg border border-nautical-300 shadow-md mx-auto sm:mx-0 flex items-center justify-center bg-gradient-to-br from-nautical-800 to-nautical-700 text-accent-300"
              >
                <svg class="w-16 h-16" viewBox="0 0 24 24" fill="none" stroke="currentColor" aria-hidden="true">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 18V5l12-2v13M9 18a3 3 0 11-6 0 3 3 0 016 0zm12-2a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
              </div>

              <div class="flex-1 min-w-0">
                <h1 class="text-3xl sm:text-4xl font-bold text-primary-900 mb-2">{{ song.title }}</h1>
                <p v-if="song.description" class="text-nautical-700 mb-4">{{ song.description }}</p>
                <MusicAudioPlayer :id="song.id" :src="song.audio_url" :title="song.title" />
              </div>
            </div>

            <!-- Lyrics -->
            <section v-if="song.lyrics" class="pt-2">
              <h2 class="text-xl font-bold text-primary-900 mb-3">Lyrics</h2>
              <pre class="lyrics">{{ song.lyrics }}</pre>
            </section>

            <!-- Credits / AI disclosure -->
            <section v-if="song.credits" class="border-t border-nautical-200 pt-4">
              <h2 class="text-sm font-semibold text-nautical-600 uppercase tracking-wide mb-2">Credits</h2>
              <pre class="credits">{{ song.credits }}</pre>
            </section>
          </div>
        </div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMusicStore } from '~/stores/music'

const route = useRoute()
const musicStore = useMusicStore()
const slug = computed(() => route.params.slug as string)

const { data: song } = await useAsyncData(
  `song-${slug.value}`,
  () => musicStore.loadSongBySlug(slug.value)
)

if (!song.value) {
  throw createError({ statusCode: 404, statusMessage: 'Song not found', fatal: true })
}

// SEO Meta Tags
useHead({
  title: song.value.title,
  meta: [
    {
      name: 'description',
      content: song.value.description || `Listen to ${song.value.title} by Kenn Williamson`
    }
  ]
})

// Social Share Meta Tags (use cover art when available)
const { useSocialShare } = await import('~/composables/useSocialShare')
useSocialShare({
  title: `${song.value.title} - Kenn Williamson`,
  description: song.value.description || `Listen to ${song.value.title}`,
  customImage: song.value.artwork_url || undefined
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

.lyrics,
.credits {
  white-space: pre-wrap;
  word-break: break-word;
  font-family: inherit;
  color: #334155; /* nautical-700 */
  line-height: 1.7;
  margin: 0;
}

.credits {
  font-size: 0.875rem;
  color: #64748b;
}
</style>
