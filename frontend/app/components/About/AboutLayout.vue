<template>
  <div class="min-h-screen mahogany-background">
    <!-- Steampunk Background -->
    <SteampunkBackground />

    <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="flex flex-col lg:flex-row gap-8">
        <!-- Sidebar Navigation -->
        <aside class="lg:w-64 flex-shrink-0">
          <div class="lg:sticky lg:top-20">
            <!-- Mobile: Dropdown Menu -->
            <div class="lg:hidden mb-4">
              <button
                @click="mobileMenuOpen = !mobileMenuOpen"
                class="w-full bg-gradient-to-r from-nautical-700 to-primary-800 border-2 border-nautical-500 px-4 py-3 rounded-lg flex items-center justify-between text-primary-100 font-semibold"
              >
                <span>{{ currentSection }}</span>
                <span :class="{ 'rotate-180': mobileMenuOpen }" class="transition-transform">▼</span>
              </button>

              <Transition name="menu-slide">
                <nav v-if="mobileMenuOpen" class="mt-2 bg-gradient-to-br from-nautical-700 to-primary-800 border-2 border-nautical-500 rounded-lg p-4">
                  <ul class="space-y-2">
                    <li v-for="section in sections" :key="section.path">
                      <NuxtLink
                        :to="section.path"
                        :class="[
                          'block px-3 py-2 rounded transition-colors',
                          isCurrentPage(section.path)
                            ? 'bg-gradient-to-r from-primary-600 to-accent-600 text-white font-semibold'
                            : 'text-nautical-200 hover:bg-nautical-600/50 hover:text-accent-300'
                        ]"
                        @click="mobileMenuOpen = false"
                      >
                        {{ section.title }}
                      </NuxtLink>
                    </li>
                    <!-- Request Access item when content is restricted -->
                    <li v-if="hasRestrictedContent && !hasTrustedContactAccess">
                      <NuxtLink
                        to="/about/request-access"
                        class="block px-3 py-2 rounded transition-colors bg-gradient-to-r from-primary-600 to-accent-600 border-2 border-primary-700 text-white hover:from-primary-700 hover:to-accent-700 font-semibold text-center shadow-md"
                        @click="mobileMenuOpen = false"
                      >
                        🔑 Request Access
                      </NuxtLink>
                    </li>
                  </ul>
                </nav>
              </Transition>
            </div>

            <!-- Desktop: Sidebar -->
            <nav class="hidden lg:block bg-gradient-to-br from-nautical-700 to-primary-800 border-2 border-nautical-500 rounded-lg p-4 shadow-lg">
              <h2 class="text-lg font-bold text-primary-100 mb-4 pb-2 border-b-2 border-accent-400">
                The Story
              </h2>
              <ul class="space-y-1">
                <li v-for="section in sections" :key="section.path">
                  <NuxtLink
                    :to="section.path"
                    :class="[
                      'block px-3 py-2 rounded transition-all duration-200',
                      isCurrentPage(section.path)
                        ? 'bg-gradient-to-r from-primary-600 to-accent-600 text-white font-semibold shadow-md shadow-accent-500/50'
                        : 'text-nautical-200 hover:bg-nautical-600/50 hover:text-accent-300 hover:translate-x-1'
                    ]"
                  >
                    {{ section.title }}
                  </NuxtLink>
                </li>
                <!-- Request Access item when content is restricted -->
                <li v-if="hasRestrictedContent && !hasTrustedContactAccess" class="pt-2 mt-2 border-t-2 border-accent-400">
                  <NuxtLink
                    to="/about/request-access"
                    class="block px-3 py-2 rounded transition-all duration-200 bg-gradient-to-r from-primary-600 to-accent-600 border-2 border-primary-700 text-white hover:from-primary-700 hover:to-accent-700 font-semibold text-center shadow-md hover:shadow-lg"
                  >
                    🔑 Request Access
                  </NuxtLink>
                </li>
              </ul>
            </nav>
          </div>
        </aside>

        <!-- Main Content Area -->
        <main class="flex-1 min-w-0">
          <!-- Content Card -->
          <article class="bg-gradient-to-br from-nautical-50 via-primary-50 to-sky-50 border-2 border-primary-700 rounded-lg shadow-xl overflow-hidden">
            <!-- Decorative Header Border -->
            <div class="h-2 bg-gradient-to-r from-primary-600 via-indigo-600 to-primary-700"></div>

            <!-- Content -->
            <div class="p-6 sm:p-8 lg:p-12">
              <div class="prose prose-lg max-w-none">
                <slot>
                  <!-- Page content goes here -->
                </slot>
              </div>
            </div>

            <!-- Navigation Footer -->
            <div v-if="prevNext.prev || prevNext.next" class="border-t-2 border-primary-300 bg-primary-50 px-6 py-4">
              <div class="flex justify-between items-center">
                <NuxtLink
                  v-if="prevNext.prev"
                  :to="prevNext.prev.path"
                  class="group flex items-center gap-2 text-primary-700 hover:text-primary-900 font-semibold transition-colors"
                >
                  <span class="group-hover:-translate-x-1 transition-transform">←</span>
                  <span>{{ prevNext.prev.title }}</span>
                </NuxtLink>
                <div v-else></div>

                <NuxtLink
                  v-if="prevNext.next"
                  :to="prevNext.next.path"
                  class="group flex items-center gap-2 text-primary-700 hover:text-primary-900 font-semibold transition-colors"
                >
                  <span>{{ prevNext.next.title }}</span>
                  <span class="group-hover:translate-x-1 transition-transform">→</span>
                </NuxtLink>
              </div>
            </div>
          </article>
        </main>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import SteampunkBackground from '~/components/Steampunk/SteampunkBackground.vue'

