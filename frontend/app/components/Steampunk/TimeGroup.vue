<template>
  <div 
    v-if="shouldShow"
    class="time-group-container"
  >
    <div class="time-group">
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
  },
  // Props to determine if higher time units exist
  hasHigherUnits: {
    type: Boolean,
    default: false
  }
})

const shouldShow = ref(props.value > 0 || props.hasHigherUnits)

// Convert value to individual digits - pad the whole value to at least 2 digits
const digits = computed(() => {
  const valueStr = String(props.value).padStart(2, '0')
  return valueStr.split('').map(d => parseInt(d))
})

// Watch for value changes - show if value > 0 OR if higher units exist
watch(() => props.value, (newValue) => {
  shouldShow.value = newValue > 0 || props.hasHigherUnits
}, { immediate: true })

// Watch for hasHigherUnits changes
watch(() => props.hasHigherUnits, (newValue) => {
  shouldShow.value = props.value > 0 || newValue
}, { immediate: true })
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

/* Animations removed */

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
  
  /* Animation styles removed */
}
</style>