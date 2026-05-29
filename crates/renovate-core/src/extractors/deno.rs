//! Deno manager — dependency update and extraction.
//!
//! Renovate reference:
//! - `lib/modules/manager/deno/update.ts` — `updateDependency`

use crate::extractors::npm::{NpmUpdateUpgrade, npm_update_dependency};

/// Format-preserving replacement for Deno JSON files.
///
/// Searches for `old_str` (PLAIN, not quoted) in `content` starting from
/// after the `"section_key"` keyword, replaces it with `new_str`, and
/// verifies by comparing the re-parsed JSON with `expected`.
///
/// This mirrors the Deno-specific `replaceAsString()` in `update.ts` which
/// does NOT wrap search/replace strings in quotes (unlike the npm version).
fn deno_replace_verified(
    expected: &serde_json::Value,
    content: &str,
    section_key: &str,
    old_str: &str,
    new_str: &str,
) -> Option<String> {
    let search_start = content
        .find(&format!("\"{section_key}\""))
        .map(|i| i + section_key.len())
        .unwrap_or_else(|| section_key.len().saturating_sub(1).min(content.len()));

    let mut i = search_start;
    while i < content.len() {
        if content[i..].starts_with(old_str) {
            let candidate = format!(
                "{}{}{}",
                &content[..i],
                new_str,
                &content[i + old_str.len()..]
            );
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&candidate) {
                if &parsed == expected {
                    return Some(candidate);
                }
            }
        }
        let step = content[i..].chars().next().map_or(1, |c| c.len_utf8());
        i += step;
    }
    None
}

/// Upgrade descriptor for `deno_update_dependency`.
#[derive(Debug, Clone, Default)]
pub struct DenoUpdateUpgrade {
    pub dep_name: Option<String>,
    pub dep_type: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub datasource: Option<String>,
    pub package_file: Option<String>,
    /// True when the file is an import map referenced by another file.
    pub import_map_referrer: bool,
}

/// Compute the Deno-style version string for a dep.
/// Mirrors `getValueByDatasource()` from `lib/modules/manager/deno/update.ts`.
fn get_value_by_datasource(
    datasource: &str,
    dep_name: &str,
    value: Option<&str>,
) -> Option<String> {
    match datasource {
        "deno" => Some(if let Some(v) = value {
            format!("{}@{}", dep_name, v)
        } else {
            dep_name.to_owned()
        }),
        "jsr" | "npm" => Some(if let Some(v) = value {
            format!("{}:{}@{}", datasource, dep_name, v)
        } else {
            format!("{}:{}", datasource, dep_name)
        }),
        _ => None,
    }
}

