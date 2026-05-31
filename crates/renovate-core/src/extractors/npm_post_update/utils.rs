//! Post-update utility functions.
//!
//! Ports `lib/modules/manager/npm/post-update/utils.ts`.

use super::PackageJson;

pub fn get_package_manager_version(pj: &PackageJson, name: &str) -> Option<String> {
    pj.get_package_manager_version(name)
}

pub fn get_node_options(max_old_space_size: Option<u64>) -> Option<String> {
    max_old_space_size.map(|n| format!("--max-old-space-size={n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "if nodeMaxMemory set on global config" — modules/manager/npm/post-update/npm.spec.ts line 494
    #[test]
    fn get_node_options_returns_flag() {
        assert_eq!(
            get_node_options(Some(4096)),
            Some("--max-old-space-size=4096".to_owned())
        );
    }

    // Ported: "if nodeMaxMemory set on repo config" — modules/manager/npm/post-update/npm.spec.ts line 539
    #[test]
    fn get_node_options_returns_none() {
        assert_eq!(get_node_options(None), None);
    }

    // Ported: "uses volta version and puts it into constraint" — modules/manager/npm/post-update/pnpm.spec.ts line 429
    #[test]
    fn get_package_manager_version_from_volta() {
        let pj = PackageJson::parse(
            r#"{"volta": {"npm": "10.2.3"}}"#,
        )
        .unwrap();
        assert_eq!(
            get_package_manager_version(&pj, "npm"),
            Some("10.2.3".to_owned())
        );
    }
}
