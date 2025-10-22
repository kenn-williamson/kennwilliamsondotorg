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
  background: linear-gradient(145deg, #f5f0e8 0%, #faf7f0 50%, #f5f0e8 100%);
  border-radius: 0 0 8px 8px;
  box-shadow: inset 0 2px 4px rgba(139, 69, 19, 0.1);
}

.tab-content {
  @apply max-w-4xl mx-auto;
}

.quick-actions-section {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold mb-4;
  color: #5d3820;
  font-family: Georgia, serif;
  text-shadow: 0 1px 1px rgba(255, 255, 255, 0.5);
}

.action-buttons {
  @apply flex gap-4;
}

.action-button {
  @apply px-6 py-3 rounded-lg font-medium transition-all;
}

.action-button.primary {
  background: linear-gradient(145deg, #B8860B 0%, #DAA520 50%, #B8860B 100%);
  border: 2px solid #8B6914;
  color: #1a0900;
  font-weight: bold;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.2);
}

.action-button.primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    0 6px 12px rgba(0, 0, 0, 0.3);
}

.action-button.primary:disabled {
  @apply opacity-50 cursor-not-allowed;
}

.history-section {
  @apply border-t pt-6;
  border-color: rgba(139, 69, 19, 0.2);
}

.loading-state,
.empty-state {
  @apply text-center py-8;
  color: #8B6914;
}

.timer-list {
  @apply space-y-4;
}
</style>
