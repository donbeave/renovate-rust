//! Test parity: maps every upstream test (`it()`/`test()` in a `.spec.ts` file)
//! to a Rust `// Ported:` comment. A test's **identity** is `(spec file, test
//! description)`. There is no partial or fuzzy state:
//!
//! - `ported`  — the upstream test exists and a Rust `// Ported:` matches it.
//! - `pending` — the upstream test exists with no Rust counterpart.
//! - `deleted` — a Rust `// Ported:` whose upstream identity is gone (the spec
//!   file was removed/moved, or the test was renamed/removed). The Rust test is
//!   **kept** and flagged for review — never auto-deleted. A rename upstream is
//!   simply an old `deleted` plus a new `pending`.
//!
//! Discovery scans the **whole** upstream repo (not a hard-coded location list)
//! for files matching the test criteria, so new / moved / removed specs are
//! picked up automatically on every regeneration.
//!
//! Matching is count-robust: descriptions are matched where they can be parsed,
//! and a removed-test (`deleted`) is only inferred from a vanished description
//! when the spec file's descriptions were fully extractable (no `it.each` /
//! template-literal openers). Otherwise only a vanished *file* marks `deleted`.
//!
//! `// Ported:` grammar (see AGENTS.md):
//!
//! ```text
//! // Ported: "extracts image lines" — lib/modules/manager/ansible/extract.spec.ts line 16
//! ```

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use std::path::Path;

use regex::Regex;
use walkdir::{DirEntry, WalkDir};

/// Where the split test-mapping tree is written, relative to the working dir.
pub(crate) const MAPPING_DIR: &str = "docs/parity/test-mapping";

/// Directory names pruned during discovery — never contain in-scope specs and
/// can be huge (node_modules) or unreadable. Pruned as whole subtrees.
const PRUNE_DIRS: [&str; 7] = [
    "node_modules",
    "dist",
    ".git",
    "coverage",
    ".cache",
    "target",
    "__snapshots__",
];

fn is_pruned(e: &DirEntry) -> bool {
    e.file_type().is_dir() && PRUNE_DIRS.contains(&e.file_name().to_string_lossy().as_ref())
}

/// True if `name` is an upstream test file by our criteria. Renovate uses
/// `.spec.ts`; `.test.ts` is accepted defensively so a future convention change
/// is still detected without editing this tool.
fn is_spec(name: &str) -> bool {
    name.ends_with(".spec.ts") || name.ends_with(".test.ts")
}

/// One upstream spec file.
pub(crate) struct SpecFile {
    /// Path relative to the upstream repo root, e.g.
    /// `lib/modules/manager/cargo/extract.spec.ts` or `test/other/x.spec.ts`.
    pub(crate) rel: String,
    pub(crate) module: String,
    /// Total `it()`/`test()` call sites (the count-robust denominator).
    pub(crate) it_count: usize,
    /// Normalized descriptions that were successfully extracted.
    pub(crate) descs: HashSet<String>,
    /// True if every call site's description was extractable, so a vanished
    /// description can be trusted to mean the test was removed.
    pub(crate) complete: bool,
    /// Every call site: `(1-based line, normalized description if parseable)`.
    /// Drives the `gaps` listing.
    pub(crate) sites: Vec<(usize, Option<String>)>,
}

/// One `// Ported:` comment harvested from a Rust test.
pub(crate) struct Ported {
    pub(crate) desc_norm: String,
    pub(crate) raw_desc: String,
    /// Spec reference with any trailing ` line N` stripped.
    pub(crate) spec_ref: String,
    pub(crate) rust_file: String,
    pub(crate) rust_line: usize,
}

// ---------------------------------------------------------------------------
// Classification
// ---------------------------------------------------------------------------

