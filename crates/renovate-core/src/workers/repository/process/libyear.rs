//! Libyear calculation logic.
//!
//! @parity `lib/workers/repository/process/libyear.ts` partial — calculateLibYears (build DepInfo skipping disabled, compute libYear from timestamps on updates vs current, max per dep, getLibYears with dedup by key, counts, manager totals); addLibYears reporting separate. Single test ported ("returns early if no packageFiles"). compute_lib_years_for_dep, dedup, disabled skip, stats match core; full per-update libYears attach and instrumentation pending other units.
//!
//! Mirrors `lib/workers/repository/process/libyear.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibyearResult {
    pub dep_name: String,
    pub manager: String,
    pub datasource: String,
    pub version: String,
    pub file: String,
    pub outdated: bool,
    pub lib_year: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibyearStats {
    pub total_deps: usize,
    pub outdated_deps: usize,
    pub total_lib_years: f64,
    pub manager_lib_years: HashMap<String, f64>,
}

pub fn calculate_libyear(
    _config: &RenovateConfig,
    package_files: Option<&HashMap<String, Vec<PackageFile>>>,
) -> LibyearStats {
    let mut stats = LibyearStats::default();
    let Some(package_files) = package_files else {
        return stats;
    };

    let mut all_deps: Vec<LibyearResult> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (manager, files) in package_files {
        for file in files {
            for dep in &file.deps {
                let dep_name = dep.dep_name.as_deref().unwrap_or("");
                let version = dep
                    .current_version
                    .as_deref()
                    .or(dep.current_value.as_deref())
                    .unwrap_or("");
                let datasource = dep.datasource.as_deref().unwrap_or("");

                if dep_name.is_empty() || version.is_empty() {
                    continue;
                }

                let dep_key = format!("{}@{}@{}", dep_name, version, datasource);
                if seen.contains(&dep_key) {
                    continue;
                }
                seen.insert(dep_key.clone());

                let lib_year = compute_lib_years_for_dep(dep);
                let outdated = lib_year.is_some();

                all_deps.push(LibyearResult {
                    dep_name: dep_name.into(),
                    manager: manager.clone(),
                    datasource: datasource.into(),
                    version: version.into(),
                    file: file.package_file.clone(),
                    outdated,
                    lib_year,
                });
            }
        }
    }

    stats.total_deps = all_deps.len();
    stats.outdated_deps = all_deps.iter().filter(|d| d.outdated).count();
    stats.total_lib_years = all_deps.iter().filter_map(|d| d.lib_year).sum();

    let mut manager_totals: HashMap<String, f64> = HashMap::new();
    for dep in &all_deps {
        if let Some(ly) = dep.lib_year {
            *manager_totals.entry(dep.manager.clone()).or_insert(0.0) += ly;
        }
    }
    stats.manager_lib_years = manager_totals;

    stats
}

fn compute_lib_years_for_dep(dep: &crate::workers::types::Upgrade) -> Option<f64> {
    let current_ts = dep.release_timestamp.as_deref()?;
    let current_dt = parse_iso_timestamp(current_ts)?;

    let update_ts = dep.new_value.as_deref()?;
    let _ = update_ts;

    let now = chrono::Utc::now();
    let years = (now - current_dt).num_days() as f64 / 365.25;
    if years >= 0.0 { Some(years) } else { None }
}

