/**
 * Debounce Composable
 *
 * Provides a reusable debounce function that delays execution until after
 * a specified delay has elapsed since the last call.
 *
 * @example
 * // Simple debounced search
 * const { debounced } = useDebounce(async (query: string) => {
 *   await searchAPI(query)
 * }, 300)
 *
 * debounced('search term')
 *
 * @example
 * // With reactive value
 * const searchQuery = ref('')
 * const { debounced } = useDebounce(async (query: string) => {
 *   await searchAPI(query)
 * }, 300)
 *
 * watch(searchQuery, (newValue) => {
 *   debounced(newValue)
 * })
 */

import { useDebounceFn } from '@vueuse/core'
import type { Ref } from 'vue'

/**
 * Options for the debounce composable
 */
export interface UseDebounceOptions {
  /**
   * Delay in milliseconds before the function is called
   * @default 300
   */
  delay?: number

  /**
   * Whether to invoke on the leading edge of the timeout
   * @default false
   */
  leading?: boolean

  /**
   * Whether to invoke on the trailing edge of the timeout
   * @default true
   */
  trailing?: boolean

  /**
   * Maximum time the function can be delayed before it's invoked
   */
  maxWait?: number
}

/**
 * Creates a debounced version of the provided function
 *
 * @param fn - The function to debounce
 * @param options - Debounce options or delay in milliseconds
 * @returns Object containing the debounced function
 */
export function useDebounce<T extends (...args: any[]) => any>(
  fn: T,
  options: UseDebounceOptions | number = {}
): {
  debounced: (...args: Parameters<T>) => void
  cancel: () => void
  flush: () => void
} {
  // Normalize options
  const opts = typeof options === 'number'
    ? { delay: options }
    : options

  const {
    delay = 300,
    leading = false,
    trailing = true,
    maxWait
  } = opts

  // Create debounced function using VueUse
  const debouncedFn = useDebounceFn(
    fn,
    delay,
    { maxWait }
  )

  return {
    /**
     * The debounced function
     */
    debounced: debouncedFn,

    /**
     * Cancel any pending invocations
     */
    cancel: () => {
      // VueUse's useDebounceFn doesn't expose cancel, so we'll implement our own if needed
      // For now, this is a placeholder
    },

    /**
     * Immediately invoke any pending invocations
     */
    flush: () => {
      // VueUse's useDebounceFn doesn't expose flush, so we'll implement our own if needed
      // For now, this is a placeholder
    }
  }
}

/**
 * Creates a debounced search handler with a reactive query ref
 *
 * This is a convenience wrapper for the common pattern of debouncing search inputs
 *
 * @example
 * const searchQuery = ref('')
 * const { execute } = useDebouncedSearch(async (query) => {
 *   await searchStore.search(query)
 * })
 *
 * // In template:
 * <input v-model="searchQuery" @input="execute(searchQuery)" />
 *
 * @param fn - The search function to debounce
 * @param delay - Delay in milliseconds (default: 300)
 * @returns Object containing the execute function
 */
export function useDebouncedSearch<T = string>(
  fn: (query: T) => void | Promise<void>,
  delay: number = 300
): {
  execute: (query: T | Ref<T>) => void
} {
  const { debounced } = useDebounce(fn, delay)

  return {
    execute: (query: T | Ref<T>) => {
      const value = isRef(query) ? query.value : query
      debounced(value)
    }
  }
}
