import MarkdownIt from 'markdown-it'
import markdownItPrism from 'markdown-it-prism'
import markdownItMermaid from '@markslides/markdown-it-mermaid'
import DOMPurify from 'dompurify'

export default defineNuxtPlugin(() => {
  // Initialize markdown-it with security and formatting options
  const md = new MarkdownIt({
    html: true,          // Enable HTML tags in source
    linkify: true,       // Auto-convert URLs to links
    typographer: true,   // Enable smartquotes and other nice typographic replacements
    breaks: false,       // Don't convert \n to <br> (proper paragraph handling)
  })

  // Add syntax highlighting plugin
  md.use(markdownItPrism, {
    defaultLanguage: 'plaintext',
  })

  // Add mermaid diagram support
  md.use(markdownItMermaid)

  // Sanitize HTML to prevent XSS attacks
  const renderSafe = (markdown: string): string => {
    const rawHtml = md.render(markdown)

    // Configure DOMPurify to allow mermaid SVG elements
    return DOMPurify.sanitize(rawHtml, {
      ADD_TAGS: ['svg', 'g', 'path', 'rect', 'text', 'tspan', 'foreignObject', 'marker', 'defs', 'line', 'polyline', 'polygon', 'circle', 'ellipse'],
      ADD_ATTR: ['style', 'class', 'id', 'data-*', 'viewBox', 'transform', 'fill', 'stroke', 'stroke-width', 'x', 'y', 'width', 'height', 'cx', 'cy', 'r', 'rx', 'ry', 'points', 'd', 'marker-end', 'marker-start'],
    })
  }

  return {
    provide: {
      markdown: {
        render: renderSafe,
        // Expose unsafe version for trusted content if needed (use with caution)
        renderUnsafe: (markdown: string) => md.render(markdown),
      },
    },
  }
})
