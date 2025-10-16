<template>
  <div class="access-requests-tab">
    <!-- Loading State -->
    <div v-if="adminStore.isLoading" class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
    </div>

    <!-- Error State -->
    <div v-else-if="adminStore.error" class="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
      <p class="text-red-800 text-sm">{{ adminStore.error }}</p>
      <button
        @click="refreshAccessRequests"
        class="mt-2 text-sm text-red-600 hover:text-red-700 underline"
      >
        Try again
      </button>
    </div>

    <!-- Access Requests List -->
    <div v-else-if="adminStore.pendingAccessRequests.length > 0" class="space-y-4">
      <div
        v-for="request in adminStore.pendingAccessRequests"
        :key="request.id"
        class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <div class="flex items-center space-x-3 mb-3">
              <div class="flex-shrink-0">
                <div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
                  <span class="text-purple-600 text-sm">🔑</span>
                </div>
              </div>
              <div class="flex-1">
                <h3 class="text-sm font-medium text-gray-900">{{ request.user_display_name }}</h3>
                <p class="text-xs text-gray-500">{{ request.user_email }} • {{ formatDate(request.created_at) }}</p>
              </div>
            </div>

            <div class="bg-gray-50 rounded-md p-4 mb-4">
              <p class="text-sm text-gray-600 mb-2"><strong>Requested Role:</strong> {{ request.requested_role }}</p>
              <p class="text-sm text-gray-900 whitespace-pre-wrap">{{ request.message }}</p>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex items-center justify-between pt-4 border-t border-gray-200">
          <div class="flex items-center space-x-3">
            <button
              @click="approveRequest(request)"
              :disabled="isProcessing"
              class="px-4 py-2 text-sm bg-green-100 text-green-700 rounded-md hover:bg-green-200 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              Approve
            </button>
            <button
              @click="rejectRequest(request)"
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
      <div class="text-gray-400 text-6xl mb-4">🔑</div>
      <h3 class="text-lg font-medium text-gray-900 mb-2">No Pending Access Requests</h3>
      <p class="text-gray-500 mb-4">All access requests have been reviewed.</p>
      <button
        @click="refreshAccessRequests"
        class="px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
      >
        Refresh
      </button>
    </div>

    <!-- Approval Modal -->
    <div v-if="showApprovalModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Approve Access Request</h3>
        <p class="text-sm text-gray-600 mb-4">
          Grant <strong>{{ selectedRequest?.requested_role }}</strong> access to <strong>{{ selectedRequest?.user_display_name }}</strong>?
        </p>
        <div class="bg-gray-50 rounded-md p-3 mb-4">
          <p class="text-sm text-gray-600 mb-1"><strong>Email:</strong> {{ selectedRequest?.user_email }}</p>
          <p class="text-sm text-gray-900 whitespace-pre-wrap mt-2">{{ selectedRequest?.message }}</p>
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
            placeholder="Add a comment about why this request was approved..."
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
        <h3 class="text-lg font-medium text-gray-900 mb-4">Reject Access Request</h3>
        <p class="text-sm text-gray-600 mb-4">
          Reject access request from <strong>{{ selectedRequest?.user_display_name }}</strong>?
        </p>
        <div class="bg-gray-50 rounded-md p-3 mb-4">
          <p class="text-sm text-gray-600 mb-1"><strong>Email:</strong> {{ selectedRequest?.user_email }}</p>
          <p class="text-sm text-gray-900 whitespace-pre-wrap mt-2">{{ selectedRequest?.message }}</p>
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
            placeholder="Explain why this request is being rejected..."
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
import type { AccessRequestWithUser } from '#shared/types'

const adminStore = useAdminStore()

// Modal state
const showApprovalModal = ref(false)
const showRejectionModal = ref(false)
const selectedRequest = ref<AccessRequestWithUser | null>(null)
const approvalReason = ref('')
const rejectionReason = ref('')
const isProcessing = ref(false)

// Load access requests directly in setup. This runs ON THE SERVER.
// Nuxt will wait for this to complete before sending the page.
console.log('🔄 Loading admin access requests for AccessRequestsTab...')
await adminStore.fetchAccessRequests()

// Refresh access requests function
const refreshAccessRequests = async () => {
  await adminStore.fetchAccessRequests()
}

// Approve request
const approveRequest = (request: AccessRequestWithUser) => {
  selectedRequest.value = request
  approvalReason.value = ''
  showApprovalModal.value = true
}

const confirmApproval = async () => {
  if (!selectedRequest.value) return

  try {
    isProcessing.value = true
    await adminStore.approveAccessRequest(selectedRequest.value.id, approvalReason.value)
    showApprovalModal.value = false
    selectedRequest.value = null
    approvalReason.value = ''
  } catch (error) {
    console.error('Approve access request error:', error)
  } finally {
    isProcessing.value = false
  }
}

const cancelApproval = () => {
  showApprovalModal.value = false
  selectedRequest.value = null
  approvalReason.value = ''
}

// Reject request
const rejectRequest = (request: AccessRequestWithUser) => {
  selectedRequest.value = request
  rejectionReason.value = ''
  showRejectionModal.value = true
}

const confirmRejection = async () => {
  if (!selectedRequest.value || !rejectionReason.value.trim()) return

  try {
    isProcessing.value = true
    await adminStore.rejectAccessRequest(selectedRequest.value.id, rejectionReason.value)
    showRejectionModal.value = false
    selectedRequest.value = null
    rejectionReason.value = ''
  } catch (error) {
    console.error('Reject access request error:', error)
  } finally {
    isProcessing.value = false
  }
}

const cancelRejection = () => {
  showRejectionModal.value = false
  selectedRequest.value = null
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
.access-requests-tab {
  @apply space-y-6;
}
</style>
