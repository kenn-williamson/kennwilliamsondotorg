<template>
  <div class="space-y-6">
    <!-- Header with Create Button -->
    <div class="flex justify-between items-center">
      <h2 class="text-2xl font-bold text-nautical-900">Songs</h2>
      <button
        @click="$emit('create-new')"
        class="px-4 py-2 bg-sky-600 text-white rounded-md hover:bg-sky-700 transition-colors font-medium flex items-center gap-2"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Add New Song
      </button>
    </div>

    <!-- Filters -->
    <div class="flex gap-4 items-center">
      <select
        v-model="statusFilter"
        class="px-4 py-2 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
      >
        <option value="all">All Status</option>
        <option value="published">Published</option>
        <option value="draft">Draft</option>
      </select>
    </div>

    <!-- Loading State -->
    <div v-if="musicStore.isLoading" class="text-center py-12">
      <svg class="animate-spin h-8 w-8 text-sky-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      <p class="text-nautical-600 mt-4">Loading songs...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="musicStore.hasError" class="bg-red-50 border border-red-300 rounded-lg p-6">
      <p class="text-red-700">{{ musicStore.error }}</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="!songs.length" class="text-center py-12">
      <svg class="w-16 h-16 text-nautical-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 18V5l12-2v13M9 18a3 3 0 11-6 0 3 3 0 016 0zm12-2a3 3 0 11-6 0 3 3 0 016 0z" />
      </svg>
      <h3 class="text-xl font-bold text-nautical-700 mb-2">No songs yet</h3>
      <p class="text-nautical-600 mb-4">Get started by adding your first song!</p>
      <button
        @click="$emit('create-new')"
        class="px-6 py-3 bg-sky-600 text-white rounded-md hover:bg-sky-700 transition-colors font-medium"
      >
        Add First Song
      </button>
    </div>

    <!-- Songs Table -->
    <div v-else class="bg-white rounded-lg shadow overflow-hidden">
      <table class="min-w-full divide-y divide-nautical-200">
        <thead class="bg-nautical-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">Title</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">Status</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">Order</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">Audio</th>
            <th class="px-6 py-3 text-right text-xs font-medium text-nautical-700 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-nautical-200">
          <tr v-for="song in songs" :key="song.id" class="hover:bg-nautical-50">
            <td class="px-6 py-4">
              <div class="text-sm font-medium text-nautical-900">{{ song.title }}</div>
              <div v-if="song.description" class="text-sm text-nautical-500 truncate max-w-md">{{ song.description }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                :class="[
                  'px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full',
                  song.status === 'published' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
                ]"
              >
                {{ song.status }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-nautical-500">{{ song.display_order }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm">
              <span v-if="song.audio_url" class="text-green-600">✓</span>
              <span v-else class="text-nautical-400">—</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button @click="$emit('edit-song', song)" class="text-sky-600 hover:text-sky-900 mr-4">Edit</button>
              <button
                @click="handleDelete(song)"
                class="text-red-600 hover:text-red-900"
                :disabled="deletingId === song.id"
              >
                {{ deletingId === song.id ? 'Deleting...' : 'Delete' }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useMusicStore } from '~/stores/music'
import type { Song } from '#shared/types'

defineEmits<{
  'create-new': []
  'edit-song': [song: Song]
}>()

const statusFilter = ref<'all' | 'draft' | 'published'>('all')
const deletingId = ref<string | null>(null)

const musicStore = useMusicStore()
const songs = computed(() => musicStore.songs)

const loadSongs = async () => {
  await musicStore.loadAdminSongs({
    page: 1,
    limit: 100,
    status: statusFilter.value === 'all' ? undefined : statusFilter.value
  })
}

watch(statusFilter, loadSongs)
loadSongs()

const handleDelete = async (song: Song) => {
  if (!confirm(`Are you sure you want to delete "${song.title}"? This action cannot be undone.`)) {
    return
  }
  deletingId.value = song.id
  try {
    await musicStore.deleteSong(song.id)
    await loadSongs()
  } catch (error) {
    console.error('Failed to delete song:', error)
  } finally {
    deletingId.value = null
  }
}
</script>
