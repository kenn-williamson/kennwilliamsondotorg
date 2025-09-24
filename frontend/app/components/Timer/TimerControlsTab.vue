<template>
  <div class="timer-controls-tab">
    <div class="tab-content">
      <!-- Quick Actions -->
      <div class="quick-actions-section">
        <h3 class="section-title">Quick Actions</h3>
        <div class="action-buttons">
          <button
            @click="resetTimer"
            class="action-button primary"
            :disabled="isResetting"
          >
            <span v-if="isResetting">Resetting...</span>
            <span v-else>Reset Timer</span>
          </button>
        </div>
      </div>

      <!-- Timer History -->
      <TimerList />
    </div>

    <!-- Reset Timer Modal -->
    <TimerResetModal
      :show="showResetModal"
      @close="closeResetModal"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import TimerList from '~/components/Timer/TimerList.vue'
import TimerResetModal from '~/components/Timer/TimerResetModal.vue'

const isResetting = ref(false)
const showResetModal = ref(false)

const resetTimer = () => {
  showResetModal.value = true
}

const closeResetModal = () => {
  showResetModal.value = false
}
</script>

<style scoped>
.timer-controls-tab {
  @apply p-6;
}

.tab-content {
  @apply max-w-4xl mx-auto;
}

.quick-actions-section {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold text-gray-900 mb-4;
}

.action-buttons {
  @apply flex gap-4;
}

.action-button {
  @apply px-6 py-3 rounded-lg font-medium transition-colors;
}

.action-button.primary {
  @apply bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed;
}

.history-section {
  @apply border-t pt-6;
}

.loading-state,
.empty-state {
  @apply text-center py-8;
}

.timer-list {
  @apply space-y-4;
}
</style>
