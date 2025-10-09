<template>
  <div class="bg-white shadow rounded-lg">
    <div class="px-6 py-4 border-b border-gray-200">
      <h2 class="text-lg font-medium text-gray-900">Data Export</h2>
      <p class="mt-1 text-sm text-gray-500">Download all your data in JSON format for GDPR/CCPA compliance.</p>
    </div>
    <div class="px-6 py-4">
      <div class="flex items-start justify-between">
        <div class="flex-1">
          <p class="text-sm text-gray-700">
            You can download a complete copy of your data including:
          </p>
          <ul class="mt-2 text-sm text-gray-600 list-disc list-inside space-y-1">
            <li>Profile information</li>
            <li>Incident timers</li>
            <li>Phrase suggestions and exclusions</li>
            <li>Active sessions</li>
            <li>Account metadata</li>
          </ul>
          <p class="mt-3 text-sm text-gray-500">
            Data will be exported in machine-readable JSON format.
          </p>
        </div>
        <div class="ml-4">
          <button
            @click="handleExport"
            :disabled="isExporting"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
          >
            <svg v-if="isExporting" class="animate-spin -ml-0.5 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
            </svg>
            <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
            </svg>
            {{ isExporting ? 'Downloading...' : 'Download My Data' }}
          </button>
        </div>
      </div>

      <!-- Success Message -->
      <div v-if="exportSuccess" class="mt-4 p-3 bg-green-50 border border-green-200 rounded-md">
        <div class="flex">
          <svg class="h-5 w-5 text-green-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
          <div class="ml-3">
            <p class="text-sm text-green-700">Data exported successfully!</p>
          </div>
        </div>
      </div>

      <!-- Error Message -->
      <div v-if="exportError" class="mt-4 p-3 bg-red-50 border border-red-200 rounded-md">
        <div class="flex">
          <svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
          </svg>
          <div class="ml-3">
            <p class="text-sm text-red-700">{{ exportError }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { authService } from '~/services/authService'
import { useFetcher } from '~/composables/useFetcher'

const isExporting = ref(false)
const exportSuccess = ref(false)
const exportError = ref<string | null>(null)

const { fetcher } = useFetcher()
const service = authService(fetcher)

const handleExport = async () => {
  isExporting.value = true
  exportSuccess.value = false
  exportError.value = null

  try {
    const blob = await service.exportUserData()

    // Create download link
    const url = window.URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url

    // Generate filename with current date
    const date = new Date().toISOString().split('T')[0]
    link.download = `data-export-${date}.json`

    // Trigger download
    document.body.appendChild(link)
    link.click()

    // Cleanup
    document.body.removeChild(link)
    window.URL.revokeObjectURL(url)

    exportSuccess.value = true

    // Clear success message after 5 seconds
    setTimeout(() => {
      exportSuccess.value = false
    }, 5000)
  } catch (error) {
    console.error('Failed to export data:', error)
    exportError.value = error instanceof Error ? error.message : 'Failed to export data'
  } finally {
    isExporting.value = false
  }
}
</script>
