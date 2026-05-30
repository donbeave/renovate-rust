//! Custom datasource format fetchers.
//!
//! Ports `lib/modules/datasource/custom/formats/` — each format knows how to
//! parse raw HTTP/file content into a `CustomReleaseResult`.

use super::CustomReleaseResult;

pub fn parse_json(content: &str) -> Option<CustomReleaseResult> {
    serde_json::from_str(content).ok()
}

pub fn parse_yaml(content: &str) -> Option<CustomReleaseResult> {
    serde_yaml::from_str(content).ok()
}

pub fn parse_toml(content: &str) -> Option<CustomReleaseResult> {
    toml::from_str(content).ok()
}

pub fn parse_plain(content: &str) -> CustomReleaseResult {
    super::parse_plain_to_releases(content)
}

pub fn parse_html(content: &str) -> CustomReleaseResult {
    super::extract_html_links(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json_releases() {
        let json = r#"{"releases": [{"version": "1.0.0"}]}"#;
        let result = parse_json(json).unwrap();
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.0.0");
    }

    #[test]
    fn parse_json_invalid() {
        assert!(parse_json("not json").is_none());
    }

    #[test]
    fn parse_yaml_releases() {
        let yaml = "releases:\n  - version: \"1.0.0\"\n";
        let result = parse_yaml(yaml).unwrap();
        assert_eq!(result.releases.len(), 1);
    }

    #[test]
    fn parse_toml_releases() {
        let toml_str = "[[releases]]\nversion = \"1.0.0\"\n";
        let result = parse_toml(toml_str).unwrap();
        assert_eq!(result.releases.len(), 1);
    }

    #[test]
    fn parse_plain_versions() {
        let plain = "1.0.0\n2.0.0\n";
        let result = parse_plain(plain);
        assert_eq!(result.releases.len(), 2);
    }

    #[test]
    fn parse_html_versions() {
        let html = r#"<a href="1.0.0">1.0.0</a>"#;
        let result = parse_html(html);
        assert_eq!(result.releases.len(), 1);
    }
}
