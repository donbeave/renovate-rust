//! The single registry of **intentional exclusions** — upstream source files
//! and tests that will never be ported because they exercise behavior with no
//! Rust analogue (TypeScript/Node-runtime specifics, type-shape assertions,
//! etc.). Hand-maintained at `docs/parity/opt-out.toml`; loaded here and consumed
//! by the source/test generators so excluded items are reported as `opt-out`
//! (with a reason) instead of `pending`, and are never picked by an agent.
//!
//! ```toml
//! [[source]]
//! file = "lib/util/node-streams.ts"
//! reason = "Node stream plumbing; Rust uses std::io, no analogue"
//!
//! [[test]]
//! spec = "lib/util/foo.spec.ts"
//! test = "rejects when Buffer is not a Buffer"   # exact it() text
//! reason = "asserts a TS-runtime type guard with no Rust equivalent"
//!
//! [[test]]
//! spec = "lib/util/bar.spec.ts"
//! all  = true                                    # whole spec excluded
//! reason = "entirely TypeScript type-shape checks"
//! ```

use std::path::Path;

use serde::Deserialize;

/// Default registry location, relative to the working dir (repo root).
pub(crate) const OPT_OUT_PATH: &str = "docs/parity/opt-out.toml";

#[derive(Debug, Default, Deserialize)]
pub(crate) struct OptOut {
    #[serde(default)]
    pub(crate) source: Vec<SourceExc>,
    #[serde(default)]
    pub(crate) test: Vec<TestExc>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SourceExc {
    /// Upstream path, e.g. `lib/util/node-streams.ts`.
    pub(crate) file: String,
    pub(crate) reason: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TestExc {
    /// Upstream spec path, e.g. `lib/util/foo.spec.ts`.
    pub(crate) spec: String,
    /// Exact `it()` description. Omit (with `all = true`) to exclude the whole spec.
    #[serde(default)]
    pub(crate) test: Option<String>,
    #[serde(default)]
    pub(crate) all: bool,
    pub(crate) reason: String,
}

impl OptOut {
    /// Load the registry. A missing file means "no exclusions"; a malformed file
    /// is a hard error so a typo can't silently drop exclusions.
    pub(crate) fn load(path: &Path) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(s) => toml::from_str(&s).map_err(|e| format!("{}: {e}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(format!("{}: {e}", path.display())),
        }
    }

    /// Reason this upstream source file is excluded, if any.
    pub(crate) fn source_reason(&self, file: &str) -> Option<&str> {
        self.source
            .iter()
            .find(|e| e.file == file)
            .map(|e| e.reason.as_str())
    }
}