const route = useRoute()
const mobileMenuOpen = ref(false)

// About section navigation structure
const allSections = [
  { title: 'Overview', path: '/about', restricted: false },
  { title: 'Origins', path: '/about/origins', restricted: true },
  { title: 'The Wilderness', path: '/about/wilderness', restricted: true },
  { title: 'Finding Faith', path: '/about/faith', restricted: true },
  { title: 'Theology & Practice', path: '/about/theology', restricted: true },
  { title: 'Professional Path', path: '/about/professional', restricted: false },
  { title: 'AI → IA', path: '/about/ai', restricted: false },
  { title: 'Life Now', path: '/about/now', restricted: true },
  { title: 'Philosophy & Vision', path: '/about/vision', restricted: true }
]

// Check if user has access to restricted content
const { hasTrustedContactAccess } = useUserRoles()

// Filter sections based on access - only show restricted sections if user has access
const sections = computed(() => {
  return allSections.filter(section => !section.restricted || hasTrustedContactAccess.value)
})

// Check if there are any restricted sections in the full list
const hasRestrictedContent = computed(() => {
  return allSections.some(section => section.restricted)
})

// Check if current page
function isCurrentPage(path) {
  return route.path === path
}

// Get current section title
const currentSection = computed(() => {
  const current = sections.value.find(s => s.path === route.path)
  return current ? current.title : 'About'
})

// Calculate prev/next navigation
const prevNext = computed(() => {
  const currentIndex = sections.value.findIndex(s => s.path === route.path)

  return {
    prev: currentIndex > 0 ? sections.value[currentIndex - 1] : null,
    next: currentIndex < sections.value.length - 1 ? sections.value[currentIndex + 1] : null
  }
})
</script>

<style scoped>
/* Nautical steampunk background - deep navy and slate tones */
.mahogany-background {
  background: linear-gradient(
    135deg,
    #0f172a 0%,
    #1e293b 25%,
    #334155 50%,
    #1e293b 75%,
    #0f172a 100%
  );
}

/* Mobile menu slide animation */
.menu-slide-enter-active,
.menu-slide-leave-active {
  transition: all 0.3s ease;
}

.menu-slide-enter-from,
.menu-slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* Prose styling for markdown-like content */
:deep(.prose) {
  @apply text-nautical-800;
}

:deep(.prose h1) {
  @apply text-3xl sm:text-4xl font-bold text-primary-900 mb-6 pb-3 border-b-2 border-primary-300;
}

:deep(.prose h2) {
  @apply text-2xl sm:text-3xl font-bold text-primary-800 mt-8 mb-4;
}

:deep(.prose h3) {
  @apply text-xl sm:text-2xl font-semibold text-primary-700 mt-6 mb-3;
}

:deep(.prose p) {
  @apply mb-4 leading-relaxed;
}

:deep(.prose ul) {
  @apply list-disc list-inside mb-4 space-y-2;
}

:deep(.prose ol) {
  @apply list-decimal list-inside mb-4 space-y-2;
}

:deep(.prose a) {
  @apply text-primary-700 underline hover:text-primary-900 transition-colors;
}

:deep(.prose blockquote) {
  @apply border-l-4 border-primary-400 pl-4 italic text-nautical-700 my-4;
}

:deep(.prose code) {
  @apply bg-primary-100 px-2 py-1 rounded text-sm font-mono text-primary-800;
}

:deep(.prose pre) {
  @apply bg-nautical-800 text-nautical-100 p-4 rounded-lg overflow-x-auto my-4;
}

/* Photo placeholder styling */
:deep(.photo-placeholder) {
  @apply bg-primary-100 border-2 border-primary-400 rounded-lg p-8 text-center text-nautical-600 italic my-6;
}
</style>
