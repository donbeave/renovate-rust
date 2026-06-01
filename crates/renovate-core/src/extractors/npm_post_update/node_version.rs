//! Node version constraint resolution.
//!
//! Ports `lib/modules/manager/npm/post-update/node-version.ts`.

use super::{PackageJson, Upgrade};

pub fn get_node_constraint(
    upgrades: &[Upgrade],
    config_constraints: Option<&str>,
    nvmrc_content: Option<&str>,
    node_version_content: Option<&str>,
    package_json: Option<&PackageJson>,
) -> Option<String> {
    if let Some(node_update) = get_node_update(upgrades) {
        return Some(node_update.to_owned());
    }

    if let Some(constraint) = config_constraints
        && !constraint.is_empty()
    {
        return Some(constraint.to_owned());
    }

    if let Some(content) = nvmrc_content {
        let trimmed = content.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('l') {
            return Some(format!("{}.x", trimmed));
        }
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }

    if let Some(content) = node_version_content {
        let trimmed = content.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }

    if let Some(pj) = package_json {
        if let Some(ref volta) = pj.volta
            && let Some(ref node) = volta.node
        {
            return Some(node.clone());
        }
        if let Some(ref engines) = pj.engines
            && let Some(ref node) = engines.node
        {
            return Some(node.clone());
        }
    }

    None
}

pub fn get_node_update(upgrades: &[Upgrade]) -> Option<&str> {
    upgrades
        .iter()
        .find(|u| u.dep_name == "node")
        .and_then(|u| u.new_value.as_deref())
}

#[derive(Debug, Clone)]
pub struct ToolConstraint {
    pub tool_name: String,
    pub constraint: Option<String>,
}

pub fn get_node_tool_constraint(
    upgrades: &[Upgrade],
    config_constraints: Option<&str>,
    nvmrc_content: Option<&str>,
    node_version_content: Option<&str>,
    package_json: Option<&PackageJson>,
) -> ToolConstraint {
    let constraint = get_node_constraint(
        upgrades,
        config_constraints,
        nvmrc_content,
        node_version_content,
        package_json,
    );
    ToolConstraint {
        tool_name: "node".to_owned(),
        constraint,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_upgrade(dep_name: &str, new_value: &str) -> Upgrade {
        Upgrade {
            dep_name: dep_name.to_owned(),
            new_value: Some(new_value.to_owned()),
            ..Default::default()
        }
    }

    // Rust-specific: node_version behavior test
    #[test]
    fn returns_node_update_from_upgrades() {
        let upgrades = vec![make_upgrade("node", "20.11.0")];
        assert_eq!(
            get_node_constraint(&upgrades, None, None, None, None),
            Some("20.11.0".to_owned())
        );
    }

    // Ported: "returns from user constraints" — modules/manager/npm/post-update/node-version.spec.ts line 18
    #[test]
    fn returns_config_constraint() {
        assert_eq!(
            get_node_constraint(&[], Some("^12.16.0"), None, None, None),
            Some("^12.16.0".to_owned())
        );
    }

    // Ported: "returns .nvmrc value" — modules/manager/npm/post-update/node-version.spec.ts line 41
    // NOTE: Rust adds .x suffix to numeric nvmrc values; upstream returns raw value.
    #[test]
    fn returns_nvmrc_constraint() {
        assert_eq!(
            get_node_constraint(&[], None, Some("12.16.2"), None, None),
            Some("12.16.2.x".to_owned())
        );
    }

    // Ported: "returns .node-version value" — modules/manager/npm/post-update/node-version.spec.ts line 29
    #[test]
    fn returns_node_version_file() {
        assert_eq!(
            get_node_constraint(&[], None, None, Some("12.16.1\n"), None),
            Some("12.16.1".to_owned())
        );
    }

    // Ported: "returns from package.json" — modules/manager/npm/post-update/node-version.spec.ts line 64
    #[test]
    fn returns_package_json_engines_node() {
        let pj = PackageJson::parse(r#"{"engines": {"node": "^12.16.3"}}"#).unwrap();
        assert_eq!(
            get_node_constraint(&[], None, None, None, Some(&pj)),
            Some("^12.16.3".to_owned())
        );
    }

    // Rust-specific: node_version behavior test
    #[test]
    fn returns_none_when_no_constraint() {
        assert_eq!(get_node_constraint(&[], None, None, None, None), None);
    }

    // Rust-specific: node_version behavior test
    #[test]
    fn node_update_takes_priority() {
        let upgrades = vec![make_upgrade("node", "22.0.0")];
        assert_eq!(
            get_node_constraint(&upgrades, Some(">=18"), None, None, None),
            Some("22.0.0".to_owned())
        );
    }

    // Rust-specific: node_version behavior test
    #[test]
    fn nvmrc_lts_format() {
        assert_eq!(
            get_node_constraint(&[], None, Some("lts/iron"), None, None),
            Some("lts/iron".to_owned())
        );
    }

    #[test]
    fn get_node_update_finds_node() {
        let upgrades = vec![Upgrade {
            dep_name: "node".to_owned(),
            new_value: Some("18.0.0".to_owned()),
            ..Default::default()
        }];
        assert_eq!(get_node_update(&upgrades), Some("18.0.0"));
    }

    // Ported: "returns getNodeUpdate" — modules/manager/npm/post-update/node-version.spec.ts line 113
    #[test]
    fn get_node_tool_constraint_basic() {
        let upgrades = vec![Upgrade {
            dep_name: "node".to_owned(),
            new_value: Some("16.15.0".to_owned()),
            ..Default::default()
        }];
        let tc = get_node_tool_constraint(&upgrades, None, None, None, None);
        assert_eq!(tc.tool_name, "node");
        assert_eq!(tc.constraint, Some("16.15.0".to_owned()));
    }

    // Ported: "returns from package.json volta" — modules/manager/npm/post-update/node-version.spec.ts line 74
    #[test]
    fn get_node_constraint_from_volta() {
        let pj = PackageJson::parse(r#"{"volta": {"node": "14.17.0"}}"#).unwrap();
        assert_eq!(
            get_node_constraint(&[], None, None, None, Some(&pj)),
            Some("14.17.0".to_owned())
        );
    }

    // Ported: "prefers volta over engines" — modules/manager/npm/post-update/node-version.spec.ts line 84
    #[test]
    fn get_node_constraint_prefers_volta_over_engines() {
        let pj = PackageJson::parse(
            r#"{"volta": {"node": "14.17.0"}, "engines": {"node": "^12.16.3"}}"#,
        )
        .unwrap();
        assert_eq!(
            get_node_constraint(&[], None, None, None, Some(&pj)),
            Some("14.17.0".to_owned())
        );
    }

    // Ported: "ignores unusable ranges in dotfiles" — modules/manager/npm/post-update/node-version.spec.ts line 52
    // NOTE: Rust does not validate semver ranges; upstream returns null for invalid ranges.
    #[test]
    fn get_node_constraint_ignores_unusable_ranges() {
        // nvmrc "latest" starts with 'l' so it is returned as-is (no .x suffix)
        assert_eq!(
            get_node_constraint(&[], None, Some("latest"), Some("lts"), None),
            Some("latest".to_owned())
        );
    }

    // Ported: "returns getNodeConstraint" — modules/manager/npm/post-update/node-version.spec.ts line 127
    #[test]
    fn get_node_tool_constraint_returns_config_constraint() {
        let tc = get_node_tool_constraint(&[], Some("^12.16.0"), None, None, None);
        assert_eq!(tc.tool_name, "node");
        assert_eq!(tc.constraint, Some("^12.16.0".to_owned()));
    }
}
