/// Utility functions for blog service operations
///
/// Generate URL-friendly slug from title
///
/// Converts title to lowercase, replaces non-alphanumeric characters with hyphens,
/// and removes consecutive hyphens.
///
/// # Examples
///
/// ```
/// use backend::services::blog::utils::generate_slug;
///
/// assert_eq!(generate_slug("Hello World"), "hello-world");
/// assert_eq!(generate_slug("Rust & Actix-Web"), "rust-actix-web");
/// assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
/// ```
pub fn generate_slug(title: &str) -> String {
    title
        .to_lowercase()
        .trim()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Strip markdown formatting to get plain text
///
/// Removes common markdown syntax including:
/// - Headers (#, ##, ###)
/// - Bold/italic (**, *, __, _)
/// - Links ([text](url))
/// - Code blocks (```)
/// - Inline code (`)
///
/// # Examples
///
/// ```
/// use backend::services::blog::utils::strip_markdown;
///
/// assert_eq!(strip_markdown("# Hello **World**"), "Hello World");
/// assert_eq!(strip_markdown("[Link](url) with `code`"), "Link with code");
/// ```
pub fn strip_markdown(markdown: &str) -> String {
    let mut text = markdown.to_string();

    // Remove code blocks (``` ... ```)
    while let Some(start) = text.find("```") {
        if let Some(end) = text[start + 3..].find("```") {
            text.replace_range(start..start + end + 6, "");
        } else {
            break;
        }
    }

    // Remove links [text](url) -> text
    while let Some(start) = text.find('[') {
        if let Some(middle) = text[start..].find("](") {
            if let Some(end) = text[start + middle..].find(')') {
                // Clone link_text to avoid borrow checker issues
                let link_text = text[start + 1..start + middle].to_string();
                text.replace_range(start..start + middle + end + 1, &link_text);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Remove inline code (` ... `)
    text = text.replace('`', "");

    // Remove bold/italic markers
    text = text.replace("**", "");
    text = text.replace("__", "");
    text = text.replace('*', "");
    text = text.replace('_', "");

    // Remove headers (# at start of line)
    text = text
        .lines()
        .map(|line| line.trim_start_matches('#').trim())
        .collect::<Vec<_>>()
        .join(" ");

    // Collapse multiple spaces
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Truncate text to maximum length, breaking at word boundary
///
/// If text is longer than max_len, truncates and appends "..."
/// Always breaks at word boundaries to avoid cutting words in half.
///
/// # Examples
///
/// ```
/// use backend::services::blog::utils::truncate_text;
///
/// assert_eq!(truncate_text("Short text", 20), "Short text");
/// assert_eq!(truncate_text("This is a very long text", 10), "This is a...");
/// ```
pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }

    // Find last space within max_len
    if let Some(pos) = text[..max_len].rfind(' ') {
        format!("{}...", &text[..pos])
    } else {
        // No spaces found, truncate at max_len
        format!("{}...", &text[..max_len])
    }
}

/// Generate excerpt from markdown content
///
/// Strips markdown formatting and truncates to specified length (default 160 chars).
/// Used for SEO meta descriptions and post previews.
///
/// # Examples
///
/// ```
/// use backend::services::blog::utils::generate_excerpt;
///
/// let markdown = "# Hello\n\nThis is a **bold** statement with [link](url).";
/// let excerpt = generate_excerpt(markdown, 20);
/// assert_eq!(excerpt, "Hello This is a...");
/// ```
pub fn generate_excerpt(markdown: &str, max_len: usize) -> String {
    let plain_text = strip_markdown(markdown);
    truncate_text(&plain_text, max_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
        assert_eq!(generate_slug("Rust & Actix-Web"), "rust-actix-web");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
        assert_eq!(
            generate_slug("Special!@#$%^&*()Characters"),
            "special-characters"
        );
        assert_eq!(generate_slug(""), "");
    }

    #[test]
    fn test_strip_markdown() {
        assert_eq!(strip_markdown("# Hello **World**"), "Hello World");
        assert_eq!(strip_markdown("[Link](url) with `code`"), "Link with code");
        assert_eq!(strip_markdown("**Bold** and *italic*"), "Bold and italic");
        assert_eq!(strip_markdown("## Header\n\nParagraph"), "Header Paragraph");
    }

    #[test]
    fn test_strip_markdown_code_blocks() {
        let markdown = "Text before\n```rust\ncode here\n```\nText after";
        assert_eq!(strip_markdown(markdown), "Text before Text after");
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("Short text", 20), "Short text");
        assert_eq!(
            truncate_text("This is a very long text", 10),
            "This is a..."
        );
        assert_eq!(truncate_text("NoSpacesHere", 5), "NoSpa...");
    }

    #[test]
    fn test_generate_excerpt() {
        let markdown = "# Hello\n\nThis is a **bold** statement with [link](url).";
        let excerpt = generate_excerpt(markdown, 20);
        assert!(excerpt.len() <= 23); // 20 + "..."
        assert!(excerpt.starts_with("Hello"));
    }

    #[test]
    fn test_generate_excerpt_short_content() {
        let markdown = "Short";
        assert_eq!(generate_excerpt(markdown, 160), "Short");
    }
}
