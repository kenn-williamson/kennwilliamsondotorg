<template>
  <div class="phrase-suggestions-tab">
    <div class="tab-content">
      <div class="suggestion-form">
        <h3 class="section-title">Suggest a New Phrase</h3>
        <p class="section-description">
          Submit a motivational phrase that could appear on timer displays. 
          Your suggestion will be reviewed before being added to the system.
        </p>

        <form @submit.prevent="submitSuggestion" class="form">
          <div class="form-group">
            <label for="phrase-text" class="form-label">
              Phrase Text
            </label>
            <Field
              id="phrase-text"
              name="phraseText"
              as="textarea"
              class="form-textarea"
              :class="{ 'border-red-500': errors.phraseText }"
              rows="3"
              placeholder="Enter your motivational phrase here..."
              :maxlength="maxPhraseLength"
            />
            <div class="form-footer">
              <div class="character-count">
                {{ (phraseText as string)?.length || 0 }}/{{ maxPhraseLength }} characters
              </div>
              <ErrorMessage name="phraseText" class="error-message" />
            </div>
          </div>

          <div class="form-actions">
            <button
              type="submit"
              class="submit-button"
              :disabled="isSubmitting"
            >
              <span v-if="isSubmitting">Submitting...</span>
              <span v-else>Submit Suggestion</span>
            </button>
            <button
              type="button"
              class="clear-button"
              @click="clearForm"
              :disabled="isSubmitting"
            >
              Clear
            </button>
          </div>
        </form>
      </div>

      <!-- Recent Submissions -->
      <div v-if="phrasesStore.userSuggestions.length > 0" class="recent-submissions">
        <h3 class="section-title">Your Recent Submissions</h3>
        <div class="submission-list">
          <div
            v-for="suggestion in phrasesStore.userSuggestions"
            :key="suggestion.id"
            class="submission-item"
          >
            <div class="submission-content">
              <p class="submission-text">"{{ suggestion.phrase_text }}"</p>
              <div class="submission-meta">
                <span class="submission-status" :class="`status-${suggestion.status}`">
                  {{ suggestion.status }}
                </span>
                <span class="submission-date">
                  {{ formatDate(suggestion.created_at) }}
                </span>
              </div>
              <div v-if="suggestion.admin_reason" class="admin-feedback">
                <strong>Admin feedback:</strong> {{ suggestion.admin_reason }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useForm, useField, Field, ErrorMessage } from 'vee-validate'
import { phraseSuggestionSchema } from '#shared/schemas'
import { usePhrasesStore } from '~/stores/phrases'
import type { PhraseSuggestion } from '#shared/types/phrases'

const phrasesStore = usePhrasesStore()

const maxPhraseLength = 200

// VeeValidate form setup
const { handleSubmit, resetForm, errors } = useForm({
  validationSchema: phraseSuggestionSchema,
  initialValues: {
    phraseText: ''
  }
})

const { value: phraseText } = useField('phraseText')

const isSubmitting = ref(false)

const loadRecentSuggestions = async () => {
  try {
    await phrasesStore.loadSuggestionsForUser()
  } catch (error) {
    console.error('Error loading recent suggestions:', error)
  }
}

// ✅ CORRECT: Use callOnce to prevent double execution during SSR/hydration
await callOnce('user-phrase-suggestions', () => loadRecentSuggestions())

const submitSuggestion = handleSubmit(async (values) => {
  isSubmitting.value = true

  try {
    await phrasesStore.submitSuggestion(values.phraseText.trim())

    // Clear form - store will automatically update with new suggestion
    clearForm()

    // Show success message (could be a toast in the future)
    alert('Suggestion submitted successfully!')

  } catch (error) {
    console.error('Error submitting suggestion:', error)
    alert('Error submitting suggestion. Please try again.')
  } finally {
    isSubmitting.value = false
  }
})

const clearForm = () => {
  resetForm()
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
.phrase-suggestions-tab {
  @apply p-6;
}

.tab-content {
  @apply max-w-2xl mx-auto;
}

.suggestion-form {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold text-gray-900 mb-2;
}

.section-description {
  @apply text-gray-600 mb-6;
}

.form {
  @apply space-y-6;
}

.form-group {
  @apply space-y-2;
}

.form-label {
  @apply block text-sm font-medium text-gray-700;
}

.form-textarea {
  @apply w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm
         focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500
         resize-y;
}

.form-footer {
  @apply flex justify-between items-start;
}

.character-count {
  @apply text-sm text-gray-500;
}

.error-message {
  @apply text-sm text-red-600;
}

.form-actions {
  @apply flex gap-4;
}

.submit-button {
  @apply bg-blue-600 text-white px-6 py-2 rounded-md font-medium
         hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed
         transition-colors;
}

.clear-button {
  @apply bg-gray-200 text-gray-800 px-6 py-2 rounded-md font-medium
         hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed
         transition-colors;
}

.recent-submissions {
  @apply border-t pt-6;
}

.submission-list {
  @apply space-y-4;
}

.submission-item {
  @apply bg-gray-50 p-4 rounded-lg;
}

.submission-content {
  @apply space-y-2;
}

.submission-text {
  @apply text-gray-900 font-medium;
}

.submission-meta {
  @apply flex gap-4 text-sm;
}

.submission-status {
  @apply px-2 py-1 rounded-full text-xs font-medium;
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

.submission-date {
  @apply text-gray-500;
}

.admin-feedback {
  @apply text-sm text-gray-600 bg-white p-2 rounded border-l-4 border-blue-200;
}
</style>
