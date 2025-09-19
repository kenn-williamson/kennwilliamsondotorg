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
            @input="filterPhrases"
          />
        </div>
        <div class="filter-stats">
          <span class="stat">
            Showing {{ filteredPhrases.length }} of {{ phrasesStore.userPhrases.length }} phrases
          </span>
        </div>
      </div>

      <!-- Phrases Table -->
      <div v-if="phrasesStore.isLoading" class="loading-state">
        <p>Loading phrases...</p>
      </div>

      <div v-else-if="filteredPhrases.length === 0" class="empty-state">
        <p v-if="searchQuery" class="text-gray-500">
          No phrases match your search
        </p>
        <p v-else class="text-gray-500">
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
            v-for="phrase in filteredPhrases"
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
                @click="togglePhraseExclusion(phrase.id)"
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
import { ref, computed, onMounted } from 'vue'
import { usePhrasesStore } from '~/stores/phrases'
import type { PhraseWithExclusion } from '#shared/types/phrases'

const phrasesStore = usePhrasesStore()

const searchQuery = ref('')
const togglingPhrases = ref(new Set<string>())

const filteredPhrases = computed(() => {
  if (!searchQuery.value.trim()) {
    return phrasesStore.userPhrases
  }
  
  const query = searchQuery.value.toLowerCase()
  return phrasesStore.userPhrases.filter(phrase => 
    phrase.phrase_text.toLowerCase().includes(query)
  )
})

onMounted(async () => {
  await loadData()
})

const loadData = async () => {
  try {
    await phrasesStore.loadPhrasesForUser()
  } catch (error) {
    console.error('Error loading phrase data:', error)
  }
}

const filterPhrases = () => {
  // Filtering is handled by computed property
}

const isPhraseExcluded = (phraseId: string) => {
  const phrase = phrasesStore.userPhrases.find(p => p.id === phraseId)
  return phrase?.is_excluded || false
}

const togglePhraseExclusion = async (phraseId: string) => {
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
}

.tab-content {
  @apply max-w-6xl mx-auto;
}

.filter-header {
  @apply mb-6;
}

.section-title {
  @apply text-xl font-semibold text-gray-900 mb-2;
}

.section-description {
  @apply text-gray-600;
}

.filter-controls {
  @apply flex justify-between items-center mb-6;
}

.search-box {
  @apply flex-1 max-w-md;
}

.search-input {
  @apply w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm
         focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500;
}

.filter-stats {
  @apply text-sm text-gray-500;
}

.loading-state,
.empty-state {
  @apply text-center py-8;
}

.phrases-table {
  @apply bg-white border border-gray-200 rounded-lg overflow-hidden;
}

.table-header {
  @apply bg-gray-50 px-6 py-3 flex font-medium text-sm text-gray-700;
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
  @apply divide-y divide-gray-200;
}

.table-row {
  @apply px-6 py-4 flex items-center text-sm;
}

.phrase-text {
  @apply text-gray-900 font-medium;
}

.created-date {
  @apply text-gray-500;
}

.author-name {
  @apply text-gray-600;
}

.toggle-button {
  @apply px-3 py-1 text-xs font-medium rounded-full transition-colors
         border border-gray-300 bg-white text-gray-700
         hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed;
}

.toggle-button.excluded {
  @apply bg-red-100 text-red-700 border-red-300 hover:bg-red-200;
}

.toggle-button:not(.excluded) {
  @apply bg-green-100 text-green-700 border-green-300 hover:bg-green-200;
}
</style>
