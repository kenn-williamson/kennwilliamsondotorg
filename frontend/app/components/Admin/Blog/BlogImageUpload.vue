<template>
  <div class="space-y-4">
    <label class="block text-sm font-medium text-nautical-700">
      Featured Image
    </label>

    <!-- Image Preview (if exists) -->
    <div v-if="previewUrl" class="relative">
      <img
        :src="previewUrl"
        :alt="altText || 'Blog post featured image'"
        class="w-full max-h-96 object-contain bg-nautical-50 rounded-lg shadow-md"
      />
      <button
        type="button"
        @click="removeImage"
        class="absolute top-2 right-2 bg-red-600 text-white p-2 rounded-full hover:bg-red-700 transition-colors shadow-lg"
        :disabled="isUploading"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Upload Area -->
    <div
      v-if="!previewUrl"
      @drop.prevent="handleDrop"
      @dragover.prevent="isDragging = true"
      @dragleave.prevent="isDragging = false"
      :class="[
        'border-2 border-dashed rounded-lg p-8 text-center transition-colors cursor-pointer',
        isDragging ? 'border-sky-500 bg-sky-50' : 'border-nautical-300 bg-nautical-50',
        isUploading ? 'opacity-50 cursor-not-allowed' : 'hover:border-sky-400 hover:bg-sky-50'
      ]"
      @click="triggerFileInput"
    >
      <input
        ref="fileInput"
        type="file"
        accept="image/*"
        @change="handleFileSelect"
        class="hidden"
        :disabled="isUploading"
      />

      <svg class="w-12 h-12 mx-auto text-nautical-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>

      <p class="text-sm text-nautical-600 mb-1">
        <span class="font-medium text-sky-600">Click to upload</span> or drag and drop
      </p>
      <p class="text-xs text-nautical-500">
        PNG, JPG, GIF up to 10MB
      </p>
    </div>

    <!-- Upload Progress -->
    <div v-if="isUploading" class="space-y-2">
      <div class="flex items-center text-sm text-nautical-600">
        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-sky-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        Uploading image...
      </div>
    </div>

    <!-- Error Message -->
    <div v-if="uploadError" class="bg-red-50 border border-red-300 rounded-md p-3">
      <p class="text-sm text-red-700">{{ uploadError }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useBlogStore } from '~/stores/blog'

// Props
const props = defineProps<{
  modelValue?: string | null
  altText?: string | null
}>()

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: string | null]
  'update:altText': [value: string]
}>()

// State
const fileInput = ref<HTMLInputElement | null>(null)
const isDragging = ref(false)
const isUploading = ref(false)
const uploadError = ref<string | null>(null)

// Store
const blogStore = useBlogStore()

// Computed
const previewUrl = computed(() => props.modelValue)

// Methods
const triggerFileInput = () => {
  if (!isUploading.value) {
    fileInput.value?.click()
  }
}

const validateFile = (file: File): boolean => {
  // Check file type
  if (!file.type.startsWith('image/')) {
    uploadError.value = 'Please select an image file'
    return false
  }

  // Check file size (10MB max)
  const maxSize = 10 * 1024 * 1024 // 10MB in bytes
  if (file.size > maxSize) {
    uploadError.value = 'Image must be smaller than 10MB'
    return false
  }

  uploadError.value = null
  return true
}

const handleFileSelect = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    await uploadImage(file)
  }
}

const handleDrop = async (event: DragEvent) => {
  isDragging.value = false
  const file = event.dataTransfer?.files[0]
  if (file) {
    await uploadImage(file)
  }
}

const uploadImage = async (file: File) => {
  if (!validateFile(file)) return

  isUploading.value = true
  uploadError.value = null

  try {
    const result = await blogStore.uploadImage(file)
    if (result) {
      emit('update:modelValue', result.url)
      // Auto-generate alt text from filename (user can edit)
      const autoAlt = file.name.replace(/\.[^/.]+$/, '').replace(/[-_]/g, ' ')
      emit('update:altText', autoAlt)
    }
  } catch (error) {
    uploadError.value = error instanceof Error ? error.message : 'Failed to upload image'
  } finally {
    isUploading.value = false
  }
}

const removeImage = () => {
  emit('update:modelValue', null)
  emit('update:altText', '')
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}
</script>
