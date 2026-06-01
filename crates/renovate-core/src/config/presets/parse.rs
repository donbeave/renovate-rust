//! Preset string parsing.
//!
//! Renovate reference: `lib/config/presets/parse.ts`.

use regex::Regex;

/// Parsed preset reference components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetReference {
    pub preset_source: String,
    pub repo: String,
    pub preset_path: Option<String>,
    pub preset_name: String,
    pub tag: Option<String>,
    pub params: Option<Vec<String>>,
    pub raw_params: Option<String>,
}

/// Error messages matching Renovate's preset error constants.
pub const PRESET_INVALID: &str = "invalid preset";
pub const PRESET_PROHIBITED_SUBPRESET: &str = "prohibited sub-preset";

/// Known internal preset package prefixes.
const PRESETS_PACKAGES: &[&str] = &[
    "abandonments",
    "compatibility",
    "config",
    "customManagers",
    "default",
    "docker",
    "global",
    "group",
    "helpers",
    "mergeConfidence",
    "monorepo",
    "npm",
    "packages",
    "preview",
    "replacements",
    "schedule",
    "security",
    "workarounds",
];

/// Parse a preset string into its component parts.
///
/// Mirrors `parsePreset()` from `lib/config/presets/parse.ts`.
pub fn parse_preset(input: &str) -> Result<PresetReference, String> {
    let mut str = input.to_owned();
    let mut preset_source = None;
    let mut preset_path = None;
    let mut repo = String::new();
    let mut preset_name = String::new();
    let mut tag = None;
    let mut raw_params = None;
    let mut params = None;

    if str.starts_with("github>") {
        preset_source = Some("github");
        str = str["github>".len()..].to_owned();
    } else if str.starts_with("gitlab>") {
        preset_source = Some("gitlab");
        str = str["gitlab>".len()..].to_owned();
    } else if str.starts_with("gitea>") {
        preset_source = Some("gitea");
        str = str["gitea>".len()..].to_owned();
    } else if str.starts_with("forgejo>") {
        preset_source = Some("forgejo");
        str = str["forgejo>".len()..].to_owned();
    } else if str.starts_with("local>") {
        preset_source = Some("local");
        str = str["local>".len()..].to_owned();
    } else if is_http_url(&str) {
        preset_source = Some("http");
    } else if !str.starts_with('@') && !str.starts_with(':') && str.contains('/') {
        preset_source = Some("local");
    }

    if str.starts_with("npm>") {
        str = str["npm>".len()..].to_owned();
    }

    let mut preset_source = preset_source.unwrap_or("npm").to_owned();

    if str.contains('(') {
        let start = str.find('(').unwrap();
        let end = str.rfind(')').unwrap_or(str.len());
        raw_params = Some(str[start + 1..end].to_owned());
        params = Some(
            str[start + 1..end]
                .split(',')
                .map(|s| s.trim().to_owned())
                .collect(),
        );
        str = str[..start].to_owned();
    }

    if preset_source == "http" {
        return Ok(PresetReference {
            preset_source,
            repo: str,
            preset_path: None,
            preset_name: String::new(),
            tag: None,
            params,
            raw_params,
        });
    }

    if PRESETS_PACKAGES
        .iter()
        .any(|pkg| str.starts_with(&format!("{pkg}:")))
    {
        let source = "internal";
        let parts: Vec<&str> = str.splitn(2, ':').collect();
        repo = parts[0].to_owned();
        preset_name = parts.get(1).unwrap_or(&"").to_string();
        preset_source = source.to_owned();
    } else if let Some(rest) = str.strip_prefix(':') {
        preset_source = "internal".to_owned();
        repo = "default".to_owned();
        preset_name = rest.to_owned();
    } else if str.starts_with('@') {
        let re = Regex::new(r"^(@[^:/]+)(?::?(.*))?$").unwrap();
        if let Some(caps) = re.captures(&str) {
            repo = caps[1].to_owned();
            let rest = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            if !repo.contains('/') {
                repo = format!("{repo}/renovate-config");
            }
            preset_name = if rest.is_empty() {
                "default".to_owned()
            } else {
                rest.to_owned()
            };
        }
    } else if str.contains("//") {
        if str.contains(':') {
            return Err(PRESET_PROHIBITED_SUBPRESET.to_owned());
        }
        let re = Regex::new(
            r"^(?P<repo>~?[\w\-. /%]+?)\/\/(?:(?P<presetPath>[\w\-./]+)\/)?(?P<presetName>[\w\-.]+)(?:#(?P<tag>[\w\-./]+?))?$",
        ).unwrap();
        let Some(caps) = re.captures(&str) else {
            return Err(PRESET_INVALID.to_owned());
        };
        repo = caps.name("repo").unwrap().as_str().to_owned();
        preset_path = caps.name("presetPath").map(|m| m.as_str().to_owned());
        preset_name = caps
            .name("presetName")
            .map(|m| m.as_str().to_owned())
            .unwrap_or_default();
        tag = caps.name("tag").map(|m| m.as_str().to_owned());
    } else {
        let re = Regex::new(
            r"^(?P<repo>~?[\w\-. /%]+)(?::(?P<presetName>[\w\-.+/]+))?(?:#(?P<tag>[\w\-./]+?))?$",
        )
        .unwrap();
        let Some(caps) = re.captures(&str) else {
            return Err(PRESET_INVALID.to_owned());
        };
        repo = caps.name("repo").unwrap().as_str().to_owned();
        preset_name = caps
            .name("presetName")
            .map(|m| m.as_str().to_owned())
            .unwrap_or_default();
        tag = caps.name("tag").map(|m| m.as_str().to_owned());

        if preset_source == "npm" && !repo.starts_with("renovate-config-") {
            repo = format!("renovate-config-{repo}");
        }
        if preset_name.is_empty() {
            preset_name = "default".to_owned();
        }
    }

    Ok(PresetReference {
        preset_source,
        repo,
        preset_path,
        preset_name,
        tag,
        params,
        raw_params,
    })
}

