<template>
  <div ref="mermaidContainer" class="mermaid-diagram my-8"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import mermaid from 'mermaid'

const props = defineProps<{
  diagram: string
}>()

const mermaidContainer = ref<HTMLElement | null>(null)
let mermaidInitialized = false

const renderDiagram = async () => {
  if (!mermaidContainer.value) return

  try {
    if (!mermaidInitialized) {
      // Initialize Mermaid with custom theme using nautical colors
      mermaid.initialize({
        startOnLoad: false,
        theme: 'base',
        themeVariables: {
          // Nautical color scheme
          primaryColor: '#dbeafe',        // primary-100 - light blue backgrounds
          primaryTextColor: '#1e293b',    // nautical-800 - dark text
          primaryBorderColor: '#2563eb',  // primary-600 - primary blue borders

          lineColor: '#475569',           // nautical-600 - steel lines
          secondaryColor: '#cffafe',      // accent-100 - light cyan
          tertiaryColor: '#fef9c3',       // gold-100 - light gold

          // Text colors for readability
          mainBkg: '#ffffff',
          textColor: '#1e293b',
          labelTextColor: '#334155',

          // Node specific colors
          nodeBorder: '#2563eb',
          clusterBkg: '#f8fafc',
          clusterBorder: '#cbd5e1',

          // Edge/arrow colors
          edgeLabelBackground: '#ffffff',

          // Accent colors
          activeColor: '#06b6d4',
          doneColor: '#10b981',
          critBkgColor: '#fbbf24',
          critBorderColor: '#f59e0b'
        },
        flowchart: {
          htmlLabels: true,
          curve: 'basis',
          padding: 15
        }
      })
      mermaidInitialized = true
    }

    // Clear container and render
    const container = mermaidContainer.value
    container.innerHTML = props.diagram
    container.removeAttribute('data-processed')

    await mermaid.run({
      querySelector: '.mermaid-diagram'
    })
  } catch (error) {
    console.error('Mermaid rendering error:', error)
    if (mermaidContainer.value) {
      mermaidContainer.value.innerHTML = '<div class="text-red-600">Error rendering diagram</div>'
    }
  }
}

onMounted(() => {
  renderDiagram()
})

// Re-render if diagram prop changes
watch(() => props.diagram, () => {
  renderDiagram()
})
</script>

<style scoped>
.mermaid-diagram {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

/* Override Mermaid defaults for better visibility */
.mermaid-diagram :deep(svg) {
  max-width: 100%;
  height: auto;
}

.mermaid-diagram :deep(.node rect),
.mermaid-diagram :deep(.node circle),
.mermaid-diagram :deep(.node ellipse),
.mermaid-diagram :deep(.node polygon) {
  stroke-width: 2px;
}

.mermaid-diagram :deep(.edgeLabel) {
  background-color: white;
  padding: 4px 8px;
  border-radius: 4px;
}

.mermaid-diagram :deep(text) {
  font-family: inherit !important;
  fill: #1e293b !important;
}
</style>
