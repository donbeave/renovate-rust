//! Tool settings options for JVM and Node memory limits.
//!
//! Ports `lib/util/exec/index.ts` `getToolSettingsOptions` and `gradleJvmArg`.

/// Resolved tool settings for memory limits.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ToolSettingsOptions {
    pub jvm_memory: Option<u64>,
    pub jvm_max_memory: Option<u64>,
    pub node_max_memory: Option<u64>,
}

/// Raw tool settings parsed from config (may contain floats).
#[derive(Debug, Clone, Default)]
pub struct RawToolSettings {
    pub jvm_memory: Option<f64>,
    pub jvm_max_memory: Option<f64>,
    pub node_max_memory: Option<f64>,
}

impl RawToolSettings {
    pub fn from_json(map: &serde_json::Map<String, serde_json::Value>) -> Self {
        let get_f64 = |key: &str| {
            map.get(key).and_then(|v| {
                if let Some(n) = v.as_u64() {
                    Some(n as f64)
                } else { v.as_f64().map(|n| n) }
            })
        };
        Self {
            jvm_memory: get_f64("jvmMemory"),
            jvm_max_memory: get_f64("jvmMaxMemory"),
            node_max_memory: get_f64("nodeMaxMemory"),
        }
    }
}

/// Resolve effective tool settings from global defaults and optional repo overrides.
pub fn get_tool_settings_options(
    global: &RawToolSettings,
    repo: Option<&RawToolSettings>,
) -> ToolSettingsOptions {
    let mut defaults = RawToolSettings {
        jvm_max_memory: Some(global.jvm_max_memory.unwrap_or(512.0)),
        jvm_memory: Some(global.jvm_memory.unwrap_or(512.0)),
        node_max_memory: global.node_max_memory,
    };

    // If global jvmMemory is missing but jvmMaxMemory is present, use max as default.
    if global.jvm_memory.is_none() && global.jvm_max_memory.is_some() {
        defaults.jvm_memory = defaults.jvm_max_memory;
    }

    let mut options = ToolSettingsOptions {
        jvm_max_memory: defaults.jvm_max_memory.map(|v| v as u64),
        jvm_memory: defaults.jvm_memory.map(|v| v as u64),
        node_max_memory: defaults.node_max_memory.map(|v| v as u64),
    };

    if let Some(repo_cfg) = repo {
        if let Some(repo_max) = repo_cfg.jvm_max_memory {
            if let Some(global_max) = options.jvm_max_memory
                && repo_max as u64 > global_max {
                    // Log would go here in full implementation.
                }
            options.jvm_max_memory = Some(
                options
                    .jvm_max_memory
                    .map(|g| std::cmp::min(g, repo_max as u64))
                    .unwrap_or(repo_max as u64),
            );
        }

        if let Some(repo_mem) = repo_cfg.jvm_memory {
            options.jvm_memory = Some(repo_mem as u64);
        }

        if let Some(repo_node) = repo_cfg.node_max_memory {
            if let Some(global_node) = options.node_max_memory {
                if repo_node as u64 > global_node {
                    // Log would go here in full implementation.
                } else {
                    options.node_max_memory = Some(repo_node as u64);
                }
            } else {
                options.node_max_memory = Some(repo_node as u64);
            }
        }
    }

    // Ensure jvmMemory does not exceed jvmMaxMemory.
    if let (Some(mem), Some(max)) = (options.jvm_memory, options.jvm_max_memory) {
        options.jvm_memory = Some(std::cmp::min(mem, max));
    }

    // Enforce minimum of 512M for JVM settings.
    if let Some(max) = options.jvm_max_memory
        && max < 512 {
            options.jvm_max_memory = Some(512);
        }
    if let Some(mem) = options.jvm_memory
        && mem < 512 {
            options.jvm_memory = Some(512);
        }

    options
}

