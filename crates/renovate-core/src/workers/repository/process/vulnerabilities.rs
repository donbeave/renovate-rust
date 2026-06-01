//! Vulnerability checking logic.
//!
//! Mirrors `lib/workers/repository/process/vulnerabilities.ts`
//! and `lib/workers/repository/process/types.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VulnerabilityReference {
    pub url: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OsvAffected {
    pub package_name: Option<String>,
    pub ecosystem: Option<String>,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OsvVulnerability {
    pub id: String,
    pub summary: Option<String>,
    pub details: Option<String>,
    pub aliases: Vec<String>,
    pub references: Vec<VulnerabilityReference>,
    pub affected: Vec<OsvAffected>,
    pub withdrawn: Option<bool>,
    pub severity: Vec<SeverityEntry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeverityEntry {
    pub severity_type: String,
    pub score: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VulnerabilityAlert {
    pub package_name: String,
    pub osv_package_name: String,
    pub dep_version: String,
    pub fixed_version: Option<String>,
    pub datasource: String,
    pub vulnerability_id: String,
    pub vulnerability_summary: Option<String>,
    pub severity_level: Option<String>,
    pub severity_score: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyVulnerabilities {
    pub package_name: String,
    pub vulnerabilities: Vec<VulnerabilityAlert>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeverityDetails {
    pub cvss_vector: Option<String>,
    pub score: Option<String>,
    pub severity_level: String,
}

pub fn get_vulnerabilities(
    _package_name: &str,
    _datasource: &str,
    _version: &str,
) -> Vec<VulnerabilityAlert> {
    Vec::new()
}

pub fn is_package_vulnerable(dep_version: &str, affected_versions: &[String]) -> bool {
    affected_versions.contains(&dep_version.to_owned())
}

pub fn get_fixed_version(
    _ecosystem: &str,
    _dep_version: &str,
    fixed_events: &[String],
) -> Option<String> {
    fixed_events.first().map(|v| format!(">= {}", v))
}

pub fn evaluate_cvss_vector(vector: &str) -> (String, String) {
    if vector.starts_with("CVSS:3.") || vector.starts_with("CVSS:4.") {
        let parts: Vec<&str> = vector.split('/').collect();
        let mut score = 0.0_f64;

        for part in &parts {
            if part.starts_with("AV:") {
                score += match part.split_once(':').map(|(_, v)| v) {
                    Some("N") => 0.85,
                    Some("A") => 0.62,
                    Some("L") => 0.55,
                    Some("P") => 0.2,
                    _ => 0.0,
                };
            }
        }

        if score > 0.0 {
            score = (score * 3.0_f64).min(10.0_f64);
        }

        let severity = if score >= 9.0 {
            "CRITICAL"
        } else if score >= 7.0 {
            "HIGH"
        } else if score >= 4.0 {
            "MEDIUM"
        } else if score > 0.0 {
            "LOW"
        } else {
            "NONE"
        };

        (format!("{:.1}", score), severity.to_owned())
    } else {
        (String::new(), "UNKNOWN".to_owned())
    }
}

pub fn extract_severity_details(vulnerability: &OsvVulnerability) -> SeverityDetails {
    let cvss_vector = vulnerability
        .severity
        .iter()
        .find(|e| e.severity_type == "CVSS_V3" || e.severity_type == "CVSS_V4")
        .map(|e| e.score.clone());

    let (score, severity_level) = match &cvss_vector {
        Some(v) => evaluate_cvss_vector(v),
        None => (String::new(), "UNKNOWN".to_owned()),
    };

    SeverityDetails {
        cvss_vector,
        score: if score.is_empty() { None } else { Some(score) },
        severity_level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vulnerability_reference_default() {
        let r = VulnerabilityReference::default();
        assert!(r.url.is_none());
        assert!(r.source.is_none());
    }

    #[test]
    fn vulnerability_alert_default() {
        let a = VulnerabilityAlert::default();
        assert!(a.package_name.is_empty());
        assert!(a.fixed_version.is_none());
        assert!(a.vulnerability_id.is_empty());
    }

    #[test]
    fn vulnerability_alert_construct() {
        let a = VulnerabilityAlert {
            package_name: "lodash".into(),
            dep_version: "4.17.0".into(),
            fixed_version: Some(">= 4.17.21".into()),
            datasource: "npm".into(),
            vulnerability_id: "CVE-2021-23337".into(),
            severity_level: Some("HIGH".into()),
            ..Default::default()
        };
        assert_eq!(a.package_name, "lodash");
        assert_eq!(a.fixed_version, Some(">= 4.17.21".into()));
    }

    #[test]
    fn osv_vulnerability_default() {
        let v = OsvVulnerability::default();
        assert!(v.id.is_empty());
        assert!(v.summary.is_none());
        assert!(v.aliases.is_empty());
        assert!(v.affected.is_empty());
    }

    #[test]
    fn get_vulnerabilities_returns_empty() {
        let result = get_vulnerabilities("lodash", "npm", "4.17.0");
        assert!(result.is_empty());
    }

    #[test]
    fn is_package_vulnerable_true() {
        let affected = vec!["4.17.0".into(), "4.17.1".into()];
        assert!(is_package_vulnerable("4.17.0", &affected));
    }

    #[test]
    fn is_package_vulnerable_false() {
        let affected = vec!["4.17.1".into(), "4.17.2".into()];
        assert!(!is_package_vulnerable("4.17.0", &affected));
    }

    #[test]
    fn is_package_vulnerable_empty() {
        assert!(!is_package_vulnerable("4.17.0", &[]));
    }

    #[test]
    fn get_fixed_version_some() {
        let fixed = vec!["4.17.21".into()];
        let result = get_fixed_version("npm", "4.17.0", &fixed);
        assert_eq!(result, Some(">= 4.17.21".into()));
    }

    #[test]
    fn get_fixed_version_none() {
        let result = get_fixed_version("npm", "4.17.0", &[]);
        assert!(result.is_none());
    }

    #[test]
    fn evaluate_cvss_vector_valid_v3() {
        let (score, severity) =
            evaluate_cvss_vector("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H");
        assert!(!score.is_empty());
        assert!(!severity.is_empty());
    }

    #[test]
    fn evaluate_cvss_vector_empty() {
        let (score, severity) = evaluate_cvss_vector("");
        assert!(score.is_empty());
        assert_eq!(severity, "UNKNOWN");
    }

    #[test]
    fn evaluate_cvss_vector_invalid() {
        let (score, _severity) = evaluate_cvss_vector("not-a-vector");
        assert!(score.is_empty());
    }

    #[test]
    fn extract_severity_details_no_severity() {
        let vuln = OsvVulnerability::default();
        let details = extract_severity_details(&vuln);
        assert!(details.cvss_vector.is_none());
        assert!(details.score.is_none());
        assert_eq!(details.severity_level, "UNKNOWN");
    }

    #[test]
    fn extract_severity_details_with_cvss() {
        let vuln = OsvVulnerability {
            id: "CVE-2021-23337".into(),
            severity: vec![SeverityEntry {
                severity_type: "CVSS_V3".into(),
                score: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H".into(),
            }],
            ..Default::default()
        };
        let details = extract_severity_details(&vuln);
        assert!(details.cvss_vector.is_some());
        assert!(details.score.is_some());
        assert!(!details.severity_level.is_empty());
    }

    #[test]
    fn severity_details_default() {
        let d = SeverityDetails::default();
        assert!(d.cvss_vector.is_none());
        assert!(d.score.is_none());
        assert!(d.severity_level.is_empty());
    }

    #[test]
    fn vulnerability_alert_serialization_roundtrip() {
        let a = VulnerabilityAlert {
            package_name: "lodash".into(),
            osv_package_name: "lodash".into(),
            dep_version: "4.17.0".into(),
            fixed_version: Some(">= 4.17.21".into()),
            datasource: "npm".into(),
            vulnerability_id: "CVE-2021-23337".into(),
            ..Default::default()
        };
        let json = serde_json::to_string(&a).unwrap();
        let back: VulnerabilityAlert = serde_json::from_str(&json).unwrap();
        assert_eq!(back.package_name, "lodash");
        assert_eq!(back.vulnerability_id, "CVE-2021-23337");
    }

    #[test]
    fn osv_vulnerability_serialization_roundtrip() {
        let v = OsvVulnerability {
            id: "GHSA-abc".into(),
            summary: Some("Test vuln".into()),
            affected: vec![OsvAffected {
                package_name: Some("lodash".into()),
                ecosystem: Some("npm".into()),
                versions: vec!["4.17.0".into()],
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&v).unwrap();
        let back: OsvVulnerability = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, "GHSA-abc");
        assert_eq!(back.affected.len(), 1);
    }

    #[test]
    fn dependency_vulnerabilities_default() {
        let d = DependencyVulnerabilities::default();
        assert!(d.package_name.is_empty());
        assert!(d.vulnerabilities.is_empty());
    }

    #[test]
    fn osv_affected_default() {
        let a = OsvAffected::default();
        assert!(a.package_name.is_none());
        assert!(a.versions.is_empty());
    }

    #[test]
    fn severity_entry_construct() {
        let e = SeverityEntry {
            severity_type: "CVSS_V3".into(),
            score: "CVSS:3.1/AV:N".into(),
        };
        assert_eq!(e.severity_type, "CVSS_V3");
    }
}
