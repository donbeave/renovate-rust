//! Manager files matching.
//!
//! Mirrors `lib/workers/repository/extract/manager-files.ts`.

use std::collections::HashMap;

use crate::workers::repository::extract::types::ManagerFile;

pub fn get_manager_files(
    managers: &[ManagerFile],
    file_list: &[String],
) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for manager in managers {
        if !manager.enabled {
            continue;
        }

        let matching: Vec<String> = file_list
            .iter()
            .filter(|file| matches_manager(manager, file))
            .cloned()
            .collect();

        if !matching.is_empty() {
            result.insert(manager.manager.clone(), matching);
        }
    }

    result
}

fn matches_manager(manager: &ManagerFile, file: &str) -> bool {
    if let Some(patterns) = &manager.manager_file_patterns {
        for pattern in patterns {
            if matches_pattern(pattern, file) {
                return true;
            }
        }
    }
    false
}

fn matches_pattern(pattern: &str, file: &str) -> bool {
    if let Some(stripped) = pattern.strip_prefix('/')
        .and_then(|s| s.strip_suffix('/'))
    {
        if let Ok(re) = regex::Regex::new(stripped) {
            return re.is_match(file);
        }
    }
    file.ends_with(pattern) || file.contains(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_manager_files_empty() {
        let result = get_manager_files(&[], &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn get_manager_files_disabled_manager() {
        let managers = vec![ManagerFile {
            manager: "npm".into(),
            file_list: vec![],
            enabled: false,
            manager_file_patterns: Some(vec!["/package\\.json$/".into()]),
            ..Default::default()
        }];
        let files = vec!["package.json".to_owned()];
        let result = get_manager_files(&managers, &files);
        assert!(result.is_empty());
    }

    #[test]
    fn get_manager_files_matching() {
        let managers = vec![ManagerFile {
            manager: "npm".into(),
            file_list: vec![],
            enabled: true,
            manager_file_patterns: Some(vec!["/package\\.json$/".into()]),
            ..Default::default()
        }];
        let files = vec!["package.json".to_owned(), "src/index.ts".to_owned()];
        let result = get_manager_files(&managers, &files);
        assert_eq!(result.get("npm").unwrap().len(), 1);
        assert_eq!(result["npm"][0], "package.json");
    }

    #[test]
    fn get_manager_files_no_match() {
        let managers = vec![ManagerFile {
            manager: "cargo".into(),
            file_list: vec![],
            enabled: true,
            manager_file_patterns: Some(vec!["/Cargo\\.toml$/".into()]),
            ..Default::default()
        }];
        let files = vec!["package.json".to_owned()];
        let result = get_manager_files(&managers, &files);
        assert!(result.is_empty());
    }

    #[test]
    fn matches_pattern_literal() {
        assert!(matches_pattern("package.json", "package.json"));
        assert!(!matches_pattern("package.json", "src/index.ts"));
    }

    #[test]
    fn matches_pattern_regex() {
        assert!(matches_pattern("/\\.json$/", "package.json"));
        assert!(!matches_pattern("/\\.json$/", "src/index.ts"));
    }
}
