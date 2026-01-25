<template>
  <!-- SSR path: pre-rendered HTML from backend -->
  <div
    v-if="html"
    ref="contentRef"
    class="markdown-content"
    v-html="html"
  />

  <!-- Client-only path: live markdown preview (for admin editor) -->
  <ClientOnly v-else>
    <div
      ref="contentRef"
      class="markdown-content"
      v-html="renderedMarkdown"
    />
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
import mermaid from 'mermaid'
import Prism from 'prismjs'

const props = defineProps<{
  /** Pre-rendered HTML from backend (SSR-safe) */
  html?: string
  /** Raw markdown for client-side rendering (admin preview) */
  markdown?: string
}>()

const contentRef = ref<HTMLElement | null>(null)
const { $markdown } = useNuxtApp()

// Render markdown with sanitization (only used for client-only path)
const renderedMarkdown = computed(() => {
  if (!props.markdown) return ''
  return $markdown.render(props.markdown)
})

// Initialize mermaid and Prism after component mounts
onMounted(() => {
  mermaid.initialize({
    theme: 'default',
    logLevel: 'error',
    securityLevel: 'loose',
    startOnLoad: false, // We'll trigger manually after decoding
    flowchart: {
      useMaxWidth: true,
      htmlLabels: true,
      curve: 'basis',
    },
  })

  enhanceContent()
})

// Enhance content with syntax highlighting and mermaid diagrams
const enhanceContent = async () => {
  await nextTick()

  if (!contentRef.value) return

  // Apply Prism syntax highlighting to code blocks
  Prism.highlightAllUnder(contentRef.value)

  // Decode and render mermaid diagrams
  await renderMermaidDiagrams()
}

// Render mermaid diagrams from backend-rendered <pre class="mermaid"> blocks
const renderMermaidDiagrams = async () => {
  if (!contentRef.value) return

  // Backend renders mermaid blocks as <pre class="mermaid">diagram code</pre>
  // The content is HTML-escaped, so we need to decode it before mermaid can parse
  const mermaidElements = contentRef.value.querySelectorAll('pre.mermaid')

  mermaidElements.forEach((el) => {
    // Decode HTML entities (backend escapes <, >, & for safety)
    const encoded = el.innerHTML
    if (encoded) {
      el.textContent = encoded
        .replace(/&lt;/g, '<')
        .replace(/&gt;/g, '>')
        .replace(/&amp;/g, '&')
    }
  })

  // Run mermaid on all .mermaid elements
  if (mermaidElements.length > 0) {
    await mermaid.run()
  }
}

// Re-enhance content when props change (for admin preview)
watch(() => props.markdown, async () => {
  await enhanceContent()
})

watch(() => props.html, async () => {
  await enhanceContent()
})
</script>

<style>
/* Import Prism syntax highlighting theme - dark theme for code blocks */
@import 'prismjs/themes/prism-tomorrow.css';

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

/* Mermaid diagrams should have a light/transparent background */
.markdown-content pre.mermaid {
  @apply bg-transparent p-0;
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

/* Fix for Tailwind preflight removing list styles */
.markdown-content ul {
  list-style-type: disc !important;
  list-style-position: outside !important;
  padding-left: 2em !important;
  margin: 1em 0 !important;
}

.markdown-content ol {
  list-style-type: decimal !important;
  list-style-position: outside !important;
  padding-left: 2em !important;
  margin: 1em 0 !important;
}

.markdown-content li {
  display: list-item !important;
  margin: 0.25em 0 !important;
}

/* Ensure normal word wrapping (no mid-word breaks) */
.markdown-content,
.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6,
.markdown-content p {
  word-break: normal;
  overflow-wrap: normal;
}
</style>
