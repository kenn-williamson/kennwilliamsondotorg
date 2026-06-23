<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex justify-between items-center">
      <h2 class="text-2xl font-bold text-nautical-900">
        {{ editingSong ? 'Edit Song' : 'Add New Song' }}
      </h2>
      <button @click="$emit('cancel')" class="text-nautical-600 hover:text-nautical-900">
        Cancel
      </button>
    </div>

    <form @submit.prevent="onSubmit" class="space-y-6">
      <!-- Title -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">
          Title <span class="text-red-500">*</span>
        </label>
        <input
          v-model="title"
          type="text"
          class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          placeholder="Song title"
          @input="onTitleInput"
        />
      </div>

      <!-- Slug -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">
          URL Slug <span class="text-xs text-nautical-500 ml-1">(auto-generated from title)</span>
        </label>
        <input
          v-model="slug"
          type="text"
          class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          placeholder="song-url-slug"
          @input="slugManuallyEdited = true"
        />
        <p class="text-xs text-nautical-500 mt-1">Lowercase letters, numbers, and hyphens. Leave blank to auto-generate.</p>
      </div>

      <!-- Description -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">Description</label>
        <textarea
          v-model="description"
          rows="3"
          class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          placeholder="A short blurb about the track (optional)"
        ></textarea>
      </div>

      <!-- Audio Upload -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">Audio File</label>
        <div v-if="audioUrl" class="space-y-2">
          <MusicAudioPlayer :id="editingSong?.id || 'preview'" :src="audioUrl" :title="title || 'Preview'" />
          <button type="button" class="text-sm text-red-600 hover:text-red-800" @click="removeAudio">Remove audio</button>
        </div>
        <div v-else>
          <input
            ref="audioInput"
            type="file"
            accept="audio/*,.mp3,.wav,.flac,.m4a,.aac,.ogg"
            class="hidden"
            @change="handleAudioSelect"
          />
          <button
            type="button"
            class="px-4 py-2 border-2 border-dashed border-nautical-300 rounded-md text-nautical-600 hover:border-sky-400 hover:bg-sky-50 transition-colors w-full"
            :disabled="uploadingAudio"
            @click="audioInput?.click()"
          >
            {{ uploadingAudio ? 'Uploading audio…' : 'Click to upload audio (mp3, wav, flac, m4a, ogg — up to 50MB)' }}
          </button>
        </div>
        <p v-if="audioError" class="text-sm text-red-600 mt-1">{{ audioError }}</p>
      </div>

      <!-- Artwork Upload -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">Cover Artwork</label>
        <div v-if="artworkUrl" class="space-y-2">
          <img :src="artworkUrl" alt="Cover artwork preview" class="w-40 h-40 object-cover rounded-md border border-nautical-300" />
          <button type="button" class="block text-sm text-red-600 hover:text-red-800" @click="removeArtwork">Remove artwork</button>
        </div>
        <div v-else>
          <input ref="artworkInput" type="file" accept="image/*" class="hidden" @change="handleArtworkSelect" />
          <button
            type="button"
            class="px-4 py-2 border-2 border-dashed border-nautical-300 rounded-md text-nautical-600 hover:border-sky-400 hover:bg-sky-50 transition-colors w-full"
            :disabled="uploadingArtwork"
            @click="artworkInput?.click()"
          >
            {{ uploadingArtwork ? 'Uploading artwork…' : 'Click to upload cover art (square works best)' }}
          </button>
        </div>
        <p v-if="artworkError" class="text-sm text-red-600 mt-1">{{ artworkError }}</p>
      </div>

      <!-- Lyrics -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">Lyrics</label>
        <textarea
          v-model="lyrics"
          rows="10"
          class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent font-mono text-sm"
          placeholder="Lyrics (line breaks are preserved)"
        ></textarea>
      </div>

      <!-- Credits / AI disclosure -->
      <div>
        <label class="block text-sm font-medium text-nautical-700 mb-2">Credits / AI Disclosure</label>
        <textarea
          v-model="credits"
          rows="3"
          class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          placeholder="e.g. Lyrics & direction: Kenn Williamson. Vocals: Kenn. Produced with AI."
        ></textarea>
      </div>

      <!-- Display order + Status -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-nautical-700 mb-2">Display Order</label>
          <input
            v-model.number="displayOrder"
            type="number"
            min="0"
            class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          />
          <p class="text-xs text-nautical-500 mt-1">Lower numbers appear first.</p>
        </div>
        <div>
          <label class="block text-sm font-medium text-nautical-700 mb-2">
            Status <span class="text-red-500">*</span>
          </label>
          <select
            v-model="status"
            class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          >
            <option value="draft" :disabled="isAlreadyPublished">Draft</option>
            <option value="published">Published</option>
          </select>
          <p v-if="isAlreadyPublished" class="text-xs text-nautical-500 mt-1">Published songs cannot be reverted to draft.</p>
        </div>
      </div>

      <!-- Error Display -->
      <div v-if="formError || musicStore.hasError" class="bg-red-50 border border-red-300 rounded-md p-4">
        <p class="text-red-700">{{ formError || musicStore.error }}</p>
      </div>

      <!-- Submit Buttons -->
      <div class="flex justify-end gap-4">
        <button
          v-if="!isAlreadyPublished"
          type="button"
          :disabled="isSubmitting"
          class="px-6 py-3 border border-nautical-300 text-nautical-700 rounded-md hover:bg-nautical-50 transition-colors font-medium disabled:opacity-50"
          @click="saveDraft"
        >
          Save as Draft
        </button>
        <button
          type="submit"
          :disabled="isSubmitting || !title.trim()"
          :class="[
            'px-6 py-3 rounded-md font-medium transition-colors',
            isSubmitting || !title.trim() ? 'bg-nautical-300 text-nautical-500 cursor-not-allowed' : 'bg-sky-600 text-white hover:bg-sky-700'
          ]"
        >
          {{ isSubmitting ? (editingSong ? 'Updating…' : 'Saving…') : (editingSong ? 'Update Song' : 'Save Song') }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { generateSlugFromTitle } from '#shared/schemas/blog'
import { useMusicStore } from '~/stores/music'
import type { Song, CreateSongRequest, UpdateSongRequest } from '#shared/types'

const props = defineProps<{
  editingSong?: Song | null
}>()

const emit = defineEmits<{
  cancel: []
  success: []
}>()

const musicStore = useMusicStore()

// Form state
const title = ref('')
const slug = ref('')
const description = ref('')
const lyrics = ref('')
const credits = ref('')
const displayOrder = ref(0)
const status = ref<'draft' | 'published'>('draft')
const audioUrl = ref<string | null>(null)
const durationSeconds = ref<number | null>(null)
const artworkUrl = ref<string | null>(null)

const slugManuallyEdited = ref(false)
const isSubmitting = ref(false)
const uploadingAudio = ref(false)
const uploadingArtwork = ref(false)
const audioError = ref<string | null>(null)
const artworkError = ref<string | null>(null)
const formError = ref<string | null>(null)

const audioInput = ref<HTMLInputElement | null>(null)
const artworkInput = ref<HTMLInputElement | null>(null)

const isAlreadyPublished = computed(() => props.editingSong?.status === 'published')

const onTitleInput = () => {
  if (!props.editingSong && !slugManuallyEdited.value) {
    slug.value = generateSlugFromTitle(title.value)
  }
}

const handleAudioSelect = async (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  audioError.value = null
  if (file.size > 50 * 1024 * 1024) {
    audioError.value = 'Audio must be smaller than 50MB'
    return
  }
  uploadingAudio.value = true
  try {
    const result = await musicStore.uploadAudio(file)
    if (result) {
      audioUrl.value = result.url
      if (result.duration_seconds != null) durationSeconds.value = result.duration_seconds
    }
    // Best-effort: pre-fill blank fields from the file's embedded tags.
    await prefillFromMetadata(file)
  } catch (error) {
    audioError.value = error instanceof Error ? error.message : 'Failed to upload audio'
  } finally {
    uploadingAudio.value = false
  }
}

/**
 * Pre-fill blank fields from the audio file's embedded metadata (ID3 etc.).
 * Admin-only and dynamically imported, so `music-metadata` never ships to the
 * public bundle. Only fills empty fields, so it never overwrites your edits,
 * and a parse failure is non-fatal (the upload still succeeds).
 */
const prefillFromMetadata = async (file: File) => {
  try {
    const { parseBlob } = await import('music-metadata')
    const { common, format } = await parseBlob(file)

    // Title -> only when blank; regenerate slug for new songs
    if (!title.value.trim() && common.title) {
      title.value = common.title
      if (!props.editingSong && !slugManuallyEdited.value) {
        slug.value = generateSlugFromTitle(title.value)
      }
    }

    // Track duration
    if (durationSeconds.value == null && format.duration) {
      durationSeconds.value = Math.round(format.duration)
    }

    // Embedded cover art -> upload as artwork, only when none is set yet
    const pic = common.picture?.[0]
    if (!artworkUrl.value && pic) {
      const mime = pic.format || 'image/jpeg'
      const ext = mime.split('/')[1]?.split('+')[0] || 'jpg'
      const artFile = new File([new Uint8Array(pic.data)], `cover.${ext}`, { type: mime })
      uploadingArtwork.value = true
      try {
        const res = await musicStore.uploadArtwork(artFile)
        if (res) artworkUrl.value = res.url
      } finally {
        uploadingArtwork.value = false
      }
    }
  } catch (err) {
    // Embedded tags are optional; never block the upload on a parse failure.
    console.warn('Could not read audio metadata:', err)
  }
}

const removeAudio = () => {
  audioUrl.value = null
  durationSeconds.value = null
  if (audioInput.value) audioInput.value.value = ''
}

const handleArtworkSelect = async (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  artworkError.value = null
  if (!file.type.startsWith('image/')) {
    artworkError.value = 'Please select an image file'
    return
  }
  uploadingArtwork.value = true
  try {
    const result = await musicStore.uploadArtwork(file)
    if (result) {
      artworkUrl.value = result.url
    }
  } catch (error) {
    artworkError.value = error instanceof Error ? error.message : 'Failed to upload artwork'
  } finally {
    uploadingArtwork.value = false
  }
}

const removeArtwork = () => {
  artworkUrl.value = null
  if (artworkInput.value) artworkInput.value.value = ''
}

const buildPayload = (): CreateSongRequest & UpdateSongRequest => ({
  title: title.value.trim(),
  slug: slug.value.trim() || generateSlugFromTitle(title.value),
  description: description.value.trim() || undefined,
  lyrics: lyrics.value.trim() || undefined,
  credits: credits.value.trim() || undefined,
  audio_url: audioUrl.value || undefined,
  artwork_url: artworkUrl.value || undefined,
  duration_seconds: durationSeconds.value ?? undefined,
  display_order: displayOrder.value || 0,
  status: status.value
})

const saveDraft = async () => {
  status.value = 'draft'
  await onSubmit()
}

const onSubmit = async () => {
  formError.value = null
  if (!title.value.trim()) {
    formError.value = 'Title is required'
    return
  }
  isSubmitting.value = true
  try {
    const payload = buildPayload()
    if (props.editingSong) {
      await musicStore.updateSong(props.editingSong.id, payload)
    } else {
      await musicStore.createSong(payload)
    }
    if (!musicStore.hasError) {
      emit('success')
    }
  } catch (error) {
    console.error('Failed to save song:', error)
  } finally {
    isSubmitting.value = false
  }
}

// Initialize from editing song
watch(
  () => props.editingSong,
  (song) => {
    if (song) {
      title.value = song.title
      slug.value = song.slug
      description.value = song.description || ''
      lyrics.value = song.lyrics || ''
      credits.value = song.credits || ''
      displayOrder.value = song.display_order
      status.value = song.status
      audioUrl.value = song.audio_url
      durationSeconds.value = song.duration_seconds
      artworkUrl.value = song.artwork_url
      slugManuallyEdited.value = true
    } else {
      title.value = ''
      slug.value = ''
      description.value = ''
      lyrics.value = ''
      credits.value = ''
      displayOrder.value = 0
      status.value = 'draft'
      audioUrl.value = null
      durationSeconds.value = null
      artworkUrl.value = null
      slugManuallyEdited.value = false
    }
  },
  { immediate: true }
)
</script>