/// Build Gradle JVM args string from resolved tool settings.
pub fn gradle_jvm_arg(options: &ToolSettingsOptions) -> String {
    let mem = options.jvm_memory.unwrap_or(512);
    let max = options.jvm_max_memory.unwrap_or(512);
    format!(r#" -Dorg.gradle.jvmargs="-Xms{}m -Xmx{}m""#, mem, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns default values if no global or repo config" — util/exec/index.spec.ts line 1194
    #[test]
    fn returns_default_values_if_no_global_or_repo_config() {
        let global = RawToolSettings::default();
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(512));
    }

    // Ported: "returns default values if empty repo config" — util/exec/index.spec.ts line 1205
    #[test]
    fn returns_default_values_if_empty_repo_config() {
        let global = RawToolSettings::default();
        let repo = RawToolSettings::default();
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(512));
    }

    // Ported: "returns default values if empty global config" — util/exec/index.spec.ts line 1216
    #[test]
    fn returns_default_values_if_empty_global_config() {
        let global = RawToolSettings::default();
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(512));
    }

    // Ported: "in global config" — util/exec/index.spec.ts line 1230
    #[test]
    fn does_not_allow_floating_point_in_global_config() {
        let global = RawToolSettings {
            jvm_memory: Some(512.5),
            jvm_max_memory: Some(600.2),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(600));
    }

    // Ported: "in repo config" — util/exec/index.spec.ts line 1243
    #[test]
    fn does_not_allow_floating_point_in_repo_config() {
        let global = RawToolSettings {
            jvm_memory: Some(1024.0),
            jvm_max_memory: Some(1024.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_memory: Some(556.8),
            jvm_max_memory: Some(600.4),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(556));
        assert_eq!(res.jvm_max_memory, Some(600));
    }

    // Ported: "when below global settings, repo settings are used" — util/exec/index.spec.ts line 1263
    #[test]
    fn repo_settings_below_global_are_used() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_memory: Some(512.0),
            jvm_max_memory: Some(700.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(700));
    }

    // Ported: "when repo settings are the same as global settings, they are used" — util/exec/index.spec.ts line 1277
    #[test]
    fn repo_settings_same_as_global_are_used() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_memory: Some(512.0),
            jvm_max_memory: Some(600.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(600));
    }

    // Ported: "when repo jvmMemory setting is higher than global setting, but lower than global jvmMaxMemory, the repo config is used" — util/exec/index.spec.ts line 1291
    #[test]
    fn repo_jvm_memory_higher_than_global_but_lower_than_max() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_memory: Some(600.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(600));
    }

    // Ported: "when repo jvmMaxMemory setting is lower than global settings, it is applied" — util/exec/index.spec.ts line 1303
    #[test]
    fn repo_jvm_max_memory_lower_than_global_is_applied() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_max_memory: Some(680.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_max_memory, Some(680));
    }

    // Ported: "when repo jvmMaxMemory setting is lower than global jvmMemory, jvmMemory is set to the same value" — util/exec/index.spec.ts line 1315
    #[test]
    fn repo_jvm_max_memory_lower_than_global_jvm_memory() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_max_memory: Some(600.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(600));
        assert_eq!(res.jvm_max_memory, Some(600));
    }

    // Ported: "when repo jvmMaxMemory setting is lower than repo jvmMemory, jvmMemory is set to the same value" — util/exec/index.spec.ts line 1328
    #[test]
    fn repo_jvm_max_memory_lower_than_repo_jvm_memory() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_memory: Some(600.0),
            jvm_max_memory: Some(600.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(600));
        assert_eq!(res.jvm_max_memory, Some(600));
    }

    // Ported: "when repo jvmMaxMemory setting is higher than global settings, they are ignored" — util/exec/index.spec.ts line 1342
    #[test]
    fn repo_jvm_max_memory_higher_than_global_is_ignored() {
        let global = RawToolSettings {
            jvm_memory: Some(768.0),
            jvm_max_memory: Some(800.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_max_memory: Some(8192.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(768));
        assert_eq!(res.jvm_max_memory, Some(800));
    }

    // Ported: "when global settings are lower than 512M, they are overridden to 512M" — util/exec/index.spec.ts line 1375
    #[test]
    fn global_settings_lower_than_512m_overridden() {
        let global = RawToolSettings {
            jvm_memory: Some(100.0),
            jvm_max_memory: Some(127.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(512));
    }

    // Ported: "when repo settings are lower than 512M, they are overridden to 512M" — util/exec/index.spec.ts line 1400
    #[test]
    fn repo_settings_lower_than_512m_overridden() {
        let global = RawToolSettings {
            jvm_memory: Some(1024.0),
            jvm_max_memory: Some(1024.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            jvm_memory: Some(500.0),
            jvm_max_memory: Some(511.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.jvm_memory, Some(512));
        assert_eq!(res.jvm_max_memory, Some(512));
    }

    // Ported: "does not return a default value if no global or repo config" — util/exec/index.spec.ts line 1439
    #[test]
    fn node_max_memory_no_default_without_config() {
        let global = RawToolSettings::default();
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.node_max_memory, None);
    }

    // Ported: "does not return default values if empty global config" — util/exec/index.spec.ts line 1449
    #[test]
    fn node_max_memory_no_default_with_empty_global() {
        let global = RawToolSettings::default();
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.node_max_memory, None);
    }

    // Ported: "in global config" — util/exec/index.spec.ts line 1462
    #[test]
    fn node_max_memory_floor_in_global_config() {
        let global = RawToolSettings {
            node_max_memory: Some(1024.1536),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, None);
        assert_eq!(res.node_max_memory, Some(1024));
    }

    // Ported: "in repo config" — util/exec/index.spec.ts line 1474
    #[test]
    fn node_max_memory_floor_in_repo_config() {
        let global = RawToolSettings::default();
        let repo = RawToolSettings {
            node_max_memory: Some(1024.1536),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.node_max_memory, Some(1024));
    }

    // Ported: "when below global settings, repo settings are used" — util/exec/index.spec.ts line 1490
    #[test]
    fn node_max_memory_repo_below_global() {
        let global = RawToolSettings {
            node_max_memory: Some(1024.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            node_max_memory: Some(700.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.node_max_memory, Some(700));
    }

    // Ported: "when repo settings are the same as global settings, they are used" — util/exec/index.spec.ts line 1502
    #[test]
    fn node_max_memory_repo_same_as_global() {
        let global = RawToolSettings {
            node_max_memory: Some(1024.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            node_max_memory: Some(1024.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.node_max_memory, Some(1024));
    }

    // Ported: "when repo nodeMaxMemory setting is lower than global settings, it is applied" — util/exec/index.spec.ts line 1514
    #[test]
    fn node_max_memory_repo_lower_than_global() {
        let global = RawToolSettings {
            node_max_memory: Some(1024.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            node_max_memory: Some(128.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.node_max_memory, Some(128));
    }

    // Ported: "when repo nodeMaxMemory setting is higher than global settings, they are ignored" — util/exec/index.spec.ts line 1526
    #[test]
    fn node_max_memory_repo_higher_than_global_ignored() {
        let global = RawToolSettings {
            node_max_memory: Some(1024.0),
            ..Default::default()
        };
        let repo = RawToolSettings {
            node_max_memory: Some(8192.0),
            ..Default::default()
        };
        let res = get_tool_settings_options(&global, Some(&repo));
        assert_eq!(res.node_max_memory, Some(1024));
    }

    // Ported: "takes the values given to it, and returns the JVM arguments" — util/exec/index.spec.ts line 1558
    #[test]
    fn gradle_jvm_arg_builds_correct_string() {
        let opts = ToolSettingsOptions {
            jvm_memory: Some(256),
            jvm_max_memory: Some(768),
            ..Default::default()
        };
        let result = gradle_jvm_arg(&opts);
        assert_eq!(result, r#" -Dorg.gradle.jvmargs="-Xms256m -Xmx768m""#);
    }
}
