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
//! cairo/1.17.2#aff2d03608351db075ec1348a3afc9ff
//!
//! [build_requires]
//! cmake/3.25.3
//! 7zip/[>1.1 <2.1]
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Type of Conan requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConanDepType {
    Requires,
    BuildRequires,
    PythonRequires,
}

/// A single extracted Conan dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConanDep {
    pub name: String,
    /// Version string (may be a range like `[>=1.0 <2.0]`).
    pub current_value: String,
    /// Content revision hash (from `#hex` suffix), if present.
    pub current_digest: Option<String>,
    pub dep_type: ConanDepType,
    /// Full conan package name: `name/version@user/channel` (default `@_/_`).
    pub package_name: String,
    /// Original text fragment to replace in the file.
    pub replace_string: String,
}

// ── Compiled regex ────────────────────────────────────────────────────────────

/// Matches conan dependency refs in a line.
/// Compatible with the TypeScript regex in lib/modules/manager/conan/extract.ts
/// - name: `[-\w]+`
/// - version: everything up to `@`, `#`, or end of relevant chars (including ranges)
/// - userChannel: `@name[/channel]` (optional)
/// - digest: `#hexchars` (optional)
static DEP_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Matches conan dep refs: name/version[@user[/channel]][#digest]
    // version: everything except @, #, newline, {, *, ", '  (including brackets for ranges)
    Regex::new(
        r#"(?:^|["'])([-\w]+)/([^@#\n{*"']+)(?:@([-\w]+(?:/[^#\n.{*"' ]+)?))?(?:#([0-9a-f]{8,}))?"#,
    )
    .unwrap()
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
                _ => dep_type,
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
        let user_channel = cap.get(3).map(|m| m.as_str());
        let digest = cap.get(4).map(|m| m.as_str().to_owned());

        // Build replaceString and packageName following TS logic.
        let mut replace_string = format!("{name}/{version}");
        let mut user_and_channel = "@_/_".to_owned();

        if let Some(uc) = user_channel {
            user_and_channel = format!("@{uc}");
            replace_string = format!("{name}/{version}@{uc}");
            if !uc.contains('/') {
                user_and_channel = format!("@{uc}/_");
            }
        }

        let package_name = format!("{name}/{version}{user_and_channel}");

        // Append digest to replace_string if present.
        let final_replace_string = if let Some(ref d) = digest {
            format!("{replace_string}#{d}")
        } else {
            replace_string
        };

        out.push(ConanDep {
            name,
            current_value: version,
            current_digest: digest,
            dep_type,
            package_name,
            replace_string: final_replace_string,
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

    // Ported: "extracts multiple image lines from conanfile.txt" — conan/extract.spec.ts line 14
    #[test]
    fn extracts_txt_requires() {
        let deps = extract_txt(SAMPLE_TXT);
        let zlib = deps.iter().find(|d| d.name == "zlib").unwrap();
        assert_eq!(zlib.current_value, "1.2.11");
        assert_eq!(zlib.dep_type, ConanDepType::Requires);
        assert!(zlib.current_digest.is_none());
    }

    // Ported: "extracts multiple image lines from conanfile.txt" — conan/extract.spec.ts line 14
    #[test]
    fn extracts_txt_build_requires() {
        let deps = extract_txt(SAMPLE_TXT);
        let cmake = deps.iter().find(|d| d.name == "cmake").unwrap();
        assert_eq!(cmake.dep_type, ConanDepType::BuildRequires);
        assert_eq!(cmake.current_value, "3.25.3");
    }

    // Ported: "extracts multiple image lines from conanfile.txt" — conan/extract.spec.ts line 14
    #[test]
    fn standard_channel_kept() {
        let deps = extract_txt(SAMPLE_TXT);
        // boost@_/_ should produce package_name boost/1.79.0@_/_
        let boost = deps.iter().find(|d| d.name == "boost").unwrap();
        assert_eq!(boost.package_name, "boost/1.79.0@_/_");
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

    // Ported: "extracts multiple image lines from conanfile.txt" — conan/extract.spec.ts line 14
    #[test]
    fn extracts_full_conanfile_txt_fixture() {
        let content = "[requires]\n\
            poco/1.9.4\n\
            zlib/[~1.2.3, loose=False]\n\
            fake/8.62.134@test/dev\n\
            cairo/1.17.2#aff2d03608351db075ec1348a3afc9ff\n\
            cairo/1.17.2@_/_#aff2d03608351db075ec1348a3afc9ff\n\
            Fake/8.62.134@\n\
            \n\
            [build_requires]\n\
            7zip/[>1.1 <2.1, include_prerelease=True]\n\
            curl/[~1.2.3, loose=False, include_prerelease=True]@test/dev\n\
            boost/[>1.1 <2.1]\n\
            catch2/[2.8]\n\
            openssl/[~=3.0]@test/prod\n\
            cmake/[>1.1 || 0.8]\n\
            cryptopp/[1.2.7 || >=1.2.9 <2.0.0]@test/local\n\
            #commentedout/1.2\n\
            # commentedout/3.4\n\
            meson/0.63.0@_/_#bc592346b33fd19c1fbffce25d1e4236\n\
            \n\
            [generators]\n\
            cmake\n";
        let deps = extract_txt(content);
        assert_eq!(deps.len(), 14);

        // Verify poco — simple requires dep
        let poco = deps.iter().find(|d| d.name == "poco").unwrap();
        assert_eq!(poco.current_value, "1.9.4");
        assert_eq!(poco.package_name, "poco/1.9.4@_/_");
        assert_eq!(poco.replace_string, "poco/1.9.4");
        assert!(poco.current_digest.is_none());

        // Range version (no skip in TS)
        let zlib = deps.iter().find(|d| d.name == "zlib").unwrap();
        assert!(zlib.current_value.starts_with('['));

        // Custom channel
        let fake = deps.iter().find(|d| d.name == "fake").unwrap();
        assert_eq!(fake.current_value, "8.62.134");
        assert!(fake.package_name.contains("@test/dev"));

        // Digest without explicit channel
        let cairo_nodash = deps
            .iter()
            .find(|d| {
                d.name == "cairo"
                    && d.replace_string.contains("#aff2d")
                    && !d.replace_string.contains("@_/_")
            })
            .unwrap();
        assert_eq!(
            cairo_nodash.current_digest.as_deref(),
            Some("aff2d03608351db075ec1348a3afc9ff")
        );
        assert_eq!(cairo_nodash.package_name, "cairo/1.17.2@_/_");

        // Digest with explicit @_/_ channel
        let cairo_at = deps
            .iter()
            .find(|d| d.name == "cairo" && d.replace_string.contains("@_/_#"))
            .unwrap();
        assert_eq!(
            cairo_at.current_digest.as_deref(),
            Some("aff2d03608351db075ec1348a3afc9ff")
        );

        // meson with _/_ and digest in build_requires
        let meson = deps.iter().find(|d| d.name == "meson").unwrap();
        assert_eq!(meson.dep_type, ConanDepType::BuildRequires);
        assert_eq!(meson.current_value, "0.63.0");
        assert_eq!(
            meson.current_digest.as_deref(),
            Some("bc592346b33fd19c1fbffce25d1e4236")
        );
    }

    // Ported: "extracts multiple 0 lines from conanfile.txt" — conan/extract.spec.ts line 129
    #[test]
    fn conanfile_without_requires_section_returns_empty() {
        // conanfile2.txt has only [generators], [options], [imports] — no [requires]
        let content = "[generators]\nxcode\ncmake\n\n[options]\npoco:shared=True\n";
        let deps = extract_txt(content);
        assert!(deps.is_empty());
    }
}
