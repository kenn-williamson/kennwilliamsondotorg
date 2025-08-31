<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8">
    <div class="max-w-lg w-full text-center">
      <!-- Construction Image -->
      <div class="mb-8">
        <img 
          src="~/assets/images/construction-castle.jpg" 
          alt="Castle under construction" 
          class="w-full h-64 object-cover rounded-lg shadow-lg mx-auto"
          @error="showFallback = true"
          @load="showFallback = false"
        />
        <!-- Fallback icon if image doesn't load -->
        <div v-if="showFallback" class="w-full h-64 bg-gradient-to-br from-sky-100 to-slate-200 rounded-lg shadow-lg mx-auto flex items-center justify-center">
          <svg class="w-24 h-24 text-sky-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" 
                  d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
          </svg>
        </div>
      </div>

      <!-- Under Construction Message -->
      <div class="space-y-6">
        <h1 class="text-4xl sm:text-5xl font-bold text-gray-900 leading-tight">
          Under 
          <span class="text-transparent bg-clip-text bg-gradient-to-r from-sky-600 to-blue-600">
            Construction
          </span>
        </h1>
        
        <p class="text-xl text-gray-600 leading-relaxed max-w-md mx-auto">
          Building something beautiful that reflects the harmony of 
          <span class="font-medium text-sky-700">truth</span>, 
          <span class="font-medium text-sky-700">beauty</span>, and 
          <span class="font-medium text-sky-700">love</span>.
        </p>

        <div class="pt-4">
          <p class="text-sm text-gray-500 italic">
            "Every great building begins with a vision and careful planning."
          </p>
        </div>

        <!-- Subtle Navigation Hint -->
        <div class="pt-8">
          <p class="text-sm text-gray-400">
            Explore the 
            <NuxtLink to="/incidents" class="text-sky-600 hover:text-sky-700 font-medium underline">
              Incidents
            </NuxtLink> 
            feature while construction continues.
          </p>
        </div>
      </div>

      <!-- Decorative Elements -->
      <div class="mt-12 flex justify-center space-x-8 opacity-30">
        <!-- Gothic Arch SVG -->
        <svg class="w-12 h-16 text-sky-300" fill="currentColor" viewBox="0 0 24 32">
          <path d="M12 0L0 24h4l8-12 8 12h4L12 0z"/>
        </svg>
        
        <!-- Gear/Construction Icon -->
        <svg class="w-12 h-12 text-slate-300 mt-2" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"/>
        </svg>
        
        <!-- Another Gothic Element -->
        <svg class="w-12 h-16 text-sky-300" fill="currentColor" viewBox="0 0 24 32">
          <path d="M12 0L0 24h4l8-12 8 12h4L12 0z" transform="scale(-1,1) translate(-24,0)"/>
        </svg>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// Page meta
useHead({
  title: 'Home',
  meta: [
    { name: 'description', content: 'KennWilliamson.org - A personal exploration of truth, beauty, and love through the lens of tradition and technology.' }
  ]
})

// Initialize auth store on client side
const authStore = useAuthStore()

// Check authentication status
onMounted(() => {
  authStore.checkAuth()
})

// Control fallback display
const showFallback = ref(false)
</script>

<style scoped>
/* Add subtle animation to construction elements */
@keyframes gentle-pulse {
  0%, 100% { opacity: 0.3; }
  50% { opacity: 0.6; }
}

.animate-pulse-gentle {
  animation: gentle-pulse 3s ease-in-out infinite;
}

/* Ensure image fallback works properly */
img[src="/construction-castle.jpg"] {
  background: linear-gradient(to bottom right, rgb(125, 211, 252), rgb(148, 163, 184));
}
</style>