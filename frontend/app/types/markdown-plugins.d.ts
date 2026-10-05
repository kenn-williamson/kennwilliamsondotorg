// Ambient type declarations for markdown-it plugins without official types

declare module '@markslides/markdown-it-mermaid' {
  import type { MarkdownIt } from 'markdown-it'

  const markdownItMermaid: (md: MarkdownIt) => void
  export default markdownItMermaid
}
