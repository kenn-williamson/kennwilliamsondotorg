<template>
  <div class="phrase-filter-tab">
    <div class="tab-content">
      <div class="filter-header">
        <h3 class="section-title">Filter Phrases</h3>
        <p class="section-description">
          Choose which phrases appear in your timer display. 
          Excluded phrases won't be shown in random selection.
        </p>
      </div>

      <!-- Search and Controls -->
      <div class="filter-controls">
        <div class="search-box">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search phrases..."
            class="search-input"
            @input="handleSearchInput"
          />
        </div>
        <div class="filter-stats">
          <span class="stat">
            Showing {{ phrasesStore.userPhrases.length }} phrases
          </span>
        </div>
      </div>

      <!-- Phrases Table -->
      <div v-if="phrasesStore.isLoading" class="loading-state">
        <p>Loading phrases...</p>
      </div>

      <div v-else-if="phrasesStore.userPhrases.length === 0" class="empty-state">
        <p v-if="searchQuery" class="text-nautical-500">
          No phrases match your search
        </p>
        <p v-else class="text-nautical-500">
          No phrases available
        </p>
      </div>

      <div v-else class="phrases-table">
        <div class="table-header">
          <div class="col-phrase">Phrase</div>
          <div class="col-created">Created</div>
          <div class="col-author">Author</div>
          <div class="col-action">Action</div>
        </div>
        
        <div class="table-body">
          <div
            v-for="phrase in phrasesStore.userPhrases"
            :key="phrase.id"
            class="table-row"
          >
            <div class="col-phrase">
              <span class="phrase-text">"{{ phrase.phrase_text }}"</span>
            </div>
            <div class="col-created">
              <span class="created-date">{{ formatDate(phrase.created_at) }}</span>
            </div>
            <div class="col-author">
              <span class="author-name">{{ phrase.created_by === 'system' ? 'System' : 'User' }}</span>
            </div>
            <div class="col-action">
              <button
                @click="handleTogglePhraseExclusion(phrase.id)"
                class="toggle-button"
                :class="{ 
                  'excluded': isPhraseExcluded(phrase.id),
                  'loading': togglingPhrases.has(phrase.id)
                }"
                :disabled="togglingPhrases.has(phrase.id)"
              >
                <span v-if="togglingPhrases.has(phrase.id)">...</span>
                <span v-else-if="isPhraseExcluded(phrase.id)">Show</span>
                <span v-else>Hide</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { usePhrasesStore } from '~/stores/phrases'
import type { PhraseWithExclusion } from '#shared/types/phrases'

const phrasesStore = usePhrasesStore()

const searchQuery = ref('')
const togglingPhrases = ref(new Set<string>())

// Load phrases on component mount
await phrasesStore.loadPhrasesForUser()

// Handle search input with debouncing (handled by store)
const handleSearchInput = () => {
  phrasesStore.searchPhrases(searchQuery.value)
}

// Sync local search query with store
watch(() => phrasesStore.searchQuery, (newQuery) => {
  searchQuery.value = newQuery
}, { immediate: true })

const isPhraseExcluded = (phraseId: string) => {
  const phrase = phrasesStore.userPhrases.find((p: PhraseWithExclusion) => p.id === phraseId)
  return phrase?.is_excluded || false
}

const handleTogglePhraseExclusion = async (phraseId: string) => {
  togglingPhrases.value.add(phraseId)
  
  try {
    await phrasesStore.togglePhraseExclusion(phraseId)
  } catch (error) {
    console.error('Error toggling phrase exclusion:', error)
    alert('Error updating phrase filter. Please try again.')
  } finally {
    togglingPhrases.value.delete(phraseId)
  }
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}
</script>

<style scoped>
.phrase-filter-tab {
  @apply p-6;
  background: linear-gradient(145deg, #f5f0e8 0%, #faf7f0 50%, #f5f0e8 100%);
  border-radius: 0 0 8px 8px;
  box-shadow: inset 0 2px 4px rgba(139, 69, 19, 0.1);
}

.tab-content {
  @apply max-w-6xl mx-auto;
}

.filter-header {
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
  @apply flex justify-between items-center mb-6;
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

.filter-stats {
  @apply text-sm;
  color: #8B6914;
}

.loading-state,
.empty-state {
  @apply text-center py-8;
  color: #8B6914;
}

.phrases-table {
  @apply rounded-lg overflow-hidden;
  background: rgba(255, 255, 255, 0.7);
  border: 2px solid rgba(139, 69, 19, 0.2);
}

.table-header {
  @apply px-6 py-3 flex font-medium text-sm;
  background: rgba(139, 69, 19, 0.15);
  color: #5d3820;
}

.col-phrase {
  @apply flex-1;
}

.col-created {
  @apply w-24;
}

.col-author {
  @apply w-32;
}

.col-action {
  @apply w-20 text-center;
}

.table-body {
  @apply divide-y;
  divide-color: rgba(139, 69, 19, 0.1);
}

.table-row {
  @apply px-6 py-4 flex items-center text-sm;
}

.phrase-text {
  @apply font-medium;
  color: #3c2414;
}

.created-date {
  color: #8B6914;
}

.author-name {
  color: #8B6914;
}

.toggle-button {
  @apply px-3 py-1 text-xs font-medium rounded-full transition-colors
         disabled:opacity-50 disabled:cursor-not-allowed;
  border: 2px solid rgba(139, 69, 19, 0.3);
  background: rgba(255, 255, 255, 0.9);
  color: #5d3820;
}

.toggle-button:hover:not(:disabled) {
  background: rgba(255, 255, 255, 1);
}

.toggle-button.excluded {
  @apply bg-red-100 text-red-700 border-red-300;
}

.toggle-button.excluded:hover:not(:disabled) {
  @apply bg-red-200;
}

.toggle-button:not(.excluded) {
  @apply bg-green-100 text-green-700 border-green-300;
}

.toggle-button:not(.excluded):hover:not(:disabled) {
  @apply bg-green-200;
}
</style>
