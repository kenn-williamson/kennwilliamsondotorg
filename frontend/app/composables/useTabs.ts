/**
 * Reactive tab management composable
 *
 * Manages tab state using URL query parameters, providing SSR-compatible
 * reactive tab switching. All components calling this composable with the
 * same route will share the same reactive state automatically.
 *
 * @example
 * ```typescript
 * const { activeTab, setActiveTab } = useTabs(
 *   ['tab1', 'tab2', 'tab3'] as const,
 *   'tab1'
 * )
 * ```
 */
export function useTabs<T extends string>(
  validTabs: readonly T[],
  defaultTab: T
) {
  const route = useRoute()
  const router = useRouter()

  // Computed based on reactive route.query.tab
  // Automatically updates when URL changes
  const activeTab = computed(() => {
    const tabParam = route.query.tab

    if (tabParam && validTabs.includes(tabParam as T)) {
      return tabParam as T
    }

    return defaultTab
  })

  /**
   * Updates the active tab by modifying the URL query parameter
   * This will trigger reactivity across all components using this composable
   */
  const setActiveTab = (tabId: T) => {
    if (!validTabs.includes(tabId)) {
      console.warn(`[useTabs] Invalid tab ID: ${tabId}`)
      return
    }

    router.push({ query: { tab: tabId } })
  }

  return {
    activeTab,
    setActiveTab
  }
}
