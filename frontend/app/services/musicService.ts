/**
 * Music Service - Uses the smart fetcher for SSR-safe requests
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type {
  Song,
  SongList,
  SongFilters,
  CreateSongRequest,
  UpdateSongRequest,
  AudioUploadResponse,
  Fetcher
} from '#shared/types'

export const musicService = (fetcher: Fetcher) => ({
  /**
   * Get paginated list of published songs (public)
   */
  getSongs: async (filters: SongFilters = {}): Promise<SongList> => {
    const params = new URLSearchParams()
    if (filters.page) params.append('page', filters.page.toString())
    if (filters.limit) params.append('limit', filters.limit.toString())
    if (filters.status) params.append('status', filters.status)

    const query = params.toString()
    const url = query ? `${API_ROUTES.PUBLIC.MUSIC.SONGS}?${query}` : API_ROUTES.PUBLIC.MUSIC.SONGS

    return fetcher<SongList>(url)
  },

  /**
   * Get a single published song by slug (public)
   */
  getSongBySlug: async (slug: string): Promise<Song> => {
    return fetcher<Song>(API_ROUTES.PUBLIC.MUSIC.SONG_BY_SLUG(slug))
  },

  /**
   * List all songs incl. drafts (admin only)
   */
  getAdminSongs: async (filters: SongFilters = {}): Promise<SongList> => {
    const params = new URLSearchParams()
    if (filters.page) params.append('page', filters.page.toString())
    if (filters.limit) params.append('limit', filters.limit.toString())
    if (filters.status) params.append('status', filters.status)

    const query = params.toString()
    const url = query
      ? `${API_ROUTES.PROTECTED.ADMIN.MUSIC.SONGS}?${query}`
      : API_ROUTES.PROTECTED.ADMIN.MUSIC.SONGS

    return fetcher<SongList>(url)
  },

  /**
   * Create a new song (admin only)
   */
  createSong: async (songData: CreateSongRequest): Promise<Song> => {
    return fetcher<Song>(API_ROUTES.PROTECTED.ADMIN.MUSIC.SONGS, {
      method: 'POST',
      body: songData
    })
  },

  /**
   * Update an existing song (admin only)
   */
  updateSong: async (id: string, songData: UpdateSongRequest): Promise<Song> => {
    return fetcher<Song>(API_ROUTES.PROTECTED.ADMIN.MUSIC.SONG_BY_ID(id), {
      method: 'PUT',
      body: songData
    })
  },

  /**
   * Delete a song (admin only)
   */
  deleteSong: async (id: string): Promise<void> => {
    return fetcher<void>(API_ROUTES.PROTECTED.ADMIN.MUSIC.SONG_BY_ID(id), {
      method: 'DELETE'
    })
  },

  /**
   * Upload a song's audio file (admin only)
   */
  uploadAudio: async (audioFile: File): Promise<AudioUploadResponse> => {
    const formData = new FormData()
    formData.append('audio', audioFile)

    return fetcher<AudioUploadResponse>(API_ROUTES.PROTECTED.ADMIN.MUSIC.UPLOAD_AUDIO, {
      method: 'POST',
      body: formData
    })
  },

  /**
   * Upload a song's cover artwork (admin only)
   */
  uploadArtwork: async (imageFile: File): Promise<{ url: string; original_url: string }> => {
    const formData = new FormData()
    formData.append('artwork', imageFile)

    return fetcher<{ url: string; original_url: string }>(
      API_ROUTES.PROTECTED.ADMIN.MUSIC.UPLOAD_ARTWORK,
      {
        method: 'POST',
        body: formData
      }
    )
  }
})