/// Module id for a spec path relative to the repo root. Paths under `lib/` use
/// the Renovate module taxonomy (manager/datasource/platform/versioning/…);
/// anything else is grouped by its top-level directory (e.g. `test`, `tools`).
fn classify(repo_rel: &str) -> String {
    let Some(rel) = repo_rel.strip_prefix("lib/") else {
        return repo_rel.split('/').next().unwrap_or("other").to_string();
    };
    let rules: &[(&str, &str)] = &[
        (r"^modules/manager/([^/]+)/", "manager/{0}"),
        (r"^modules/manager/[^/]+\.spec\.ts$", "manager/_common"),
        (r"^modules/datasource/([^/]+)/", "datasource/{0}"),
        (
            r"^modules/datasource/[^/]+\.spec\.ts$",
            "datasource/_common",
        ),
        (r"^modules/platform/([^/]+)/", "platform/{0}"),
        (r"^modules/platform/[^/]+\.spec\.ts$", "platform/_common"),
        (r"^modules/versioning/([^/]+)/", "versioning/{0}"),
        (
            r"^modules/versioning/[^/]+\.spec\.ts$",
            "versioning/_common",
        ),
        (r"^workers/([^/]+)/", "worker/{0}"),
        (r"^workers/", "worker/_root"),
        (r"^config/([^/]+)/", "config/{0}"),
        (r"^config/", "config/_root"),
        (r"^util/([^/]+)/", "util/{0}"),
        (r"^util/", "util/_root"),
        (r"^logger/", "logger"),
        (r"^instrumentation/", "instrumentation"),
        (r"^constants/", "constants"),
        (r"^data/", "data"),
        (r"^types/", "types"),
        (r"^[^/]+\.spec\.ts$", "cli/_root"),
    ];
    for (pat, tmpl) in rules {
        let re = Regex::new(pat).expect("static regex");
        if let Some(c) = re.captures(rel) {
            return match c.get(1) {
                Some(m) => tmpl.replace("{0}", m.as_str()),
                None => (*tmpl).to_string(),
            };
        }
    }
    "other".to_string()
}

// ---------------------------------------------------------------------------
// Upstream scanning
// ---------------------------------------------------------------------------

/// Lazily-built per-process regexes for spec scanning.
struct SpecRes {
    it_call: Regex,
    xit: Regex,
    desc_double: Regex,
    desc_single: Regex,
    desc_backtick: Regex,
}

impl SpecRes {
    fn new() -> Self {
        // One call site per it/test, up to two chained `.each/.skip/...`.
        let modifiers = r"(?:\.(?:each|skip|only|failing|concurrent|todo)){0,2}";
        Self {
            it_call: Regex::new(&format!(r"^[ \t]*(?:it|test){modifiers}[ \t]*[(`]"))
                .expect("static regex"),
            xit: Regex::new(r"^[ \t]*x(?:it|test)[ \t]*\(").expect("static regex"),
            desc_double: Regex::new(&format!(
                r#"^[ \t]*x?(?:it|test){modifiers}[ \t]*\(\s*"((?:\\.|[^"\\])*)""#
            ))
            .expect("static regex"),
            desc_single: Regex::new(&format!(
                r"^[ \t]*x?(?:it|test){modifiers}[ \t]*\(\s*'((?:\\.|[^'\\])*)'"
            ))
            .expect("static regex"),
            desc_backtick: Regex::new(&format!(
                r"^[ \t]*x?(?:it|test){modifiers}[ \t]*\(\s*`([^`]*)`"
            ))
            .expect("static regex"),
        }
    }

    fn is_site(&self, line: &str) -> bool {
        self.it_call.is_match(line) || self.xit.is_match(line)
    }

    /// Extract the normalized description from an `it(...)` opener, if its first
    /// argument is a plain string literal.
    fn extract_desc(&self, line: &str) -> Option<String> {
        for re in [&self.desc_double, &self.desc_single, &self.desc_backtick] {
            if let Some(c) = re.captures(line) {
                return Some(normalize_desc(&c[1]));
            }
        }
        None
    }
}

