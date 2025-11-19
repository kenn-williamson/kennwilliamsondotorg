<template>
  <ClientOnly>
    <div class="markdown-content">
      <MdPreview
        :model-value="markdown"
        :theme="isDark ? 'dark' : 'light'"
        preview-theme="default"
        code-theme="github"
        :mermaid-config="mermaidConfig"
      />
    </div>
    <template #fallback>
      <div class="prose prose-lg max-w-none">
        <div class="animate-pulse">
          <div class="h-4 bg-nautical-200 rounded w-3/4 mb-4"></div>
          <div class="h-4 bg-nautical-200 rounded w-full mb-4"></div>
          <div class="h-4 bg-nautical-200 rounded w-5/6"></div>
        </div>
      </div>
    </template>
  </ClientOnly>
</template>

<script setup lang="ts">
import { MdPreview } from 'md-editor-v3'
import 'md-editor-v3/lib/preview.css'

defineProps<{
  markdown: string
}>()

// Optional: Dark mode support (can hook into app theme if needed)
const isDark = ref(false)

// Mermaid configuration
const mermaidConfig = {
  theme: 'default',
  logLevel: 'error',
  securityLevel: 'loose',
  startOnLoad: true,
  flowchart: {
    useMaxWidth: true,
    htmlLabels: true,
    curve: 'basis'
  }
}
</script>

<style>
/* Make md-editor background transparent to show parent gradient */
.markdown-content,
.markdown-content .md-editor,
.markdown-content .md-editor-previewOnly,
.markdown-content .md-editor-preview,
.markdown-content .md-editor-preview-wrapper,
.markdown-content .default-theme,
.markdown-content .md-editor-scrn {
  background: transparent !important;
  background-color: transparent !important;
}

/* Base text color */
.markdown-content {
  @apply text-nautical-800;
}

/* Fix for Tailwind preflight removing list styles */
.markdown-content .md-editor-preview ul {
  list-style-type: disc !important;
  list-style-position: outside !important;
  padding-left: 2em !important;
  margin: 1em 0 !important;
}

.markdown-content .md-editor-preview ol {
  list-style-type: decimal !important;
  list-style-position: outside !important;
  padding-left: 2em !important;
  margin: 1em 0 !important;
}

.markdown-content .md-editor-preview li {
  display: list-item !important;
  margin: 0.25em 0 !important;
}

/* Override md-editor-preview's word-break: break-all to prevent mid-word breaks */
.md-editor-preview {
  word-break: normal !important;
}

/* Typography matching About Me pages - professional, readable */
.markdown-content {
  @apply text-nautical-800;
}

.markdown-content h1 {
  @apply text-3xl sm:text-4xl font-bold text-primary-900 mb-6 pb-3 border-b-2 border-primary-300;
}

.markdown-content h2 {
  @apply text-2xl sm:text-3xl font-bold text-primary-800 mt-8 mb-4;
}

.markdown-content h3 {
  @apply text-xl sm:text-2xl font-semibold text-primary-700 mt-6 mb-3;
}

.markdown-content h4 {
  @apply text-lg sm:text-xl font-semibold text-primary-700 mt-4 mb-2;
}

.markdown-content p {
  @apply mb-4 leading-relaxed;
}

.markdown-content a {
  @apply text-primary-700 underline hover:text-primary-900 transition-colors;
}

.markdown-content blockquote {
  @apply border-l-4 border-primary-400 pl-4 italic text-nautical-700 my-4;
}

.markdown-content code {
  @apply bg-primary-100 px-2 py-1 rounded text-sm font-mono text-primary-800;
}

.markdown-content pre {
  @apply bg-nautical-800 text-nautical-100 p-4 rounded-lg overflow-x-auto my-4;
}

.markdown-content pre code {
  @apply bg-transparent p-0 text-nautical-100;
}

.markdown-content strong {
  @apply font-semibold text-nautical-900;
}

.markdown-content em {
  @apply italic;
}
</style>
