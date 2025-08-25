<template>
  <header class="header">
    <h1 class="title">Welcome to KennWilliamson.org</h1>
  </header>
</template>

<script setup lang="ts">
</script>

<style scoped>
.header {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 2rem 1rem;
}

.title {
  font-size: 2.5rem;
  font-weight: bold;
  color: var(--color-heading);
  text-align: center;
  margin: 0;
}

@media (max-width: 768px) {
  .title {
    font-size: 2rem;
  }
}

@media (max-width: 480px) {
  .title {
    font-size: 1.5rem;
  }
}
</style>