/// Scan the whole upstream repo for test files and their `it()` descriptions.
pub(crate) fn scan_specs(repo_root: &Path) -> Vec<SpecFile> {
    let res = SpecRes::new();
    let mut specs = Vec::new();
    let walker = WalkDir::new(repo_root)
        .into_iter()
        .filter_entry(|e| !is_pruned(e));
    for entry in walker.filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if !is_spec(&entry.file_name().to_string_lossy()) {
            continue;
        }
        let rel = match entry.path().strip_prefix(repo_root) {
            Ok(r) => r.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };
        let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
        let mut extracted = 0;
        let mut descs = HashSet::new();
        let mut sites = Vec::new();
        for (i, line) in text.lines().enumerate() {
            if !res.is_site(line) {
                continue;
            }
            let desc = res.extract_desc(line);
            if let Some(d) = &desc {
                extracted += 1;
                descs.insert(d.clone());
            }
            sites.push((i + 1, desc));
        }
        let it_count = sites.len();
        let module = classify(&rel);
        specs.push(SpecFile {
            rel,
            module,
            it_count,
            descs,
            complete: extracted == it_count,
            sites,
        });
    }
    specs.sort_by(|a, b| a.rel.cmp(&b.rel));
    specs
}

// ---------------------------------------------------------------------------
// Normalization
// ---------------------------------------------------------------------------

/// Strip a trailing ` line N` / ` lines N` and any parenthetical commentary
/// from a spec reference.
fn normalize_ref(raw: &str) -> String {
    let line_re = Regex::new(r"\s+lines?\s+\d+").expect("static regex");
    let mut s = line_re.replace_all(raw.trim(), "").to_string();
    if let Some(idx) = s.find(" (") {
        s.truncate(idx);
    }
    s.trim().trim_end_matches(',').trim().to_string()
}

