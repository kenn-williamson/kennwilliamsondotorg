<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex justify-between items-center">
      <h2 class="text-2xl font-bold text-nautical-900">
        {{ editingPost ? 'Edit Post' : 'Create New Post' }}
      </h2>
      <button
        @click="$emit('cancel')"
        class="text-nautical-600 hover:text-nautical-900"
      >
        Cancel
      </button>
    </div>

    <form @submit.prevent="onSubmit" class="space-y-6">
      <!-- Title Field -->
      <div>
        <label for="title" class="block text-sm font-medium text-nautical-700 mb-2">
          Title <span class="text-red-500">*</span>
        </label>
        <Field
          name="title"
          type="text"
          :class="[
            'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
            errors.title ? 'border-red-300 bg-red-50' : 'border-nautical-300'
          ]"
          placeholder="Enter post title"
          @input="onTitleChange"
        />
        <ErrorMessage name="title" class="text-red-600 text-sm mt-1" />
      </div>

      <!-- Slug Field -->
      <div>
        <label for="slug" class="block text-sm font-medium text-nautical-700 mb-2">
          URL Slug
          <span class="text-xs text-nautical-500 ml-1">(auto-generated from title)</span>
        </label>
        <Field
          name="slug"
          type="text"
          :class="[
            'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
            errors.slug ? 'border-red-300 bg-red-50' : 'border-nautical-300'
          ]"
          placeholder="post-url-slug"
          @input="onSlugChange"
        />
        <ErrorMessage name="slug" class="text-red-600 text-sm mt-1" />
        <p class="text-xs text-nautical-500 mt-1">
          Only lowercase letters, numbers, and hyphens are allowed. Leave blank to auto-generate.
        </p>
      </div>

      <!-- Excerpt Field -->
      <div>
        <label for="excerpt" class="block text-sm font-medium text-nautical-700 mb-2">
          Excerpt
        </label>
        <Field
          as="textarea"
          name="excerpt"
          rows="3"
          :class="[
            'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
            errors.excerpt ? 'border-red-300 bg-red-50' : 'border-nautical-300'
          ]"
          placeholder="A brief summary of the post (optional)"
        />
        <ErrorMessage name="excerpt" class="text-red-600 text-sm mt-1" />
      </div>

      <!-- Content Field (Markdown Editor with Preview) -->
      <div>
        <div class="flex justify-between items-center mb-2">
          <label for="content" class="block text-sm font-medium text-nautical-700">
            Content <span class="text-red-500">*</span>
          </label>
          <button
            type="button"
            @click="showPreview = !showPreview"
            class="text-sm text-sky-600 hover:text-sky-700 font-medium"
          >
            {{ showPreview ? 'Hide Preview' : 'Show Preview' }}
          </button>
        </div>

        <div class="grid grid-cols-1 gap-4" :class="showPreview ? 'md:grid-cols-2' : ''">
          <!-- Editor -->
          <div>
            <Field
              as="textarea"
              name="content"
              rows="20"
              :class="[
                'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200 font-mono text-sm',
                errors.content ? 'border-red-300 bg-red-50' : 'border-nautical-300'
              ]"
              placeholder="Write your post content in Markdown..."
            />
            <ErrorMessage name="content" class="text-red-600 text-sm mt-1" />
            <p class="text-xs text-nautical-500 mt-1">
              Supports Markdown formatting
            </p>
          </div>

          <!-- Preview -->
          <div v-if="showPreview" class="border border-nautical-300 rounded-md p-4 bg-white overflow-auto" style="max-height: 500px;">
            <div class="text-xs font-medium text-nautical-500 mb-2 uppercase">Preview</div>
            <BlogPostContent v-if="values.content" :markdown="values.content" />
            <p v-else class="text-nautical-400 italic">Start typing to see preview...</p>
          </div>
        </div>
      </div>

      <!-- Featured Image Upload -->
      <BlogImageUpload
        :model-value="values.featured_image_url"
        @update:model-value="(val: string | null) => setFieldValue('featured_image_url', val)"
        :alt-text="values.featured_image_alt"
        @update:alt-text="(val: string | null) => setFieldValue('featured_image_alt', val)"
      />

      <!-- Featured Image Alt Text (if image exists) -->
      <div v-if="values.featured_image_url">
        <label for="featured_image_alt" class="block text-sm font-medium text-nautical-700 mb-2">
          Image Alt Text <span class="text-red-500">*</span>
        </label>
        <Field
          name="featured_image_alt"
          type="text"
          :class="[
            'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
            errors.featured_image_alt ? 'border-red-300 bg-red-50' : 'border-nautical-300'
          ]"
          placeholder="Describe the image for accessibility"
        />
        <ErrorMessage name="featured_image_alt" class="text-red-600 text-sm mt-1" />
      </div>

      <!-- Tags Field -->
      <div>
        <label for="tags" class="block text-sm font-medium text-nautical-700 mb-2">
          Tags
        </label>
        <input
          type="text"
          v-model="tagsInput"
          @keydown.enter.prevent="addTag"
          @keydown.comma.prevent="addTag"
          class="w-full px-4 py-3 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
          placeholder="Type a tag and press Enter"
        />
        <div v-if="values.tags && values.tags.length > 0" class="flex flex-wrap gap-2 mt-2">
          <span
            v-for="(tag, index) in values.tags"
            :key="index"
            class="inline-flex items-center gap-1 px-3 py-1 bg-sky-100 text-sky-700 rounded-full text-sm"
          >
            {{ tag }}
            <button
              type="button"
              @click="removeTag(index)"
              class="hover:text-sky-900"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </span>
        </div>
        <ErrorMessage name="tags" class="text-red-600 text-sm mt-1" />
      </div>

      <!-- Status Field -->
      <div>
        <label for="status" class="block text-sm font-medium text-nautical-700 mb-2">
          Status <span class="text-red-500">*</span>
        </label>
        <Field
          as="select"
          name="status"
          :class="[
            'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
            errors.status ? 'border-red-300 bg-red-50' : 'border-nautical-300'
          ]"
        >
          <option value="draft">Draft</option>
          <option value="published">Published</option>
        </Field>
        <ErrorMessage name="status" class="text-red-600 text-sm mt-1" />
      </div>

      <!-- Meta Description Field (SEO) -->
      <div>
        <label for="meta_description" class="block text-sm font-medium text-nautical-700 mb-2">
          Meta Description (SEO)
        </label>
        <Field
          as="textarea"
          name="meta_description"
          rows="2"
          :class="[
            'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
            errors.meta_description ? 'border-red-300 bg-red-50' : 'border-nautical-300'
          ]"
          placeholder="Brief description for search engines (max 160 characters)"
          maxlength="160"
        />
        <ErrorMessage name="meta_description" class="text-red-600 text-sm mt-1" />
        <p class="text-xs text-nautical-500 mt-1">
          {{ values.meta_description?.length || 0 }}/160 characters
        </p>
      </div>

      <!-- Error Display -->
      <div v-if="blogStore.hasError" class="bg-red-50 border border-red-300 rounded-md p-4">
        <p class="text-red-700">{{ blogStore.error }}</p>
      </div>

      <!-- Submit Buttons -->
      <div class="flex justify-end gap-4">
        <button
          type="button"
          @click="saveDraft"
          :disabled="isSubmitting"
          class="px-6 py-3 border border-nautical-300 text-nautical-700 rounded-md hover:bg-nautical-50 transition-colors font-medium disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Save as Draft
        </button>
        <button
          type="submit"
          :disabled="isSubmitting || !isFormValid"
          :class="[
            'px-6 py-3 rounded-md font-medium transition-colors',
            isSubmitting || !isFormValid
              ? 'bg-nautical-300 text-nautical-500 cursor-not-allowed'
              : 'bg-sky-600 text-white hover:bg-sky-700'
          ]"
        >
          <span v-if="isSubmitting" class="flex items-center">
            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            {{ editingPost ? 'Updating...' : 'Saving...' }}
          </span>
          <span v-else>{{ editingPost ? 'Update Post' : 'Save Post' }}</span>
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useForm, Field, ErrorMessage } from 'vee-validate'
import { blogPostSchema, generateSlugFromTitle } from '#shared/schemas/blog'
import { useBlogStore } from '~/stores/blog'
import type { BlogPost } from '#shared/types'

