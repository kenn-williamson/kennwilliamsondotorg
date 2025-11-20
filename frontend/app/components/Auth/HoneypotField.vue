<template>
  <!--
    Honeypot field for bot detection
    This field is hidden from legitimate users but will be filled by bots
    DO NOT remove or modify the styling - it must remain invisible
  -->
  <input
    v-model="honeypotValue"
    type="text"
    name="website"
    autocomplete="off"
    tabindex="-1"
    aria-hidden="true"
    class="honeypot-field"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'

const honeypotValue = ref('')

// Expose method to get the honeypot value
const getValue = (): string => {
  return honeypotValue.value
}

// Expose method to reset the field
const reset = (): void => {
  honeypotValue.value = ''
}

defineExpose({
  getValue,
  reset
})
</script>

<style scoped>
/*
  CRITICAL: Do not modify these styles
  The field must be completely hidden from users but still present in the DOM
*/
.honeypot-field {
  position: absolute;
  left: -9999px;
  width: 1px;
  height: 1px;
  opacity: 0;
  pointer-events: none;
  /* Prevent autofill */
  background: transparent;
  border: none;
  /* Additional hiding techniques */
  clip: rect(0, 0, 0, 0);
  clip-path: inset(50%);
  overflow: hidden;
  white-space: nowrap;
}
</style>
