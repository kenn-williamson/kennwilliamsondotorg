/**
 * Music Store - Centralized state management for songs
 */

import type {
  Song,
  SongList,
  SongFilters,
  CreateSongRequest,
  UpdateSongRequest
} from '#shared/types'
import { musicService } from '~/services/musicService'
import { useSmartFetch } from '~/composables/useSmartFetch'

export const useMusicStore = defineStore('music', () => {
  // State
  const songs = ref<Song[]>([])
  const currentSong = ref<Song | null>(null)
  const totalSongs = ref(0)
  const currentPage = ref(1)
  const totalPages = ref(1)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Service instance - uses useSmartFetch for automatic routing
  const smartFetch = useSmartFetch()
  const musicServiceInstance = musicService(smartFetch)

  // Private action handler
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null

    try {
      return await action()
    } catch (err: any) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[MusicStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      return undefined
    } finally {
      isLoading.value = false
    }
  }

  // Actions
  const loadSongs = async (filters: SongFilters = {}) => {
    const data = await _handleAction(() => musicServiceInstance.getSongs(filters), 'loadSongs')
    if (data) {
      songs.value = data.songs
      totalSongs.value = data.total
      currentPage.value = data.page
      totalPages.value = data.total_pages
    }
    return data
  }

  const loadAdminSongs = async (filters: SongFilters = {}) => {
    const data = await _handleAction(() => musicServiceInstance.getAdminSongs(filters), 'loadAdminSongs')
    if (data) {
      songs.value = data.songs
      totalSongs.value = data.total
      currentPage.value = data.page
      totalPages.value = data.total_pages
    }
    return data
  }

  const loadSongBySlug = async (slug: string) => {
    const data = await _handleAction(() => musicServiceInstance.getSongBySlug(slug), 'loadSongBySlug')
    if (data) {
      currentSong.value = data
    }
    return data
  }

  const createSong = async (songData: CreateSongRequest) => {
    const data = await _handleAction(() => musicServiceInstance.createSong(songData), 'createSong')
    if (data) {
      songs.value.unshift(data)
    }
    return data
  }

  const updateSong = async (id: string, songData: UpdateSongRequest) => {
    const data = await _handleAction(() => musicServiceInstance.updateSong(id, songData), 'updateSong')
    if (data) {
      const index = songs.value.findIndex(s => s.id === id)
      if (index !== -1) {
        songs.value[index] = data
      }
      if (currentSong.value?.id === id) {
        currentSong.value = data
      }
    }
    return data
  }

  const deleteSong = async (id: string) => {
    await _handleAction(() => musicServiceInstance.deleteSong(id), 'deleteSong')
    songs.value = songs.value.filter(s => s.id !== id)
    if (currentSong.value?.id === id) {
      currentSong.value = null
    }
  }

  const uploadAudio = async (audioFile: File) => {
    return await _handleAction(() => musicServiceInstance.uploadAudio(audioFile), 'uploadAudio')
  }

  const uploadArtwork = async (imageFile: File) => {
    return await _handleAction(() => musicServiceInstance.uploadArtwork(imageFile), 'uploadArtwork')
  }

  // Computed
  const hasError = computed(() => !!error.value)

  return {
    // State
    songs,
    currentSong,
    totalSongs,
    currentPage,
    totalPages,
    isLoading,
    error,
    hasError,

    // Actions
    loadSongs,
    loadAdminSongs,
    loadSongBySlug,
    createSong,
    updateSong,
    deleteSong,
    uploadAudio,
    uploadArtwork
  }
})
