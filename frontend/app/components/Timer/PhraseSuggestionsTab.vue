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
  background: linear-gradient(145deg, #f5f0e8 0%, #faf7f0 50%, #f5f0e8 100%);
  border-radius: 0 0 8px 8px;
  box-shadow: inset 0 2px 4px rgba(139, 69, 19, 0.1);
}

.tab-content {
  @apply max-w-2xl mx-auto;
}

.suggestion-form {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold mb-2;
  color: #5d3820;
  font-family: Georgia, serif;
  text-shadow: 0 1px 1px rgba(255, 255, 255, 0.5);
}

.section-description {
  @apply mb-6;
  color: #8B6914;
}

.form {
  @apply space-y-6;
}

.form-group {
  @apply space-y-2;
}

.form-label {
  @apply block text-sm font-medium;
  color: #5d3820;
}

.form-textarea {
  @apply w-full px-3 py-2 rounded-md shadow-sm resize-y;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid rgba(139, 69, 19, 0.3);
  color: #3c2414;
}

.form-textarea:focus {
  @apply outline-none;
  border-color: #B8860B;
  box-shadow: 0 0 0 3px rgba(184, 134, 11, 0.1);
}

.form-footer {
  @apply flex justify-between items-start;
}

.character-count {
  @apply text-sm;
  color: #8B6914;
}

.error-message {
  @apply text-sm text-red-600;
}

.form-actions {
  @apply flex gap-4;
}

.submit-button {
  @apply px-6 py-2 rounded-md font-medium transition-all;
  background: linear-gradient(145deg, #B8860B 0%, #DAA520 50%, #B8860B 100%);
  border: 2px solid #8B6914;
  color: #1a0900;
  font-weight: bold;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.2);
}

.submit-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    0 6px 12px rgba(0, 0, 0, 0.3);
}

.submit-button:disabled {
  @apply opacity-50 cursor-not-allowed;
}

.clear-button {
  @apply px-6 py-2 rounded-md font-medium transition-all;
  background: rgba(139, 69, 19, 0.2);
  border: 2px solid rgba(139, 69, 19, 0.3);
  color: #5d3820;
}

.clear-button:hover:not(:disabled) {
  background: rgba(139, 69, 19, 0.3);
}

.clear-button:disabled {
  @apply opacity-50 cursor-not-allowed;
}

.recent-submissions {
  @apply border-t pt-6;
  border-color: rgba(139, 69, 19, 0.2);
}

.submission-list {
  @apply space-y-4;
}

.submission-item {
  @apply p-4 rounded-lg;
  background: rgba(255, 255, 255, 0.6);
  border: 1px solid rgba(139, 69, 19, 0.2);
}

.submission-content {
  @apply space-y-2;
}

.submission-text {
  @apply font-medium;
  color: #3c2414;
}

.submission-meta {
  @apply flex gap-4 text-sm;
}

.submission-status {
  @apply px-2 py-1 rounded-full text-xs font-medium;
}

.status-pending {
  @apply bg-gold-100 text-gold-800;
}

.status-approved {
  @apply bg-green-100 text-green-800;
}

.status-rejected {
  @apply bg-red-100 text-red-800;
}

.submission-date {
  color: #8B6914;
}

.admin-feedback {
  @apply text-sm p-2 rounded border-l-4;
  color: #5d3820;
  background: rgba(255, 255, 255, 0.8);
  border-color: #B8860B;
}
</style>
