<template>
  <div class="space-y-8">
    <!-- Timer Privacy Section -->
    <div class="space-y-4">
      <h3 class="text-base font-semibold text-gray-900 pb-2 border-b border-gray-200">
        Timer Privacy
      </h3>

      <!-- Make Timer Public Toggle -->
      <div class="flex items-center justify-between py-4">
        <div class="flex-1 mr-4">
          <h4 class="text-sm font-medium text-gray-900">
            Make Timer Public
          </h4>
          <p class="text-sm text-gray-500 mt-1">
            Allow others to view your timer at /{{ user?.slug }}/incident-timer
          </p>
        </div>
      <button
        type="button"
        @click="toggleIsPublic"
        :disabled="isLoading"
        :class="[
          'relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-sky-500 focus:ring-offset-2',
          isPublic ? 'bg-sky-600' : 'bg-gray-200',
          isLoading ? 'opacity-50 cursor-not-allowed' : ''
        ]"
        role="switch"
        :aria-checked="isPublic"
        aria-label="Toggle timer public visibility"
      >
        <span
          :class="[
            'pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out',
            isPublic ? 'translate-x-5' : 'translate-x-0'
          ]"
        />
      </button>
      </div>

      <!-- Show in Public List Toggle -->
      <div class="flex items-center justify-between py-4 border-t border-gray-200">
      <div class="flex-1 mr-4">
        <h4
          :class="[
            'text-sm font-medium',
            !isPublic ? 'text-gray-400' : 'text-gray-900'
          ]"
        >
          Show in Public List
        </h4>
        <p
          :class="[
            'text-sm mt-1',
            !isPublic ? 'text-gray-400' : 'text-gray-500'
          ]"
        >
          Display your timer in the public timers list on the incidents page
        </p>
      </div>
      <button
        type="button"
        @click="toggleShowInList"
        :disabled="isLoading || !isPublic"
        :class="[
          'relative inline-flex h-6 w-11 flex-shrink-0 rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-sky-500 focus:ring-offset-2',
          showInList && isPublic ? 'bg-sky-600' : 'bg-gray-200',
          isLoading || !isPublic ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
        ]"
        role="switch"
        :aria-checked="showInList && isPublic"
        aria-label="Toggle show in public list"
      >
        <span
          :class="[
            'pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out',
            showInList && isPublic ? 'translate-x-5' : 'translate-x-0'
          ]"
        />
      </button>
      </div>

      <!-- Info message when not public -->
      <div
        v-if="!isPublic"
        class="bg-blue-50 border border-blue-200 rounded-md p-4"
      >
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-blue-700">
            Enable "Make Timer Public" first to show in the public list
          </p>
          </div>
        </div>
      </div>

      <!-- Error message -->
      <div
        v-if="error"
        class="bg-red-50 border border-red-200 rounded-md p-4"
      >
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-red-700">{{ error }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Future preferences sections can be added here -->
    <!-- Example: Notification Preferences, Display Preferences, etc. -->
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useIncidentTimerStore } from '~/stores/incident-timers'

// Get user from session
const { user, fetch: refreshSession } = useUserSession()

// Timer store for preferences actions
const timerStore = useIncidentTimerStore()

// Local state for toggles (initialized from user preferences)
const isPublic = ref(user.value?.preferences?.timer_is_public ?? true)
const showInList = ref(user.value?.preferences?.timer_show_in_list ?? true)

// Loading and error state
const isLoading = ref(false)
const error = ref<string | null>(null)

// Watch for user changes (when preferences update)
watch(() => user.value?.preferences, (newPrefs) => {
  if (newPrefs) {
    isPublic.value = newPrefs.timer_is_public ?? true
    showInList.value = newPrefs.timer_show_in_list ?? true
  }
}, { deep: true })

// Toggle handlers with store integration
const toggleIsPublic = async () => {
  const newIsPublic = !isPublic.value
  const newShowInList = newIsPublic ? showInList.value : false

  isLoading.value = true
  error.value = null

  try {
    await timerStore.updateUserPreferences({
      timer_is_public: newIsPublic,
      timer_show_in_list: newShowInList
    })

    // Refresh client-side reactive refs from updated server cookie
    await refreshSession()

    // Update local state
    isPublic.value = newIsPublic
    showInList.value = newShowInList
  } catch (err: any) {
    error.value = err.message || 'Failed to update preferences'
    console.error('Error updating preferences:', err)
  } finally {
    isLoading.value = false
  }
}

const toggleShowInList = async () => {
  if (!isPublic.value) return

  const newShowInList = !showInList.value

  isLoading.value = true
  error.value = null

  try {
    await timerStore.updateUserPreferences({
      timer_is_public: isPublic.value,
      timer_show_in_list: newShowInList
    })

    // Refresh client-side reactive refs from updated server cookie
    await refreshSession()

    // Update local state
    showInList.value = newShowInList
  } catch (err: any) {
    error.value = err.message || 'Failed to update preferences'
    console.error('Error updating preferences:', err)
  } finally {
    isLoading.value = false
  }
}
</script>
