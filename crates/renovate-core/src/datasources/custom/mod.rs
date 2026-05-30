//! Custom datasource framework.
//!
//! Ports `lib/modules/datasource/custom/` — user-defined datasources that fetch
//! data from arbitrary URLs in JSON/YAML/TOML/plain/html format and transform
//! via JSONata expressions.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod formats;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomRelease {
    pub version: String,
    #[serde(rename = "isDeprecated", skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[serde(
        rename = "releaseTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_timestamp: Option<String>,
    #[serde(rename = "sourceUrl", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(
        rename = "sourceDirectory",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_directory: Option<String>,
    #[serde(rename = "changelogUrl", skip_serializing_if = "Option::is_none")]
    pub changelog_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "isStable", skip_serializing_if = "Option::is_none")]
    pub is_stable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomReleaseResult {
    pub releases: Vec<CustomRelease>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    #[serde(rename = "sourceUrl", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(
        rename = "sourceDirectory",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_directory: Option<String>,
    #[serde(rename = "changelogUrl", skip_serializing_if = "Option::is_none")]
    pub changelog_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum CustomFormat {
    #[default]
    Json,
    Yaml,
    Toml,
    Plain,
    Html,
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct CustomDatasourceConfig {
    pub format: CustomFormat,
    pub default_registry_url_template: String,
    pub transform_templates: Vec<String>,
}

pub fn compile_template(template: &str, package_name: &str, current_value: Option<&str>) -> String {
    let mut result = template.to_owned();
    result = result.replace("{{packageName}}", package_name);
    if let Some(cv) = current_value {
        result = result.replace("{{currentValue}}", cv);
    } else {
        result = result.replace("{{currentValue}}", "");
    }
    result
}

pub fn parse_plain_to_releases(content: &str) -> CustomReleaseResult {
    let releases = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|version| CustomRelease {
            version: version.to_owned(),
            is_deprecated: None,
            release_timestamp: None,
            source_url: None,
            source_directory: None,
            changelog_url: None,
            digest: None,
            is_stable: None,
        })
        .collect();

    CustomReleaseResult {
        releases,
        tags: None,
        source_url: None,
        source_directory: None,
        changelog_url: None,
        homepage: None,
    }
}

pub fn extract_html_links(content: &str) -> CustomReleaseResult {
    let href_re = regex::Regex::new(r#"href="([^"]*)""#).unwrap();
    let releases: Vec<CustomRelease> = href_re
        .captures_iter(content)
        .filter_map(|cap| cap.get(1))
        .map(|m| CustomRelease {
            version: m.as_str().to_owned(),
            is_deprecated: None,
            release_timestamp: None,
            source_url: None,
            source_directory: None,
            changelog_url: None,
            digest: None,
            is_stable: None,
        })
        .collect();

    CustomReleaseResult {
        releases,
        tags: None,
        source_url: None,
        source_directory: None,
        changelog_url: None,
        homepage: None,
    }
}

pub fn massage_custom_datasource_config(
    custom_datasource_name: &str,
    custom_datasources: &BTreeMap<String, CustomDatasourceDefinition>,
    package_name: &str,
    current_value: Option<&str>,
    registry_url: Option<&str>,
) -> Option<CustomDatasourceConfig> {
    let ds_def = custom_datasources.get(custom_datasource_name)?;
    let url_template = registry_url
        .map(|s| s.to_owned())
        .or_else(|| ds_def.default_registry_url_template.clone())?;

    let compiled_url = compile_template(&url_template, package_name, current_value);
    let transform = ds_def
        .transform_templates
        .iter()
        .map(|t| compile_template(t, package_name, current_value))
        .collect();

    Some(CustomDatasourceConfig {
        format: ds_def.format.clone(),
        default_registry_url_template: compiled_url,
        transform_templates: transform,
    })
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomDatasourceDefinition {
    #[serde(default)]
    pub format: CustomFormat,
    #[serde(
        rename = "defaultRegistryUrlTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_registry_url_template: Option<String>,
    #[serde(
        rename = "transformTemplates",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transform_templates: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_template_replaces_package_name() {
        assert_eq!(
            compile_template("https://example.com/{{packageName}}", "lodash", None),
            "https://example.com/lodash"
        );
    }

    #[test]
    fn compile_template_replaces_current_value() {
        assert_eq!(
            compile_template(
                "https://example.com/{{packageName}}/{{currentValue}}",
                "lodash",
                Some("4.17.21")
            ),
            "https://example.com/lodash/4.17.21"
        );
    }

    #[test]
    fn parse_plain_to_releases_basic() {
        let result = parse_plain_to_releases("1.0.0\n1.1.0\n2.0.0\n");
        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "1.0.0");
    }

    #[test]
    fn parse_plain_to_releases_skips_empty() {
        let result = parse_plain_to_releases("1.0.0\n\n2.0.0\n");
        assert_eq!(result.releases.len(), 2);
    }

    #[test]
    fn extract_html_links_basic() {
        let html = r#"<a href="1.0.0/">1.0.0/</a> <a href="2.0.0/">2.0.0/</a>"#;
        let result = extract_html_links(html);
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "1.0.0/");
    }

    #[test]
    fn extract_html_links_empty() {
        let result = extract_html_links("no links here");
        assert!(result.releases.is_empty());
    }

    #[test]
    fn massage_config_returns_none_for_missing() {
        let ds: BTreeMap<String, CustomDatasourceDefinition> = BTreeMap::new();
        assert_eq!(
            massage_custom_datasource_config("test", &ds, "pkg", None, None),
            None
        );
    }

    #[test]
    fn massage_config_compiles_template() {
        let mut ds = BTreeMap::new();
        ds.insert(
            "myds".to_owned(),
            CustomDatasourceDefinition {
                format: CustomFormat::Json,
                default_registry_url_template: Some(
                    "https://example.com/{{packageName}}".to_owned(),
                ),
                transform_templates: vec![],
            },
        );
        let config =
            massage_custom_datasource_config("myds", &ds, "lodash", None, None).unwrap();
        assert_eq!(
            config.default_registry_url_template,
            "https://example.com/lodash"
        );
    }

    #[test]
    fn massage_config_uses_registry_url_override() {
        let mut ds = BTreeMap::new();
        ds.insert(
            "myds".to_owned(),
            CustomDatasourceDefinition {
                format: CustomFormat::Json,
                default_registry_url_template: Some(
                    "https://default.com/{{packageName}}".to_owned(),
                ),
                transform_templates: vec![],
            },
        );
        let config = massage_custom_datasource_config(
            "myds",
            &ds,
            "lodash",
            None,
            Some("https://override.com/{{packageName}}"),
        )
        .unwrap();
        assert_eq!(
            config.default_registry_url_template,
            "https://override.com/lodash"
        );
    }

    #[test]
    fn custom_format_default_is_json() {
        assert_eq!(CustomFormat::default(), CustomFormat::Json);
    }
}
