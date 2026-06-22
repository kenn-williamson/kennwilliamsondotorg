/**
 * Music Types - Frontend API types for the music (songs) feature
 */

export interface Song {
  id: string
  slug: string
  title: string
  description: string | null
  lyrics: string | null
  credits: string | null
  audio_url: string | null
  artwork_url: string | null
  duration_seconds: number | null
  display_order: number
  status: 'draft' | 'published'
  published_at: string | null
  created_at: string
  updated_at: string
}

export interface SongList {
  songs: Song[]
  total: number
  page: number
  total_pages: number
}

export interface SongFilters {
  page?: number
  limit?: number
  status?: string
}

export interface CreateSongRequest {
  title: string
  slug?: string
  description?: string
  lyrics?: string
  credits?: string
  audio_url?: string
  artwork_url?: string
  duration_seconds?: number
  display_order?: number
  status: 'draft' | 'published'
}

export interface UpdateSongRequest {
  title?: string
  slug?: string
  description?: string
  lyrics?: string
  credits?: string
  audio_url?: string
  artwork_url?: string
  duration_seconds?: number
  display_order?: number
  status?: 'draft' | 'published'
}

export interface AudioUploadResponse {
  url: string
  duration_seconds: number | null
}
