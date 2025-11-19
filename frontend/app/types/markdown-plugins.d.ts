// Ambient type declarations for markdown-it plugins without official types

declare module 'markdown-it-prism' {
  import type MarkdownIt from 'markdown-it'

  interface PrismOptions {
    defaultLanguage?: string
    defaultLanguageForUnknown?: string
    defaultLanguageForUnspecified?: string
    plugins?: string[]
  }

  const markdownItPrism: MarkdownIt.PluginWithOptions<PrismOptions>
  export default markdownItPrism
}

declare module '@markslides/markdown-it-mermaid' {
  import type MarkdownIt from 'markdown-it'

  const markdownItMermaid: MarkdownIt.PluginSimple
  export default markdownItMermaid
}
