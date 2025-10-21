<template>
  <div class="min-h-screen mahogany-background">
    <!-- Steampunk Background -->
    <SteampunkBackground />

    <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="flex flex-col lg:flex-row gap-8">
        <!-- Sidebar Navigation -->
        <aside class="lg:w-64 flex-shrink-0">
          <div class="lg:sticky lg:top-8">
            <!-- Mobile: Dropdown Menu -->
            <div class="lg:hidden mb-4">
              <button
                @click="mobileMenuOpen = !mobileMenuOpen"
                class="w-full bg-gradient-to-r from-amber-100 to-yellow-100 border-2 border-amber-600 px-4 py-3 rounded-lg flex items-center justify-between text-amber-900 font-semibold"
              >
                <span>{{ currentSection }}</span>
                <span :class="{ 'rotate-180': mobileMenuOpen }" class="transition-transform">▼</span>
              </button>

              <Transition name="menu-slide">
                <nav v-if="mobileMenuOpen" class="mt-2 bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-amber-600 rounded-lg p-4">
                  <ul class="space-y-2">
                    <li v-for="section in sections" :key="section.path">
                      <NuxtLink
                        :to="section.path"
                        :class="[
                          'block px-3 py-2 rounded transition-colors',
                          isCurrentPage(section.path)
                            ? 'bg-amber-600 text-white font-semibold'
                            : 'text-gray-700 hover:bg-amber-100'
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
                        class="block px-3 py-2 rounded transition-colors bg-gradient-to-r from-purple-100 to-indigo-100 border-2 border-purple-400 text-purple-900 hover:from-purple-200 hover:to-indigo-200 font-semibold text-center"
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
            <nav class="hidden lg:block bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-amber-600 rounded-lg p-4 shadow-lg">
              <h2 class="text-lg font-bold text-amber-900 mb-4 pb-2 border-b-2 border-amber-300">
                The Story
              </h2>
              <ul class="space-y-1">
                <li v-for="section in sections" :key="section.path">
                  <NuxtLink
                    :to="section.path"
                    :class="[
                      'block px-3 py-2 rounded transition-all duration-200',
                      isCurrentPage(section.path)
                        ? 'bg-gradient-to-r from-amber-600 to-orange-600 text-white font-semibold shadow-md'
                        : 'text-gray-700 hover:bg-amber-100 hover:translate-x-1'
                    ]"
                  >
                    {{ section.title }}
                  </NuxtLink>
                </li>
                <!-- Request Access item when content is restricted -->
                <li v-if="hasRestrictedContent && !hasTrustedContactAccess" class="pt-2 mt-2 border-t-2 border-amber-300">
                  <NuxtLink
                    to="/about/request-access"
                    class="block px-3 py-2 rounded transition-all duration-200 bg-gradient-to-r from-purple-100 to-indigo-100 border-2 border-purple-400 text-purple-900 hover:from-purple-200 hover:to-indigo-200 font-semibold text-center hover:shadow-md"
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
          <article class="bg-gradient-to-br from-amber-50 via-orange-50 to-red-50 border-2 border-amber-600 rounded-lg shadow-xl overflow-hidden">
            <!-- Decorative Header Border -->
            <div class="h-2 bg-gradient-to-r from-amber-600 via-orange-600 to-red-600"></div>

            <!-- Content -->
            <div class="p-6 sm:p-8 lg:p-12">
              <div class="prose prose-lg max-w-none">
                <slot>
                  <!-- Page content goes here -->
                </slot>
              </div>
            </div>

            <!-- Navigation Footer -->
            <div v-if="prevNext.prev || prevNext.next" class="border-t-2 border-amber-300 bg-amber-50 px-6 py-4">
              <div class="flex justify-between items-center">
                <NuxtLink
                  v-if="prevNext.prev"
                  :to="prevNext.prev.path"
                  class="group flex items-center gap-2 text-amber-700 hover:text-amber-900 font-semibold transition-colors"
                >
                  <span class="group-hover:-translate-x-1 transition-transform">←</span>
                  <span>{{ prevNext.prev.title }}</span>
                </NuxtLink>
                <div v-else></div>

                <NuxtLink
                  v-if="prevNext.next"
                  :to="prevNext.next.path"
                  class="group flex items-center gap-2 text-amber-700 hover:text-amber-900 font-semibold transition-colors"
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
/* Mahogany background matching timer aesthetic */
.mahogany-background {
  background: linear-gradient(
    135deg,
    #3e2723 0%,
    #4e342e 25%,
    #5d4037 50%,
    #4e342e 75%,
    #3e2723 100%
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
  @apply text-gray-800;
}

:deep(.prose h1) {
  @apply text-3xl sm:text-4xl font-bold text-amber-900 mb-6 pb-3 border-b-2 border-amber-300;
}

:deep(.prose h2) {
  @apply text-2xl sm:text-3xl font-bold text-amber-800 mt-8 mb-4;
}

:deep(.prose h3) {
  @apply text-xl sm:text-2xl font-semibold text-amber-700 mt-6 mb-3;
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
  @apply text-amber-700 underline hover:text-amber-900 transition-colors;
}

:deep(.prose blockquote) {
  @apply border-l-4 border-amber-400 pl-4 italic text-gray-700 my-4;
}

:deep(.prose code) {
  @apply bg-amber-100 px-2 py-1 rounded text-sm font-mono text-red-700;
}

:deep(.prose pre) {
  @apply bg-gray-800 text-gray-100 p-4 rounded-lg overflow-x-auto my-4;
}

/* Photo placeholder styling */
:deep(.photo-placeholder) {
  @apply bg-amber-100 border-2 border-amber-400 rounded-lg p-8 text-center text-gray-600 italic my-6;
}
</style>
