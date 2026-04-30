//! Conan `conanfile.txt` / `conanfile.py` dependency extractor.
//!
//! Extracts `name/version` pairs from the `[requires]` and `[build_requires]`
//! sections of `conanfile.txt`, and from `requires = ...` assignments in
//! `conanfile.py`.
//!
//! Renovate reference:
//! - `lib/modules/manager/conan/extract.ts`
//! - Patterns: `/(^|/)conanfile\.(txt|py)$/`
//! - Datasource: Conan Center (`conan-io/conan-center-index`)
//!
//! ## Supported forms
//!
//! ```text
//! [requires]
//! zlib/1.2.11
//! boost/1.79.0@_/_
//!
//! [build_requires]
//! cmake/3.25.3
//! ```
//!
//! ## Skip reasons
//!
//! - `name/version@user/channel` (non-standard channel) — skipped, uses custom registry
//! - `name/*` or `name/[>=1.0]` — range spec, not a pinned version

use std::sync::LazyLock;

use regex::Regex;

/// Type of Conan requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConanDepType {
    Requires,
    BuildRequires,
    PythonRequires,
}

/// Why a Conan dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConanSkipReason {
    /// Has a user/channel component (e.g. `@user/channel`, not `@_/_`).
    CustomChannel,
    /// Version is a range (`*`, `[>=...]`) rather than a pin.
    RangeVersion,
}

/// A single extracted Conan dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConanDep {
    pub name: String,
    pub current_value: String,
    pub dep_type: ConanDepType,
    pub skip_reason: Option<ConanSkipReason>,
}

// ── Compiled regex ────────────────────────────────────────────────────────────

/// Matches `name/version[@user/channel][#revision]` in a line.
/// Captures: (1) name, (2) version, (3) optional @user/channel
static DEP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:^|["',\s])([\w][\w.\-+]*)/([^@#\n{*"',\s\[\]]+)(?:@([\w./_-]+))?"#).unwrap()
});

/// Extract Conan deps from a `conanfile.txt` file.
pub fn extract_txt(content: &str) -> Vec<ConanDep> {
    let mut deps = Vec::new();
    let mut dep_type = ConanDepType::Requires;

    for line in content.lines() {
        let trimmed = line.trim();

        // Section headers.
        if trimmed.starts_with('[') {
            dep_type = match trimmed.to_lowercase().as_str() {
                "[requires]" => ConanDepType::Requires,
                "[build_requires]" | "[build_requirements]" => ConanDepType::BuildRequires,
                _ => dep_type, // keep current for unknown sections
            };
            continue;
        }

        // Skip comments.
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        parse_dep_line(trimmed, dep_type, &mut deps);
    }

    deps
}

/// Extract Conan deps from a `conanfile.py` file.
pub fn extract_py(content: &str) -> Vec<ConanDep> {
    let mut deps = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }

        let dep_type = if trimmed.contains("python_requires") {
            ConanDepType::PythonRequires
        } else if trimmed.contains("build_require") {
            ConanDepType::BuildRequires
        } else if trimmed.contains("require") {
            ConanDepType::Requires
        } else {
            continue;
        };

        parse_dep_line(trimmed, dep_type, &mut deps);
    }

    deps
}

fn parse_dep_line(line: &str, dep_type: ConanDepType, out: &mut Vec<ConanDep>) {
    for cap in DEP_RE.captures_iter(line) {
        let name = cap[1].to_owned();
        let version = cap[2].trim().to_owned();
        let channel = cap.get(3).map(|m| m.as_str());

        // Skip range versions.
        if version.contains('*') || version.starts_with('[') {
            out.push(ConanDep {
                name,
                current_value: version,
                dep_type,
                skip_reason: Some(ConanSkipReason::RangeVersion),
            });
            continue;
        }

        // Skip custom user/channel (anything that isn't `@_/_` or absent).
        if let Some(ch) = channel
            && ch != "_/_"
        {
            out.push(ConanDep {
                name,
                current_value: version,
                dep_type,
                skip_reason: Some(ConanSkipReason::CustomChannel),
            });
            continue;
        }

        out.push(ConanDep {
            name,
            current_value: version,
            dep_type,
            skip_reason: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TXT: &str = r#"
[requires]
zlib/1.2.11
boost/1.79.0@_/_
openssl/3.0.7@_/_

[build_requires]
cmake/3.25.3
ninja/1.11.1

[generators]
cmake
"#;

    const SAMPLE_PY: &str = r#"
from conan import ConanFile

class MyConan(ConanFile):
    requires = "zlib/1.2.11", "boost/1.79.0"
    build_requires = "cmake/3.25.3"
"#;

    #[test]
    fn extracts_txt_requires() {
        let deps = extract_txt(SAMPLE_TXT);
        let zlib = deps.iter().find(|d| d.name == "zlib").unwrap();
        assert_eq!(zlib.current_value, "1.2.11");
        assert_eq!(zlib.dep_type, ConanDepType::Requires);
        assert!(zlib.skip_reason.is_none());
    }

    #[test]
    fn extracts_txt_build_requires() {
        let deps = extract_txt(SAMPLE_TXT);
        let cmake = deps.iter().find(|d| d.name == "cmake").unwrap();
        assert_eq!(cmake.dep_type, ConanDepType::BuildRequires);
        assert_eq!(cmake.current_value, "3.25.3");
    }

    #[test]
    fn standard_channel_kept() {
        let deps = extract_txt(SAMPLE_TXT);
        // boost@_/_ should be actionable
        let boost = deps.iter().find(|d| d.name == "boost").unwrap();
        assert!(boost.skip_reason.is_none());
    }

    // Ported: "extracts multiple image lines from conanfile.py" — conan/extract.spec.ts line 134
    #[test]
    fn extracts_py_requires() {
        let deps = extract_py(SAMPLE_PY);
        assert!(
            deps.iter()
                .any(|d| d.name == "zlib" && d.current_value == "1.2.11")
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "boost" && d.current_value == "1.79.0")
        );
    }

    // Ported: "returns null for empty" — conan/extract.spec.ts line 10
    #[test]
    fn empty_returns_empty() {
        assert!(extract_txt("").is_empty());
        assert!(extract_py("").is_empty());
    }
}
