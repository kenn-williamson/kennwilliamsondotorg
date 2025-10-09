<template>
  <div class="mt-12 pt-8 border-t-2 border-amber-300">
    <div class="bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-amber-400 rounded-lg p-8 text-center">
      <!-- Not logged in -->
      <div v-if="!loggedIn" class="space-y-4">
        <div class="text-amber-900 text-lg font-semibold">
          Want to read the full story?
        </div>
        <p class="text-gray-700 mb-6">
          There's more to discover about my journey—from family origins to faith transformation and life now.
        </p>
        <NuxtLink
          :to="`/login?redirect=${encodeURIComponent(currentPath)}`"
          class="inline-block bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-700 hover:to-orange-700 text-white font-bold py-3 px-8 rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
        >
          Login to See More
        </NuxtLink>
      </div>

      <!-- Logged in without access -->
      <div v-else-if="!hasTrustedContactAccess" class="space-y-4">
        <div class="text-amber-900 text-lg font-semibold">
          Interested in the full story?
        </div>
        <p class="text-gray-700 mb-6">
          Additional sections include personal details about family, faith journey, and current life that require trusted contact access.
        </p>
        <NuxtLink
          to="/about/request-access"
          class="inline-block bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-700 hover:to-orange-700 text-white font-bold py-3 px-8 rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105"
        >
          Request Access to See More
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const { loggedIn } = useUserSession()
const { hasTrustedContactAccess } = useUserRoles()

const currentPath = computed(() => route.path)
</script>
