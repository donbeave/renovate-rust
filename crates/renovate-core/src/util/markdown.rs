//! Markdown utilities — mirrors `lib/util/markdown.ts`.

/// Create a markdown link: `[text](url)`.
pub fn link(text: &str, url: &str) -> String {
    format!("[{text}]({url})")
}

/// Bold text: `**text**`.
pub fn bold(text: &str) -> String {
    format!("**{text}**")
}

/// Italic text: `*text*`.
pub fn italic(text: &str) -> String {
    format!("*{text}*")
}

/// Markdown header with `level` `#` characters.
///
/// `level` is clamped to 1..=6.
pub fn header(text: &str, level: usize) -> String {
    let level = level.clamp(1, 6);
    let hashes = "#".repeat(level);
    format!("{hashes} {text}")
}

/// Fenced code block with optional language annotation.
pub fn code_block(code: &str, language: &str) -> String {
    format!("```{language}\n{code}\n```")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_basic() {
        assert_eq!(
            link("click", "https://example.com"),
            "[click](https://example.com)"
        );
    }

    #[test]
    fn link_empty_text() {
        assert_eq!(link("", "https://example.com"), "[](https://example.com)");
    }

    #[test]
    fn link_empty_url() {
        assert_eq!(link("click", ""), "[click]()");
    }

    #[test]
    fn link_special_chars() {
        assert_eq!(
            link("foo & bar", "https://ex.com?a=1&b=2"),
            "[foo & bar](https://ex.com?a=1&b=2)"
        );
    }

    #[test]
    fn bold_basic() {
        assert_eq!(bold("hello"), "**hello**");
    }

    #[test]
    fn bold_empty() {
        assert_eq!(bold(""), "****");
    }

    #[test]
    fn bold_with_existing_markdown() {
        assert_eq!(bold("*italic*"), "***italic***");
    }

    #[test]
    fn italic_basic() {
        assert_eq!(italic("hello"), "*hello*");
    }

    #[test]
    fn italic_empty() {
        assert_eq!(italic(""), "**");
    }

    #[test]
    fn header_h1() {
        assert_eq!(header("Title", 1), "# Title");
    }

    #[test]
    fn header_h2() {
        assert_eq!(header("Section", 2), "## Section");
    }

    #[test]
    fn header_h3() {
        assert_eq!(header("Subsection", 3), "### Subsection");
    }

    #[test]
    fn header_h6() {
        assert_eq!(header("Deep", 6), "###### Deep");
    }

    #[test]
    fn header_clamps_too_low() {
        assert_eq!(header("Title", 0), "# Title");
    }

    #[test]
    fn header_clamps_too_high() {
        assert_eq!(header("Title", 10), "###### Title");
    }

    #[test]
    fn code_block_basic() {
        assert_eq!(
            code_block("fn main() {}", "rust"),
            "```rust\nfn main() {}\n```"
        );
    }

    #[test]
    fn code_block_no_language() {
        assert_eq!(code_block("echo hello", ""), "```\necho hello\n```");
    }

    #[test]
    fn code_block_multiline() {
        assert_eq!(
            code_block("line1\nline2", "bash"),
            "```bash\nline1\nline2\n```"
        );
    }

    #[test]
    fn code_block_javascript() {
        assert_eq!(
            code_block("console.log('hi');", "javascript"),
            "```javascript\nconsole.log('hi');\n```"
        );
    }
}
