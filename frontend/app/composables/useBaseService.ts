import { ref, computed, getCurrentInstance, onUnmounted } from 'vue'

export function useBaseService() {
  const backendFetch = useBackendFetch()
  const authFetch = useAuthFetch()
  
  // Service state
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed properties
  const hasError = computed(() => !!error.value)

  // State management
  const setLoading = (loading: boolean): void => {
    isLoading.value = loading
  }

  const setError = (err: string | null): void => {
    error.value = err
  }

  const clearError = (): void => {
    error.value = null
  }

  // Simple error handler (can be enhanced later)
  const handleError = (err: any, context?: string): void => {
    const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
    console.error(`[BaseService] Error${context ? ` in ${context}` : ''}:`, errorMessage)
    // TODO: Add toast notifications or other error handling here
  }

  // Simple success handler (can be enhanced later)
  const handleSuccess = (message: string): void => {
    console.log(`[BaseService] Success: ${message}`)
    // TODO: Add toast notifications or other success handling here
  }

  // Wrapper for API calls with error handling
  const executeRequest = async <T>(
    requestFn: () => Promise<T>,
    context?: string
  ): Promise<T> => {
    setLoading(true)
    setError(null)
    
    try {
      const result = await requestFn()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      setError(errorMessage)
      handleError(err, context)
      throw err
    } finally {
      setLoading(false)
    }
  }

  // Wrapper for API calls with success handling
  const executeRequestWithSuccess = async <T>(
    requestFn: () => Promise<T>,
    successMessage: string,
    context?: string
  ): Promise<T> => {
    const result = await executeRequest(requestFn, context)
    handleSuccess(successMessage)
    return result
  }

  // Cleanup on unmount (only if called from a component)
  const instance = getCurrentInstance()
  if (instance) {
    onUnmounted(() => {
      clearError()
    })
  }

  return {
    // State
    isLoading: computed(() => isLoading.value),
    error: computed(() => error.value),
    hasError,
    
    // State management
    setLoading,
    setError,
    clearError,
    
    // Request execution
    executeRequest,
    executeRequestWithSuccess,
    
    // API client access
    backendFetch,
    authFetch
  }
}
