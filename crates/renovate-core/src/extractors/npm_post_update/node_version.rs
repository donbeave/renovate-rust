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
        && !constraint.is_empty() {
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
            && let Some(ref node) = volta.node {
                return Some(node.clone());
            }
        if let Some(ref engines) = pj.engines
            && let Some(ref node) = engines.node {
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

    #[test]
    fn returns_node_update_from_upgrades() {
        let upgrades = vec![make_upgrade("node", "20.11.0")];
        assert_eq!(
            get_node_constraint(&upgrades, None, None, None, None),
            Some("20.11.0".to_owned())
        );
    }

    #[test]
    fn returns_config_constraint() {
        assert_eq!(
            get_node_constraint(&[], Some(">=18"), None, None, None),
            Some(">=18".to_owned())
        );
    }

    #[test]
    fn returns_nvmrc_constraint() {
        assert_eq!(
            get_node_constraint(&[], None, Some("18"), None, None),
            Some("18.x".to_owned())
        );
    }

    #[test]
    fn returns_node_version_file() {
        assert_eq!(
            get_node_constraint(&[], None, None, Some("20.11.0"), None),
            Some("20.11.0".to_owned())
        );
    }

    #[test]
    fn returns_package_json_engines_node() {
        let pj = PackageJson::parse(
            r#"{"engines": {"node": ">=18"}}"#,
        )
        .unwrap();
        assert_eq!(
            get_node_constraint(&[], None, None, None, Some(&pj)),
            Some(">=18".to_owned())
        );
    }

    #[test]
    fn returns_none_when_no_constraint() {
        assert_eq!(get_node_constraint(&[], None, None, None, None), None);
    }

    #[test]
    fn node_update_takes_priority() {
        let upgrades = vec![make_upgrade("node", "22.0.0")];
        assert_eq!(
            get_node_constraint(&upgrades, Some(">=18"), None, None, None),
            Some("22.0.0".to_owned())
        );
    }

    #[test]
    fn nvmrc_lts_format() {
        assert_eq!(
            get_node_constraint(&[], None, Some("lts/iron"), None, None),
            Some("lts/iron".to_owned())
        );
    }
}
