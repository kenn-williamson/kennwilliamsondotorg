<template>
  <div class="random-phrase">
    <!-- Loading State -->
    <div v-if="pending" class="phrase-loading">
      <div class="loading-dots">
        <span></span>
        <span></span>
        <span></span>
      </div>
      <p class="loading-text">Loading wisdom...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="phrase-error">
      <p class="error-text">Vigilance Maintained - Until the Next Challenge Arises</p>
    </div>

    <!-- Phrase Display -->
    <div v-else-if="phraseData" class="phrase-content">
      <p class="phrase-text">{{ phraseData }}</p>
    </div>

    <!-- Fallback -->
    <div v-else class="phrase-fallback">
      <p class="phrase-text">Vigilance Maintained - Until the Next Challenge Arises</p>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  userSlug?: string // Optional user slug - if not provided, uses authenticated endpoint
  refreshInterval?: number // Optional refresh interval in milliseconds
}

const props = withDefaults(defineProps<Props>(), {
  userSlug: undefined,
  refreshInterval: 0 // 0 means no auto-refresh
})

// Smart phrase fetching - uses public endpoint if userSlug provided, auth endpoint otherwise
const { data: phraseData, pending, error, refresh } = await useFetch<string>(
  () => props.userSlug ? `/api/${props.userSlug}/phrase` : '/api/phrases/random',
  {
    key: props.userSlug ? `phrase-${props.userSlug}` : 'phrase-auth',
    server: true // Enable SSR
  }
)

// Auto-refresh functionality
let refreshTimer: NodeJS.Timeout | null = null

onMounted(() => {
  if (props.refreshInterval > 0) {
    refreshTimer = setInterval(() => {
      refresh()
    }, props.refreshInterval)
  }
})

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
  }
})

// Expose refresh method for manual refresh
defineExpose({
  refresh
})
</script>

<style scoped>
.random-phrase {
  width: 100%;
  text-align: center;
}

.phrase-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 16px;
}

.loading-dots {
  display: flex;
  gap: 4px;
}

.loading-dots span {
  width: 8px;
  height: 8px;
  background: #FFD700;
  border-radius: 50%;
  animation: loading-bounce 1.4s ease-in-out infinite both;
}

.loading-dots span:nth-child(1) {
  animation-delay: -0.32s;
}

.loading-dots span:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes loading-bounce {
  0%, 80%, 100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

.loading-text {
  font-family: 'Georgia', serif;
  font-size: 14px;
  color: #D4AF37;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  margin: 0;
}

.phrase-error {
  padding: 16px;
}

.error-text {
  font-family: 'Georgia', serif;
  font-size: 18px;
  font-weight: bold;
  color: #8B7355;
  text-shadow: 
    -1px -1px 0px rgba(255, 255, 255, 0.3),
    -0.5px -0.5px 0px rgba(255, 255, 255, 0.2),
    1px 1px 0px rgba(0, 0, 0, 0.4),
    0.5px 0.5px 0px rgba(0, 0, 0, 0.3);
  letter-spacing: 1px;
  line-height: 1.3;
  margin: 0;
}

.phrase-content {
  padding: 16px;
}

.phrase-text {
  font-family: 'Georgia', serif;
  font-size: 18px;
  font-weight: bold;
  color: #8B7355;
  text-shadow: 
    -1px -1px 0px rgba(255, 255, 255, 0.3),
    -0.5px -0.5px 0px rgba(255, 255, 255, 0.2),
    1px 1px 0px rgba(0, 0, 0, 0.4),
    0.5px 0.5px 0px rgba(0, 0, 0, 0.3);
  letter-spacing: 1px;
  line-height: 1.3;
  margin: 0 0 8px 0;
}


.phrase-fallback {
  padding: 16px;
}

/* Responsive Design */
@media (max-width: 768px) {
  .phrase-text,
  .error-text {
    font-size: 16px;
  }
  
  .loading-text {
    font-size: 12px;
  }
}

@media (max-width: 480px) {
  .phrase-text,
  .error-text {
    font-size: 14px;
  }
  
  .loading-text {
    font-size: 11px;
  }
}
</style>
