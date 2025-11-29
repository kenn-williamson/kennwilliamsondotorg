import MarkdownIt from 'markdown-it'
import markdownItMermaid from '@markslides/markdown-it-mermaid'
import DOMPurify from 'dompurify'
import Prism from 'prismjs'

// Pre-import languages to avoid dynamic require.resolve (which doesn't work in browser)
// Core languages commonly used in blog posts
import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-typescript'
import 'prismjs/components/prism-css'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-bash'
import 'prismjs/components/prism-yaml'
import 'prismjs/components/prism-markdown'
import 'prismjs/components/prism-sql'
import 'prismjs/components/prism-rust'
import 'prismjs/components/prism-python'
import 'prismjs/components/prism-docker'
import 'prismjs/components/prism-toml'
import 'prismjs/components/prism-markup' // HTML, XML, SVG
import 'prismjs/components/prism-csharp'


export default defineNuxtPlugin(() => {
  // Initialize markdown-it with security and formatting options
  const md = new MarkdownIt({
    html: true,          // Enable HTML tags in source
    linkify: true,       // Auto-convert URLs to links
    typographer: true,   // Enable smartquotes and other nice typographic replacements
    breaks: false,       // Don't convert \n to <br> (proper paragraph handling)
    highlight: (code: string, lang: string): string => {
      // Use Prism for syntax highlighting if language is supported
      const language = lang && Prism.languages[lang] ? lang : 'plaintext'
      if (Prism.languages[language]) {
        return Prism.highlight(code, Prism.languages[language], language)
      }
      // Fallback: escape HTML and return as-is
      return md.utils.escapeHtml(code)
    },
  })

  // Add mermaid diagram support
  md.use(markdownItMermaid)

  // Sanitize HTML to prevent XSS attacks
  const renderSafe = (markdown: string): string => {
    const rawHtml = md.render(markdown)

    // Configure DOMPurify to allow mermaid SVG elements and br for mermaid line breaks
    return DOMPurify.sanitize(rawHtml, {
      ADD_TAGS: ['svg', 'g', 'path', 'rect', 'text', 'tspan', 'foreignObject', 'marker', 'defs', 'line', 'polyline', 'polygon', 'circle', 'ellipse', 'br'],
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
