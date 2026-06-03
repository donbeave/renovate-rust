//! Relative-link helpers shared by the source-mapping and test-mapping page
//! generators. All inputs are paths sharing a common base (the mapping-tree
//! root for within-tree links, or the repo root for links out to `crates/`).

/// Relative path from one file to another, both given relative to the same
/// base. E.g. `rel_link("_by-module/manager/cargo.md", "lib/x.spec.ts.md")`
/// → `../../lib/x.spec.ts.md`.
pub(crate) fn rel_link(from: &str, to: &str) -> String {
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

/// Markdown link from a generated page to a Rust file in the repo, at an
/// optional line (`#L<n>` so a click lands on the exact line). `page_repo` is
/// the page's path relative to the repo root; `rust_file` is relative to
/// `crates/`. The href is repo-relative so it resolves on GitHub and locally.
pub(crate) fn rust_link(page_repo: &str, rust_file: &str, line: Option<usize>) -> String {
    let target_repo = format!("crates/{rust_file}");
    let mut href = rel_link(page_repo, &target_repo);
    let label = match line {
        Some(l) => {
            href.push_str(&format!("#L{l}"));
            format!("crates/{rust_file}:{l}")
        }
        None => format!("crates/{rust_file}"),
    };
    format!("[`{label}`]({href})")
}
