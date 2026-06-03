//! Source-file parity: maps upstream Renovate `.ts` **implementation** files to
//! their Rust counterparts and reports per-file port status.
//!
//! Truth lives next to the code as `@parity` tags in `.rs` doc comments, so the
//! mapping cannot drift from the implementation. This module harvests the tags,
//! diffs them against the upstream `lib/**/*.ts` file list (tests excluded), and
//! renders a regenerable mapping table.
//!
//! Tag grammar (one per line, in a `//!`, `///`, or `//` comment):
//!
//! ```text
//! //! @parity lib/modules/manager/cargo/extract.ts full
//! //! @parity `lib/modules/manager/cargo/index.ts` partial — datasources not stored
//! ```
//!
//! `<status>` is one of `full`, `partial`, `stub`, `out-of-scope`.
//! An upstream file with **no** tag pointing at it is reported as `pending`.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use regex::Regex;
use walkdir::WalkDir;

/// Statuses an author may assert via a tag; `pending` is implicit (no tag).
const TAG_STATUSES: [&str; 4] = ["full", "partial", "stub", "out-of-scope"];

/// One `@parity` tag harvested from a Rust file.
#[derive(Clone)]
pub(crate) struct Tag {
    pub(crate) ts: String,
    pub(crate) status: String,
    pub(crate) note: String,
    pub(crate) rust_file: String,
}

