use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchStringsStrategy {
    Any,
    Combination,
    Recursive,
}

#[derive(Debug, Clone, Default)]
pub struct RegexManagerConfig {
    pub match_strings: Vec<String>,
    pub match_strings_strategy: MatchStringsStrategy,
    pub dep_name_template: Option<String>,
    pub dep_type_template: Option<String>,
    pub current_value_template: Option<String>,
    pub datasource_template: Option<String>,
    pub versioning_template: Option<String>,
    pub registry_url_template: Option<String>,
    pub auto_replace_string_template: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CustomDependency {
    pub dep_name: Option<String>,
    pub dep_type: Option<String>,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: Option<String>,
    pub versioning: Option<String>,
    pub registry_url: Option<String>,
    pub replace_string: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PackageFileInfo {
    pub package_file_dir: String,
    pub package_file_name: String,
    pub content: String,
    pub package_file: String,
}

pub fn extract_package_file(
    content: &str,
    package_file: &str,
    config: &RegexManagerConfig,
) -> Option<Vec<CustomDependency>> {
    let package_file_name = package_file.rsplit('/').next().unwrap_or(package_file).to_owned();
    let package_file_dir = package_file.rsplit('/').nth(1).unwrap_or(".").to_owned();

    let info = PackageFileInfo {
        package_file_dir,
        package_file_name,
        content: content.to_owned(),
        package_file: package_file.to_owned(),
    };

    let deps = match config.match_strings_strategy {
        MatchStringsStrategy::Any => handle_any(config, &info),
        MatchStringsStrategy::Combination => handle_combination(config, &info),
        MatchStringsStrategy::Recursive => handle_recursive(config, &info),
    };

    if deps.is_empty() {
        return None;
    }
    Some(deps)
}

pub fn handle_any(
    config: &RegexManagerConfig,
    info: &PackageFileInfo,
) -> Vec<CustomDependency> {
    let mut deps = Vec::new();

    for pattern in &config.match_strings {
        let Ok(re) = Regex::new(pattern) else {
            continue;
        };
        for cap in re.captures_iter(&info.content) {
            let groups: Vec<(String, String)> = cap
                .iter()
                .enumerate()
                .skip(1)
                .filter_map(|(i, m)| {
                    m.map(|m| {
                        let name = re.capture_names().nth(i - 1).flatten().map(|n| n.to_owned()).unwrap_or_else(|| format!("group{}", i));
                        (name, m.as_str().to_owned())
                    })
                })
                .collect();

            let full_match = cap.get(0).map(|m| m.as_str().to_owned()).unwrap_or_default();
            if let Some(dep) = create_dependency(&groups, &full_match, config) {
                deps.push(dep);
            }
        }
    }

    deps.into_iter().filter(is_valid_dependency).collect()
}

pub fn handle_combination(
    config: &RegexManagerConfig,
    info: &PackageFileInfo,
) -> Vec<CustomDependency> {
    let mut all_groups: Vec<(String, String)> = Vec::new();
    let mut combined_replace: Option<String> = None;

    for pattern in &config.match_strings {
        let Ok(re) = Regex::new(pattern) else {
            continue;
        };
        for cap in re.captures_iter(&info.content) {
            for (i, m) in cap.iter().enumerate().skip(1) {
                if let Some(m) = m {
                    let name = re.capture_names().nth(i - 1).flatten().map(|n| n.to_owned()).unwrap_or_else(|| format!("group{}", i));
                    all_groups.push((name, m.as_str().to_owned()));
                }
            }
            if combined_replace.is_none() {
                let groups_map: std::collections::HashMap<&str, &str> = all_groups.iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                if groups_map.contains_key("currentValue") || groups_map.contains_key("currentDigest") {
                    combined_replace = cap.get(0).map(|m| m.as_str().to_owned());
                }
            }
        }
    }

    if all_groups.is_empty() {
        return Vec::new();
    }

    let replace = combined_replace.unwrap_or_default();
    create_dependency(&all_groups, &replace, config)
        .into_iter()
        .filter(is_valid_dependency)
        .collect()
}

pub fn handle_recursive(
    config: &RegexManagerConfig,
    info: &PackageFileInfo,
) -> Vec<CustomDependency> {
    let regexes: Vec<Regex> = config.match_strings.iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    process_recursive(&info.content, 0, &std::collections::HashMap::new(), &regexes, config)
        .into_iter()
        .filter(is_valid_dependency)
        .collect()
}

fn process_recursive(
    content: &str,
    index: usize,
    combined_groups: &std::collections::HashMap<String, String>,
    regexes: &[Regex],
    config: &RegexManagerConfig,
) -> Vec<CustomDependency> {
    if index >= regexes.len() {
        let groups: Vec<(String, String)> = combined_groups.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        return create_dependency(&groups, content, config).into_iter().collect();
    }

    let mut results = Vec::new();
    for cap in regexes[index].captures_iter(content) {
        let mut new_groups = combined_groups.clone();
        for (i, m) in cap.iter().enumerate().skip(1) {
            if let Some(m) = m {
                let name = regexes[index].capture_names().nth(i - 1).flatten().map(|n| n.to_owned()).unwrap_or_else(|| format!("group{}", i));
                new_groups.insert(name, m.as_str().to_owned());
            }
        }
        let sub_content = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        results.extend(process_recursive(sub_content, index + 1, &new_groups, regexes, config));
    }
    results
}

fn create_dependency(
    groups: &[(String, String)],
    replace_string: &str,
    _config: &RegexManagerConfig,
) -> Option<CustomDependency> {
    let map: std::collections::HashMap<&str, &str> = groups.iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    let dep_name = map.get("depName").map(|s| s.to_string());
    let current_value = map.get("currentValue").map(|s| s.to_string());
    let current_digest = map.get("currentDigest").map(|s| s.to_string());

    if dep_name.is_none() && current_value.is_none() && current_digest.is_none() {
        return None;
    }

    Some(CustomDependency {
        dep_name,
        dep_type: map.get("depType").map(|s| s.to_string()),
        current_value,
        current_digest,
        datasource: map.get("datasource").map(|s| s.to_string()),
        versioning: map.get("versioning").map(|s| s.to_string()),
        registry_url: map.get("registryUrl").map(|s| s.to_string()),
        replace_string: Some(replace_string.to_owned()),
    })
}

fn is_valid_dependency(dep: &CustomDependency) -> bool {
    dep.dep_name.is_some() && (dep.current_value.is_some() || dep.current_digest.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_any_basic_extraction() {
        let content = "image: nginx:1.23.0\nimage: redis:7.0.0";
        let config = RegexManagerConfig {
            match_strings: vec![r#"image:\s*(?<depName>[^\s:]+):(?<currentValue>[^\s]+)"#.to_owned()],
            match_strings_strategy: MatchStringsStrategy::Any,
            ..Default::default()
        };
        let info = PackageFileInfo {
            package_file_dir: ".".to_owned(),
            package_file_name: "test.yaml".to_owned(),
            content: content.to_owned(),
            package_file: "test.yaml".to_owned(),
        };
        let deps = handle_any(&config, &info);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name.as_deref(), Some("nginx"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.23.0"));
        assert_eq!(deps[1].dep_name.as_deref(), Some("redis"));
    }

    #[test]
    fn handle_combination_merges_groups() {
        let content = "name: foo\nversion: 1.0.0";
        let config = RegexManagerConfig {
            match_strings: vec![
                r#"name:\s*(?<depName>\S+)"#.to_owned(),
                r#"version:\s*(?<currentValue>\S+)"#.to_owned(),
            ],
            match_strings_strategy: MatchStringsStrategy::Combination,
            ..Default::default()
        };
        let info = PackageFileInfo {
            package_file_dir: ".".to_owned(),
            package_file_name: "test.txt".to_owned(),
            content: content.to_owned(),
            package_file: "test.txt".to_owned(),
        };
        let deps = handle_combination(&config, &info);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name.as_deref(), Some("foo"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.0.0"));
    }

    #[test]
    fn handle_recursive_narrows_scope() {
        let content = "[block]\nimage: nginx:1.23.0";
        let config = RegexManagerConfig {
            match_strings: vec![
                r#"\[block\]\s*(?<content>[\s\S]*)"#.to_owned(),
                r#"image:\s*(?<depName>\S+):(?<currentValue>\S+)"#.to_owned(),
            ],
            match_strings_strategy: MatchStringsStrategy::Recursive,
            ..Default::default()
        };
        let info = PackageFileInfo {
            package_file_dir: ".".to_owned(),
            package_file_name: "test.cfg".to_owned(),
            content: content.to_owned(),
            package_file: "test.cfg".to_owned(),
        };
        let deps = handle_recursive(&config, &info);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name.as_deref(), Some("nginx"));
    }

    #[test]
    fn extract_returns_none_when_no_matches() {
        let content = "nothing to match here";
        let config = RegexManagerConfig {
            match_strings: vec![r#"image:\s*(?<depName>\S+):(?<currentValue>\S+)"#.to_owned()],
            ..Default::default()
        };
        assert!(extract_package_file(content, "test.yaml", &config).is_none());
    }

    #[test]
    fn invalid_regex_is_skipped() {
        let content = "image: nginx:1.23.0";
        let config = RegexManagerConfig {
            match_strings: vec!["[invalid regex".to_owned()],
            ..Default::default()
        };
        let info = PackageFileInfo {
            package_file_dir: ".".to_owned(),
            package_file_name: "test.yaml".to_owned(),
            content: content.to_owned(),
            package_file: "test.yaml".to_owned(),
        };
        let deps = handle_any(&config, &info);
        assert!(deps.is_empty());
    }
}
