<template>
  <div 
    class="flip-clock-piece"
    :class="{ 'flip': isFlipping }"
    @animationend="handleAnimationEnd"
  >
    <div class="card">
      <!-- Top half - shows current number -->
      <div class="card__top">{{ displayValue }}</div>
      
      <!-- Bottom half - shows current number bottom -->
      <div class="card__bottom" :data-value="displayValue"></div>
      
      <!-- Back card for animation - shows next number -->
      <div class="card__back" :data-value="currentBackValue">
        <div class="card__bottom" :data-value="nextValue"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  value: {
    type: [String, Number],
    required: true
  }
})

const displayValue = ref(String(props.value))
const nextValue = ref(String(props.value))
const currentBackValue = ref(String(props.value))
const isFlipping = ref(false)

// Watch for value changes and trigger flip animation
watch(() => props.value, (newValue, oldValue) => {
  if (newValue !== oldValue) {
    const newValueStr = String(newValue)
    
    if (displayValue.value !== newValueStr) {
      // Set up the back card with old value and new bottom
      currentBackValue.value = displayValue.value
      nextValue.value = newValueStr
      
      // Trigger flip animation
      isFlipping.value = true
    }
  }
})

const handleAnimationEnd = () => {
  if (isFlipping.value) {
    // Update to new value
    displayValue.value = nextValue.value
    currentBackValue.value = nextValue.value
    
    // Reset animation
    isFlipping.value = false
  }
}

// Initialize display
onMounted(() => {
  const valueStr = String(props.value)
  displayValue.value = valueStr
  nextValue.value = valueStr
  currentBackValue.value = valueStr
})
</script>

<style scoped>
.flip-clock-piece {
  display: inline-block;
  margin: 0 5px;
  perspective: 400px;
}

.card {
  display: block;
  position: relative;
  padding-bottom: 0.72em;
  font-size: 3rem;
  line-height: 0.95;
}

.card__top,
.card__bottom,
.card__back::before,
.card__back::after {
  display: block;
  height: 0.72em;
  color: #2c1810;
  background: 
    linear-gradient(145deg, 
      #FFD700 0%, 
      #FFA500 25%,
      #FFD700 50%,
      #DAA520 75%,
      #FFD700 100%);
  padding: 0.25em 0.25em;
  border-radius: 0.15em 0.15em 0 0;
  backface-visibility: hidden;
  transform-style: preserve-3d;
  width: 1.8em;
  transform: translateZ(0);
  border: 2px solid #C0C0C0;
  box-shadow: 
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    inset 0 -2px 4px rgba(0, 0, 0, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.4);
  
  font-family: 'Georgia', serif;
  font-weight: bold;
  text-shadow: 
    /* Dark top shadow to create the "cut-in" effect */
    0px -2px 2px rgba(0, 0, 0, 0.4), 
    
    /* Bright bottom highlight to simulate light catching the edge */
    0px 2px 2px rgba(255, 255, 255, 0.3);
  
  /* Remove flexbox centering - use text-align instead for proper centering */
  text-align: center;
  line-height: 0.72em;
}

.card__bottom {
  color: #2c1810;
  position: absolute;
  top: 50%;
  left: 0;
  border-top: solid 1px rgba(0, 0, 0, 0.3);
  background: 
    linear-gradient(145deg, 
      #FFD700 0%, 
      #FFA500 25%,
      #FFD700 50%,
      #DAA520 75%,
      #FFD700 100%);
  border-radius: 0 0 0.15em 0.15em;
  pointer-events: none;
  overflow: hidden;
  text-align: center;
  line-height: 0.72em;
}

.card__bottom::after {
  display: block;
  margin-top: -0.72em;
  text-align: center;
  line-height: 0.72em;
}

.card__back::before,
.card__bottom::after {
  content: attr(data-value);
}

.card__back {
  position: absolute;
  top: 0;
  height: 100%;
  left: 0%;
  pointer-events: none;
}

.card__back::before {
  position: relative;
  z-index: -1;
  overflow: hidden;
  text-align: center;
  line-height: 0.72em;
}

.flip .card__back::before {
  animation: flipTop 0.3s cubic-bezier(.37,.01,.94,.35);
  animation-fill-mode: both;
  transform-origin: center bottom;
}

.flip .card__back .card__bottom {
  transform-origin: center top;
  animation-fill-mode: both;
  animation: flipBottom 0.6s cubic-bezier(.15,.45,.28,1);
}

@keyframes flipTop {
  0% {
    transform: rotateX(0deg);
    z-index: 2;
  }
  0%, 99% {
    opacity: 0.99;
  }
  100% {
    transform: rotateX(-90deg);
    opacity: 0;
  }
}

@keyframes flipBottom {
  0%, 50% {
    z-index: -1;
    transform: rotateX(90deg);
    opacity: 0;
  }
  51% {
    opacity: 0.99;
  }
  100% {
    opacity: 0.99;
    transform: rotateX(0deg);
    z-index: 5;
  }
}

/* Responsive sizing */
@media (max-width: 768px) {
  .digit-card {
    width: 60px;
    height: 80px;
  }
  
  .digit-number {
    font-size: 36px;
  }
}

@media (max-width: 480px) {
  .digit-card {
    width: 50px;
    height: 70px;
  }
  
  .digit-number {
    font-size: 28px;
  }
}
</style>