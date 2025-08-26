<script setup lang="ts">
import HeaderComponent from './components/HeaderComponent.vue'
import CounterBanner from './components/CounterBanner.vue'
</script>

<template>
  <div class="app-container">
    <HeaderComponent />
    <img src="./assets/smokey1.png" alt="Smokey" class="smokey-image" />
    <CounterBanner />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  align-items: center;
  justify-content: center;
  gap: 2rem;
}

.smokey-image {
  max-width: 512px;
  width: 100%;
  height: auto;
}
</style>
