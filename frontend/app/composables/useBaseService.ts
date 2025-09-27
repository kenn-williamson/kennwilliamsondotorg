import { ref, computed, getCurrentInstance, onUnmounted } from 'vue'

export function useBaseService() {
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const hasError = computed(() => !!error.value)
  const setLoading = (loading: boolean): void => {
    isLoading.value = loading
  }

  const setError = (err: string | null): void => {
    error.value = err
  }

  const clearError = (): void => {
    error.value = null
  }

  const handleError = (err: any, context?: string): void => {
    const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
    console.error(`[BaseService] Error${context ? ` in ${context}` : ''}:`, errorMessage)
    // TODO: Add toast notifications or other error handling here
  }

  const handleSuccess = (message: string): void => {
    console.log(`[BaseService] Success: ${message}`)
    // TODO: Add toast notifications or other success handling here
  }
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

  const executeRequestWithSuccess = async <T>(
    requestFn: () => Promise<T>,
    successMessage: string,
    context?: string
  ): Promise<T> => {
    const result = await executeRequest(requestFn, context)
    handleSuccess(successMessage)
    return result
  }
  const instance = getCurrentInstance()
  if (instance) {
    onUnmounted(() => {
      clearError()
    })
  }

  return {
    isLoading: computed(() => isLoading.value),
    error: computed(() => error.value),
    hasError,
    
    setLoading,
    setError,
    clearError,
    
    executeRequest,
    executeRequestWithSuccess
  }
}
