<template>
  <div class="phrase-suggestions-tab">
    <!-- Loading State -->
    <div v-if="adminStore.isLoading" class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
    </div>

    <!-- Error State -->
    <div v-else-if="adminStore.error" class="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
      <p class="text-red-800 text-sm">{{ adminStore.error }}</p>
      <button 
        @click="refreshSuggestions"
        class="mt-2 text-sm text-red-600 hover:text-red-700 underline"
      >
        Try again
      </button>
    </div>

    <!-- Suggestions List -->
    <div v-else-if="adminStore.pendingSuggestions.length > 0" class="space-y-4">
      <div 
        v-for="suggestion in adminStore.pendingSuggestions" 
        :key="suggestion.id"
        class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <div class="flex items-center space-x-3 mb-3">
              <div class="flex-shrink-0">
                <div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                  <span class="text-blue-600 text-sm">✍️</span>
                </div>
              </div>
              <div class="flex-1">
                <h3 class="text-sm font-medium text-gray-900">Suggested by {{ suggestion.user_display_name }}</h3>
                <p class="text-xs text-gray-500">{{ formatDate(suggestion.created_at) }}</p>
              </div>
            </div>
            
            <div class="bg-gray-50 rounded-md p-4 mb-4">
              <p class="text-gray-900 font-medium italic">"{{ suggestion.phrase_text }}"</p>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex items-center justify-between pt-4 border-t border-gray-200">
          <div class="flex items-center space-x-3">
            <button
              @click="approveSuggestion(suggestion)"
              :disabled="isProcessing"
              class="px-4 py-2 text-sm bg-green-100 text-green-700 rounded-md hover:bg-green-200 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              Approve
            </button>
            <button
              @click="rejectSuggestion(suggestion)"
              :disabled="isProcessing"
              class="px-4 py-2 text-sm bg-red-100 text-red-700 rounded-md hover:bg-red-200 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              Reject
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-12">
      <div class="text-gray-400 text-6xl mb-4">✍️</div>
      <h3 class="text-lg font-medium text-gray-900 mb-2">No Pending Suggestions</h3>
      <p class="text-gray-500 mb-4">All phrase suggestions have been reviewed.</p>
      <button 
        @click="refreshSuggestions"
        class="px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
      >
        Refresh
      </button>
    </div>

    <!-- Approval Modal -->
    <div v-if="showApprovalModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Approve Suggestion</h3>
        <p class="text-sm text-gray-600 mb-4">
          Approve this phrase suggestion from <strong>{{ selectedSuggestion?.user_display_name }}</strong>?
        </p>
        <div class="bg-gray-50 rounded-md p-3 mb-4">
          <p class="text-gray-900 italic">"{{ selectedSuggestion?.phrase_text }}"</p>
        </div>
        <div class="mb-4">
          <label for="approval-reason" class="block text-sm font-medium text-gray-700 mb-2">
            Admin Comment (optional)
          </label>
          <textarea
            id="approval-reason"
            v-model="approvalReason"
            rows="3"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-gray-900 focus:border-gray-900"
            placeholder="Add a comment about why this suggestion was approved..."
          ></textarea>
        </div>
        <div class="flex justify-end space-x-3">
          <button
            @click="cancelApproval"
            class="px-4 py-2 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors"
          >
            Cancel
          </button>
          <button
            @click="confirmApproval"
            :disabled="isProcessing"
            class="px-4 py-2 text-sm bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            Approve
          </button>
        </div>
      </div>
    </div>

    <!-- Rejection Modal -->
    <div v-if="showRejectionModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Reject Suggestion</h3>
        <p class="text-sm text-gray-600 mb-4">
          Reject this phrase suggestion from <strong>{{ selectedSuggestion?.user_display_name }}</strong>?
        </p>
        <div class="bg-gray-50 rounded-md p-3 mb-4">
          <p class="text-gray-900 italic">"{{ selectedSuggestion?.phrase_text }}"</p>
        </div>
        <div class="mb-4">
          <label for="rejection-reason" class="block text-sm font-medium text-gray-700 mb-2">
            Reason for Rejection *
          </label>
          <textarea
            id="rejection-reason"
            v-model="rejectionReason"
            rows="3"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-gray-900 focus:border-gray-900"
            placeholder="Explain why this suggestion is being rejected..."
            required
          ></textarea>
        </div>
        <div class="flex justify-end space-x-3">
          <button
            @click="cancelRejection"
            class="px-4 py-2 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors"
          >
            Cancel
          </button>
          <button
            @click="confirmRejection"
            :disabled="!rejectionReason.trim() || isProcessing"
            class="px-4 py-2 text-sm bg-red-600 text-white rounded-md hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            Reject
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAdminStore } from '~/stores/admin'

const adminStore = useAdminStore()

// Modal state
const showApprovalModal = ref(false)
const showRejectionModal = ref(false)
const selectedSuggestion = ref<any>(null)
const approvalReason = ref('')
const rejectionReason = ref('')
const isProcessing = ref(false)

// Load suggestions on mount
onMounted(async () => {
  await adminStore.fetchSuggestions()
})

// Refresh suggestions function
const refreshSuggestions = async () => {
  await adminStore.fetchSuggestions()
}

// Approve suggestion
const approveSuggestion = (suggestion: any) => {
  selectedSuggestion.value = suggestion
  approvalReason.value = ''
  showApprovalModal.value = true
}

const confirmApproval = async () => {
  if (!selectedSuggestion.value) return
  
  try {
    isProcessing.value = true
    await adminStore.approveSuggestion(selectedSuggestion.value.id, approvalReason.value)
    showApprovalModal.value = false
    selectedSuggestion.value = null
    approvalReason.value = ''
  } catch (error) {
    console.error('Approve suggestion error:', error)
  } finally {
    isProcessing.value = false
  }
}

const cancelApproval = () => {
  showApprovalModal.value = false
  selectedSuggestion.value = null
  approvalReason.value = ''
}

// Reject suggestion
const rejectSuggestion = (suggestion: any) => {
  selectedSuggestion.value = suggestion
  rejectionReason.value = ''
  showRejectionModal.value = true
}

const confirmRejection = async () => {
  if (!selectedSuggestion.value || !rejectionReason.value.trim()) return
  
  try {
    isProcessing.value = true
    await adminStore.rejectSuggestion(selectedSuggestion.value.id, rejectionReason.value)
    showRejectionModal.value = false
    selectedSuggestion.value = null
    rejectionReason.value = ''
  } catch (error) {
    console.error('Reject suggestion error:', error)
  } finally {
    isProcessing.value = false
  }
}

const cancelRejection = () => {
  showRejectionModal.value = false
  selectedSuggestion.value = null
  rejectionReason.value = ''
}

// Format date helper
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>

<style scoped>
.phrase-suggestions-tab {
  @apply space-y-6;
}
</style>
