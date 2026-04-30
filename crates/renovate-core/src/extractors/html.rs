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

    // Ported: "extractPackageFile" — html/extract.spec.ts line 8
    #[test]
    fn extracts_from_sample_html_fixture() {
        // Inlined from html/__fixtures__/sample.html — 10 cdnjs deps, unpkg URLs ignored.
        let html = r#"
<script type="text/javascript"
        src="https://cdnjs.cloudflare.com/ajax/libs/prop-types/15.6.1/prop-types.min.js"></script>
<script type="text/javascript"
        src="https://cdnjs.cloudflare.com/ajax/libs/react/16.3.2/umd/react.production.min.js"></script>
<script type="text/javascript"
        src="https://cdnjs.cloudflare.com/ajax/libs/react-dom/16.3.2/umd/react-dom.production.min.js"></script>
<script type="application/javascript" src="https://unpkg.com/babel-standalone@6.26.0/babel.js"></script>
<script type="text/javascript"
        src="https://cdnjs.cloudflare.com/ajax/libs/react-transition-group/2.2.1/react-transition-group.min.js"></script>
<script type="text/javascript"
        src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.3/umd/popper.min.js"></script>
<script type="text/javascript"
        src="https://cdnjs.cloudflare.com/ajax/libs/react-popper/0.10.4/umd/react-popper.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/reactstrap/7.1.0/reactstrap.min.js"></script>
<script src=" https://cdnjs.cloudflare.com/ajax/libs/react-router/4.3.1/react-router.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/react-markdown/4.0.6/react-markdown.js"></script>
<script src="https://unpkg.com/react-router-dom@4.3.1/umd/react-router-dom.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/axios/0.18.0/axios.min.js"
        integrity="sha256-mpnrJ5DpEZZkwkE1ZgkEQQJW/46CSEh/STrZKOB/qoM=" crossorigin="anonymous"></script>
"#;
        let deps = extract(html);
        assert_eq!(deps.len(), 10);
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "prop-types" && d.current_value == "15.6.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "react" && d.current_value == "16.3.2")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "react-dom" && d.current_value == "16.3.2")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "react-transition-group" && d.current_value == "2.2.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "popper.js" && d.current_value == "1.14.3")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "react-popper" && d.current_value == "0.10.4")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "reactstrap" && d.current_value == "7.1.0")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "react-router" && d.current_value == "4.3.1")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "react-markdown" && d.current_value == "4.0.6")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "axios" && d.current_value == "0.18.0")
        );
    }

    // Ported: "returns null" — html/extract.spec.ts line 21
    #[test]
    fn nothing_html_returns_empty() {
        // No cdnjs URLs → returns null/empty
        let html = "<html><head><title>Hello</title></head><body>Renovate</body></html>";
        assert!(extract(html).is_empty());
    }
}
