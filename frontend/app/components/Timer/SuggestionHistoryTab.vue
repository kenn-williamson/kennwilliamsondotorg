<template>
  <div class="suggestion-history-tab">
    <div class="tab-content">
      <div class="history-header">
        <h3 class="section-title">Your Phrase Suggestions</h3>
        <p class="section-description">
          Track the status of your phrase suggestions and view admin feedback.
        </p>
      </div>

      <!-- Filter Controls -->
      <div class="filter-controls">
        <div class="status-filter">
          <label for="status-filter" class="filter-label">Filter by status:</label>
          <select
            id="status-filter"
            v-model="statusFilter"
            class="filter-select"
            @change="filterSuggestions"
          >
            <option value="">All</option>
            <option value="pending">Pending</option>
            <option value="approved">Approved</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>
        <div class="search-box">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search suggestions..."
            class="search-input"
            @input="filterSuggestions"
          />
        </div>
      </div>

      <!-- Suggestions List -->
      <div v-if="phrasesStore.isLoading" class="loading-state">
        <p>Loading your suggestions...</p>
      </div>

      <div v-else-if="filteredSuggestions.length === 0" class="empty-state">
        <p v-if="searchQuery || statusFilter" class="text-gray-500">
          No suggestions match your filters
        </p>
        <p v-else class="text-gray-500">
          You haven't submitted any phrase suggestions yet
        </p>
        <p class="text-sm text-gray-400 mt-2">
          Go to the "Suggest Phrases" tab to submit your first suggestion
        </p>
      </div>

      <div v-else class="suggestions-list">
        <div
          v-for="suggestion in filteredSuggestions"
          :key="suggestion.id"
          class="suggestion-item"
        >
          <div class="suggestion-content">
            <div class="suggestion-header">
              <div class="suggestion-text">
                "{{ suggestion.phrase_text }}"
              </div>
              <div class="suggestion-status" :class="`status-${suggestion.status}`">
                {{ suggestion.status }}
              </div>
            </div>
            
            <div class="suggestion-meta">
              <div class="meta-item">
                <span class="meta-label">Submitted:</span>
                <span class="meta-value">{{ formatDate(suggestion.created_at) }}</span>
              </div>
              
              <div v-if="suggestion.updated_at !== suggestion.created_at" class="meta-item">
                <span class="meta-label">Updated:</span>
                <span class="meta-value">{{ formatDate(suggestion.updated_at) }}</span>
              </div>
              
              <div v-if="suggestion.admin_id" class="meta-item">
                <span class="meta-label">Reviewed by:</span>
                <span class="meta-value">Admin</span>
              </div>
            </div>

            <div v-if="suggestion.admin_reason" class="admin-feedback">
              <div class="feedback-header">
                <strong>Admin Feedback:</strong>
              </div>
              <div class="feedback-content">
                {{ suggestion.admin_reason }}
              </div>
            </div>

            <div v-if="suggestion.status === 'rejected'" class="action-buttons">
              <button
                @click="editSuggestion(suggestion)"
                class="action-button edit"
                :disabled="isEditing"
              >
                Edit & Resubmit
              </button>
              <button
                @click="deleteSuggestion(suggestion.id)"
                class="action-button delete"
                :disabled="isDeleting"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePhrasesStore } from '~/stores/phrases'
import type { PhraseSuggestion } from '#shared/types/phrases'

const phrasesStore = usePhrasesStore()

const statusFilter = ref('')
const searchQuery = ref('')
const isEditing = ref(false)
const isDeleting = ref(false)

const loadSuggestions = async () => {
  try {
    await phrasesStore.loadSuggestionsForUser()
  } catch (error) {
    console.error('Error loading suggestions:', error)
  }
}

// ✅ CORRECT: Use callOnce to prevent double execution during SSR/hydration
await callOnce('user-suggestion-history', () => loadSuggestions())

const filteredSuggestions = computed(() => {
  let filtered = phrasesStore.userSuggestions

  // Filter by status
  if (statusFilter.value) {
    filtered = filtered.filter((s: PhraseSuggestion) => s.status === statusFilter.value)
  }

  // Filter by search query
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((s: PhraseSuggestion) => 
      s.phrase_text.toLowerCase().includes(query)
    )
  }

  // Sort by created_at descending (newest first)
  return filtered.sort((a: PhraseSuggestion, b: PhraseSuggestion) => 
    new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
  )
})

const filterSuggestions = () => {
  // Filtering is handled by computed property
}

const editSuggestion = (suggestion: any) => {
  // For now, just show the suggestion text for editing
  // In the future, this could open a modal or navigate to edit form
  const newText = prompt('Edit your phrase suggestion:', suggestion.phrase_text)
  if (newText && newText.trim() !== suggestion.phrase_text) {
    resubmitSuggestion(suggestion.id, newText.trim())
  }
}

