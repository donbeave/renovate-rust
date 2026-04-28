//! `setup.cfg` dependency extractor.
//!
//! Parses the `[options]` and `[options.extras_require]` sections of a
//! `setup.cfg` file and returns Python package dependencies for PyPI lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/setup-cfg/extract.ts`
//! - `lib/modules/manager/setup-cfg/index.ts` — pattern `/(^|/)setup\\.cfg$/`
//!
//! ## Supported records
//!
//! | Section                    | Record           | Dep type |
//! |----------------------------|------------------|----------|
//! | `[options]`                | `install_requires` | install |
//! | `[options]`                | `setup_requires`   | setup   |
//! | `[options]`                | `tests_require`    | test    |
//! | `[options.extras_require]` | any key            | extra   |
//!
//! Continuation lines (indented lines following the key `=` line) are treated
//! as additional deps for the current record. Blank lines or new keys reset
//! the record.

use std::sync::LazyLock;

use regex::Regex;

/// Why a dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetupCfgSkipReason {
    /// No version constraint specified.
    NoVersion,
    /// Git or VCS source (`git+https://…`).
    GitSource,
}

/// A single extracted dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupCfgDep {
    pub name: String,
    pub current_value: String,
    pub dep_type: String,
    pub skip_reason: Option<SetupCfgSkipReason>,
}

static SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\[([^\]]+)\]").unwrap());

static RECORD_KEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([\w.-]+)\s*=\s*(.*)").unwrap());

/// Extract deps from a `setup.cfg` file.
pub fn extract(content: &str) -> Vec<SetupCfgDep> {
    let mut out = Vec::new();
    let mut section: Option<String> = None;
    let mut record: Option<String> = None;
    let mut dep_type: Option<String> = None;

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split('#').next().unwrap_or("").trim_end();

        // Blank line resets the current record (continuation ends).
        if line.trim().is_empty() {
            record = None;
            dep_type = None;
            continue;
        }

        // New section header.
        if let Some(cap) = SECTION.captures(line) {
            section = Some(cap[1].trim().to_owned());
            record = None;
            dep_type = None;
            continue;
        }

        // New record key (not indented).
        if !line.starts_with(' ') && !line.starts_with('\t') {
            if let Some(cap) = RECORD_KEY.captures(line) {
                let key = cap[1].trim().to_owned();
                dep_type = classify_record(section.as_deref(), &key);
                if dep_type.is_some() {
                    record = Some(key);
                    // Inline deps after `install_requires = dep1, dep2`.
                    let inline = cap[2].trim();
                    if !inline.is_empty() {
                        for part in inline.split(',') {
                            if let Some(d) = parse_dep(part, dep_type.as_deref().unwrap()) {
                                out.push(d);
                            }
                        }
                    }
                } else {
                    record = None;
                    dep_type = None;
                }
            } else {
                record = None;
                dep_type = None;
            }
            continue;
        }

        // Indented continuation line within an active record.
        if record.is_some() && dep_type.is_some() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                // Strip env markers before `;`
                let trimmed = trimmed.split(';').next().unwrap_or("").trim();
                if let Some(d) = parse_dep(trimmed, dep_type.as_deref().unwrap()) {
                    out.push(d);
                }
            }
        }
    }

    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn classify_record(section: Option<&str>, record: &str) -> Option<String> {
    match section {
        Some("options") => match record {
            "install_requires" => Some("install".to_owned()),
            "setup_requires" => Some("setup".to_owned()),
            "tests_require" => Some("test".to_owned()),
            _ => None,
        },
        Some("options.extras_require") => Some("extra".to_owned()),
        _ => None,
    }
}

static DEP_RE: LazyLock<Regex> = LazyLock::new(|| {
    // name: PEP 508 identifier, optional extras [foo,bar], optional specifier
    Regex::new(r"^([A-Za-z0-9][A-Za-z0-9._-]*)(?:\[[^\]]*\])?\s*(.*)?$").unwrap()
});

