//! pip-compile package-file adapter.
//!
//! Renovate reference:
//! - `lib/modules/manager/pip-compile/index.ts` `extractPackageFile`
//! - `lib/modules/manager/pip-compile/extract.spec.ts`
//!
//! The upstream manager delegates individual input files to the existing
//! pip_requirements, pip_setup, and pep621 extractors. Its full
//! `extractAllPackageFiles()` lock-file resolver is intentionally separate and
//! not implemented here.

use crate::extractors::{pep621, pip, pip_setup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipCompileDep {
    Pip(pip::PipExtractedDep),
    Pep621(pep621::Pep621ExtractedDep),
}

impl PipCompileDep {
    pub fn dep_name(&self) -> &str {
        match self {
            Self::Pip(dep) => &dep.name,
            Self::Pep621(dep) => &dep.name,
        }
    }

    pub fn dep_type(&self) -> Option<&str> {
        match self {
            Self::Pip(_) => None,
            Self::Pep621(dep) => Some(dep.dep_type.as_renovate_str()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipCompileExtract {
    pub deps: Vec<PipCompileDep>,
}

pub fn extract_package_file(content: &str, package_file: &str) -> Option<PipCompileExtract> {
    let deps = if package_file.ends_with(".in") {
        pip::extract_package_file(content)
            .deps
            .into_iter()
            .map(PipCompileDep::Pip)
            .collect()
    } else if package_file.ends_with("setup.py") {
        pip_setup::extract(content)
            .into_iter()
            .map(PipCompileDep::Pip)
            .collect()
    } else if package_file.ends_with("pyproject.toml") {
        pep621::extract_package_file(content)
            .ok()?
            .deps
            .into_iter()
            .map(PipCompileDep::Pep621)
            .collect()
    } else {
        return None;
    };

    Some(PipCompileExtract { deps })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns object for requirements.in" — pip-compile/extract.spec.ts line 40
    #[test]
    fn returns_object_for_requirements_in() {
        let package_file = extract_package_file(
            "attrs==23.1.0 \\\n    --hash=sha256:abc123\n",
            "requirements.in",
        )
        .expect("package file");

        assert_eq!(package_file.deps[0].dep_name(), "attrs");
    }

    // Ported: "returns object for setup.py" — pip-compile/extract.spec.ts line 50
    #[test]
    fn returns_object_for_setup_py() {
        let package_file = extract_package_file(
            r#"
from setuptools import setup

setup(
    install_requires=[
        "celery>=5.0",
    ],
)
"#,
            "lib/setup.py",
        )
        .expect("package file");

        assert_eq!(package_file.deps[0].dep_name(), "celery");
    }

    // Ported: "returns object for pyproject.toml" — pip-compile/extract.spec.ts line 60
    #[test]
    fn returns_object_for_pyproject_toml() {
        let package_file = extract_package_file(
            r#"
[build-system]
requires = ["setuptools", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "test-project"
requires-python = ">=3.11"
version = "1.2.3"
dependencies = [
  "aiohttp",
  "pydantic>=2.0.0",
]

[project.optional-dependencies]
dev = [
  "black",
  "flake8",
]
"#,
            "pyproject.toml",
        )
        .expect("package file");

        assert_eq!(package_file.deps[0].dep_type(), Some("requires-python"));
        assert_eq!(package_file.deps[1].dep_name(), "aiohttp");
    }

    // Ported: "returns null on not supported package files" — pip-compile/extract.spec.ts line 93
    #[test]
    fn returns_null_on_not_supported_package_files() {
        for package_file in ["random.py", "app.cfg", "already_locked.txt", "setup.cfg"] {
            assert!(extract_package_file("some content", package_file).is_none());
        }
    }
}
