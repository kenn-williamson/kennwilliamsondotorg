interface ApiError {
  message: string
  status?: number
}

export class BaseService {
  protected apiBase: string

  constructor(apiBase: string) {
    this.apiBase = apiBase
  }

  protected async makeRequest<T>(
    endpoint: string,
    options: {
      method?: 'GET' | 'POST' | 'PUT' | 'DELETE'
      body?: any
      headers?: Record<string, string>
    } = {}
  ): Promise<T> {
    const { method = 'GET', body, headers = {} } = options

    try {
      return await $fetch<T>(`${this.apiBase}${endpoint}`, {
        method,
        body,
        headers,
      })
    } catch (error: any) {
      // Transform fetch errors into a consistent format
      const apiError: ApiError = {
        message: error.data?.message || error.message || 'Request failed',
        status: error.status || 500,
      }
      throw apiError
    }
  }

  protected getAuthHeaders(authStore: any): Record<string, string> {
    return authStore.getAuthHeaders()
  }
}