/// Return upstream **implementation** `.ts` paths (keyed `lib/...`), excluding
/// tests, mocks, and snapshots.
pub(crate) fn scan_upstream(lib: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for entry in WalkDir::new(lib).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let name = entry.file_name().to_string_lossy();
        if !name.ends_with(".ts") {
            continue;
        }
        if name.ends_with(".spec.ts") || name.ends_with(".test.ts") || name.ends_with(".mock.ts") {
            continue;
        }
        // Skip anything under a __mocks__ / __fixtures__ / __snapshots__ dir.
        if path.components().any(|c| {
            let s = c.as_os_str().to_string_lossy();
            s.starts_with("__") && s.ends_with("__")
        }) {
            continue;
        }
        // Key relative to the upstream root so it reads `lib/...`.
        if let Some(parent) = lib.parent()
            && let Ok(rel) = path.strip_prefix(parent)
        {
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Harvest every `@parity` tag from `.rs` files under `root`.
pub(crate) fn scan_tags(root: &Path) -> Result<Vec<Tag>, std::io::Error> {
    // `@parity <ts> <status> [— note]`, ts optionally backtick-wrapped.
    let re = Regex::new(
        r"@parity\s+`?(?P<ts>lib/[^\s`]+\.ts)`?\s+(?P<status>full|partial|stub|out-of-scope)\b[ \t]*(?:[—\-:]\s*)?(?P<note>.*)",
    )
    .expect("static regex compiles");

    let mut tags = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        // Skip the parity tool's own source: its doc comments contain example
        // tags that would otherwise register as real mappings.
        if entry
            .path()
            .components()
            .any(|c| c.as_os_str() == "parity-cli")
        {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(root)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .replace('\\', "/");
        let text = std::fs::read_to_string(entry.path())?;
        for line in text.lines() {
            // Only consider comment lines so we never match a string literal.
            let trimmed = line.trim_start();
            if !(trimmed.starts_with("//") || trimmed.starts_with('*')) {
                continue;
            }
            if let Some(c) = re.captures(line) {
                tags.push(Tag {
                    ts: c["ts"].to_string(),
                    status: c["status"].to_string(),
                    note: c["note"].trim().to_string(),
                    rust_file: rel.clone(),
                });
            }
        }
    }
    tags.sort_by(|a, b| (&a.ts, &a.rust_file).cmp(&(&b.ts, &b.rust_file)));
    Ok(tags)
}

/// A reconciled row: one upstream file with its resolved status + rust files.
struct Row {
    ts: String,
    status: String,
    rust_files: Vec<String>,
    note: String,
}

/// When several tags hit the same file, surface the *least complete* asserted
/// status so the row stays honest about remaining work.
fn weakest(a: &str, b: &str) -> String {
    let rank = |s: &str| match s {
        "out-of-scope" => 4,
        "full" => 3,
        "partial" => 2,
        "stub" => 1,
        _ => 0, // pending
    };
    if rank(a) <= rank(b) {
        a.to_string()
    } else {
        b.to_string()
    }
}

fn reconcile(upstream: &[String], tags: &[Tag]) -> Vec<Row> {
    let mut by_ts: BTreeMap<String, Row> = BTreeMap::new();
    for ts in upstream {
        by_ts.insert(
            ts.clone(),
            Row {
                ts: ts.clone(),
                status: "pending".to_string(),
                rust_files: Vec::new(),
                note: String::new(),
            },
        );
    }
    for t in tags {
        let row = by_ts.entry(t.ts.clone()).or_insert_with(|| Row {
            ts: t.ts.clone(),
            status: "pending".to_string(),
            rust_files: Vec::new(),
            note: String::new(),
        });
        row.status = if row.status == "pending" {
            t.status.clone()
        } else {
            weakest(&row.status, &t.status)
        };
        if !row.rust_files.contains(&t.rust_file) {
            row.rust_files.push(t.rust_file.clone());
        }
        if !t.note.is_empty() {
            if row.note.is_empty() {
                row.note = t.note.clone();
            } else if !row.note.contains(&t.note) {
                row.note.push_str("; ");
                row.note.push_str(&t.note);
            }
        }
    }
    by_ts.into_values().collect()
}

/// Group key for a `lib/...ts` path, e.g. `manager/cargo`, `util/git`.
fn module_of(ts: &str) -> String {
    // (prefix, number of path segments that name the module)
    let rules: &[(&str, usize)] = &[
        ("lib/modules/manager/", 4),
        ("lib/modules/datasource/", 4),
        ("lib/modules/platform/", 4),
        ("lib/modules/versioning/", 4),
        ("lib/workers/", 3),
        ("lib/config/", 3),
        ("lib/util/", 3),
    ];
    let parts: Vec<&str> = ts.split('/').collect();
    for (prefix, take) in rules {
        if ts.starts_with(prefix) {
            // e.g. lib/modules/manager/cargo/extract.ts, take=4 -> manager/cargo
            let kind = parts.get(take - 2).copied().unwrap_or("_");
            let name = parts.get(take - 1).copied().unwrap_or("_");
            // Single-file module (lib/util/foo.ts): name still carries `.ts`.
            if name.ends_with(".ts") {
                return format!("{kind}/_root");
            }
            return format!("{kind}/{name}");
        }
    }
    // Fallback: first segment under lib/.
    parts.get(1).copied().unwrap_or("lib").to_string()
}

fn count(rows: &[&Row], status: &str) -> usize {
    rows.iter().filter(|r| r.status == status).count()
}

pub(crate) fn render_report(upstream: &[String], tags: &[Tag]) -> String {
    let rows = reconcile(upstream, tags);
    let total = rows.len();
    let refs: Vec<&Row> = rows.iter().collect();
    let full = count(&refs, "full");
    let partial = count(&refs, "partial");
    let stub = count(&refs, "stub");
    let oos = count(&refs, "out-of-scope");
    let pending = count(&refs, "pending");
    let mapped = full + partial + stub;
    let in_scope = total - oos;

    let mut out = String::new();
    out.push_str("# Renovate Source-Map (impl files)\n\n");
    out.push_str("Auto-generated by `cargo run -p parity-cli -- source`.\n");
    out.push_str("**Do not hand-edit.** Status lives in `@parity` tags in the Rust source.\n\n");
    out.push_str(
        "Maps every upstream `lib/**/*.ts` implementation file (tests excluded) to its \
         Rust counterpart(s).\n\n",
    );
    out.push_str("Status: `full` · `partial` · `stub` · `pending` (no tag) · `out-of-scope`.\n\n");
    out.push_str(&format!(
        "**Coverage:** {mapped}/{in_scope} in-scope files mapped \
         (full={full} partial={partial} stub={stub} pending={pending} out-of-scope={oos}). \
         Total upstream files: {total}.\n\n",
    ));

    // Group rows by module.
    let mut groups: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
    for r in &rows {
        groups.entry(module_of(&r.ts)).or_default().push(r);
    }

    // Summary table.
    out.push_str("## Summary by module\n\n");
    out.push_str("| Module | Files | full | partial | stub | pending | oos |\n");
    out.push_str("|---|--:|--:|--:|--:|--:|--:|\n");
    for (module, grp) in &groups {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | {} |\n",
            module,
            grp.len(),
            count(grp, "full"),
            count(grp, "partial"),
            count(grp, "stub"),
            count(grp, "pending"),
            count(grp, "out-of-scope"),
        ));
    }
    out.push('\n');

    // Per-module detail.
    out.push_str("## Per-file mapping\n\n");
    for (module, grp) in &groups {
        out.push_str(&format!("### `{module}`\n\n"));
        out.push_str("| TS source | Status | Rust file(s) | Note |\n");
        out.push_str("|---|---|---|---|\n");
        for r in grp {
            let rust = if r.rust_files.is_empty() {
                "—".to_owned()
            } else {
                r.rust_files
                    .iter()
                    .map(|f| format!("`{f}`"))
                    .collect::<Vec<_>>()
                    .join("<br>")
            };
            let note = if r.note.is_empty() { "—" } else { &r.note };
            out.push_str(&format!(
                "| `{}` | {} | {} | {} |\n",
                r.ts, r.status, rust, note
            ));
        }
        out.push('\n');
    }

    out
}

/// Print stale tags (point at a non-existent upstream file) and bad statuses to
/// stderr. Returns `true` if any problem was found.
pub(crate) fn report_stale(upstream: &[String], tags: &[Tag]) -> bool {
    let upstream_set: HashSet<&String> = upstream.iter().collect();
    let mut bad = false;
    for t in tags {
        if !upstream_set.contains(&t.ts) {
            eprintln!("stale: {} -> {} (upstream file missing)", t.rust_file, t.ts);
            bad = true;
        }
        if !TAG_STATUSES.contains(&t.status.as_str()) {
            eprintln!("bad-status: {} -> {} `{}`", t.rust_file, t.ts, t.status);
            bad = true;
        }
    }
    bad
}
