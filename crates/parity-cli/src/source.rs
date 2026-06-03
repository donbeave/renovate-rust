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

use crate::paths::{rel_link, rust_link};

/// Statuses an author may assert via a tag; `pending` is implicit (no tag).
const TAG_STATUSES: [&str; 4] = ["full", "partial", "stub", "out-of-scope"];

/// Where the split source-mapping tree is written, relative to the working dir.
pub(crate) const MAPPING_DIR: &str = "docs/parity/source-mapping";

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

/// Top-level group for a module id (e.g. `manager/cargo` → `managers`). Drives
/// the one-page-per-group split.
fn group_of(module: &str) -> &'static str {
    match module.split('/').next().unwrap_or("") {
        "manager" => "managers",
        "datasource" => "datasources",
        "platform" => "platforms",
        "versioning" => "versioning",
        "util" => "util",
        "config" => "config",
        "worker" => "workers",
        "logger" | "instrumentation" | "constants" | "data" | "types" => "infra",
        _ => "cli",
    }
}

fn status_counts(rows: &[&Row]) -> (usize, usize, usize, usize, usize) {
    (
        count(rows, "full"),
        count(rows, "partial"),
        count(rows, "stub"),
        count(rows, "pending"),
        count(rows, "out-of-scope"),
    )
}

/// Render the per-file table rows for one module into `out`, with Rust files
/// linked from `page_repo` (the group page's repo-relative path).
fn render_module_section(out: &mut String, module: &str, grp: &[&Row], page_repo: &str) {
    out.push_str(&format!("### `{module}`\n\n"));
    out.push_str("| TS source | Status | Rust file(s) | Note |\n");
    out.push_str("|---|---|---|---|\n");
    for r in grp {
        let rust = if r.rust_files.is_empty() {
            "—".to_owned()
        } else {
            r.rust_files
                .iter()
                .map(|f| rust_link(page_repo, f, None))
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

/// Regenerate the split source-mapping tree (README + one page per group).
/// Wipes `out_dir` first so removed modules leave no orphan pages.
pub(crate) fn write_pages(
    out_dir: &Path,
    upstream: &[String],
    tags: &[Tag],
) -> std::io::Result<usize> {
    let rows = reconcile(upstream, tags);

    // module -> rows, and group -> [module].
    let mut by_module: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
    for r in &rows {
        by_module.entry(module_of(&r.ts)).or_default().push(r);
    }
    let mut by_group: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for module in by_module.keys() {
        by_group.entry(group_of(module)).or_default().push(module);
    }

    let mut files: Vec<(String, String)> = Vec::new();
    files.push((
        "README.md".to_string(),
        render_readme(&rows, &by_module, &by_group),
    ));
    for (group, modules) in &by_group {
        let path = format!("{group}.md");
        files.push((path.clone(), render_group_page(group, modules, &by_module)));
    }

    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }
    std::fs::create_dir_all(out_dir)?;
    for (rel, content) in &files {
        std::fs::write(out_dir.join(rel), content)?;
    }
    Ok(files.len())
}

fn render_readme(
    rows: &[Row],
    by_module: &BTreeMap<String, Vec<&Row>>,
    by_group: &BTreeMap<&str, Vec<&str>>,
) -> String {
    let refs: Vec<&Row> = rows.iter().collect();
    let (full, partial, stub, pending, oos) = status_counts(&refs);
    let mapped = full + partial + stub;
    let total = rows.len();
    let in_scope = total - oos;

    let mut out = String::new();
    out.push_str("# Source Mapping (impl files)\n\n");
    out.push_str("Auto-generated by `cargo run -p parity-cli -- source`. **Do not hand-edit.**\n");
    out.push_str("Status lives in `@parity` tags in the Rust source. Click a group to see its ");
    out.push_str("modules and per-file mapping.\n\n");
    out.push_str("Status: `full` · `partial` · `stub` · `pending` (no tag) · `out-of-scope`.\n\n");
    out.push_str(&format!(
        "**Coverage:** {mapped}/{in_scope} in-scope files mapped \
         (full={full} partial={partial} stub={stub} pending={pending} out-of-scope={oos}). \
         Total upstream files: {total}.\n\n",
    ));

    // By group.
    out.push_str("## By group\n\n");
    out.push_str("| Group | Files | full | partial | stub | pending | oos |\n");
    out.push_str("|---|--:|--:|--:|--:|--:|--:|\n");
    for (group, modules) in by_group {
        let grp_rows: Vec<&Row> = modules
            .iter()
            .flat_map(|m| by_module[*m].iter().copied())
            .collect();
        let (f, p, s, pe, o) = status_counts(&grp_rows);
        out.push_str(&format!(
            "| [{}]({}.md) | {} | {} | {} | {} | {} | {} |\n",
            group,
            group,
            grp_rows.len(),
            f,
            p,
            s,
            pe,
            o,
        ));
    }
    out.push('\n');

    // By module (links to the module's group page).
    out.push_str("## By module\n\n");
    out.push_str("| Module | Files | full | partial | stub | pending | oos |\n");
    out.push_str("|---|--:|--:|--:|--:|--:|--:|\n");
    for (module, grp) in by_module {
        let (f, p, s, pe, o) = status_counts(grp);
        out.push_str(&format!(
            "| [`{}`]({}.md) | {} | {} | {} | {} | {} | {} |\n",
            module,
            group_of(module),
            grp.len(),
            f,
            p,
            s,
            pe,
            o,
        ));
    }
    out.push('\n');
    out
}

fn render_group_page(
    group: &str,
    modules: &[&str],
    by_module: &BTreeMap<String, Vec<&Row>>,
) -> String {
    let page_repo = format!("{MAPPING_DIR}/{group}.md");
    let grp_rows: Vec<&Row> = modules
        .iter()
        .flat_map(|m| by_module[*m].iter().copied())
        .collect();
    let (full, partial, stub, pending, oos) = status_counts(&grp_rows);
    let mapped = full + partial + stub;
    let in_scope = grp_rows.len() - oos;

    let mut out = String::new();
    out.push_str(&format!("# Source Mapping — `{group}`\n\n"));
    out.push_str(&format!(
        "[← all groups]({})\n\n",
        rel_link(&page_repo, &format!("{MAPPING_DIR}/README.md"))
    ));
    out.push_str(&format!(
        "**Coverage:** {mapped}/{in_scope} in-scope files mapped \
         (full={full} partial={partial} stub={stub} pending={pending} out-of-scope={oos}) \
         across {} modules.\n\n",
        modules.len(),
    ));
    for module in modules {
        render_module_section(&mut out, module, &by_module[*module], &page_repo);
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