/// Lowercase, unescape common backslash escapes, and collapse whitespace so a
/// Rust comment and the upstream source converge on the same key.
fn normalize_desc(s: &str) -> String {
    let unescaped = s.replace("\\\"", "\"").replace("\\'", "'");
    unescaped
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

// ---------------------------------------------------------------------------
// Rust // Ported: scanning
// ---------------------------------------------------------------------------

/// Scan Rust `// Ported:` comments under `root`.
pub(crate) fn scan_ported(root: &Path) -> Result<Vec<Ported>, std::io::Error> {
    // `// Ported: "<desc>" — <ref>`. This is a *comment*, not a string literal,
    // so descriptions frequently contain unescaped inner quotes copied verbatim
    // from `it(...)`. The robust parse anchors the reference on its reliable
    // `.spec.ts` token: `desc` is greedy and backtracks to the last quote before
    // the separator, capturing any inner quotes. Optional ` (note)`, em/en/hyphen.
    let re = Regex::new(
        r#"//\s*Ported:\s*["'](?P<desc>.*)["'](?:\s*\([^)]*\))?\s*[—–-]\s*(?P<ref>[^\n]*?\.spec\.ts)"#,
    )
    .expect("static regex");

    let mut out = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        // Skip the parity tool's own source: its docs contain example comments.
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
        for (i, line) in text.lines().enumerate() {
            if !line.contains("// Ported:") {
                continue;
            }
            match re.captures(line) {
                Some(c) => out.push(Ported {
                    desc_norm: normalize_desc(&c["desc"]),
                    raw_desc: c["desc"].to_string(),
                    spec_ref: normalize_ref(&c["ref"]),
                    rust_file: rel.clone(),
                    rust_line: i + 1,
                }),
                None => out.push(Ported {
                    desc_norm: String::new(),
                    raw_desc: String::new(),
                    spec_ref: String::new(),
                    rust_file: rel.clone(),
                    rust_line: i + 1,
                }),
            }
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Resolver
// ---------------------------------------------------------------------------

/// Resolves a `// Ported:` ref to an upstream spec. Two modes:
///
/// - **exact** (post-migration): the ref already equals an upstream repo path.
/// - **legacy** (used by `normalize` once): also tries the historical
///   shorthand forms (`modules/...`, `<manager>/...`, bare filename) so the
///   one-time migration can rewrite them to canonical form.
struct Resolver<'a> {
    by_rel: HashMap<&'a str, &'a SpecFile>,
    by_basename: HashMap<String, Vec<&'a SpecFile>>,
    legacy: bool,
}

/// Historical ref prefixes, tried only during the one-time `normalize`.
const LEGACY_PREFIXES: [&str; 10] = [
    "",
    "lib/",
    "lib/modules/manager/",
    "lib/modules/datasource/",
    "lib/modules/platform/",
    "lib/modules/versioning/",
    "lib/modules/",
    "lib/workers/",
    "lib/util/",
    "lib/config/",
];

impl<'a> Resolver<'a> {
    fn new(specs: &'a [SpecFile], legacy: bool) -> Self {
        let mut by_rel = HashMap::new();
        let mut by_basename: HashMap<String, Vec<&SpecFile>> = HashMap::new();
        for s in specs {
            by_rel.insert(s.rel.as_str(), s);
            let base = s.rel.rsplit('/').next().unwrap_or(&s.rel).to_string();
            by_basename.entry(base).or_default().push(s);
        }
        Self {
            by_rel,
            by_basename,
            legacy,
        }
    }

    fn resolve(&self, raw_ref: &str) -> Option<&'a SpecFile> {
        if raw_ref.is_empty() {
            return None;
        }
        // Exact, canonical match first.
        if let Some(s) = self.by_rel.get(raw_ref) {
            return Some(s);
        }
        if !self.legacy {
            return None;
        }
        // Legacy shorthand expansion (one-time migration only).
        let r = raw_ref.strip_prefix("lib/").unwrap_or(raw_ref);
        for prefix in LEGACY_PREFIXES {
            let cand = format!("{prefix}{r}");
            let cand = cand.trim_start_matches('/');
            if let Some(s) = self.by_rel.get(cand) {
                return Some(s);
            }
        }
        let base = r.rsplit('/').next().unwrap_or(r);
        match self.by_basename.get(base) {
            Some(v) if v.len() == 1 => Some(v[0]),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Reconcile
// ---------------------------------------------------------------------------

struct SpecStat<'a> {
    spec: &'a SpecFile,
    ported: usize,
    /// Distinct Rust files holding this spec's ported tests (the migration
    /// targets). Empty when the spec has no ported tests yet.
    rust_files: Vec<String>,
}

#[derive(Clone)]
enum DeleteReason {
    FileGone,
    TestRemoved,
}

struct Deleted {
    rust_file: String,
    rust_line: usize,
    spec_ref: String,
    desc: String,
    module: String,
    reason: DeleteReason,
}

struct Analysis<'a> {
    stats: Vec<SpecStat<'a>>,
    deleted: Vec<Deleted>,
}

fn analyze<'a>(specs: &'a [SpecFile], ported: &[Ported]) -> Analysis<'a> {
    let resolver = Resolver::new(specs, false);
    let mut covered: HashMap<&str, HashSet<String>> = HashMap::new();
    let mut rust_by_spec: HashMap<&str, BTreeSet<String>> = HashMap::new();
    let mut deleted = Vec::new();

    for p in ported {
        if p.spec_ref.is_empty() {
            // Unparseable comment — treat as needing review (file gone bucket).
            deleted.push(Deleted {
                rust_file: p.rust_file.clone(),
                rust_line: p.rust_line,
                spec_ref: "(unparseable)".to_string(),
                desc: p.raw_desc.clone(),
                module: "other".to_string(),
                reason: DeleteReason::FileGone,
            });
            continue;
        }
        match resolver.resolve(&p.spec_ref) {
            None => deleted.push(Deleted {
                rust_file: p.rust_file.clone(),
                rust_line: p.rust_line,
                spec_ref: p.spec_ref.clone(),
                desc: p.raw_desc.clone(),
                module: classify(&p.spec_ref),
                reason: DeleteReason::FileGone,
            }),
            Some(spec) => {
                // Description vanished from a fully-extractable file → removed.
                if spec.complete && !spec.descs.contains(&p.desc_norm) {
                    deleted.push(Deleted {
                        rust_file: p.rust_file.clone(),
                        rust_line: p.rust_line,
                        spec_ref: p.spec_ref.clone(),
                        desc: p.raw_desc.clone(),
                        module: spec.module.clone(),
                        reason: DeleteReason::TestRemoved,
                    });
                } else {
                    covered
                        .entry(spec.rel.as_str())
                        .or_default()
                        .insert(p.desc_norm.clone());
                    rust_by_spec
                        .entry(spec.rel.as_str())
                        .or_default()
                        .insert(p.rust_file.clone());
                }
            }
        }
    }

    let stats = specs
        .iter()
        .map(|s| {
            let n = covered.get(s.rel.as_str()).map_or(0, HashSet::len);
            let rust_files = rust_by_spec
                .get(s.rel.as_str())
                .map(|set| set.iter().cloned().collect())
                .unwrap_or_default();
            SpecStat {
                spec: s,
                ported: n.min(s.it_count),
                rust_files,
            }
        })
        .collect();

    Analysis { stats, deleted }
}

