<template>
  <div class="min-h-screen">
    <!-- Hero Section with Cathedral Background -->
    <section class="relative h-[calc(100vh-4rem)] flex items-center justify-center">
      <!-- Background Video/Image with Overlay -->
      <div class="absolute inset-0">
        <!-- Static background image (shows after video or if video fails) -->
        <img
          src="~/assets/images/cathedral2.jpg"
          alt="Steampunk Cathedral with Tree of Life"
          class="w-full h-full object-cover"
        />

        <!-- Video overlay (plays on load, then fades out) -->
        <video
          ref="heroVideo"
          autoplay
          muted
          playsinline
          @ended="onVideoEnded"
          @error="onVideoError"
          :class="['absolute inset-0 w-full h-full object-cover transition-opacity duration-1000', videoPlaying ? 'opacity-100' : 'opacity-0']"
        >
          <!-- High-quality video for desktop (768px and up) -->
          <source src="~/assets/images/cathedral-animated.mp4" type="video/mp4" media="(min-width: 768px)" />
          <!-- Lower-quality video for mobile (below 768px) -->
          <source src="~/assets/images/cathedral-animated-mobile.mp4" type="video/mp4" />
        </video>

        <!-- Dark overlay for text readability -->
        <div class="absolute inset-0 bg-gradient-to-b from-nautical-900/70 via-nautical-900/60 to-nautical-900/80"></div>
      </div>

      <!-- Hero Content -->
      <div class="relative z-10 text-center px-4 sm:px-6 lg:px-8 max-w-4xl mx-auto">
        <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-white leading-tight mb-6">
          Kenn Williamson
        </h1>
        <p class="text-xl sm:text-2xl lg:text-3xl text-accent-300 font-semibold mb-4">
          Enterprise Architect, Disciple, Builder
        </p>
        <p class="text-lg sm:text-xl text-nautical-200 mb-8 max-w-2xl mx-auto">
          Exploring the intersection of ancient wisdom and modern technology
        </p>

        <!-- CTA Buttons -->
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <NuxtLink
            to="/about"
            class="px-8 py-3 bg-gradient-to-r from-primary-600 to-accent-600 hover:from-primary-700 hover:to-accent-700 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            Read My Story
          </NuxtLink>
          <NuxtLink
            to="/incidents"
            class="px-8 py-3 bg-nautical-700 hover:bg-nautical-600 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            View Projects
          </NuxtLink>
        </div>
      </div>

      <!-- Scroll Indicator -->
      <div class="absolute bottom-16 left-1/2 transform -translate-x-1/2 animate-bounce z-20">
        <svg class="w-6 h-6 text-accent-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
        </svg>
      </div>
    </section>

    <!-- Quick Bio Section -->
    <section class="py-16 px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-nautical-50 to-gray-100">
      <div class="max-w-4xl mx-auto text-center">
        <h2 class="text-3xl sm:text-4xl font-bold text-nautical-900 mb-6">
          Who I Am
        </h2>
        <p class="text-lg sm:text-xl text-nautical-700 leading-relaxed">
          I'm a single dad to three kids, an Enterprise Architect helping companies navigate AI
          realistically, learning what it means to be a disciple of Christ, and trying to walk in love.
          I read Church Fathers, design and build software systems, and believe
          <span class="font-semibold text-primary-700">truth</span>,
          <span class="font-semibold text-primary-700">beauty</span>, and
          <span class="font-semibold text-primary-700">love</span>
          aren't just ideals but a way of life.
        </p>
      </div>
    </section>

    <!-- Feature Grid -->
    <section class="py-16 px-4 sm:px-6 lg:px-8 bg-white">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl sm:text-4xl font-bold text-nautical-900 text-center mb-12">
          Explore
        </h2>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <!-- Professional Card -->
          <div class="group bg-gradient-to-br from-nautical-50 to-nautical-100 rounded-lg p-8 shadow-lg hover:shadow-xl transition-all duration-300 border-2 border-nautical-200 hover:border-primary-400">
            <div class="flex items-center mb-4">
              <!-- Gear Icon -->
              <svg class="w-12 h-12 text-primary-600 mr-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"/>
              </svg>
              <h3 class="text-2xl font-bold text-nautical-900">
                AI Realism & Software Architecture
              </h3>
            </div>
            <p class="text-nautical-700 mb-6">
              Enterprise Architect at SEQTEK, specializing in practical AI integration and helping
              companies navigate the AI revolution with realistic expectations.
            </p>
            <div class="flex flex-wrap gap-3">
              <NuxtLink
                to="/about/professional"
                class="text-primary-600 hover:text-primary-700 font-semibold underline"
              >
                Professional Path →
              </NuxtLink>
              <NuxtLink
                to="/about/ai"
                class="text-primary-600 hover:text-primary-700 font-semibold underline"
              >
                AI Philosophy →
              </NuxtLink>
            </div>
          </div>

          <!-- Journey Card -->
          <div class="group bg-gradient-to-br from-primary-50 to-accent-50 rounded-lg p-8 shadow-lg hover:shadow-xl transition-all duration-300 border-2 border-primary-200 hover:border-primary-400">
            <div class="flex items-center mb-4">
              <!-- Compass Icon -->
              <svg class="w-12 h-12 text-primary-600 mr-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 21v-4m0 0V5a2 2 0 012-2h6.5l1 1H21l-3 6 3 6h-8.5l-1-1H5a2 2 0 00-2 2zm9-13.5V9"/>
              </svg>
              <h3 class="text-2xl font-bold text-nautical-900">
                From Wilderness to Kingdom
              </h3>
            </div>
            <p class="text-nautical-700 mb-6">
              A story of transformation through faith, family, and finding purpose. From lost years
              and broken places to becoming a disciple, father, and builder.
            </p>
            <NuxtLink
              to="/about"
              class="text-primary-600 hover:text-primary-700 font-semibold underline"
            >
              Read My Story →
            </NuxtLink>
          </div>

          <!-- Incident Timer Card -->
          <div class="group bg-gradient-to-br from-sky-50 to-primary-50 rounded-lg p-8 shadow-lg hover:shadow-xl transition-all duration-300 border-2 border-sky-200 hover:border-primary-400">
            <div class="flex items-center mb-4">
              <!-- Clock Icon -->
              <svg class="w-12 h-12 text-primary-600 mr-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <h3 class="text-2xl font-bold text-nautical-900">
                Incident Timer
              </h3>
            </div>
            <p class="text-nautical-700 mb-6">
              Track time since last incidents with a steampunk-themed UI. A practical tool with
              full CRUD operations, public sharing, and beautiful design. Proof that I build things.
            </p>
            <NuxtLink
              to="/incidents"
              class="text-primary-600 hover:text-primary-700 font-semibold underline"
            >
              Try It Live →
            </NuxtLink>
          </div>

          <!-- Theology Card -->
          <div class="group bg-gradient-to-br from-purple-50 to-indigo-50 rounded-lg p-8 shadow-lg hover:shadow-xl transition-all duration-300 border-2 border-purple-200 hover:border-primary-400">
            <div class="flex items-center mb-4">
              <!-- Book Icon -->
              <svg class="w-12 h-12 text-primary-600 mr-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
              </svg>
              <h3 class="text-2xl font-bold text-nautical-900">
                Faith, Mimetic Theory, & Christian Anarchism
              </h3>
            </div>
            <p class="text-nautical-700 mb-6">
              Exploring non-sacrificial Gospel readings, René Girard's mimetic theory, and what
              it means to take Jesus' words about loving enemies literally.
            </p>
            <div class="flex flex-wrap gap-3">
              <NuxtLink
                to="/about/faith"
                class="text-primary-600 hover:text-primary-700 font-semibold underline"
              >
                Finding Faith →
              </NuxtLink>
              <NuxtLink
                to="/about/theology"
                class="text-primary-600 hover:text-primary-700 font-semibold underline"
              >
                Theology & Practice →
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Connect Section -->
    <section class="py-12 px-4 sm:px-6 lg:px-8 bg-nautical-100">
      <div class="max-w-4xl mx-auto text-center">
        <h2 class="text-2xl sm:text-3xl font-bold text-nautical-900 mb-4">
          Let's Connect
        </h2>
        <p class="text-nautical-700 mb-6">
          Find me on LinkedIn
        </p>
        <div class="flex justify-center">
          <!-- LinkedIn Link -->
          <a
            href="https://linkedin.com/in/kenngineering"
            target="_blank"
            rel="noopener noreferrer"
            class="flex items-center gap-2 px-6 py-3 bg-primary-600 hover:bg-primary-700 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            <!-- LinkedIn Icon -->
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
            </svg>
            <span>Connect on LinkedIn</span>
          </a>
        </div>
      </div>
    </section>

    <!-- Footer CTA Section -->
    <section class="py-16 px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-nautical-800 to-nautical-900 text-white">
      <div class="max-w-4xl mx-auto text-center">
        <h2 class="text-3xl sm:text-4xl font-bold mb-6">
          Welcome to the Story
        </h2>
        <p class="text-lg sm:text-xl text-nautical-300 mb-8">
          It's still being written.
        </p>
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <NuxtLink
            to="/about"
            class="px-8 py-3 bg-gradient-to-r from-primary-600 to-accent-600 hover:from-primary-700 hover:to-accent-700 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            Learn More About Me
          </NuxtLink>
          <NuxtLink
            to="/incidents"
            class="px-8 py-3 bg-nautical-700 hover:bg-nautical-600 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
          >
            Explore Projects
          </NuxtLink>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

// Video state management
const heroVideo = ref(null)
const videoPlaying = ref(true)

// Handle video ended - fade out to reveal static image
const onVideoEnded = () => {
  videoPlaying.value = false
}

// Handle video error - show static image immediately
const onVideoError = () => {
  console.warn('Hero video failed to load, showing static image')
  videoPlaying.value = false
}

// Attempt to play video on mount (in case autoplay is blocked)
onMounted(() => {
  if (heroVideo.value) {
    heroVideo.value.play().catch(() => {
      // Autoplay was blocked, fall back to static image
      videoPlaying.value = false
    })
  }
})

// Page meta
useHead({
  title: 'Home - Kenn Williamson',
  meta: [
    {
      name: 'description',
      content: 'Kenn Williamson: Enterprise Architect, single dad, and disciple of Christ. Exploring the intersection of ancient wisdom and modern technology, building software in Rust, and pursuing truth, beauty, and love.'
    }
  ]
})

// Social media sharing
useSocialShare({
  title: 'Kenn Williamson',
  imageKey: 'homepage'
})
</script>

<style scoped>
/* Ensure full viewport height for hero */
section:first-child {
  min-height: 100vh;
}

/* Smooth scroll behavior */
html {
  scroll-behavior: smooth;
}
</style>
