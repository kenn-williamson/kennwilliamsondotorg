<template>
  <header class="sticky top-0 z-50 bg-white/95 backdrop-blur-sm border-b border-sky-200 shadow-sm">
    <nav class="container mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <!-- Logo/Site Name (Left) -->
        <div class="flex-shrink-0">
          <NuxtLink 
            to="/" 
            class="text-2xl font-bold text-sky-800 hover:text-sky-600 transition-colors duration-200"
          >
            KennWilliamson
          </NuxtLink>
        </div>

        <!-- Desktop Navigation (Center) -->
        <div class="hidden md:block">
          <div class="flex items-center space-x-8">
            <NuxtLink 
              to="/about" 
              class="nav-link"
              :class="{ 'nav-link-active': $route.path === '/about' }"
            >
              About
            </NuxtLink>
            <NuxtLink 
              to="/incidents" 
              class="nav-link"
              :class="{ 'nav-link-active': $route.path === '/incidents' }"
            >
              Incidents
            </NuxtLink>
          </div>
        </div>

        <!-- Authentication Section (Right) -->
        <div class="hidden md:flex items-center space-x-4">
          <!-- Authenticated State -->
          <div v-if="loggedIn" class="relative">
            <button 
              @click="toggleUserMenu"
              class="flex items-center justify-center w-10 h-10 rounded-full bg-sky-600 text-white font-medium hover:bg-sky-700 transition-colors duration-200"
              :aria-expanded="showUserMenu"
              aria-haspopup="true"
            >
              {{ userInitial }}
            </button>
            
            <!-- User Dropdown Menu -->
            <div 
              v-if="showUserMenu"
              class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg border border-gray-200 py-1 z-50"
              @click.stop
            >
              <button 
                @click="goToAccountSettings"
                class="block w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 text-left transition-colors duration-200"
              >
                Account Settings
              </button>
              <hr class="my-1 border-gray-100">
              <button 
                @click="logout"
                class="block w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 text-left transition-colors duration-200"
              >
                Sign Out
              </button>
            </div>
          </div>

          <!-- Unauthenticated State -->
          <div v-else class="flex items-center space-x-3">
            <NuxtLink 
              to="/login" 
              class="px-4 py-2 text-sm font-medium text-sky-700 hover:text-sky-600 transition-colors duration-200"
            >
              Sign In
            </NuxtLink>
            <NuxtLink 
              to="/register" 
              class="px-4 py-2 text-sm font-medium bg-sky-600 text-white rounded-md hover:bg-sky-700 transition-colors duration-200"
            >
              Register
            </NuxtLink>
          </div>
        </div>

        <!-- Mobile Menu Button -->
        <div class="md:hidden">
          <button 
            @click="toggleMobileMenu"
            class="p-2 rounded-md text-sky-700 hover:text-sky-600 hover:bg-sky-50 transition-colors duration-200"
            :aria-expanded="showMobileMenu"
            aria-label="Toggle menu"
          >
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path 
                v-if="!showMobileMenu"
                stroke-linecap="round" 
                stroke-linejoin="round" 
                stroke-width="2" 
                d="M4 6h16M4 12h16M4 18h16"
              />
              <path 
                v-else
                stroke-linecap="round" 
                stroke-linejoin="round" 
                stroke-width="2" 
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
      </div>

      <!-- Mobile Menu -->
      <div v-if="showMobileMenu" class="md:hidden border-t border-sky-200 mt-2 pt-4 pb-4">
        <div class="flex flex-col space-y-3">
          <!-- Navigation Links -->
          <NuxtLink 
            to="/about" 
            class="mobile-nav-link"
            @click="showMobileMenu = false"
          >
            About
          </NuxtLink>
          <NuxtLink 
            to="/incidents" 
            class="mobile-nav-link"
            @click="showMobileMenu = false"
          >
            Incidents
          </NuxtLink>
          
          <hr class="border-sky-200 my-2">
          
          <!-- Authentication Links -->
          <div v-if="loggedIn" class="flex flex-col space-y-3">
            <div class="px-3 py-2 text-sm text-gray-600">
              Signed in as <span class="font-medium">{{ user?.email }}</span>
            </div>
            <button 
              @click="goToAccountSettings"
              class="mobile-nav-link text-left"
            >
              Account Settings
            </button>
            <button 
              @click="logout"
              class="mobile-nav-link text-left"
            >
              Sign Out
            </button>
          </div>
          <div v-else class="flex flex-col space-y-3">
            <NuxtLink 
              to="/login" 
              class="mobile-nav-link"
              @click="showMobileMenu = false"
            >
              Sign In
            </NuxtLink>
            <NuxtLink 
              to="/register" 
              class="mobile-nav-link"
              @click="showMobileMenu = false"
            >
              Register
            </NuxtLink>
          </div>
        </div>
      </div>
    </nav>
  </header>
</template>

<script setup>
const { loggedIn, user, clear } = useUserSession()
const router = useRouter()

// Reactive state
const showMobileMenu = ref(false)
const showUserMenu = ref(false)

// Computed properties
const userInitial = computed(() => {
  if (!user.value?.email) return 'U'
  return user.value.email.charAt(0).toUpperCase()
})

// Methods
const toggleMobileMenu = () => {
  showMobileMenu.value = !showMobileMenu.value
  if (showMobileMenu.value) {
    showUserMenu.value = false
  }
}

const toggleUserMenu = () => {
  showUserMenu.value = !showUserMenu.value
  if (showUserMenu.value) {
    showMobileMenu.value = false
  }
}

const logout = async () => {
  await clear()
  showUserMenu.value = false
  showMobileMenu.value = false
  await router.push('/')
}

const goToAccountSettings = () => {
  showUserMenu.value = false
  showMobileMenu.value = false
  // TODO: Navigate to account settings when implemented
  console.log('Navigate to account settings')
}

// Click outside to close menus
onMounted(() => {
  const handleClickOutside = (event) => {
    const header = event.target.closest('header')
    if (!header) {
      showUserMenu.value = false
      showMobileMenu.value = false
    }
  }
  document.addEventListener('click', handleClickOutside)
  
  onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
  })
})
</script>

<style scoped>
.nav-link {
  @apply px-3 py-2 text-sm font-medium text-gray-700 hover:text-sky-600 transition-colors duration-200;
}

.nav-link-active {
  @apply text-sky-600;
}

.mobile-nav-link {
  @apply block px-3 py-2 text-base font-medium text-gray-700 hover:text-sky-600 hover:bg-sky-50 rounded-md transition-colors duration-200;
}
</style>