/// Print deleted/orphan `// Ported:` comments to stderr for `check`. Returns
/// `true` if any were found.
pub(crate) fn report_orphans(specs: &[SpecFile], ported: &[Ported]) -> bool {
    let Analysis { deleted, .. } = analyze(specs, ported);
    for d in &deleted {
        let reason = match d.reason {
            DeleteReason::FileGone => "file removed/moved",
            DeleteReason::TestRemoved => "test removed/renamed",
        };
        eprintln!(
            "deleted: {}:{} -> `{}` ({reason})",
            d.rust_file, d.rust_line, d.spec_ref
        );
    }
    !deleted.is_empty()
}

// ---------------------------------------------------------------------------
// Split mapping tree (README -> module page -> spec page)
// ---------------------------------------------------------------------------

/// Per-`it()` destination: `(spec rel, normalized desc) -> (rust file, line)`.
/// First `// Ported:` wins. Built with the same matching rule as `analyze`.
fn build_dest_map(
    specs: &[SpecFile],
    ported: &[Ported],
) -> HashMap<(String, String), (String, usize)> {
    let resolver = Resolver::new(specs, false);
    let mut map = HashMap::new();
    for p in ported {
        if let Some(spec) = resolver.resolve(&p.spec_ref) {
            if spec.complete && !spec.descs.contains(&p.desc_norm) {
                continue; // description removed upstream — counts as deleted, not a dest
            }
            map.entry((spec.rel.clone(), p.desc_norm.clone()))
                .or_insert((p.rust_file.clone(), p.rust_line));
        }
    }
    map
}

/// Relative markdown link from one page to another, both given as paths
/// relative to the mapping-tree root.
fn rel_link(from: &str, to: &str) -> String {
    let fc: Vec<&str> = from.split('/').collect();
    let tc: Vec<&str> = to.split('/').collect();
    let fdir = &fc[..fc.len().saturating_sub(1)];
    let tdir = &tc[..tc.len().saturating_sub(1)];
    let mut i = 0;
    while i < fdir.len() && i < tdir.len() && fdir[i] == tdir[i] {
        i += 1;
    }
    let mut parts: Vec<String> = Vec::new();
    for _ in i..fdir.len() {
        parts.push("..".to_string());
    }
    for c in &tdir[i..] {
        parts.push((*c).to_string());
    }
    parts.push(tc[tc.len() - 1].to_string());
    parts.join("/")
}

