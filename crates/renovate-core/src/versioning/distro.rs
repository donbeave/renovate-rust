//! Distro versioning.
//!
//! Ports `lib/modules/versioning/distro.ts` — versioning used by Linux
//! distribution packages (Debian, Ubuntu, Red Hat, etc.).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistroFamily {
    Debian,
    Ubuntu,
    RedHat,
    Fedora,
    CentOS,
    AmazonLinux,
    Alpine,
}

impl DistroFamily {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "debian" => Some(Self::Debian),
            "ubuntu" => Some(Self::Ubuntu),
            "redhat" | "rhel" => Some(Self::RedHat),
            "fedora" => Some(Self::Fedora),
            "centos" => Some(Self::CentOS),
            "amzn" | "amazon" | "amazon-linux" => Some(Self::AmazonLinux),
            "alpine" => Some(Self::Alpine),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
    pub codename: Option<String>,
    raw_minor: String,
}

impl DistroVersion {
    pub fn parse(version: &str) -> Option<Self> {
        let trimmed = version.trim();
        if trimmed.is_empty() {
            return None;
        }

        let parts: Vec<&str> = trimmed.split('.').collect();
        let major = parts.first()?.parse::<u32>().ok()?;
        let raw_minor = parts.get(1).map(|s| s.to_string()).unwrap_or_default();
        let minor = raw_minor.parse::<u32>().unwrap_or(0);
        let patch = parts.get(2).and_then(|s| s.parse::<u32>().ok());

        Some(Self {
            major,
            minor,
            patch,
            codename: None,
            raw_minor,
        })
    }

    pub fn parse_with_codename(version: &str) -> Option<Self> {
        let trimmed = version.trim();

        if let Some((ver_part, codename)) = trimmed.split_once('/') {
            let mut dv = Self::parse(ver_part)?;
            dv.codename = Some(codename.trim().to_string());
            return Some(dv);
        }

        if let Some((ver_part, codename)) = trimmed.split_once('-') {
            if let Ok(_) = ver_part.parse::<u32>() {
                let mut dv = Self::parse(ver_part)?;
                dv.codename = Some(codename.trim().to_string());
                return Some(dv);
            }
        }

        Self::parse(trimmed)
    }

    pub fn to_string_original(&self) -> String {
        let mut s = if self.raw_minor.is_empty() {
            format!("{}", self.major)
        } else {
            format!("{}.{}", self.major, self.raw_minor)
        };
        if let Some(p) = self.patch {
            s.push_str(&format!(".{p}"));
        }
        if let Some(ref cn) = self.codename {
            s.push_str(&format!("-{cn}"));
        }
        s
    }
}

impl std::fmt::Display for DistroVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;
        if let Some(p) = self.patch {
            write!(f, ".{p}")?;
        }
        if let Some(ref cn) = self.codename {
            write!(f, "-{cn}")?;
        }
        Ok(())
    }
}

pub fn compare_distro_versions(a: &DistroVersion, b: &DistroVersion) -> std::cmp::Ordering {
    match a.major.cmp(&b.major) {
        std::cmp::Ordering::Equal => {}
        ord => return ord,
    }
    match a.minor.cmp(&b.minor) {
        std::cmp::Ordering::Equal => {}
        ord => return ord,
    }
    a.patch.unwrap_or(0).cmp(&b.patch.unwrap_or(0))
}

pub fn is_distro_version_gte(version: &str, minimum: &str) -> bool {
    let v = match DistroVersion::parse(version) {
        Some(v) => v,
        None => return false,
    };
    let m = match DistroVersion::parse(minimum) {
        Some(m) => m,
        None => return false,
    };
    compare_distro_versions(&v, &m) != std::cmp::Ordering::Less
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_version() {
        let v = DistroVersion::parse("22.04").unwrap();
        assert_eq!(v.major, 22);
        assert_eq!(v.minor, 4);
    }

    #[test]
    fn parse_three_part() {
        let v = DistroVersion::parse("8.6.1").unwrap();
        assert_eq!(v.major, 8);
        assert_eq!(v.minor, 6);
        assert_eq!(v.patch, Some(1));
    }

    #[test]
    fn parse_single_number() {
        let v = DistroVersion::parse("11").unwrap();
        assert_eq!(v.major, 11);
        assert_eq!(v.minor, 0);
    }

    #[test]
    fn parse_empty_returns_none() {
        assert!(DistroVersion::parse("").is_none());
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert!(DistroVersion::parse("abc").is_none());
    }

    #[test]
    fn parse_with_codename_slash() {
        let v = DistroVersion::parse_with_codename("22.04/jammy").unwrap();
        assert_eq!(v.major, 22);
        assert_eq!(v.minor, 4);
        assert_eq!(v.codename.as_deref(), Some("jammy"));
    }

    #[test]
    fn parse_with_codename_dash() {
        let v = DistroVersion::parse_with_codename("11-bullseye").unwrap();
        assert_eq!(v.major, 11);
        assert_eq!(v.minor, 0);
        assert_eq!(v.codename.as_deref(), Some("bullseye"));
    }

    #[test]
    fn display_version() {
        let v = DistroVersion::parse("22.04").unwrap();
        assert_eq!(v.to_string_original(), "22.04");
    }

    #[test]
    fn display_version_with_patch() {
        let v = DistroVersion::parse("8.6.1").unwrap();
        assert_eq!(v.to_string_original(), "8.6.1");
    }

    #[test]
    fn compare_equal() {
        let a = DistroVersion::parse("22.04").unwrap();
        let b = DistroVersion::parse("22.04").unwrap();
        assert_eq!(compare_distro_versions(&a, &b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn compare_less() {
        let a = DistroVersion::parse("20.04").unwrap();
        let b = DistroVersion::parse("22.04").unwrap();
        assert_eq!(compare_distro_versions(&a, &b), std::cmp::Ordering::Less);
    }

    #[test]
    fn compare_greater() {
        let a = DistroVersion::parse("24.04").unwrap();
        let b = DistroVersion::parse("22.04").unwrap();
        assert_eq!(compare_distro_versions(&a, &b), std::cmp::Ordering::Greater);
    }

    #[test]
    fn is_gte_true() {
        assert!(is_distro_version_gte("22.04", "20.04"));
    }

    #[test]
    fn is_gte_false() {
        assert!(!is_distro_version_gte("18.04", "20.04"));
    }

    #[test]
    fn is_gte_equal() {
        assert!(is_distro_version_gte("22.04", "22.04"));
    }

    #[test]
    fn distro_family_from_name() {
        assert_eq!(DistroFamily::from_name("ubuntu"), Some(DistroFamily::Ubuntu));
        assert_eq!(DistroFamily::from_name("Debian"), Some(DistroFamily::Debian));
        assert_eq!(DistroFamily::from_name("fedora"), Some(DistroFamily::Fedora));
        assert_eq!(DistroFamily::from_name("unknown"), None);
    }
}
