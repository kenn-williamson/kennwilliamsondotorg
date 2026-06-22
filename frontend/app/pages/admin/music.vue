<template>
  <div class="min-h-screen bg-nautical-50 px-4 py-8">
    <div class="max-w-7xl mx-auto">
      <!-- Page Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-nautical-900 mb-2">Music Management</h1>
        <p class="text-nautical-600">Add, edit, and manage your songs</p>
      </div>

      <!-- Content -->
      <div class="mt-6">
        <!-- List View -->
        <MusicListTab
          v-if="!showEditor"
          @create-new="handleCreateNew"
          @edit-song="handleEditSong"
        />

        <!-- Editor View -->
        <MusicEditorForm
          v-else
          :editing-song="editingSong"
          @cancel="handleCancel"
          @success="handleSuccess"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { Song } from '#shared/types'

// Page meta
definePageMeta({
  middleware: 'admin'
})

// SEO
useHead({
  title: 'Music Management - Admin',
  meta: [
    {
      name: 'description',
      content: 'Manage songs - add, edit, and publish music'
    }
  ]
})

// Editor state
const showEditor = ref(false)
const editingSong = ref<Song | null>(null)

const handleCreateNew = () => {
  editingSong.value = null
  showEditor.value = true
}

const handleEditSong = (song: Song) => {
  editingSong.value = song
  showEditor.value = true
}

const handleCancel = () => {
  editingSong.value = null
  showEditor.value = false
}

const handleSuccess = () => {
  editingSong.value = null
  showEditor.value = false
}
</script>
