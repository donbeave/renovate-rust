//! HTML `*.html` / `*.htm` CDNJS dependency extractor.
//!
//! Scans `<script>` and `<link>` tags for cdnjs.cloudflare.com URLs and
//! extracts the library name, version, and asset path.
//!
//! Renovate reference:
//! - `lib/modules/manager/html/extract.ts`
//! - Pattern: `/\.html?$/`
//! - Datasource: CDNJS
//!
//! ## Matched URL form
//!
//! `//cdnjs.cloudflare.com/ajax/libs/{depName}/{currentValue}/{asset}`
//!
//! Example:
//! ```html
//! <script src="//cdnjs.cloudflare.com/ajax/libs/jquery/3.6.0/jquery.min.js"></script>
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single CDNJS dependency extracted from an HTML file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlCdnjsDep {
    /// Library name (e.g. `jquery`).
    pub dep_name: String,
    /// Version string (e.g. `3.6.0`).
    pub current_value: String,
    /// Asset path within the library (e.g. `jquery.min.js`).
    pub asset: String,
}

/// Matches a full `<script>` or `<link>` tag (non-greedy, single-line).
static TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)<\s*(?:script|link)\s+[^>]*?/?>").unwrap());

/// Matches the cdnjs URL within a tag.
static CDNJS_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"//cdnjs\.cloudflare\.com/ajax/libs/(?P<depName>[^/]+?)/(?P<version>[^/]+?)/(?P<asset>[-/_.a-zA-Z0-9]+)",
    )
    .unwrap()
});

/// Extract all CDNJS deps from an HTML file.
pub fn extract(content: &str) -> Vec<HtmlCdnjsDep> {
    let mut deps = Vec::new();
    let mut offset = 0;

    while let Some(tag_match) = TAG_RE.find(&content[offset..]) {
        let tag = tag_match.as_str();
        offset += tag_match.start() + tag.len();

        if let Some(caps) = CDNJS_URL_RE.captures(tag) {
            let dep_name = caps.name("depName").map(|m| m.as_str()).unwrap_or("");
            let version = caps.name("version").map(|m| m.as_str()).unwrap_or("");
            let asset = caps.name("asset").map(|m| m.as_str()).unwrap_or("");

            if !dep_name.is_empty() && !version.is_empty() && !asset.is_empty() {
                deps.push(HtmlCdnjsDep {
                    dep_name: dep_name.to_owned(),
                    current_value: version.to_owned(),
                    asset: asset.to_owned(),
                });
            }
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_script_tag() {
        let html = r#"<script src="//cdnjs.cloudflare.com/ajax/libs/jquery/3.6.0/jquery.min.js"></script>"#;
        let deps = extract(html);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "jquery");
        assert_eq!(deps[0].current_value, "3.6.0");
        assert_eq!(deps[0].asset, "jquery.min.js");
    }

    #[test]
    fn extracts_link_tag() {
        let html = r#"<link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css">"#;
        let deps = extract(html);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "font-awesome");
        assert_eq!(deps[0].current_value, "6.0.0");
        assert_eq!(deps[0].asset, "css/all.min.css");
    }

    #[test]
    fn extracts_multiple_tags() {
        let html = r#"
<script src="//cdnjs.cloudflare.com/ajax/libs/jquery/3.6.0/jquery.min.js"></script>
<link href="//cdnjs.cloudflare.com/ajax/libs/bootstrap/5.1.3/css/bootstrap.min.css">
"#;
        let deps = extract(html);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "jquery");
        assert_eq!(deps[1].dep_name, "bootstrap");
        assert_eq!(deps[1].current_value, "5.1.3");
    }

    #[test]
    fn no_cdnjs_tags_returns_empty() {
        let html = r#"<script src="/local/script.js"></script>"#;
        assert!(extract(html).is_empty());
    }

    #[test]
    fn non_cdnjs_script_ignored() {
        let html = r#"<script src="https://cdn.example.com/libs/foo/1.0/foo.js"></script>"#;
        assert!(extract(html).is_empty());
    }

    #[test]
    fn https_cdnjs_url() {
        let html = r#"<script src="https://cdnjs.cloudflare.com/ajax/libs/lodash.js/4.17.21/lodash.min.js"></script>"#;
        let deps = extract(html);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "lodash.js");
        assert_eq!(deps[0].current_value, "4.17.21");
    }
}
