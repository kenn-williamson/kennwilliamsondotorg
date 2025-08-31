<template>
  <div class="min-h-screen bg-gradient-to-br from-amber-50 via-orange-50 to-red-50 px-4 sm:px-6 lg:px-8 py-12">
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="text-center mb-12">
        <h1 class="text-4xl sm:text-5xl font-bold text-gray-900 mb-4">
          About 
          <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-600 to-orange-600">
            Kenn
          </span>
        </h1>
        <p class="text-xl text-gray-600 max-w-2xl mx-auto leading-relaxed">
          A man of the margins exploring the intersection of tradition and innovation, 
          seeking truth, beauty, and love in all endeavors.
        </p>
      </div>

      <!-- Content Placeholder -->
      <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-orange-200 p-8 sm:p-12">
        <div class="prose prose-lg max-w-none">
          <div class="text-center py-16">
            <div class="mb-8">
              <!-- Frontier/Nature inspired icon -->
              <svg class="w-24 h-24 text-amber-600 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" 
                      d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>
              </svg>
              <!-- Mountain/Frontier silhouette -->
              <div class="mt-4 flex justify-center opacity-40">
                <svg class="w-32 h-16 text-amber-500" fill="currentColor" viewBox="0 0 200 100">
                  <polygon points="0,100 40,30 80,60 120,20 160,50 200,100"/>
                </svg>
              </div>
            </div>
            
            <h2 class="text-2xl font-bold text-gray-800 mb-4">Content Coming Soon</h2>
            <p class="text-gray-600 mb-8 max-w-lg mx-auto">
              This section will feature personal stories, philosophy, and the journey of building 
              something meaningful at the intersection of sacred traditions and technological innovation.
            </p>
            
            <div class="space-y-4 text-sm text-gray-500">
              <p><strong>Aesthetic Influences:</strong> Gothic cathedrals, Japanese traditional art, American frontier spirit</p>
              <p><strong>Philosophical Foundation:</strong> Truth, Beauty, Love as guiding principles</p>
              <p><strong>Artistic Inspiration:</strong> Millais, Bosch, Thomas Moran, van Gogh, Rembrandt, Caravaggio</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Decorative Elements - Japanese/Traditional Pattern -->
      <div class="mt-12 flex justify-center space-x-12 opacity-20">
        <!-- Traditional Japanese Mon-inspired elements -->
        <div class="w-16 h-16 rounded-full border-2 border-amber-400 flex items-center justify-center">
          <div class="w-8 h-8 bg-gradient-to-br from-amber-400 to-orange-400 rounded-full"></div>
        </div>
        
        <!-- Mountain/Wave pattern -->
        <svg class="w-16 h-16 text-amber-400" fill="currentColor" viewBox="0 0 100 100">
          <path d="M20,80 Q30,60 40,80 Q50,50 60,80 Q70,60 80,80 L80,100 L20,100 Z"/>
        </svg>
        
        <!-- Another traditional element -->
        <div class="w-16 h-16 rounded-full border-2 border-orange-400 flex items-center justify-center">
          <svg class="w-8 h-8 text-orange-400" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10,2 L12,8 L18,8 L13,12 L15,18 L10,14 L5,18 L7,12 L2,8 L8,8 Z"/>
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
// Page meta
useHead({
  title: 'About',
  meta: [
    { name: 'description', content: 'Learn about Kenn Williamson - a developer and thinker exploring the harmony between tradition and innovation, guided by principles of truth, beauty, and love.' }
  ]
})
</script>

<style scoped>
/* Subtle traditional pattern overlay */
.bg-traditional-pattern {
  background-image: 
    repeating-linear-gradient(
      45deg,
      transparent,
      transparent 10px,
      rgba(251, 191, 36, 0.03) 10px,
      rgba(251, 191, 36, 0.03) 20px
    );
}
</style>