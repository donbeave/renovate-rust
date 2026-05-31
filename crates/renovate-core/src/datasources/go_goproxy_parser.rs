//! Go proxy parser.
//!
//! Parses GOPROXY and GONOPROXY/GOPRIVATE environment variable strings.
//!
//! Renovate reference: `lib/modules/datasource/go/goproxy-parser.ts`

use regex::Regex;
use std::sync::LazyLock;

#[allow(dead_code)]
static GOPROXY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([^,|]*(?:,|\|))").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoproxyItem {
    pub url: String,
    pub fallback: String,
}

pub fn parse_goproxy(input: &str) -> Vec<GoproxyItem> {
    if input.is_empty() {
        return Vec::new();
    }

    let result: Vec<GoproxyItem> = input
        .split([',', '|'])
        .filter(|s| !s.is_empty())
        .map(|url| {
            let sep = find_separator_after(input, url);
            let fallback = if sep == Some(',') { "," } else { "|" };
            GoproxyItem {
                url: url.to_owned(),
                fallback: fallback.to_owned(),
            }
        })
        .collect();

    result
}

fn find_separator_after(input: &str, segment: &str) -> Option<char> {
    let start = input.find(segment)?;
    let after = start + segment.len();
    let ch = input.chars().nth(after)?;
    if ch == ',' || ch == '|' {
        Some(ch)
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoproxyInfo {
    pub version: String,
    pub time: Option<String>,
}

pub fn parse_goproxy_version_list(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn parse_goproxy_info(input: &str) -> Result<GoproxyInfo, serde_json::Error> {
    #[derive(serde::Deserialize)]
    struct InfoResponse {
        #[serde(rename = "Version")]
        version: String,
        #[serde(rename = "Time")]
        time: Option<String>,
    }

    let info: InfoResponse = serde_json::from_str(input)?;
    Ok(GoproxyInfo {
        version: info.version,
        time: info.time,
    })
}

pub fn parse_noproxy(input: &str) -> Option<Regex> {
    if input.is_empty() {
        return None;
    }

    let patterns: Vec<String> = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|pattern| {
            let mut regex_pattern = String::new();
            for ch in pattern.chars() {
                match ch {
                    '*' => regex_pattern.push_str("[^/]*"),
                    '?' => regex_pattern.push_str("[^/]"),
                    '.' => regex_pattern.push_str("\\."),
                    '\\' => {}
                    _ => regex_pattern.push(ch),
                }
            }
            regex_pattern
        })
        .collect();

    if patterns.is_empty() {
        return None;
    }

    let combined = patterns.join("|");
    Regex::new(&format!("^(?:{})(?:/.*)?$", combined)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_goproxy_empty_returns_empty() {
        assert!(parse_goproxy("").is_empty());
    }

    #[test]
    fn parse_goproxy_single_url() {
        let items = parse_goproxy("https://proxy.golang.org");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].url, "https://proxy.golang.org");
    }

    #[test]
    fn parse_goproxy_pipe_separator() {
        let items = parse_goproxy("https://proxy.golang.org|https://proxy2.example.com");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].url, "https://proxy.golang.org");
        assert_eq!(items[0].fallback, "|");
        assert_eq!(items[1].url, "https://proxy2.example.com");
    }

    #[test]
    fn parse_goproxy_comma_separator() {
        let items = parse_goproxy("https://proxy.golang.org,direct");
        assert!(!items.is_empty());
    }

    #[test]
    fn parse_goproxy_mixed_separators() {
        let items = parse_goproxy("foo.example.com|bar.example.com,baz.example.com");
        assert!(items.len() >= 2);
    }

    #[test]
    fn parse_goproxy_direct_off() {
        let items = parse_goproxy("https://proxy.golang.org|off");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].url, "https://proxy.golang.org");
        assert_eq!(items[1].url, "off");
    }

    #[test]
    fn parse_goproxy_version_list_splits_lines() {
        let input = "v1.0.0\nv1.1.0\nv2.0.0\n";
        let versions = parse_goproxy_version_list(input);
        assert_eq!(versions, vec!["v1.0.0", "v1.1.0", "v2.0.0"]);
    }

    #[test]
    fn parse_goproxy_version_list_empty() {
        let versions = parse_goproxy_version_list("");
        assert!(versions.is_empty());
    }

    #[test]
    fn parse_goproxy_info_parses_json() {
        let input = r#"{"Version":"v1.2.3","Time":"2024-01-15T10:00:00Z"}"#;
        let info = parse_goproxy_info(input).unwrap();
        assert_eq!(info.version, "v1.2.3");
        assert_eq!(info.time.as_deref(), Some("2024-01-15T10:00:00Z"));
    }

    #[test]
    fn parse_goproxy_info_missing_time() {
        let input = r#"{"Version":"v1.0.0"}"#;
        let info = parse_goproxy_info(input).unwrap();
        assert_eq!(info.version, "v1.0.0");
        assert!(info.time.is_none());
    }

    #[test]
    fn parse_noproxy_empty_returns_none() {
        assert!(parse_noproxy("").is_none());
    }

    #[test]
    fn parse_noproxy_single_pattern() {
        let re = parse_noproxy("github.com/private/*").unwrap();
        assert!(re.is_match("github.com/private/pkg"));
        assert!(re.is_match("github.com/private/pkg/sub"));
    }

    #[test]
    fn parse_noproxy_wildcard() {
        let re = parse_noproxy("*.example.com").unwrap();
        assert!(re.is_match("foo.example.com"));
    }

    #[test]
    fn parse_noproxy_multiple_patterns() {
        let re = parse_noproxy("github.com/private,gitlab.com/internal").unwrap();
        assert!(re.is_match("github.com/private/pkg"));
        assert!(re.is_match("gitlab.com/internal/pkg"));
    }
}