fn parse_dep(raw: &str, dep_type: &str) -> Option<SetupCfgDep> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    // Git/VCS installs.
    if raw.starts_with("git+") || raw.starts_with("svn+") || raw.starts_with("hg+") {
        return Some(SetupCfgDep {
            name: raw.to_owned(),
            current_value: String::new(),
            dep_type: dep_type.to_owned(),
            skip_reason: Some(SetupCfgSkipReason::GitSource),
        });
    }

    let cap = DEP_RE.captures(raw)?;
    let name = normalize_name(&cap[1]);
    let specifier = cap[2].trim().to_owned();

    let skip_reason = if specifier.is_empty() {
        Some(SetupCfgSkipReason::NoVersion)
    } else {
        None
    };

    Some(SetupCfgDep {
        name,
        current_value: specifier,
        dep_type: dep_type.to_owned(),
        skip_reason,
    })
}

/// Normalize PyPI name: lowercase, replace `-`/`_`/`.` with `-`.
fn normalize_name(name: &str) -> String {
    name.to_ascii_lowercase()
        .replace(['.', '_'], "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[metadata]
name = mypackage
version = 1.0.0

[options]
install_requires =
    requests>=2.25.0
    flask>=2.0,<3.0
    click==8.0.0

setup_requires =
    setuptools>=45

tests_require =
    pytest>=6.0
    coverage

[options.extras_require]
dev =
    black
    isort>=5.0
docs =
    sphinx>=4.0
"#;

    #[test]
    fn extracts_install_requires() {
        let deps = extract(SAMPLE);
        let install: Vec<_> = deps.iter().filter(|d| d.dep_type == "install").collect();
        assert_eq!(install.len(), 3);

        let requests = install.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(requests.current_value, ">=2.25.0");
        assert!(requests.skip_reason.is_none());

        let flask = install.iter().find(|d| d.name == "flask").unwrap();
        assert_eq!(flask.current_value, ">=2.0,<3.0");

        let click = install.iter().find(|d| d.name == "click").unwrap();
        assert_eq!(click.current_value, "==8.0.0");
    }

    #[test]
    fn extracts_setup_requires() {
        let deps = extract(SAMPLE);
        let setup: Vec<_> = deps.iter().filter(|d| d.dep_type == "setup").collect();
        assert_eq!(setup.len(), 1);
        assert_eq!(setup[0].name, "setuptools");
        assert_eq!(setup[0].current_value, ">=45");
    }

    #[test]
    fn extracts_tests_require() {
        let deps = extract(SAMPLE);
        let test: Vec<_> = deps.iter().filter(|d| d.dep_type == "test").collect();
        assert_eq!(test.len(), 2);

        let pytest = test.iter().find(|d| d.name == "pytest").unwrap();
        assert_eq!(pytest.current_value, ">=6.0");

        let coverage = test.iter().find(|d| d.name == "coverage").unwrap();
        assert_eq!(coverage.skip_reason, Some(SetupCfgSkipReason::NoVersion));
    }

    #[test]
    fn extracts_extras_require() {
        let deps = extract(SAMPLE);
        let extra: Vec<_> = deps.iter().filter(|d| d.dep_type == "extra").collect();
        // black (no ver), isort (ver), sphinx (ver) — black is extra too
        assert!(extra.iter().any(|d| d.name == "isort"));
        assert!(extra.iter().any(|d| d.name == "sphinx"));
    }

    #[test]
    fn skips_git_source() {
        let content = "[options]\ninstall_requires =\n    git+https://github.com/org/repo.git@v1.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(SetupCfgSkipReason::GitSource));
    }

    #[test]
    fn normalizes_package_name() {
        let content = "[options]\ninstall_requires =\n    My_Package>=1.0\n";
        let deps = extract(content);
        assert_eq!(deps[0].name, "my-package");
    }

    #[test]
    fn strips_env_markers() {
        let content = "[options]\ninstall_requires =\n    tomli>=1.0; python_version < '3.11'\n";
        let deps = extract(content);
        assert_eq!(deps[0].name, "tomli");
        assert_eq!(deps[0].current_value, ">=1.0");
    }

    #[test]
    fn ignores_unrelated_sections() {
        let content = "[tool:pytest]\npython_files = test_*.py\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }
}
