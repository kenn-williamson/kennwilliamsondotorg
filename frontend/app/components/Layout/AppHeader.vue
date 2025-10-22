<template>
  <header class="sticky top-0 z-50 bg-nautical-900/80 backdrop-blur-md border-b-2 border-nautical-600/50 shadow-lg nautical-header">
    <nav class="container mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <!-- Logo/Site Name (Left) -->
        <div class="flex-shrink-0">
          <NuxtLink
            to="/"
            class="text-2xl font-bold text-primary-100 hover:text-accent-300 transition-colors duration-200"
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
              class="flex items-center justify-center w-10 h-10 rounded-full bg-gradient-to-br from-nautical-600 to-primary-700 text-accent-100 font-medium hover:from-nautical-500 hover:to-primary-600 transition-all duration-200 ring-2 ring-nautical-500/50 hover:ring-accent-400/50"
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
              <button 
                v-if="isAdmin"
                @click="goToAdminPanel"
                class="block w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 text-left transition-colors duration-200"
              >
                Admin Panel
              </button>
              <hr class="my-1 border-gray-100">
              <button 
                @click="handleLogout"
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
              class="px-4 py-2 text-sm font-medium text-nautical-200 hover:text-accent-300 transition-colors duration-200"
            >
              Sign In
            </NuxtLink>
            <NuxtLink
              to="/register"
              class="px-4 py-2 text-sm font-medium bg-gradient-to-r from-primary-600 to-accent-600 text-white rounded-md hover:from-primary-500 hover:to-accent-500 transition-all duration-200 shadow-lg hover:shadow-accent-500/50"
            >
              Register
            </NuxtLink>
          </div>
        </div>

        <!-- Mobile Menu Button -->
        <div class="md:hidden">
          <button
            @click="toggleMobileMenu"
            class="p-2 rounded-md text-nautical-200 hover:text-accent-300 hover:bg-nautical-800/50 transition-colors duration-200"
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
      <div v-if="showMobileMenu" class="md:hidden border-t border-nautical-600/50 mt-2 pt-4 pb-4 bg-nautical-800/30">
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


          <hr class="border-nautical-600/50 my-2">

          <!-- Authentication Links -->
          <div v-if="loggedIn" class="flex flex-col space-y-3">
            <div class="px-3 py-2 text-sm text-slate-300">
              Signed in as <span class="font-medium text-accent-300">{{ user?.email }}</span>
            </div>
            <button 
              @click="goToAccountSettings"
              class="mobile-nav-link text-left"
            >
              Account Settings
            </button>
            <button 
              v-if="isAdmin"
              @click="goToAdminPanel"
              class="mobile-nav-link text-left"
            >
              Admin Panel
            </button>
            <button 
              @click="handleLogout"
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
import { useAuthActions } from '~/composables/useAuthActions'

const { loggedIn, user, clear } = useUserSession()
const { logout, isLoading } = useAuthActions()
const router = useRouter()

const showMobileMenu = ref(false)
const showUserMenu = ref(false)

const userInitial = computed(() => {
  if (!user.value?.email) return 'U'
  return user.value.email.charAt(0).toUpperCase()
})

const isAdmin = computed(() => {
  return user.value?.roles?.includes('admin') || false
})
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

const handleLogout = async () => {
  await logout()
  showUserMenu.value = false
  showMobileMenu.value = false
  await router.push('/')
}

const goToAccountSettings = () => {
  showUserMenu.value = false
  showMobileMenu.value = false
  router.push('/profile')
}

const goToAdminPanel = () => {
  showUserMenu.value = false
  showMobileMenu.value = false
  router.push('/admin')
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
/* Nautical steampunk geometric pattern overlay */
.nautical-header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image:
    repeating-linear-gradient(
      90deg,
      transparent,
      transparent 20px,
      rgba(148, 163, 184, 0.03) 20px,
      rgba(148, 163, 184, 0.03) 40px
    ),
    repeating-linear-gradient(
      0deg,
      transparent,
      transparent 20px,
      rgba(148, 163, 184, 0.03) 20px,
      rgba(148, 163, 184, 0.03) 40px
    );
  pointer-events: none;
  z-index: -1;
}

/* Metallic gradient border effect */
.nautical-header::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(
    90deg,
    rgba(71, 85, 105, 0.5),
    rgba(59, 130, 246, 0.6),
    rgba(148, 163, 184, 0.7),
    rgba(59, 130, 246, 0.6),
    rgba(71, 85, 105, 0.5)
  );
  pointer-events: none;
}

.nav-link {
  @apply px-3 py-2 text-sm font-medium text-nautical-200 hover:text-accent-300 transition-all duration-200;
  position: relative;
}

.nav-link::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, #67e8f9, transparent);
  transition: width 0.3s ease;
}

.nav-link:hover::after {
  width: 80%;
}

.nav-link-active {
  @apply text-accent-300;
}

.nav-link-active::after {
  width: 80%;
}

.mobile-nav-link {
  @apply block px-3 py-2 text-base font-medium text-nautical-200 hover:text-accent-300 hover:bg-nautical-700/50 rounded-md transition-colors duration-200;
}
</style>