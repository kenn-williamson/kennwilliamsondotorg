/**
 * useNowPlaying - coordinates single-track playback across audio players.
 *
 * Holds the id of the currently-playing track in module-level state so that
 * starting one player pauses any others. Only mutated client-side (audio
 * playback never runs during SSR), so the shared ref is safe here.
 */
import { ref } from 'vue'

const currentId = ref<string | null>(null)

export function useNowPlaying() {
  const setCurrent = (id: string | null) => {
    currentId.value = id
  }

  return { currentId, setCurrent }
}
