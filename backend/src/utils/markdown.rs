//! Markdown to HTML conversion utilities
//!
//! Uses pulldown-cmark for consistent markdown rendering across the application.

use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/// Convert markdown content to HTML
///
/// Supports standard markdown plus:
/// - Strikethrough (~~text~~)
/// - Tables
/// - Mermaid diagrams (rendered as `<pre class="mermaid">` for client-side rendering)
///
/// # Example
///
/// ```
/// use backend::utils::markdown_to_html;
///
/// let html = markdown_to_html("# Hello\n\nThis is **bold**.");
/// assert!(html.contains("<h1>Hello</h1>"));
/// assert!(html.contains("<strong>bold</strong>"));
/// ```
pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut in_mermaid = false;

    let events = parser.map(|event| match &event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
            if lang.as_ref() == "mermaid" =>
        {
            in_mermaid = true;
            Event::Html("<pre class=\"mermaid\">".into())
        }
        Event::End(TagEnd::CodeBlock) if in_mermaid => {
            in_mermaid = false;
            Event::Html("</pre>".into())
        }
        Event::Text(text) if in_mermaid => {
            // Pass through the mermaid code as-is (HTML escaped)
            Event::Html(html_escape(text).into())
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, events);
    html_output
}

/// Escape HTML special characters in text
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let html = markdown_to_html("# Hello\n\nThis is **bold** and *italic*.");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_code_blocks() {
        let markdown = "Here is `inline code` and:\n\n```rust\nfn main() {}\n```";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<code>inline code</code>"));
        assert!(html.contains("<pre>"));
    }

    #[test]
    fn test_strikethrough() {
        let html = markdown_to_html("This is ~~deleted~~ text.");
        assert!(html.contains("<del>deleted</del>"));
    }

    #[test]
    fn test_tables() {
        let markdown = "| Header |\n|--------|\n| Cell   |";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header</th>"));
        assert!(html.contains("<td>Cell</td>"));
    }

    #[test]
    fn test_mermaid_blocks() {
        let markdown = "```mermaid\nflowchart TD\n    A --> B\n```";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<pre class=\"mermaid\">"));
        assert!(html.contains("flowchart TD"));
        assert!(!html.contains("language-mermaid"));
    }

    #[test]
    fn test_mermaid_html_escaping() {
        let markdown = "```mermaid\nA -->|\"label\"| B\n```";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<pre class=\"mermaid\">"));
        assert!(html.contains("&gt;"));
    }

    #[test]
    fn test_non_mermaid_code_blocks_unchanged() {
        let markdown = "```rust\nfn main() {}\n```";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<pre>"));
        assert!(html.contains("<code"));
        assert!(!html.contains("class=\"mermaid\""));
    }
}