const resubmitSuggestion = async (suggestionId: string, newText: string) => {
  isEditing.value = true
  try {
    // This would need to be implemented in the backend
    // For now, we'll just show an alert
    alert('Edit functionality will be implemented in the next phase')
  } catch (error) {
    console.error('Error editing suggestion:', error)
    alert('Error editing suggestion. Please try again.')
  } finally {
    isEditing.value = false
  }
}

const deleteSuggestion = async (suggestionId: string) => {
  if (!confirm('Are you sure you want to delete this suggestion?')) {
    return
  }

  isDeleting.value = true
  try {
    // This would need to be implemented in the backend
    // For now, we'll just show an alert
    alert('Delete functionality will be implemented in the next phase')
  } catch (error) {
    console.error('Error deleting suggestion:', error)
    alert('Error deleting suggestion. Please try again.')
  } finally {
    isDeleting.value = false
  }
}

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
.suggestion-history-tab {
  @apply p-6;
  background: linear-gradient(145deg, #f5f0e8 0%, #faf7f0 50%, #f5f0e8 100%);
  border-radius: 0 0 8px 8px;
  box-shadow: inset 0 2px 4px rgba(139, 69, 19, 0.1);
}

.tab-content {
  @apply max-w-4xl mx-auto;
}

.history-header {
  @apply mb-6;
}

.section-title {
  @apply text-xl font-semibold mb-2;
  color: #5d3820;
  font-family: Georgia, serif;
  text-shadow: 0 1px 1px rgba(255, 255, 255, 0.5);
}

.section-description {
  color: #8B6914;
}

.filter-controls {
  @apply flex gap-4 mb-6;
}

.status-filter {
  @apply flex items-center gap-2;
}

.filter-label {
  @apply text-sm font-medium;
  color: #5d3820;
}

.filter-select {
  @apply px-3 py-2 rounded-md shadow-sm;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid rgba(139, 69, 19, 0.3);
  color: #3c2414;
}

.filter-select:focus {
  @apply outline-none;
  border-color: #B8860B;
  box-shadow: 0 0 0 3px rgba(184, 134, 11, 0.1);
}

.search-box {
  @apply flex-1 max-w-md;
}

.search-input {
  @apply w-full px-3 py-2 rounded-md shadow-sm;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid rgba(139, 69, 19, 0.3);
  color: #3c2414;
}

.search-input:focus {
  @apply outline-none;
  border-color: #B8860B;
  box-shadow: 0 0 0 3px rgba(184, 134, 11, 0.1);
}

.loading-state,
.empty-state {
  @apply text-center py-8;
  color: #8B6914;
}

.suggestions-list {
  @apply space-y-4;
}

.suggestion-item {
  @apply rounded-lg p-4;
  background: rgba(255, 255, 255, 0.7);
  border: 2px solid rgba(139, 69, 19, 0.2);
}

.suggestion-content {
  @apply space-y-3;
}

.suggestion-header {
  @apply flex justify-between items-start;
}

.suggestion-text {
  @apply font-medium text-lg flex-1;
  color: #3c2414;
}

.suggestion-status {
  @apply px-3 py-1 rounded-full text-sm font-medium;
}

.status-pending {
  @apply bg-yellow-100 text-yellow-800;
}

.status-approved {
  @apply bg-green-100 text-green-800;
}

.status-rejected {
  @apply bg-red-100 text-red-800;
}

.suggestion-meta {
  @apply flex flex-wrap gap-4 text-sm;
  color: #8B6914;
}

.meta-item {
  @apply flex gap-1;
}

.meta-label {
  @apply font-medium;
}

.meta-value {
  color: #8B6914;
}

.admin-feedback {
  @apply p-3 rounded border-l-4;
  background: rgba(255, 255, 255, 0.8);
  border-color: #B8860B;
}

.feedback-header {
  @apply text-sm font-medium mb-1;
  color: #5d3820;
}

.feedback-content {
  @apply text-sm;
  color: #5d3820;
}

.action-buttons {
  @apply flex gap-2 pt-2;
}

.action-button {
  @apply px-3 py-1 text-sm font-medium rounded transition-colors
         disabled:opacity-50 disabled:cursor-not-allowed;
}

.action-button.edit {
  @apply bg-blue-100 text-blue-700;
}

.action-button.edit:hover:not(:disabled) {
  @apply bg-blue-200;
}

.action-button.delete {
  @apply bg-red-100 text-red-700;
}

.action-button.delete:hover:not(:disabled) {
  @apply bg-red-200;
}
</style>