fn is_http_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "parses github>owner/repo:file" — config/presets/parse.spec.ts
    #[test]
    fn parse_github_preset() {
        let result = parse_preset("github>owner/repo:file").unwrap();
        assert_eq!(result.preset_source, "github");
        assert_eq!(result.repo, "owner/repo");
        assert_eq!(result.preset_name, "file");
    }

    #[test]
    fn parse_github_preset_with_tag() {
        let result = parse_preset("github>owner/repo:file#v1.0").unwrap();
        assert_eq!(result.preset_source, "github");
        assert_eq!(result.tag.as_deref(), Some("v1.0"));
    }

    #[test]
    fn parse_gitlab_preset() {
        let result = parse_preset("gitlab>owner/repo:preset").unwrap();
        assert_eq!(result.preset_source, "gitlab");
        assert_eq!(result.repo, "owner/repo");
        assert_eq!(result.preset_name, "preset");
    }

    #[test]
    fn parse_gitea_preset() {
        let result = parse_preset("gitea>owner/repo:preset").unwrap();
        assert_eq!(result.preset_source, "gitea");
    }

    #[test]
    fn parse_forgejo_preset() {
        let result = parse_preset("forgejo>owner/repo:preset").unwrap();
        assert_eq!(result.preset_source, "forgejo");
    }

    #[test]
    fn parse_local_preset() {
        let result = parse_preset("local>owner/repo:preset").unwrap();
        assert_eq!(result.preset_source, "local");
    }

    #[test]
    fn parse_http_preset() {
        let result = parse_preset("https://example.com/renovate.json").unwrap();
        assert_eq!(result.preset_source, "http");
        assert_eq!(result.repo, "https://example.com/renovate.json");
        assert_eq!(result.preset_name, "");
    }

    #[test]
    fn parse_default_internal_preset() {
        let result = parse_preset(":pinVersions").unwrap();
        assert_eq!(result.preset_source, "internal");
        assert_eq!(result.repo, "default");
        assert_eq!(result.preset_name, "pinVersions");
    }

    #[test]
    fn parse_named_internal_preset() {
        let result = parse_preset("group:monorepo").unwrap();
        assert_eq!(result.preset_source, "internal");
        assert_eq!(result.repo, "group");
        assert_eq!(result.preset_name, "monorepo");
    }

    #[test]
    fn parse_scoped_npm_preset() {
        let result = parse_preset("@scope:preset").unwrap();
        assert_eq!(result.preset_source, "npm");
        assert_eq!(result.repo, "@scope/renovate-config");
        assert_eq!(result.preset_name, "preset");
    }

    #[test]
    fn parse_plain_npm_preset() {
        let result = parse_preset("foo").unwrap();
        assert_eq!(result.preset_source, "npm");
        assert_eq!(result.repo, "renovate-config-foo");
        assert_eq!(result.preset_name, "default");
    }

    #[test]
    fn parse_preset_with_params() {
        let result = parse_preset("group:monorepo(arg1, arg2)").unwrap();
        assert_eq!(
            result.params.as_deref(),
            Some(&["arg1".to_owned(), "arg2".to_owned()][..])
        );
        assert_eq!(result.raw_params.as_deref(), Some("arg1, arg2"));
    }

    #[test]
    fn parse_non_scoped_with_subdir() {
        let result = parse_preset("owner/repo//path/to/preset").unwrap();
        assert_eq!(result.repo, "owner/repo");
        assert_eq!(result.preset_path.as_deref(), Some("path/to"));
        assert_eq!(result.preset_name, "preset");
    }

    #[test]
    fn parse_prohibited_subpreset_with_colon() {
        let result = parse_preset("owner/repo//path:subpreset");
        assert_eq!(result.unwrap_err(), PRESET_PROHIBITED_SUBPRESET);
    }

    #[test]
    fn parse_local_auto_detect() {
        let result = parse_preset("some/path").unwrap();
        assert_eq!(result.preset_source, "local");
    }
}
