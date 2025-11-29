<template>
  <ClientOnly>
    <div
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

const props = defineProps<{
  markdown: string
}>()

const { $markdown } = useNuxtApp()

// Render markdown with sanitization
const renderedMarkdown = computed(() => {
  return $markdown.render(props.markdown)
})

// Initialize mermaid after component mounts
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

  renderMermaidDiagrams()
})

// Decode base64 mermaid code from data attributes and render
const renderMermaidDiagrams = async () => {
  await nextTick()

  // The markdown-it-mermaid plugin stores code in data-mermaid-code (base64 encoded)
  const mermaidElements = document.querySelectorAll('.mermaid[data-mermaid-code]')
  mermaidElements.forEach((el) => {
    const encoded = el.getAttribute('data-mermaid-code')
    if (encoded && !el.textContent?.trim()) {
      try {
        el.textContent = atob(encoded)
      } catch {
        console.error('Failed to decode mermaid content')
      }
    }
  })

  // Now run mermaid on all .mermaid elements
  if (mermaidElements.length > 0) {
    await mermaid.run()
  }
}

// Re-render mermaid diagrams when content changes
watch(() => props.markdown, async () => {
  await renderMermaidDiagrams()
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