fn module_page_path(module: &str) -> String {
    format!("_by-module/{module}.md")
}

fn spec_page_path(spec_rel: &str) -> String {
    format!("{spec_rel}.md")
}

fn spec_status(it: usize, ported: usize) -> &'static str {
    if it == 0 {
        "—"
    } else if ported >= it {
        "ported"
    } else if ported == 0 {
        "pending"
    } else {
        "partial"
    }
}

fn render_readme(by_module: &BTreeMap<&str, Vec<&SpecStat>>, deleted: &[Deleted]) -> String {
    let total_it: usize = by_module.values().flatten().map(|s| s.spec.it_count).sum();
    let total_por: usize = by_module.values().flatten().map(|s| s.ported).sum();
    let mut out = String::new();
    out.push_str("# Test Mapping\n\n");
    out.push_str("Auto-generated by `cargo run -p parity-cli -- test`. **Do not hand-edit.**\n\n");
    out.push_str("Upstream `it()`/`test()` → Rust `// Ported:`. Click a module to see its spec ");
    out.push_str("files; click a spec to see each test and its Rust destination.\n\n");
    out.push_str(&format!(
        "**Coverage:** {total_por}/{total_it} upstream tests ported. **Deleted (review):** {}.\n\n",
        deleted.len()
    ));
    out.push_str("| Module | Spec files | it() | ported | pending | deleted | % |\n");
    out.push_str("|---|--:|--:|--:|--:|--:|--:|\n");
    let mut del_by_module: BTreeMap<&str, usize> = BTreeMap::new();
    for d in deleted {
        *del_by_module.entry(d.module.as_str()).or_default() += 1;
    }
    for (module, grp) in by_module {
        let it: usize = grp.iter().map(|s| s.spec.it_count).sum();
        let por: usize = grp.iter().map(|s| s.ported).sum();
        let del = del_by_module.get(module).copied().unwrap_or(0);
        let pct = por
            .checked_mul(100)
            .and_then(|n| n.checked_div(it))
            .unwrap_or(100);
        let link = rel_link("README.md", &module_page_path(module));
        out.push_str(&format!(
            "| [`{}`]({}) | {} | {} | {} | {} | {} | {}% |\n",
            module,
            link,
            grp.len(),
            it,
            por,
            it - por,
            del,
            pct,
        ));
    }
    out.push('\n');

    if !deleted.is_empty() {
        out.push_str("## Deleted upstream — review\n\n");
        out.push_str("Rust tests whose upstream identity is gone. Kept for review; never ");
        out.push_str("auto-removed.\n\n");
        out.push_str("| Rust test | Upstream ref | Reason |\n|---|---|---|\n");
        let mut rows: Vec<&Deleted> = deleted.iter().collect();
        rows.sort_by(|a, b| (&a.rust_file, a.rust_line).cmp(&(&b.rust_file, b.rust_line)));
        for d in rows {
            let reason = match d.reason {
                DeleteReason::FileGone => "spec file removed/moved",
                DeleteReason::TestRemoved => "test removed/renamed",
            };
            out.push_str(&format!(
                "| `crates/{}:{}` \"{}\" | `{}` | {} |\n",
                d.rust_file, d.rust_line, d.desc, d.spec_ref, reason
            ));
        }
        out.push('\n');
    }
    out
}