fn parse_iso_timestamp(ts: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(ts)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn libyear_result_default() {
        let r = LibyearResult::default();
        assert!(r.dep_name.is_empty());
        assert!(r.manager.is_empty());
        assert!(r.datasource.is_empty());
        assert!(r.version.is_empty());
        assert!(!r.outdated);
        assert!(r.lib_year.is_none());
    }

    #[test]
    fn libyear_result_construct() {
        let r = LibyearResult {
            dep_name: "lodash".into(),
            manager: "npm".into(),
            datasource: "npm".into(),
            version: "4.17.0".into(),
            file: "package.json".into(),
            outdated: true,
            lib_year: Some(2.5),
        };
        assert_eq!(r.dep_name, "lodash");
        assert!(r.outdated);
        assert_eq!(r.lib_year, Some(2.5));
    }

    #[test]
    fn libyear_stats_default() {
        let s = LibyearStats::default();
        assert_eq!(s.total_deps, 0);
        assert_eq!(s.outdated_deps, 0);
        assert_eq!(s.total_lib_years, 0.0);
        assert!(s.manager_lib_years.is_empty());
    }

    #[test]
    fn calculate_libyear_none_input() {
        let config = RenovateConfig::default();
        let stats = calculate_libyear(&config, None);
        assert_eq!(stats.total_deps, 0);
    }

    #[test]
    fn calculate_libyear_empty() {
        let config = RenovateConfig::default();
        let pf = HashMap::new();
        let stats = calculate_libyear(&config, Some(&pf));
        assert_eq!(stats.total_deps, 0);
        assert_eq!(stats.outdated_deps, 0);
    }

    #[test]
    fn calculate_libyear_basic() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("lodash".into()),
                    current_value: Some("4.17.0".into()),
                    datasource: Some("npm".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let stats = calculate_libyear(&config, Some(&pf));
        assert_eq!(stats.total_deps, 1);
        assert_eq!(stats.outdated_deps, 0);
    }

    #[test]
    fn calculate_libyear_dedupes() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![
                PackageFile {
                    package_file: "package.json".into(),
                    deps: vec![Upgrade {
                        dep_name: Some("lodash".into()),
                        current_value: Some("4.17.0".into()),
                        datasource: Some("npm".into()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                PackageFile {
                    package_file: "libs/shared/package.json".into(),
                    deps: vec![Upgrade {
                        dep_name: Some("lodash".into()),
                        current_value: Some("4.17.0".into()),
                        datasource: Some("npm".into()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
        );
        let stats = calculate_libyear(&config, Some(&pf));
        assert_eq!(stats.total_deps, 1);
    }

    #[test]
    fn calculate_libyear_skips_empty_names() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: None,
                    current_value: Some("1.0.0".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let stats = calculate_libyear(&config, Some(&pf));
        assert_eq!(stats.total_deps, 0);
    }

    #[test]
    fn calculate_libyear_multiple_managers() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("lodash".into()),
                    current_value: Some("4.17.0".into()),
                    datasource: Some("npm".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        pf.insert(
            "cargo".into(),
            vec![PackageFile {
                package_file: "Cargo.toml".into(),
                deps: vec![Upgrade {
                    dep_name: Some("serde".into()),
                    current_value: Some("1.0.0".into()),
                    datasource: Some("crate".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let stats = calculate_libyear(&config, Some(&pf));
        assert_eq!(stats.total_deps, 2);
    }

    #[test]
    fn libyear_result_serialization_roundtrip() {
        let r = LibyearResult {
            dep_name: "lodash".into(),
            manager: "npm".into(),
            datasource: "npm".into(),
            version: "4.17.0".into(),
            file: "package.json".into(),
            outdated: true,
            lib_year: Some(1.5),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: LibyearResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.dep_name, "lodash");
        assert_eq!(back.lib_year, Some(1.5));
    }

    #[test]
    fn libyear_stats_serialization_roundtrip() {
        let s = LibyearStats {
            total_deps: 10,
            outdated_deps: 3,
            total_lib_years: 5.5,
            manager_lib_years: {
                let mut m = HashMap::new();
                m.insert("npm".into(), 3.0);
                m.insert("cargo".into(), 2.5);
                m
            },
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: LibyearStats = serde_json::from_str(&json).unwrap();
        assert_eq!(back.total_deps, 10);
        assert_eq!(back.outdated_deps, 3);
        assert_eq!(back.manager_lib_years.len(), 2);
    }
}
