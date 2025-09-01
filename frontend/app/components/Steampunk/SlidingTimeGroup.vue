<template>
  <div class="time-group-container">
    <Transition
      name="mechanical-slide"
      @enter="onEnter"
      @leave="onLeave"
    >
      <div 
        v-if="shouldShow"
        class="time-group"
        :class="{ 'entering': isEntering, 'leaving': isLeaving }"
      >
        <!-- Unit Label -->
        <div class="unit-label">
          <span class="label-text">{{ label }}</span>
        </div>
        
        <!-- Digit Container -->
        <div class="digits-container">
          <FlippingDigit 
            v-for="(digit, index) in digits"
            :key="`${label}-${index}`"
            :value="digit"
            class="digit-spacing"
          />
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
const props = defineProps({
  label: {
    type: String,
    required: true
  },
  value: {
    type: Number,
    required: true,
    default: 0
  }
})

const shouldShow = ref(false)
const isEntering = ref(false)
const isLeaving = ref(false)

// Convert value to individual digits - pad the whole value to at least 2 digits
const digits = computed(() => {
  const valueStr = String(props.value).padStart(2, '0')
  return valueStr.split('').map(d => parseInt(d))
})

// Watch for value changes to trigger slide animations
watch(() => props.value, (newValue, oldValue) => {
  const wasZero = oldValue === 0
  const isZero = newValue === 0
  
  if (wasZero && !isZero) {
    // Slide in
    shouldShow.value = true
    nextTick(() => {
      isEntering.value = true
      setTimeout(() => {
        isEntering.value = false
      }, 800)
    })
  } else if (!wasZero && isZero) {
    // Slide out
    isLeaving.value = true
    setTimeout(() => {
      shouldShow.value = false
      isLeaving.value = false
    }, 800)
  }
})

const onEnter = (el) => {
  // Animation handled by CSS classes
}

const onLeave = (el) => {
  // Animation handled by CSS classes
}

// Initialize visibility based on initial value
onMounted(() => {
  shouldShow.value = props.value > 0
})
</script>

<style scoped>
.time-group-container {
  display: inline-block;
  margin: 0 16px;
}

.time-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  transform-origin: center bottom;
}

.unit-label {
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  
  /* Steampunk label styling */
  background: linear-gradient(145deg, #1e3a8a 0%, #1e40af 50%, #1e3a8a 100%);
  border: 2px solid #C0C0C0;
  border-radius: 12px;
  padding: 4px 12px;
  box-shadow: 
    inset 0 1px 2px rgba(255, 255, 255, 0.2),
    inset 0 -1px 2px rgba(0, 0, 0, 0.3),
    0 2px 4px rgba(0, 0, 0, 0.3);
}

  .label-text {
    font-family: 'Georgia', serif;
    font-size: 14px;
    font-weight: bold;
    color: #FFD700;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
    letter-spacing: 0.5px;
  }

.digits-container {
  display: flex;
  gap: 4px;
  align-items: center;
}

.digit-spacing {
  margin: 0 2px;
}

/* Mechanical slide animations */
.mechanical-slide-enter-active,
.mechanical-slide-leave-active {
  transition: all 0.8s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.mechanical-slide-enter-from {
  transform: translateX(120px) scaleX(0.3);
  opacity: 0;
}

.mechanical-slide-leave-to {
  transform: translateX(120px) scaleX(0.3);
  opacity: 0;
}

.time-group.entering {
  animation: slide-in 0.8s cubic-bezier(0.25, 0.8, 0.25, 1) forwards;
}

.time-group.leaving {
  animation: slide-out 0.8s cubic-bezier(0.25, 0.8, 0.25, 1) forwards;
}

@keyframes slide-in {
  0% {
    transform: translateX(120px) scaleX(0.3) rotateY(20deg);
    opacity: 0;
  }
  30% {
    opacity: 0.6;
  }
  70% {
    transform: translateX(-10px) scaleX(1.05) rotateY(-5deg);
  }
  100% {
    transform: translateX(0) scaleX(1) rotateY(0deg);
    opacity: 1;
  }
}

@keyframes slide-out {
  0% {
    transform: translateX(0) scaleX(1) rotateY(0deg);
    opacity: 1;
  }
  30% {
    transform: translateX(-10px) scaleX(1.05) rotateY(-5deg);
    opacity: 0.6;
  }
  100% {
    transform: translateX(120px) scaleX(0.3) rotateY(20deg);
    opacity: 0;
  }
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .time-group-container {
    margin: 0 12px;
  }
  
  .unit-label {
    height: 28px;
    padding: 2px 8px;
  }
  
  .label-text {
    font-size: 12px;
  }
  
  .digits-container {
    gap: 2px;
  }
}

@media (max-width: 480px) {
  .time-group-container {
    margin: 0 8px;
  }
  
  .unit-label {
    height: 24px;
    padding: 2px 6px;
  }
  
  .label-text {
    font-size: 10px;
  }
  
  .mechanical-slide-enter-from,
  .slide-in {
    transform: translateX(80px) scaleX(0.3);
  }
  
  .mechanical-slide-leave-to,
  .slide-out {
    transform: translateX(80px) scaleX(0.3);
  }
}
</style>