fn render_module_page(module: &str, grp: &[&SpecStat], mpath: &str) -> String {
    let it: usize = grp.iter().map(|s| s.spec.it_count).sum();
    let por: usize = grp.iter().map(|s| s.ported).sum();
    let mut out = String::new();
    out.push_str(&format!("# Module: `{module}`\n\n"));
    out.push_str(&format!(
        "[← all modules]({})\n\n",
        rel_link(mpath, "README.md")
    ));
    out.push_str(&format!(
        "**Coverage:** {por}/{it} tests ported across {} spec files.\n\n",
        grp.len()
    ));
    out.push_str("| Spec file | it() | ported | pending | Rust test file(s) | Status |\n");
    out.push_str("|---|--:|--:|--:|---|---|\n");
    for s in grp {
        let itc = s.spec.it_count;
        let pending = itc - s.ported;
        let spath = spec_page_path(&s.spec.rel);
        let link = rel_link(mpath, &spath);
        let rust = if s.rust_files.is_empty() {
            "—".to_owned()
        } else {
            s.rust_files
                .iter()
                .map(|f| format!("`crates/{f}`"))
                .collect::<Vec<_>>()
                .join("<br>")
        };
        out.push_str(&format!(
            "| [`{}`]({}) | {} | {} | {} | {} | {} |\n",
            s.spec.rel,
            link,
            itc,
            s.ported,
            pending,
            rust,
            spec_status(itc, s.ported),
        ));
    }
    out.push('\n');
    out
}

fn render_spec_page(
    s: &SpecStat,
    dest: &HashMap<(String, String), (String, usize)>,
    mpath: &str,
    spath: &str,
    module: &str,
) -> String {
    let rel = &s.spec.rel;
    let mut out = String::new();
    out.push_str(&format!("# `{rel}`\n\n"));
    out.push_str(&format!(
        "[← `{}`]({}) · [all modules]({})\n\n",
        module,
        rel_link(spath, mpath),
        rel_link(spath, "README.md"),
    ));
    out.push_str(&format!(
        "**{}/{} ported** ({} pending) · status: {}\n\n",
        s.ported,
        s.spec.it_count,
        s.spec.it_count - s.ported,
        spec_status(s.spec.it_count, s.ported),
    ));
    out.push_str("| Line | Test | Status | Rust destination |\n");
    out.push_str("|--:|---|---|---|\n");
    for (line, desc) in &s.spec.sites {
        match desc {
            Some(d) => {
                let raw = d.replace('|', "\\|");
                match dest.get(&(rel.clone(), d.clone())) {
                    Some((rf, rl)) => out.push_str(&format!(
                        "| {line} | {raw} | ported | `crates/{rf}:{rl}` |\n"
                    )),
                    None => out.push_str(&format!("| {line} | {raw} | pending | — |\n")),
                }
            }
            None => out.push_str(&format!(
                "| {line} | _(it.each / template — verify manually)_ | ? | — |\n"
            )),
        }
    }
    out.push('\n');
    out
}

/// Regenerate the whole split mapping tree under `out_dir`. The tree is wiped
/// first so removed specs/modules leave no orphan pages.
pub(crate) fn write_pages(
    out_dir: &Path,
    specs: &[SpecFile],
    ported: &[Ported],
) -> std::io::Result<usize> {
    let dest = build_dest_map(specs, ported);
    let Analysis { stats, deleted } = analyze(specs, ported);

    let mut by_module: BTreeMap<&str, Vec<&SpecStat>> = BTreeMap::new();
    for s in &stats {
        by_module.entry(s.spec.module.as_str()).or_default().push(s);
    }

    let mut files: Vec<(String, String)> = Vec::new();
    files.push(("README.md".to_string(), render_readme(&by_module, &deleted)));
    for (module, grp) in &by_module {
        let mpath = module_page_path(module);
        files.push((mpath.clone(), render_module_page(module, grp, &mpath)));
        for s in grp {
            let spath = spec_page_path(&s.spec.rel);
            files.push((
                spath.clone(),
                render_spec_page(s, &dest, &mpath, &spath, module),
            ));
        }
    }

    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }
    for (rel, content) in &files {
        let full = out_dir.join(rel);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(full, content)?;
    }
    Ok(files.len())
}

// ---------------------------------------------------------------------------
// Gaps — pending upstream tests for a module
// ---------------------------------------------------------------------------