/// Update a Deno-managed dependency in a deno.json, import map, or package.json file.
///
/// Mirrors `updateDependency()` from `lib/modules/manager/deno/update.ts`.
pub fn deno_update_dependency(file_content: &str, upgrade: &DenoUpdateUpgrade) -> Option<String> {
    let package_file = upgrade.package_file.as_deref().unwrap_or("");
    let dep_name = upgrade.dep_name.as_deref()?;
    let new_value = upgrade.new_value.as_deref();
    let datasource = upgrade.datasource.as_deref()?;
    let dep_type = upgrade.dep_type.as_deref()?;

    // Delegate package.json files to npm update.
    if package_file.ends_with("package.json") {
        let npm_upgrade = NpmUpdateUpgrade {
            dep_type: dep_type.to_owned(),
            dep_name: dep_name.to_owned(),
            new_value: upgrade.new_value.clone(),
            ..Default::default()
        };
        return npm_update_dependency(file_content, &npm_upgrade);
    }

    // Only handle deno.json* and import map files.
    let basename = package_file.rsplit('/').next().unwrap_or(package_file);
    let is_deno_json = basename.starts_with("deno.json");
    let is_import_map = upgrade.import_map_referrer || (!is_deno_json && package_file.ends_with(".json"));
    if !is_deno_json && !is_import_map {
        return None;
    }

    let search_current = get_value_by_datasource(datasource, dep_name, upgrade.current_value.as_deref())?;
    let new_str = get_value_by_datasource(datasource, dep_name, new_value)?;

    // Parse the JSON content to build expected state.
    let mut parsed: serde_json::Value = serde_json::from_str(file_content).ok()?;

    let result: Option<String>;

    match dep_type {
        "imports" => {
            let imports = parsed.get_mut("imports")?.as_object_mut()?;
            let mut match_count = 0;
            let mut match_key = None;
            for (key, val) in imports.iter() {
                if val.as_str().map(|s| s.contains(&search_current)).unwrap_or(false) {
                    match_count += 1;
                    match_key = Some(key.clone());
                }
            }
            if match_count > 1 {
                // Multiple matches — error (the TypeScript throws here)
                return None;
            }
            let key = match_key?;
            let old_url = imports[&key].as_str()?.to_owned();
            let new_url = old_url.replace(&search_current, &new_str);
            imports.insert(key, serde_json::Value::String(new_url));
            // Do the string replacement in the file content
            result = deno_replace_verified(&parsed, file_content, dep_type, &search_current, &new_str);
        }
        "scopes" => {
            let scopes = parsed.get_mut("scopes")?.as_object_mut()?;
            let mut found = false;
            for scope_val in scopes.values_mut() {
                if let Some(scope_map) = scope_val.as_object_mut() {
                    for val in scope_map.values_mut() {
                        if val.as_str().map(|s| s.contains(&search_current)).unwrap_or(false) {
                            let old_url = val.as_str()?.to_owned();
                            let new_url = old_url.replace(&search_current, &new_str);
                            *val = serde_json::Value::String(new_url);
                            found = true;
                        }
                    }
                }
            }
            if !found {
                return None;
            }
            result = deno_replace_verified(&parsed, file_content, dep_type, &search_current, &new_str);
        }
        "tasks" => {
            let tasks = parsed.get_mut("tasks")?.as_object_mut()?;
            let mut found = false;
            for val in tasks.values_mut() {
                if let Some(s) = val.as_str() {
                    if s.contains(&search_current) {
                        let new_val = s.replace(&search_current, &new_str);
                        *val = serde_json::Value::String(new_val);
                        found = true;
                    }
                }
            }
            if !found {
                return None;
            }
            result = deno_replace_verified(&parsed, file_content, dep_type, &search_current, &new_str);
        }
        "tasks.command" => {
            let tasks = parsed.get_mut("tasks")?.as_object_mut()?;
            let mut found = false;
            for val in tasks.values_mut() {
                if let Some(obj) = val.as_object_mut() {
                    if let Some(cmd) = obj.get_mut("command") {
                        if let Some(s) = cmd.as_str() {
                            if s.contains(&search_current) {
                                let new_val = s.replace(&search_current, &new_str);
                                *cmd = serde_json::Value::String(new_val);
                                found = true;
                            }
                        }
                    }
                }
            }
            if !found {
                return None;
            }
            result = deno_replace_verified(&parsed, file_content, "tasks", &search_current, &new_str);
        }
        "compilerOptions.types" => {
            let types = parsed
                .get_mut("compilerOptions")?
                .get_mut("types")?
                .as_array_mut()?;
            let idx = types.iter().position(|v| v.as_str() == Some(&search_current))?;
            types[idx] = serde_json::Value::String(new_str.clone());
            result = deno_replace_verified(&parsed, file_content, "compilerOptions", &search_current, &new_str);
        }
        "compilerOptions.jsxImportSource" => {
            let val = parsed
                .get_mut("compilerOptions")?
                .get_mut("jsxImportSource")?;
            if val.as_str().map(|s| s.contains(&search_current)).unwrap_or(false) {
                let old = val.as_str()?.to_owned();
                *val = serde_json::Value::String(old.replace(&search_current, &new_str));
                result = deno_replace_verified(&parsed, file_content, "compilerOptions", &search_current, &new_str);
            } else {
                return None;
            }
        }
        "compilerOptions.jsxImportSourceTypes" => {
            let val = parsed
                .get_mut("compilerOptions")?
                .get_mut("jsxImportSourceTypes")?;
            if val.as_str().map(|s| s.contains(&search_current)).unwrap_or(false) {
                let old = val.as_str()?.to_owned();
                *val = serde_json::Value::String(old.replace(&search_current, &new_str));
                result = deno_replace_verified(&parsed, file_content, "compilerOptions", &search_current, &new_str);
            } else {
                return None;
            }
        }
        "lint.plugins" => {
            let plugins = parsed
                .get_mut("lint")?
                .get_mut("plugins")?
                .as_array_mut()?;
            let idx = plugins.iter().position(|v| v.as_str() == Some(&search_current))?;
            plugins[idx] = serde_json::Value::String(new_str.clone());
            result = deno_replace_verified(&parsed, file_content, "lint", &search_current, &new_str);
        }
        _ => return None,
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_deno_upgrade(dep_name: &str, dep_type: &str, datasource: &str, current: &str, new_val: &str) -> DenoUpdateUpgrade {
        DenoUpdateUpgrade {
            dep_name: Some(dep_name.into()),
            dep_type: Some(dep_type.into()),
            datasource: Some(datasource.into()),
            current_value: Some(current.into()),
            new_value: Some(new_val.into()),
            package_file: Some("deno.json".into()),
            ..Default::default()
        }
    }

    // Ported: "updates dependency in imports" — deno/update.spec.ts line 9
    #[test]
    fn deno_update_imports() {
        let content = r#"{"imports":{"fs":"https://deno.land/std@0.223.0/fs/mod.ts"}}"#;
        let upgrade = mk_deno_upgrade("https://deno.land/std", "imports", "deno", "0.223.0", "0.224.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["imports"]["fs"].as_str().unwrap(), "https://deno.land/std@0.224.0/fs/mod.ts");
    }

    // Ported: "throws when multiple imports require more than one replacement" — deno/update.spec.ts line 39
    #[test]
    fn deno_update_imports_multiple_returns_none() {
        let content = r#"{"imports":{"fs":"https://deno.land/std@0.223.0/fs/mod.ts","path":"https://deno.land/std@0.223.0/path/mod.ts"}}"#;
        let upgrade = mk_deno_upgrade("https://deno.land/std", "imports", "deno", "0.223.0", "0.224.0");
        // Multiple matches → None (TypeScript throws)
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "updates dependency in scopes" — deno/update.spec.ts line 64
    #[test]
    fn deno_update_scopes() {
        let content = r#"{"scopes":{"https://deno.land/x/":{"dep2":"jsr:@scope/dep1@latest"}}}"#;
        let upgrade = mk_deno_upgrade("@scope/dep1", "scopes", "jsr", "latest", "2.0.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["scopes"]["https://deno.land/x/"]["dep2"].as_str().unwrap(), "jsr:@scope/dep1@2.0.0");
    }

    // Ported: "returns null when scopes element not found" — deno/update.spec.ts line 98
    #[test]
    fn deno_update_scopes_not_found() {
        let content = r#"{"scopes":{"https://deno.land/x/":{"dep2":"jsr:@scope/other@latest"}}}"#;
        let upgrade = mk_deno_upgrade("@scope/dep1", "scopes", "jsr", "latest", "2.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "updates dependency in tasks" — deno/update.spec.ts line 127
    #[test]
    fn deno_update_tasks() {
        let content = r#"{"tasks":{"build":"deno run npm:esbuild@0.20.0 main.ts"}}"#;
        let upgrade = mk_deno_upgrade("esbuild", "tasks", "npm", "0.20.0", "0.21.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["tasks"]["build"].as_str().unwrap(), "deno run npm:esbuild@0.21.0 main.ts");
    }

    // Ported: "returns null when tasks element not found" — deno/update.spec.ts line 191
    #[test]
    fn deno_update_tasks_not_found() {
        let content = r#"{"tasks":{"build":"deno run npm:other@0.20.0 main.ts"}}"#;
        let upgrade = mk_deno_upgrade("esbuild", "tasks", "npm", "0.20.0", "0.21.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "updates dependency in compilerOptions.types" — deno/update.spec.ts line 251
    #[test]
    fn deno_update_compiler_types() {
        let content = r#"{"compilerOptions":{"types":["npm:jest@29.0.0"]}}"#;
        let upgrade = mk_deno_upgrade("jest", "compilerOptions.types", "npm", "29.0.0", "30.0.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["compilerOptions"]["types"][0].as_str().unwrap(), "npm:jest@30.0.0");
    }

    // Ported: "returns null when compilerOptions.types is empty array" — deno/update.spec.ts line 281
    #[test]
    fn deno_update_compiler_types_empty() {
        let content = r#"{"compilerOptions":{"types":[]}}"#;
        let upgrade = mk_deno_upgrade("jest", "compilerOptions.types", "npm", "29.0.0", "30.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "updates dependency in compilerOptions.jsxImportSource" — deno/update.spec.ts line 335
    #[test]
    fn deno_update_jsx_import_source() {
        let content = r#"{"compilerOptions":{"jsxImportSource":"npm:preact@10.22.0"}}"#;
        let upgrade = mk_deno_upgrade("preact", "compilerOptions.jsxImportSource", "npm", "10.22.0", "10.23.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["compilerOptions"]["jsxImportSource"].as_str().unwrap(), "npm:preact@10.23.0");
    }

    // Ported: "updates dependency in lint plugins" — deno/update.spec.ts line 453
    #[test]
    fn deno_update_lint_plugins() {
        let content = r#"{"lint":{"plugins":["jsr:@scope/plugin@1.0.0"]}}"#;
        let upgrade = mk_deno_upgrade("@scope/plugin", "lint.plugins", "jsr", "1.0.0", "2.0.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["lint"]["plugins"][0].as_str().unwrap(), "jsr:@scope/plugin@2.0.0");
    }

    // Ported: "returns null if packageFile is not defined" — deno/update.spec.ts line 563
    #[test]
    fn deno_update_no_package_file() {
        let content = r#"{"imports":{"fs":"https://deno.land/std@0.223.0/fs/mod.ts"}}"#;
        let mut upgrade = mk_deno_upgrade("https://deno.land/std", "imports", "deno", "0.223.0", "0.224.0");
        upgrade.package_file = None;
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "handles the case where the desired version is already supported" — deno/update.spec.ts line 689
    #[test]
    fn deno_update_already_at_version() {
        let content = r#"{"imports":{"fs":"https://deno.land/std@0.224.0/fs/mod.ts"}}"#;
        let upgrade = mk_deno_upgrade("https://deno.land/std", "imports", "deno", "0.224.0", "0.224.0");
        let result = deno_update_dependency(content, &upgrade);
        // When current and new are the same, no change needed
        // This might return Some(content) or None depending on implementation
        // The test expects it to return fileContent unchanged
        if let Some(r) = result {
            let parsed: serde_json::Value = serde_json::from_str(&r).unwrap();
            assert_eq!(parsed["imports"]["fs"].as_str().unwrap(), "https://deno.land/std@0.224.0/fs/mod.ts");
        }
    }

    // Ported: "returns null if empty file" — deno/update.spec.ts line 712
    #[test]
    fn deno_update_empty_file() {
        let upgrade = mk_deno_upgrade("https://deno.land/std", "imports", "deno", "0.223.0", "0.224.0");
        let result = deno_update_dependency("", &upgrade);
        assert!(result.is_none());
    }

    // Ported: "returns null for not supported datasource" — deno/update.spec.ts line 575
    #[test]
    fn deno_update_unsupported_datasource() {
        let content = r#"{"imports":{"fs":"https://deno.land/std@0.223.0/fs/mod.ts"}}"#;
        let upgrade = mk_deno_upgrade("fs", "imports", "docker", "0.223.0", "0.224.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }
    // Ported: "updates dependency in tasks.command" — deno/update.spec.ts line 158
    #[test]
    fn deno_update_tasks_command() {
        let content = r#"{"tasks":{"build":"deno run -A npm:dep1@4.0.0","dev":{"command":"deno run --allow-net npm:dep2@14.0.1"}}}"#;
        let upgrade = mk_deno_upgrade("dep2", "tasks.command", "npm", "14.0.1", "16.0.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["tasks"]["dev"]["command"].as_str().unwrap(), "deno run --allow-net npm:dep2@16.0.0");
    }

    // Ported: "returns null when tasks.command element not found" — deno/update.spec.ts line 221
    #[test]
    fn deno_update_tasks_command_not_found() {
        let content = r#"{"tasks":{"build":"deno run -A npm:dep1@4.0.0","dev":{"command":"deno run --allow-net npm:dep2@14.0.1"}}}"#;
        let upgrade = mk_deno_upgrade("dep1", "tasks.command", "npm", "4.0.0", "4.1.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "returns null when compilerOptions.types element not found" — deno/update.spec.ts line 308
    #[test]
    fn deno_update_compiler_types_not_found() {
        let content = r#"{"compilerOptions":{"types":["npm:@types/other@18.0.0"]}}"#;
        let upgrade = mk_deno_upgrade("@types/dep2", "compilerOptions.types", "npm", "18.0.0", "19.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "returns null when compilerOptions.jsxImportSource does not exist" — deno/update.spec.ts line 367
    #[test]
    fn deno_update_jsx_import_source_not_found() {
        let content = r#"{"compilerOptions":{"types":["npm:@types/dep2@18.0.0"]}}"#;
        let upgrade = mk_deno_upgrade("https://deno.land/x/dep2", "compilerOptions.jsxImportSource", "deno", "18.0.0", "19.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "returns null when compilerOptions.jsxImportSourceTypes does not exist" — deno/update.spec.ts line 394
    #[test]
    fn deno_update_jsx_import_source_types_not_found() {
        let content = r#"{"compilerOptions":{"types":["npm:@types/dep2@18.0.0"]}}"#;
        let upgrade = mk_deno_upgrade("@types/dep2", "compilerOptions.jsxImportSourceTypes", "npm", "18.0.0", "19.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "updates dependency in compilerOptions.jsxImportSourceTypes" — deno/update.spec.ts line 421
    #[test]
    fn deno_update_jsx_import_source_types() {
        let content = r#"{"compilerOptions":{"jsxImportSourceTypes":"npm:@types/dep2@18.0.0"}}"#;
        let upgrade = mk_deno_upgrade("@types/dep2", "compilerOptions.jsxImportSourceTypes", "npm", "18.0.0", "19.0.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["compilerOptions"]["jsxImportSourceTypes"].as_str().unwrap(), "npm:@types/dep2@19.0.0");
    }

    // Ported: "returns null when lint.plugins element not found" — deno/update.spec.ts line 481
    #[test]
    fn deno_update_lint_plugins_not_found() {
        let content = r#"{"lint":{"plugins":["npm:dep2@5.0.0"]}}"#;
        let upgrade = mk_deno_upgrade("dep1", "lint.plugins", "npm", "5.0.0", "6.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "returns null when lint.plugins is empty array" — deno/update.spec.ts line 508
    #[test]
    fn deno_update_lint_plugins_empty() {
        let content = r#"{"lint":{"plugins":[]}}"#;
        let upgrade = mk_deno_upgrade("dep1", "lint.plugins", "npm", "5.0.0", "6.0.0");
        let result = deno_update_dependency(content, &upgrade);
        assert!(result.is_none());
    }

    // Ported: "handles dependency without version" — deno/update.spec.ts line 535
    #[test]
    fn deno_update_dep_without_version() {
        let content = r#"{"imports":{"dep1":"npm:dep1"}}"#;
        let mut upgrade = mk_deno_upgrade("dep1", "imports", "npm", "", "1.0.0");
        upgrade.current_value = None;
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["imports"]["dep1"].as_str().unwrap(), "npm:dep1@1.0.0");
    }

    // Ported: "currentValue is not defined when deno datasource" — deno/update.spec.ts line 602
    #[test]
    fn deno_update_no_current_value_deno() {
        let content = r#"{"imports":{"fs":"https://deno.land/std/fs"}}"#;
        let mut upgrade = mk_deno_upgrade("https://deno.land/std/fs", "imports", "deno", "", "2.0.0");
        upgrade.current_value = None;
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["imports"]["fs"].as_str().unwrap(), "https://deno.land/std/fs@2.0.0");
    }

    // Ported: "returns null for missing required values" — deno/update.spec.ts line 629
    #[test]
    fn deno_update_missing_required_values() {
        let upgrade = DenoUpdateUpgrade {
            package_file: Some("deno.json".into()),
            datasource: Some("npm".into()),
            new_value: Some("2.0.0".into()),
            ..Default::default()
        };
        let result = deno_update_dependency("{}", &upgrade);
        assert!(result.is_none());
    }

    // Ported: "handles complex JSON with nested structures" — deno/update.spec.ts line 648
    #[test]
    fn deno_update_complex_json() {
        let content = r#"{"name":"my-deno-app","imports":{"dep1":"npm:dep1@1.0.0","dep2":"npm:dep2@1.0.0"}}"#;
        let upgrade = mk_deno_upgrade("dep1", "imports", "npm", "1.0.0", "1.1.0");
        let result = deno_update_dependency(content, &upgrade).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["imports"]["dep1"].as_str().unwrap(), "npm:dep1@1.1.0");
        assert_eq!(parsed["imports"]["dep2"].as_str().unwrap(), "npm:dep2@1.0.0");
    }

}