// Props
const props = defineProps<{
  editingPost?: BlogPost | null
}>()

// Emits
const emit = defineEmits<{
  'cancel': []
  'success': []
}>()

// Store
const blogStore = useBlogStore()

// Form setup
const { handleSubmit, errors, isSubmitting, setFieldValue, values, resetForm } = useForm({
  validationSchema: blogPostSchema,
  initialValues: {
    title: props.editingPost?.title || '',
    slug: props.editingPost?.slug || '',
    excerpt: props.editingPost?.excerpt || '',
    content: props.editingPost?.content || '',
    featured_image_url: props.editingPost?.featured_image_url || null,
    featured_image_alt: props.editingPost?.featured_image_alt || null,
    tags: props.editingPost?.tags || [],
    status: props.editingPost?.status || 'draft',
    meta_description: props.editingPost?.meta_description || ''
  }
})

// Tags input state
const tagsInput = ref('')

// Preview state
const showPreview = ref(false)

// Track if slug was manually edited
const slugManuallyEdited = ref(!!props.editingPost?.slug)

// Computed
const isFormValid = computed(() => {
  return values.title && values.content && values.status &&
    !errors.value.title && !errors.value.content
})

// Methods
const onTitleChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  setFieldValue('title', target.value)

  // Auto-generate slug if creating new post and slug hasn't been manually edited
  if (!props.editingPost && !slugManuallyEdited.value) {
    const generatedSlug = generateSlugFromTitle(target.value)
    setFieldValue('slug', generatedSlug)
  }
}

const onSlugChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  setFieldValue('slug', target.value)
  // Mark slug as manually edited if user types in it
  slugManuallyEdited.value = true
}

const addTag = () => {
  const tag = tagsInput.value.trim().toLowerCase()
  const currentTags = values.tags as string[]
  if (tag && !currentTags.includes(tag) && currentTags.length < 10) {
    setFieldValue('tags', [...currentTags, tag])
    tagsInput.value = ''
  }
}

const removeTag = (index: number) => {
  const currentTags = [...(values.tags as string[])]
  currentTags.splice(index, 1)
  setFieldValue('tags', currentTags)
}

const saveDraft = async () => {
  setFieldValue('status', 'draft')
  await onSubmit()
}

const onSubmit = handleSubmit(async (values) => {
  try {
    const postData = {
      title: values.title,
      slug: values.slug || generateSlugFromTitle(values.title),
      excerpt: values.excerpt || undefined,
      content: values.content,
      featured_image_url: values.featured_image_url || undefined,
      featured_image_alt: values.featured_image_alt || undefined,
      tags: values.tags || [],
      status: values.status as 'draft' | 'published',
      meta_description: values.meta_description || undefined
    }

    if (props.editingPost) {
      await blogStore.updatePost(props.editingPost.id, postData)
    } else {
      await blogStore.createPost(postData)
    }

    // Success - emit event and reset form
    emit('success')
    resetForm()
    slugManuallyEdited.value = false
    showPreview.value = false
  } catch (error) {
    console.error('Failed to save post:', error)
  }
})

// Watch for editing post changes
watch(() => props.editingPost, (newPost) => {
  if (newPost) {
    resetForm({
      values: {
        title: newPost.title,
        slug: newPost.slug,
        excerpt: newPost.excerpt || '',
        content: newPost.content,
        featured_image_url: newPost.featured_image_url || null,
        featured_image_alt: newPost.featured_image_alt || null,
        tags: newPost.tags || [],
        status: newPost.status,
        meta_description: newPost.meta_description || ''
      }
    })
    slugManuallyEdited.value = true // Existing post slug shouldn't be auto-updated
  } else {
    // Creating new post - allow auto-generation
    slugManuallyEdited.value = false
  }
}, { immediate: true })
</script>