/// List the upstream `it()` call sites that have no `// Ported:` counterpart,
/// for every spec whose module id or path contains `filter`. Prints to stdout.
pub(crate) fn gaps(specs: &[SpecFile], ported: &[Ported], filter: &str) -> bool {
    let resolver = Resolver::new(specs, false);
    let mut covered: HashMap<&str, HashSet<&str>> = HashMap::new();
    for p in ported {
        if let Some(spec) = resolver.resolve(&p.spec_ref) {
            covered
                .entry(spec.rel.as_str())
                .or_default()
                .insert(p.desc_norm.as_str());
        }
    }

    let mut matched = false;
    for spec in specs {
        if !(spec.module == filter || spec.module.contains(filter) || spec.rel.contains(filter)) {
            continue;
        }
        let cov = covered.get(spec.rel.as_str());
        let ported_n = cov.map_or(0, HashSet::len).min(spec.it_count);
        if ported_n >= spec.it_count {
            continue;
        }
        matched = true;
        println!(
            "## {}  —  {}/{} ({} missing)",
            spec.rel,
            ported_n,
            spec.it_count,
            spec.it_count - ported_n
        );
        for (line, desc) in &spec.sites {
            match desc {
                Some(d) if cov.is_some_and(|c| c.contains(d.as_str())) => {} // ported
                Some(d) => println!("  L{line:<5} {d}"),
                None => println!("  L{line:<5} (it.each / template — verify manually)"),
            }
        }
        println!();
    }
    if !matched {
        eprintln!("no pending specs match `{filter}` (already fully ported, or unknown module)");
    }
    matched
}

// ---------------------------------------------------------------------------
// One-time migration
// ---------------------------------------------------------------------------

/// Rewrite every `// Ported:` spec reference to the canonical repo-relative
/// path, in place. Only the path token is touched — the quoted description and
/// ` line N` suffix stay byte-for-byte identical, so the verified test text
/// never changes. Idempotent. Returns `(rewritten, unresolved)`.
pub(crate) fn normalize(root: &Path, specs: &[SpecFile]) -> Result<(usize, usize), std::io::Error> {
    let resolver = Resolver::new(specs, true);
    // The spec-path token in a ref, e.g. `dockerfile/extract.spec.ts`.
    let path_re = Regex::new(r"[A-Za-z0-9_./@-]+\.spec\.ts").expect("static regex");

    let mut rewritten = 0;
    let mut unresolved = 0;
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        if entry
            .path()
            .components()
            .any(|c| c.as_os_str() == "parity-cli")
        {
            continue;
        }
        let text = std::fs::read_to_string(entry.path())?;
        let mut out = String::with_capacity(text.len());
        for line in text.lines() {
            out.push_str(&rewrite_line(
                line,
                &path_re,
                &resolver,
                &mut rewritten,
                &mut unresolved,
            ));
            out.push('\n');
        }
        if !text.ends_with('\n') {
            out.pop();
        }
        if out != text {
            std::fs::write(entry.path(), &out)?;
        }
    }
    Ok((rewritten, unresolved))
}

fn rewrite_line(
    line: &str,
    path_re: &Regex,
    resolver: &Resolver,
    rewritten: &mut usize,
    unresolved: &mut usize,
) -> String {
    if !line.contains("// Ported:") {
        return line.to_string();
    }
    // The reference path is the LAST `*.spec.ts` token on the line (the
    // description comes first and may itself mention a path).
    let Some(m) = path_re.find_iter(line).last() else {
        return line.to_string();
    };
    let raw = m.as_str();
    match resolver.resolve(raw) {
        Some(spec) if spec.rel != raw => {
            *rewritten += 1;
            let mut s = String::with_capacity(line.len());
            s.push_str(&line[..m.start()]);
            s.push_str(&spec.rel);
            s.push_str(&line[m.end()..]);
            s
        }
        Some(_) => line.to_string(),
        None => {
            *unresolved += 1;
            eprintln!("normalize: unresolved ref `{raw}` in: {}", line.trim());
            line.to_string()
        }
    }
}
