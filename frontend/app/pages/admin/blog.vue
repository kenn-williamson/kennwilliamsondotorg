<template>
  <div class="min-h-screen bg-nautical-50 px-4 py-8">
    <div class="max-w-7xl mx-auto">
      <!-- Page Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-nautical-900 mb-2">Blog Management</h1>
        <p class="text-nautical-600">Create, edit, and manage your blog posts</p>
      </div>

      <!-- Content -->
      <div class="mt-6">
        <!-- List View -->
        <BlogListTab
          v-if="!showEditor"
          @create-new="handleCreateNew"
          @edit-post="handleEditPost"
        />

        <!-- Editor View -->
        <BlogEditorForm
          v-else
          :editing-post="editingPost"
          @cancel="handleCancel"
          @success="handleSuccess"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { BlogPost } from '#shared/types'

// Page meta
definePageMeta({
  middleware: 'admin'
})

// SEO
useHead({
  title: 'Blog Management - Admin',
  meta: [
    {
      name: 'description',
      content: 'Manage blog posts - create, edit, and publish articles'
    }
  ]
})

// Editor state
const showEditor = ref(false)
const editingPost = ref<BlogPost | null>(null)

// Methods
const handleCreateNew = () => {
  editingPost.value = null
  showEditor.value = true
}

const handleEditPost = (post: BlogPost) => {
  editingPost.value = post
  showEditor.value = true
}

const handleCancel = () => {
  editingPost.value = null
  showEditor.value = false
}

const handleSuccess = () => {
  editingPost.value = null
  showEditor.value = false
}
</script>
