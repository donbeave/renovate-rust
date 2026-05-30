//! Ruby Bundler `Gemfile` dependency extractor.
//!
//! Mirrors `lib/modules/manager/bundler/extract.ts` — `extractPackageFile`.

use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use regex::Regex;

// ── Public types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BundlerDepType {
    #[default]
    Regular,
    Dev,
}

impl BundlerDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            BundlerDepType::Regular => "dependencies",
            BundlerDepType::Dev => "devDependencies",
        }
    }
}

/// Why a bundler dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundlerSkipReason {
    /// Path/local gem (not resolvable via rubygems).
    InternalPackage,
}

impl Deref for BundlerSkipReason {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl BundlerSkipReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            BundlerSkipReason::InternalPackage => "internal-package",
        }
    }
}

/// A single extracted gem dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundlerExtractedDep {
    pub name: String,
    /// Version constraint including surrounding quotes, e.g. `'~> 7.0'` or `"~> 7.0"`.
    pub current_value: String,
    pub dep_type: BundlerDepType,
    pub skip_reason: Option<BundlerSkipReason>,
    pub locked_version: Option<String>,
    pub registry_urls: Vec<String>,
    /// `"rubygems"`, `"ruby-version"`, or `"git-refs"`.
    pub datasource: String,
    pub package_name: Option<String>,
    pub source_url: Option<String>,
    pub current_digest: Option<String>,
}

impl Default for BundlerExtractedDep {
    fn default() -> Self {
        Self {
            name: String::new(),
            current_value: String::new(),
            dep_type: BundlerDepType::Regular,
            skip_reason: None,
            locked_version: None,
            registry_urls: Vec::new(),
            datasource: "rubygems".to_owned(),
            package_name: None,
            source_url: None,
            current_digest: None,
        }
    }
}

/// Result of extracting a Gemfile (mirrors `PackageFileContent`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BundlerPackageFile {
    pub registry_urls: Vec<String>,
    pub deps: Vec<BundlerExtractedDep>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// TypeScript `gemMatchRegex` — captures depName and optional currentValue (with quotes).
static GEM_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^\s*gem\s+['"](?P<depName>[^'"]+)['"](?:\s*,\s*(?P<currentValue>['"][^'"]+['"](?:\s*,\s*['"][^'"]+['"])?))?"#,
    )
    .unwrap()
});

/// TypeScript `variableMatchRegex`.
static VARIABLE_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?P<key>\w+)\s*=\s*['"](?P<value>[^'"]+)['"]"#).unwrap());

/// Inline `source:` option on gem lines.
static SOURCE_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"source:\s*(?:['"](?P<registryUrl>[^'"]+)['"]|(?P<sourceName>\w+))?"#).unwrap()
});

/// `path:` option on gem lines.
static PATH_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"path:\s*['"][^'"]+['"]"#).unwrap());

/// TypeScript `gitRefsMatchRegex`.
static GIT_REFS_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?:git:\s*['"](?P<gitUrl>[^'"]+)['"]|,\s*github:\s*['"](?P<repoName>[^'"]+)['"])(?:\s*,\s*branch:\s*['"](?P<branchName>[^'"]+)['"])?(?:\s*,\s*ref:\s*['"](?P<refName>[^'"]+)['"])?(?:\s*,\s*tag:\s*['"](?P<tagName>[^'"]+)['"])?"#,
    )
    .unwrap()
});

/// Ruby version line: `ruby '2.6.5'` or `ruby "2.6.5"`.
static RUBY_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^ruby\s+['"](?P<ver>[^'"]+)['"]\s*$"#).unwrap());

/// Top-level `source 'url'` (no `do`).
static SOURCE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^source\s+(?:['"](?P<registryUrl>[^'"]+)['"]|(?P<sourceName>\w+))\s*$"#).unwrap()
});

/// `source 'url' do` block header.
static SOURCE_BLOCK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^source\s+(?:['"](?P<registryUrl>[^'"]+)['"]|(?P<sourceName>\w+))\s+do\s*$"#)
        .unwrap()
});

/// `group :dev, :test do` block header.
static GROUP_START: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^group\s+(.*?)\s+do\s*(?:#.*)?$").unwrap());

/// `platforms :ruby do` block header.
static PLATFORMS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^platforms?\s+(.*?)\s+do\s*(?:#.*)?$").unwrap());

/// `if condition` (no `do`).
static IF_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:if|unless)\s+").unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract from a Gemfile without a lock file.
pub fn extract(content: &str) -> Option<BundlerPackageFile> {
    extract_with_lock(content, None)
}

/// Extract from a Gemfile with an optional lock file for `locked_version` population.
pub fn extract_with_lock(content: &str, lock_content: Option<&str>) -> Option<BundlerPackageFile> {
    let lines: Vec<&str> = content.lines().collect();
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut pkg = BundlerPackageFile::default();

    extract_lines(&lines, &mut variables, &mut pkg, None);

    if pkg.deps.is_empty() && pkg.registry_urls.is_empty() {
        return None;
    }

    if let Some(lock) = lock_content {
        let locked = extract_lock_file_entries(lock);
        for dep in &mut pkg.deps {
            if let Some(ver) = locked.get(&dep.name) {
                dep.locked_version = Some(ver.clone());
            }
        }
    }

    Some(pkg)
}

// ── Internal ──────────────────────────────────────────────────────────────────

/// Whether a (trimmed) line opens a `do...end` or `if...end` block.
fn is_block_start(trimmed: &str) -> bool {
    trimmed.ends_with(" do")
        || trimmed == "do"
        || trimmed.starts_with("if ")
        || trimmed == "if"
        || trimmed.starts_with("unless ")
        || trimmed == "unless"
}

/// Collect inner lines until the matching `end` (depth-aware).
/// Returns `(inner_lines, index_after_end)`.
fn collect_until_matching_end<'a>(lines: &[&'a str], start: usize) -> (Vec<&'a str>, usize) {
    let mut inner = Vec::new();
    let mut depth: usize = 0;
    let mut i = start;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed == "end" {
            if depth == 0 {
                return (inner, i + 1);
            }
            depth -= 1;
            inner.push(lines[i]);
        } else if is_block_start(trimmed) {
            depth += 1;
            inner.push(lines[i]);
        } else {
            inner.push(lines[i]);
        }
        i += 1;
    }
    (inner, i)
}

fn extract_lines(
    lines: &[&str],
    variables: &mut HashMap<String, String>,
    result: &mut BundlerPackageFile,
    source_registry_url: Option<&str>,
) {
    let mut i = 0;
    while i < lines.len() {
        let raw = lines[i];
        let trimmed = raw.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            i += 1;
            continue;
        }

        // Variable: foo = 'url'
        if let Some(cap) = VARIABLE_MATCH.captures(trimmed) {
            variables.insert(cap["key"].to_owned(), cap["value"].to_owned());
            i += 1;
            continue;
        }

        // Top-level source (no do)
        if let Some(cap) = SOURCE_LINE.captures(trimmed) {
            if source_registry_url.is_none() {
                if let Some(u) = cap.name("registryUrl") {
                    result.registry_urls.push(u.as_str().to_owned());
                } else if let Some(n) = cap.name("sourceName")
                    && let Some(url) = variables.get(n.as_str())
                {
                    result.registry_urls.push(url.clone());
                }
            }
            i += 1;
            continue;
        }

        // Ruby version line
        if let Some(cap) = RUBY_VERSION.captures(trimmed) {
            let mut dep = BundlerExtractedDep {
                name: "ruby".to_owned(),
                current_value: cap["ver"].to_owned(),
                datasource: "ruby-version".to_owned(),
                ..Default::default()
            };
            if let Some(url) = source_registry_url {
                dep.registry_urls = vec![url.to_owned()];
            }
            result.deps.push(dep);
            i += 1;
            continue;
        }

        // Source block: source 'url' do
        if let Some(cap) = SOURCE_BLOCK.captures(trimmed) {
            let url = if let Some(u) = cap.name("registryUrl") {
                u.as_str().to_owned()
            } else if let Some(n) = cap.name("sourceName") {
                variables.get(n.as_str()).cloned().unwrap_or_default()
            } else {
                String::new()
            };
            i += 1;

            // Mirror TypeScript: group blocks inside source block are processed eagerly
            // via processGroupBlock, adding deps to result first; then remaining content
            // is processed by extractPackageFile (giving the TypeScript ordering quirk).
            let mut group_deps: Vec<BundlerExtractedDep> = Vec::new();
            let mut non_group_lines: Vec<&str> = Vec::new();

            while i < lines.len() && lines[i].trim() != "end" {
                let inner_raw = lines[i];
                let inner_trimmed = inner_raw.trim();

                if let Some(gcap) = GROUP_START.captures(inner_trimmed) {
                    let is_dev = is_dev_group(&gcap[1]);
                    i += 1;
                    let (inner, end_i) = collect_until_matching_end(lines, i);
                    i = end_i;
                    let mut sub = BundlerPackageFile::default();
                    extract_lines(&inner, variables, &mut sub, Some(&url));
                    for mut dep in sub.deps {
                        if is_dev {
                            dep.dep_type = BundlerDepType::Dev;
                        }
                        if !url.is_empty() && dep.registry_urls.is_empty() {
                            dep.registry_urls = vec![url.clone()];
                        }
                        group_deps.push(dep);
                    }
                    // Group header goes into non_group_lines (TypeScript sourceContent includes it)
                    non_group_lines.push(inner_raw);
                } else {
                    non_group_lines.push(inner_raw);
                    i += 1;
                }
            }
            if i < lines.len() {
                i += 1; // skip outer 'end'
            }

            // Process remaining source content (group headers without bodies → no extra deps)
            let mut source_sub = BundlerPackageFile::default();
            extract_lines(&non_group_lines, variables, &mut source_sub, Some(&url));

            // TypeScript ordering: group deps first, then source deps
            for dep in group_deps {
                result.deps.push(dep);
            }
            for mut dep in source_sub.deps {
                if !url.is_empty() && dep.registry_urls.is_empty() {
                    dep.registry_urls = vec![url.clone()];
                }
                result.deps.push(dep);
            }
            continue;
        }

        // Group block: group :dev do
        if let Some(cap) = GROUP_START.captures(trimmed) {
            let is_dev = is_dev_group(&cap[1]);
            i += 1;
            let (inner, end_i) = collect_until_matching_end(lines, i);
            i = end_i;
            let mut sub = BundlerPackageFile::default();
            extract_lines(&inner, variables, &mut sub, source_registry_url);
            for mut dep in sub.deps {
                if is_dev {
                    dep.dep_type = BundlerDepType::Dev;
                }
                result.deps.push(dep);
            }
            continue;
        }

        // Platforms block: platforms :ruby do
        if PLATFORMS_RE.is_match(trimmed) {
            i += 1;
            let (inner, end_i) = collect_until_matching_end(lines, i);
            i = end_i;
            let mut sub = BundlerPackageFile::default();
            extract_lines(&inner, variables, &mut sub, source_registry_url);
            result.deps.extend(sub.deps);
            continue;
        }

        // If/unless block
        if IF_RE.is_match(trimmed) {
            i += 1;
            let (inner, end_i) = collect_until_matching_end(lines, i);
            i = end_i;
            let mut sub = BundlerPackageFile::default();
            extract_lines(&inner, variables, &mut sub, source_registry_url);
            result.deps.extend(sub.deps);
            continue;
        }

        // Generic `do` block (catch-all)
        if trimmed.ends_with(" do") || trimmed == "do" {
            i += 1;
            let (inner, end_i) = collect_until_matching_end(lines, i);
            i = end_i;
            let mut sub = BundlerPackageFile::default();
            extract_lines(&inner, variables, &mut sub, source_registry_url);
            result.deps.extend(sub.deps);
            continue;
        }

        // Gem line
        if let Some(mut dep) = parse_gem_line(trimmed, variables) {
            if let Some(url) = source_registry_url
                && dep.registry_urls.is_empty()
            {
                dep.registry_urls = vec![url.to_owned()];
            }
            result.deps.push(dep);
        }

        i += 1;
    }
}

fn is_dev_group(args: &str) -> bool {
    args.contains("development") || args.contains("test")
}

fn parse_gem_line(line: &str, variables: &HashMap<String, String>) -> Option<BundlerExtractedDep> {
    let cap = GEM_MATCH.captures(line)?;
    let name = cap["depName"].to_owned();
    let current_value = cap
        .name("currentValue")
        .map(|m| m.as_str().to_owned())
        .unwrap_or_default();

    let mut dep = BundlerExtractedDep {
        name,
        current_value,
        ..Default::default()
    };

    // path: → internal-package
    if PATH_MATCH.is_match(line) {
        dep.skip_reason = Some(BundlerSkipReason::InternalPackage);
        return Some(dep);
    }

    // git: / github: → git-refs datasource
    if let Some(gc) = GIT_REFS_MATCH.captures(line) {
        if let Some(git_url) = gc.name("gitUrl") {
            let url = git_url.as_str().to_owned();
            dep.package_name = Some(url.clone());
            if is_http_url(&url) {
                dep.source_url = Some(url.trim_end_matches(".git").to_owned());
            }
        } else if let Some(repo) = gc.name("repoName") {
            let pkg = format!("https://github.com/{}", repo.as_str());
            dep.package_name = Some(pkg.clone());
            dep.source_url = Some(pkg);
        }
        if let Some(r) = gc.name("refName") {
            dep.current_digest = Some(r.as_str().to_owned());
        } else if let Some(b) = gc.name("branchName") {
            dep.current_value = b.as_str().to_owned();
        } else if let Some(t) = gc.name("tagName") {
            dep.current_value = t.as_str().to_owned();
        }
        dep.datasource = "git-refs".to_owned();
        return Some(dep);
    }

    // Inline source: option
    if let Some(sc) = SOURCE_MATCH.captures(line) {
        if let Some(url) = sc.name("registryUrl") {
            dep.registry_urls = vec![url.as_str().to_owned()];
        } else if let Some(n) = sc.name("sourceName")
            && let Some(url) = variables.get(n.as_str())
        {
            dep.registry_urls = vec![url.clone()];
        }
    }

    Some(dep)
}

fn is_http_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

// ── Lock-file parsing (unchanged) ─────────────────────────────────────────────

/// Parse a Gemfile.lock and return a map of `gem_name → version`.
pub fn extract_lock_file_entries(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if content.is_empty() {
        return map;
    }

    let platforms = extract_platforms(content);
    let mut in_gem_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        let indent = line.len() - line.trim_start().len();

        if indent == 0 && trimmed == "GEM" {
            in_gem_section = true;
        } else if indent == 0 && !trimmed.is_empty() && in_gem_section {
            in_gem_section = false;
        } else if indent == 4
            && in_gem_section
            && let (Some(open), Some(close)) = (line.rfind('('), line.rfind(')'))
            && open < close
        {
            let version = &line[open + 1..close];
            let name = line[..open].trim();
            let cleaned = strip_platform_suffix(version, &platforms);
            if !name.is_empty() && version_looks_valid(&cleaned) && !map.contains_key(name) {
                map.insert(name.to_owned(), cleaned);
            }
        }
    }
    map
}

fn extract_platforms(content: &str) -> Vec<String> {
    let mut platforms = Vec::new();
    let mut in_platforms = false;
    for line in content.lines() {
        let trimmed = line.trim();
        let indent = line.len() - line.trim_start().len();
        if indent == 0 && trimmed == "PLATFORMS" {
            in_platforms = true;
        } else if indent == 0 && !trimmed.is_empty() && in_platforms {
            break;
        } else if indent == 2 && in_platforms && !trimmed.is_empty() {
            platforms.push(trimmed.to_owned());
        }
    }
    platforms
}

fn strip_platform_suffix(version: &str, platforms: &[String]) -> String {
    for p in platforms {
        let suffix = format!("-{p}");
        if version.ends_with(suffix.as_str()) {
            return version[..version.len() - p.len() - 1].to_owned();
        }
    }
    version.to_owned()
}

fn version_looks_valid(v: &str) -> bool {
    v.chars().next().is_some_and(|c| c.is_ascii_digit())
}

// ── Update-locked helpers (unchanged) ────────────────────────────────────────

#[derive(Debug)]
pub enum BundlerUpdateLockedStatus {
    AlreadyUpdated,
    Unsupported,
    UpdateFailed,
}

impl BundlerUpdateLockedStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BundlerUpdateLockedStatus::AlreadyUpdated => "already-updated",
            BundlerUpdateLockedStatus::Unsupported => "unsupported",
            BundlerUpdateLockedStatus::UpdateFailed => "update-failed",
        }
    }
}

pub fn update_locked_bundler_dependency(
    dep_name: Option<&str>,
    new_version: Option<&str>,
    lock_file_content: Option<&str>,
) -> BundlerUpdateLockedStatus {
    let (Some(dep_name), Some(new_version)) = (dep_name, new_version) else {
        return BundlerUpdateLockedStatus::Unsupported;
    };
    let content = lock_file_content.unwrap_or("");
    let locked = extract_lock_file_entries(content);
    if locked.get(dep_name).is_some_and(|v| v == new_version) {
        BundlerUpdateLockedStatus::AlreadyUpdated
    } else {
        BundlerUpdateLockedStatus::Unsupported
    }
}

// ── Auth helpers ─────────────────────────────────────────────────────────────

/// Return all host rules for `host_type` that have a `resolved_host` and
/// complete credentials (username+password or token).
///
/// Mirrors `findAllAuthenticatable` from
/// `lib/modules/manager/bundler/host-rules.ts`.
pub fn find_all_authenticatable(host_type: &str) -> Vec<crate::util::host_rules::HostRule> {
    crate::util::host_rules::find_all(host_type)
        .into_iter()
        .filter(|r| {
            r.resolved_host.as_deref().is_some_and(|h| !h.is_empty())
                && ((r.username.as_deref().is_some_and(|u| !u.is_empty())
                    && r.password.as_deref().is_some_and(|p| !p.is_empty()))
                    || r.token.as_deref().is_some_and(|t| !t.is_empty()))
        })
        .collect()
}

pub fn get_authentication_header_value(
    username: Option<&str>,
    password: Option<&str>,
    token: Option<&str>,
) -> String {
    if let Some(u) = username {
        let encoded = percent_encode_username(u);
        let pw = password.unwrap_or("");
        return format!("{encoded}:{pw}");
    }
    token.unwrap_or("").to_owned()
}

fn percent_encode_username(username: &str) -> String {
    username
        .chars()
        .flat_map(|c| {
            if c.is_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
                vec![c.to_string()]
            } else {
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);
                s.bytes().map(|b| format!("%{b:02X}")).collect()
            }
        })
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn deps(pkg: &BundlerPackageFile) -> &[BundlerExtractedDep] {
        &pkg.deps
    }

    fn regular(pkg: &BundlerPackageFile) -> Vec<&BundlerExtractedDep> {
        pkg.deps
            .iter()
            .filter(|d| d.dep_type == BundlerDepType::Regular)
            .collect()
    }

    fn dev(pkg: &BundlerPackageFile) -> Vec<&BundlerExtractedDep> {
        pkg.deps
            .iter()
            .filter(|d| d.dep_type == BundlerDepType::Dev)
            .collect()
    }

    // ── Fixture constants ──────────────────────────────────────────────────────

    const GEMFILE_LOCK: &str = include_str!("../../tests/fixtures/bundler/Gemfile.rubyci.lock");
    const RAILS_LOCK: &str = include_str!("../../tests/fixtures/bundler/Gemfile.rails.lock");
    const WEBPACKER_LOCK: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.webpacker.lock");
    const MASTODON_LOCK: &str = include_str!("../../tests/fixtures/bundler/Gemfile.mastodon.lock");
    const GITLAB_FOSS_LOCK: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.gitlab-foss.lock");
    const SOURCE_BLOCK_NL_LOCK: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.sourceBlockWithNewLines.lock");

    const RAILS_GEMFILE: &str = include_str!("../../tests/fixtures/bundler/Gemfile.rails");
    const WEBPACKER_GEMFILE: &str = include_str!("../../tests/fixtures/bundler/Gemfile.webpacker");
    const MASTODON_GEMFILE: &str = include_str!("../../tests/fixtures/bundler/Gemfile.mastodon");
    const RUBYCI_GEMFILE: &str = include_str!("../../tests/fixtures/bundler/Gemfile.rubyci");
    const GITLAB_FOSS_GEMFILE: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.gitlab-foss");
    const SOURCE_GROUP_GEMFILE: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.sourceGroup");
    const SOURCE_BLOCK_GEMFILE: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.sourceBlock");
    const SOURCE_BLOCK_NL_GEMFILE: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.sourceBlockWithNewLines");
    const SOURCE_BLOCK_GROUPS_GEMFILE: &str =
        include_str!("../../tests/fixtures/bundler/Gemfile.sourceBlockWithGroups");

    // ── Baseline unit tests ────────────────────────────────────────────────────

    #[test]
    fn simple_gem_with_version() {
        let pkg = extract("gem 'rails', '~> 7.0.0'\n").unwrap();
        assert_eq!(deps(&pkg).len(), 1);
        assert_eq!(deps(&pkg)[0].name, "rails");
        assert_eq!(deps(&pkg)[0].current_value, "'~> 7.0.0'");
        assert!(deps(&pkg)[0].skip_reason.is_none());
    }

    #[test]
    fn gem_no_version() {
        let pkg = extract("gem 'devise'\n").unwrap();
        assert_eq!(deps(&pkg).len(), 1);
        assert_eq!(deps(&pkg)[0].name, "devise");
        assert!(deps(&pkg)[0].current_value.is_empty());
    }

    #[test]
    fn multi_version_constraint() {
        let pkg = extract("gem 'pg', '>= 0.18', '< 2.0'\n").unwrap();
        assert_eq!(deps(&pkg).len(), 1);
        assert_eq!(deps(&pkg)[0].current_value, "'>= 0.18', '< 2.0'");
    }

    #[test]
    fn git_gem_uses_git_refs_datasource() {
        let content = "gem 'nokogiri', git: 'https://github.com/sparklemotion/nokogiri.git'\n";
        let pkg = extract(content).unwrap();
        assert_eq!(deps(&pkg).len(), 1);
        assert_eq!(deps(&pkg)[0].datasource, "git-refs");
        assert!(deps(&pkg)[0].skip_reason.is_none());
        assert_eq!(
            deps(&pkg)[0].package_name.as_deref(),
            Some("https://github.com/sparklemotion/nokogiri.git")
        );
        assert_eq!(
            deps(&pkg)[0].source_url.as_deref(),
            Some("https://github.com/sparklemotion/nokogiri")
        );
    }

    #[test]
    fn github_gem_uses_git_refs_datasource() {
        let content = "gem 'rails', github: 'rails/rails'\n";
        let pkg = extract(content).unwrap();
        assert_eq!(deps(&pkg)[0].datasource, "git-refs");
        assert_eq!(
            deps(&pkg)[0].package_name.as_deref(),
            Some("https://github.com/rails/rails")
        );
    }

    #[test]
    fn path_gem_has_internal_package_skip_reason() {
        let content = "gem 'myapp', path: '../myapp'\n";
        let pkg = extract(content).unwrap();
        assert_eq!(
            deps(&pkg)[0].skip_reason.as_deref(),
            Some("internal-package")
        );
    }

    #[test]
    fn group_block_dev_deps() {
        let content = r#"
gem 'rails', '~> 7.0'
gem 'pg', '>= 0.18'

group :development, :test do
  gem 'rspec-rails'
  gem 'byebug'
end
"#;
        let pkg = extract(content).unwrap();
        let reg = regular(&pkg);
        let devs = dev(&pkg);
        assert_eq!(reg.len(), 2);
        assert_eq!(devs.len(), 2);
        assert!(devs.iter().any(|d| d.name == "rspec-rails"));
        assert!(devs.iter().any(|d| d.name == "byebug"));
    }

    #[test]
    fn development_only_group() {
        let content = r#"
group :development do
  gem 'rubocop', '~> 1.0'
  gem 'pry'
end
"#;
        let pkg = extract(content).unwrap();
        let devs = dev(&pkg);
        assert_eq!(devs.len(), 2);
        let rubocop = devs.iter().find(|d| d.name == "rubocop").unwrap();
        assert_eq!(rubocop.current_value, "'~> 1.0'");
    }

    #[test]
    fn source_line_goes_to_registry_urls_not_deps() {
        let content = r#"
source 'https://rubygems.org'
ruby '3.2.0'
gem 'rails', '~> 7.0'
"#;
        let pkg = extract(content).unwrap();
        // ruby version is now extracted as a dep
        assert_eq!(deps(&pkg).len(), 2);
        assert!(deps(&pkg).iter().any(|d| d.name == "ruby"));
        assert!(deps(&pkg).iter().any(|d| d.name == "rails"));
        assert_eq!(pkg.registry_urls, vec!["https://rubygems.org"]);
    }

    #[test]
    fn comments_skipped() {
        let content = r#"
# This is a comment
gem 'rails' # inline comment
"#;
        let pkg = extract(content).unwrap();
        assert_eq!(deps(&pkg).len(), 1);
        assert_eq!(deps(&pkg)[0].name, "rails");
    }

    #[test]
    fn double_quoted_gems() {
        let pkg = extract(r#"gem "rails", "~> 7.0""#).unwrap();
        assert_eq!(deps(&pkg).len(), 1);
        assert_eq!(deps(&pkg)[0].name, "rails");
        assert_eq!(deps(&pkg)[0].current_value, r#""~> 7.0""#);
    }

    #[test]
    fn real_world_gemfile() {
        let content = r#"
source 'https://rubygems.org'
ruby '3.2.0'

gem 'rails', '~> 7.0.4'
gem 'pg', '>= 0.18', '< 2.0'
gem 'puma', '~> 5.0'
gem 'devise', '~> 4.9'
gem 'nokogiri', git: 'https://github.com/sparklemotion/nokogiri.git'
gem 'local_gem', path: '../local_gem'

group :development, :test do
  gem 'rspec-rails', '~> 6.0'
  gem 'byebug'
  gem 'factory_bot_rails'
end

group :development do
  gem 'rubocop', '~> 1.0'
  gem 'rubocop-rails', require: false
end
"#;
        let pkg = extract(content).unwrap();
        let reg = regular(&pkg);
        let devs = dev(&pkg);

        // ruby(1) + rails, pg, puma, devise, nokogiri(git), local_gem(path) = 7 regular
        assert_eq!(reg.len(), 7);
        // rspec-rails, byebug, factory_bot_rails, rubocop, rubocop-rails = 5 dev
        assert_eq!(devs.len(), 5);

        let rails = reg.iter().find(|d| d.name == "rails").unwrap();
        assert_eq!(rails.current_value, "'~> 7.0.4'");

        let nokogiri = reg.iter().find(|d| d.name == "nokogiri").unwrap();
        assert_eq!(nokogiri.datasource, "git-refs");
        assert!(nokogiri.skip_reason.is_none());

        let local_gem = reg.iter().find(|d| d.name == "local_gem").unwrap();
        assert_eq!(local_gem.skip_reason, Some(BundlerSkipReason::InternalPackage));
    }

    #[test]
    fn empty_gemfile_returns_none() {
        // No recognizable Bundler content → None
        assert!(extract("nothing here").is_none());
        // Source only (no deps) → Some (registryUrls populated)
        assert!(extract("source 'https://rubygems.org'\n").is_some());
    }

    // ── Ported: update-locked tests ────────────────────────────────────────────

    // Ported: "detects already updated" — modules/manager/bundler/update-locked.spec.ts line 9
    #[test]
    fn bundler_update_locked_detects_already_updated() {
        let result =
            update_locked_bundler_dependency(Some("activejob"), Some("5.2.3"), Some(GEMFILE_LOCK));
        assert_eq!(result.as_str(), "already-updated");
    }

    // Ported: "returns unsupported for empty lockfile" — modules/manager/bundler/update-locked.spec.ts line 20
    #[test]
    fn bundler_update_locked_unsupported_for_no_content() {
        let result = update_locked_bundler_dependency(Some("activejob"), Some("5.2.3"), None);
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns unsupported for empty depName" — modules/manager/bundler/update-locked.spec.ts line 31
    #[test]
    fn bundler_update_locked_unsupported_for_no_dep_name() {
        let result = update_locked_bundler_dependency(None, Some("5.2.3"), Some(GEMFILE_LOCK));
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns unsupported" — modules/manager/bundler/update-locked.spec.ts line 43
    #[test]
    fn bundler_update_locked_unsupported_version_not_in_lock() {
        let result =
            update_locked_bundler_dependency(Some("activejob"), Some("5.2.0"), Some(GEMFILE_LOCK));
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns update-failed in case of errors" — modules/manager/bundler/update-locked.spec.ts line 55
    #[test]
    fn bundler_update_locked_update_failed_on_invalid_lock() {
        let result = update_locked_bundler_dependency(
            Some("activejob"),
            Some("5.2.0"),
            Some("invalid content"),
        );
        assert!(matches!(
            result,
            BundlerUpdateLockedStatus::Unsupported | BundlerUpdateLockedStatus::UpdateFailed
        ));
    }

    // ── Ported: host-rules tests ───────────────────────────────────────────────

    // Ported: "returns the authentication header with the password" — modules/manager/bundler/host-rules.spec.ts line 15
    #[test]
    fn bundler_auth_header_with_password() {
        let val = get_authentication_header_value(Some("test"), Some("password"), None);
        assert_eq!(val, "test:password");
    }

    // Ported: "returns the authentication header with the token" — modules/manager/bundler/host-rules.spec.ts line 24
    #[test]
    fn bundler_auth_header_with_token() {
        let val = get_authentication_header_value(None, None, Some("token"));
        assert_eq!(val, "token");
    }

    // Ported: "escapes special characters in the username but not the password" — modules/manager/bundler/host-rules.spec.ts line 32
    #[test]
    fn bundler_auth_header_encodes_username_at_sign() {
        let val = get_authentication_header_value(Some("test@example.com"), Some("p@ssword"), None);
        assert_eq!(val, "test%40example.com:p@ssword");
    }

    // ── Ported: locked-version tests ──────────────────────────────────────────

    // Ported: "Parse Rails Gem Lock File" — modules/manager/bundler/locked-version.spec.ts line 13
    #[test]
    fn bundler_locked_version_parse_rails() {
        let entries = extract_lock_file_entries(RAILS_LOCK);
        assert_eq!(entries.len(), 185);
    }

    // Ported: "Parse WebPacker Gem Lock File" — modules/manager/bundler/locked-version.spec.ts line 19
    #[test]
    fn bundler_locked_version_parse_webpacker() {
        let entries = extract_lock_file_entries(WEBPACKER_LOCK);
        assert_eq!(entries.len(), 53);
    }

    // Ported: "Parse Mastodon Gem Lock File" — modules/manager/bundler/locked-version.spec.ts line 25
    #[test]
    fn bundler_locked_version_parse_mastodon() {
        let entries = extract_lock_file_entries(MASTODON_LOCK);
        assert_eq!(entries.len(), 266);
    }

    // Ported: "Parse Ruby CI Gem Lock File" — modules/manager/bundler/locked-version.spec.ts line 31
    #[test]
    fn bundler_locked_version_parse_rubyci() {
        let entries = extract_lock_file_entries(GEMFILE_LOCK);
        assert_eq!(entries.len(), 64);
    }

    // Ported: "Parse Gitlab Foss Gem Lock File" — modules/manager/bundler/locked-version.spec.ts line 37
    #[test]
    fn bundler_locked_version_parse_gitlab_foss() {
        let entries = extract_lock_file_entries(GITLAB_FOSS_LOCK);
        assert_eq!(entries.len(), 478);
    }

    // Ported: "returns empty map for empty string" — modules/manager/bundler/locked-version.spec.ts line 43
    #[test]
    fn bundler_locked_version_empty_string() {
        assert!(extract_lock_file_entries("").is_empty());
    }

    // Ported: "returns empty map when errors occur" — modules/manager/bundler/locked-version.spec.ts line 47
    #[test]
    fn bundler_locked_version_invalid_input_empty() {
        assert!(extract_lock_file_entries("not a gemfile lock").is_empty());
    }

    // Ported: "strips platform suffixes from dependencies" — modules/manager/bundler/locked-version.spec.ts line 53
    #[test]
    fn bundler_locked_version_strips_platform_suffix() {
        let content = "GEM\n  remote: https://rubygems.org/\n  specs:\n    sqlite3 (2.7.4-aarch64-linux-gnu)\n    sqlite3 (2.7.4-arm64-darwin)\n    sqlite3 (2.7.4-x86_64-darwin)\n    nokogiri (1.18.10-aarch64-linux-gnu)\n      racc (~> 1.4)\n    nokogiri (1.18.10-x86_64-darwin)\n      racc (~> 1.4)\n    regular_gem (1.0.0)\n\nPLATFORMS\n  aarch64-linux-gnu\n  arm64-darwin\n  x86_64-darwin\n\nDEPENDENCIES\n  sqlite3 (>= 2.1)\n";
        let entries = extract_lock_file_entries(content);
        assert_eq!(entries.get("sqlite3"), Some(&"2.7.4".to_owned()));
        assert_eq!(entries.get("nokogiri"), Some(&"1.18.10".to_owned()));
        assert_eq!(entries.get("regular_gem"), Some(&"1.0.0".to_owned()));
    }

    // Ported: "extracts simple versions from parentheses" — modules/manager/bundler/locked-version.spec.ts line 84
    #[test]
    fn bundler_locked_version_simple_versions() {
        let content = "GEM\n  remote: https://rubygems.org/\n  specs:\n    simple_gem (1.0.0)\n    another_gem (2.3.4)\n";
        let entries = extract_lock_file_entries(content);
        assert_eq!(entries.get("simple_gem"), Some(&"1.0.0".to_owned()));
        assert_eq!(entries.get("another_gem"), Some(&"2.3.4".to_owned()));
    }

    // Ported: "extracts complex version formats from parentheses" — modules/manager/bundler/locked-version.spec.ts line 98
    #[test]
    fn bundler_locked_version_complex_versions() {
        let content = "GEM\n  remote: https://rubygems.org/\n  specs:\n    gem_with_prerelease (1.0.0.beta1)\n    gem_with_patch (1.2.3.4)\n    gem_with_alpha (2.0.0.alpha)\n";
        let entries = extract_lock_file_entries(content);
        assert_eq!(
            entries.get("gem_with_prerelease"),
            Some(&"1.0.0.beta1".to_owned())
        );
        assert_eq!(entries.get("gem_with_patch"), Some(&"1.2.3.4".to_owned()));
        assert_eq!(
            entries.get("gem_with_alpha"),
            Some(&"2.0.0.alpha".to_owned())
        );
    }

    // Ported: "correctly extracts gem names when versions contain special characters" — modules/manager/bundler/locked-version.spec.ts line 114
    #[test]
    fn bundler_locked_version_gem_names_with_special_chars() {
        let content = "GEM\n  remote: https://rubygems.org/\n  specs:\n    gem-with-dashes (1.0.0)\n    gem_with_underscores (2.0.0)\n    gem.with.dots (3.0.0)\n";
        let entries = extract_lock_file_entries(content);
        assert_eq!(entries.get("gem-with-dashes"), Some(&"1.0.0".to_owned()));
        assert_eq!(
            entries.get("gem_with_underscores"),
            Some(&"2.0.0".to_owned())
        );
        assert_eq!(entries.get("gem.with.dots"), Some(&"3.0.0".to_owned()));
    }

    // Ported: "handles gems with platform-specific versions" — modules/manager/bundler/locked-version.spec.ts line 130
    #[test]
    fn bundler_locked_version_platform_specific_versions() {
        let content = "GEM\n  remote: https://rubygems.org/\n  specs:\n    platform_gem (1.5.0-x86_64-linux)\n    another_platform_gem (2.1.0-arm64-darwin)\n\nPLATFORMS\n  x86_64-linux\n  arm64-darwin\n";
        let entries = extract_lock_file_entries(content);
        assert_eq!(entries.get("platform_gem"), Some(&"1.5.0".to_owned()));
        assert_eq!(
            entries.get("another_platform_gem"),
            Some(&"2.1.0".to_owned())
        );
    }

    // ── Ported: extract.spec.ts tests ─────────────────────────────────────────

    // Ported: "returns null for empty" — bundler/extract.spec.ts line 36
    #[test]
    fn extract_returns_none_for_empty() {
        assert!(extract("nothing here").is_none());
    }

    // Ported: "parses rails Gemfile" — bundler/extract.spec.ts line 40
    #[test]
    fn extract_parses_rails_gemfile() {
        let pkg = extract_with_lock(RAILS_GEMFILE, Some(RAILS_LOCK)).unwrap();
        assert_eq!(pkg.deps.len(), 68);
        // Deps that have lockedVersion must have a valid version string
        for dep in pkg.deps.iter().filter(|d| d.locked_version.is_some()) {
            let ver = dep.locked_version.as_ref().unwrap();
            assert!(
                ver.chars().next().is_some_and(|c| c.is_ascii_digit()),
                "invalid locked version {:?} for {}",
                ver,
                dep.name
            );
        }
    }

    // Ported: "parses sourceGroups" — bundler/extract.spec.ts line 57
    #[test]
    fn extract_parses_source_groups() {
        let pkg = extract(SOURCE_GROUP_GEMFILE).unwrap();
        assert_eq!(pkg.deps.len(), 7);
    }

    // Ported: "parse webpacker Gemfile" — bundler/extract.spec.ts line 63
    #[test]
    fn extract_parses_webpacker_gemfile() {
        let pkg = extract_with_lock(WEBPACKER_GEMFILE, Some(WEBPACKER_LOCK)).unwrap();
        assert_eq!(pkg.deps.len(), 5);
        assert!(
            pkg.deps.iter().all(|d| d.locked_version.is_some()),
            "all webpacker deps should have locked_version"
        );
    }

    // Ported: "parse mastodon Gemfile" — bundler/extract.spec.ts line 75
    #[test]
    fn extract_parses_mastodon_gemfile() {
        let pkg = extract_with_lock(MASTODON_GEMFILE, Some(MASTODON_LOCK)).unwrap();
        assert_eq!(pkg.deps.len(), 125);
    }

    // Ported: "parse Ruby CI Gemfile" — bundler/extract.spec.ts line 91
    #[test]
    fn extract_parses_rubyci_gemfile() {
        let pkg = extract_with_lock(RUBYCI_GEMFILE, Some(GEMFILE_LOCK)).unwrap();
        assert_eq!(pkg.deps.len(), 14);
        assert!(
            pkg.deps.iter().all(|d| d.locked_version.is_some()),
            "all rubyci deps should have locked_version"
        );
    }

    // Ported: "parse Gitlab Foss Gemfile" — bundler/extract.spec.ts line 104
    #[test]
    fn extract_parses_gitlab_foss_gemfile() {
        let pkg = extract_with_lock(GITLAB_FOSS_GEMFILE, Some(GITLAB_FOSS_LOCK)).unwrap();
        assert_eq!(pkg.deps.len(), 252);
        assert!(
            pkg.deps.iter().all(|d| d.locked_version.is_some()),
            "all gitlab-foss deps should have locked_version"
        );
    }

    // Ported: "parse source blocks in Gemfile" — bundler/extract.spec.ts line 116
    #[test]
    fn extract_parses_source_block_gemfile() {
        let pkg = extract(SOURCE_BLOCK_GEMFILE).unwrap();
        // Source block has 2 gems
        assert_eq!(pkg.deps.len(), 2);
        assert!(pkg.deps.iter().all(|d| !d.registry_urls.is_empty()));
    }

    // Ported: "parse source blocks with spaces in Gemfile" — bundler/extract.spec.ts line 122
    #[test]
    fn extract_parses_source_block_with_new_lines() {
        let pkg = extract_with_lock(SOURCE_BLOCK_NL_GEMFILE, Some(SOURCE_BLOCK_NL_LOCK)).unwrap();
        assert_eq!(pkg.deps.len(), 2);
    }

    // Ported: "parses source blocks with groups in Gemfile" — bundler/extract.spec.ts line 132
    #[test]
    fn extract_parses_source_block_with_groups() {
        let pkg = extract(SOURCE_BLOCK_GROUPS_GEMFILE).unwrap();
        assert_eq!(pkg.deps.len(), 4);
        let url = "https://hub.tech.my.domain.de/artifactory/api/gems/my-gems-prod-local/";
        for dep in &pkg.deps {
            assert_eq!(
                dep.registry_urls,
                vec![url],
                "dep {} missing registry url",
                dep.name
            );
        }
        assert!(
            pkg.deps
                .iter()
                .any(|d| d.name == "internal_test_gem" && d.current_value == r#""~> 1""#)
        );
        assert!(
            pkg.deps
                .iter()
                .any(|d| d.name == "internal_production_gem" && d.current_value == r#""~> 1""#)
        );
        assert!(
            pkg.deps
                .iter()
                .any(|d| d.name == "sfn_my_dep1" && d.current_value == r#""~> 1""#)
        );
        assert!(
            pkg.deps
                .iter()
                .any(|d| d.name == "sfn_my_dep2" && d.current_value == r#""~> 1""#)
        );
    }

    // Ported: "parses source variable in Gemfile" — bundler/extract.spec.ts line 146
    #[test]
    fn extract_parses_source_variable() {
        let gemfile = "foo = 'https://gems.foo.com'\nbar = 'https://gems.bar.com'\n\nsource foo\n\nsource bar do\n  gem \"some_internal_gem\"\nend\n";
        let pkg = extract(gemfile).unwrap();
        assert_eq!(pkg.registry_urls, vec!["https://gems.foo.com"]);
        assert_eq!(pkg.deps.len(), 1);
        assert_eq!(pkg.deps[0].name, "some_internal_gem");
        assert_eq!(pkg.deps[0].registry_urls, vec!["https://gems.bar.com"]);
    }

    // Ported: "parses inline source in Gemfile" — bundler/extract.spec.ts line 171
    #[test]
    fn extract_parses_inline_source() {
        let gemfile = concat!(
            "baz = 'https://gems.baz.com'\n",
            "gem 'inline_gem'\n",
            "gem \"inline_source_gem\", source: 'https://gems.foo.com'\n",
            "gem 'inline_source_gem_with_version', \"~> 1\", source: 'https://gems.bar.com'\n",
            "gem 'inline_source_gem_with_variable_source', source: baz\n",
            "gem 'inline_source_gem_with_require_after', source: 'https://gems.foo.com', require: false\n",
        );
        let pkg = extract(gemfile).unwrap();
        let find = |name: &str| pkg.deps.iter().find(|d| d.name == name).unwrap();
        assert!(find("inline_gem").registry_urls.is_empty());
        assert_eq!(
            find("inline_source_gem").registry_urls,
            vec!["https://gems.foo.com"]
        );
        assert_eq!(
            find("inline_source_gem_with_version").registry_urls,
            vec!["https://gems.bar.com"]
        );
        assert_eq!(
            find("inline_source_gem_with_variable_source").registry_urls,
            vec!["https://gems.baz.com"]
        );
    }

    // Ported: "parses git refs in Gemfile" — bundler/extract.spec.ts line 223
    #[test]
    fn extract_parses_git_refs() {
        let gemfile = concat!(
            "gem 'foo', git: 'https://github.com/foo/foo', ref: 'fd184883048b922b176939f851338d0a4971a532'\n",
            "gem 'bar', git: 'https://github.com/bar/bar', tag: 'v1.0.0'\n",
            "gem 'baz', github: 'baz/baz', branch: 'master'\n",
        );
        let pkg = extract(gemfile).unwrap();
        assert_eq!(pkg.deps.len(), 3);

        let foo = pkg.deps.iter().find(|d| d.name == "foo").unwrap();
        assert_eq!(foo.datasource, "git-refs");
        assert_eq!(
            foo.package_name.as_deref(),
            Some("https://github.com/foo/foo")
        );
        assert_eq!(
            foo.source_url.as_deref(),
            Some("https://github.com/foo/foo")
        );
        assert_eq!(
            foo.current_digest.as_deref(),
            Some("fd184883048b922b176939f851338d0a4971a532")
        );

        let bar = pkg.deps.iter().find(|d| d.name == "bar").unwrap();
        assert_eq!(bar.datasource, "git-refs");
        assert_eq!(
            bar.package_name.as_deref(),
            Some("https://github.com/bar/bar")
        );
        assert_eq!(bar.current_value, "v1.0.0");

        let baz = pkg.deps.iter().find(|d| d.name == "baz").unwrap();
        assert_eq!(baz.datasource, "git-refs");
        assert_eq!(
            baz.package_name.as_deref(),
            Some("https://github.com/baz/baz")
        );
        assert_eq!(
            baz.source_url.as_deref(),
            Some("https://github.com/baz/baz")
        );
        assert_eq!(baz.current_value, "master");
    }

    // Ported: "parses multiple current values Gemfile" — bundler/extract.spec.ts line 259
    #[test]
    fn extract_parses_multiple_current_values() {
        let gemfile = concat!(
            "gem 'gem_without_values'\n",
            "gem 'gem_with_one_value', \">= 3.0.5\"\n",
            "gem 'gem_with_multiple_values', \">= 3.0.5\", \"< 3.2\"\n",
        );
        let pkg = extract(gemfile).unwrap();
        assert_eq!(pkg.deps.len(), 3);

        let find = |name: &str| pkg.deps.iter().find(|d| d.name == name).unwrap();
        assert!(find("gem_without_values").current_value.is_empty());
        assert_eq!(find("gem_with_one_value").current_value, r#"">= 3.0.5""#);
        assert_eq!(
            find("gem_with_multiple_values").current_value,
            r#"">= 3.0.5", "< 3.2""#
        );
    }

    // ── findAllAuthenticatable ───────────────────────────────────────────────

    fn make_rule(
        host_type: &str,
        match_host: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
        token: Option<&str>,
    ) -> crate::util::host_rules::HostRule {
        crate::util::host_rules::HostRule {
            host_type: Some(host_type.to_owned()),
            match_host: match_host.map(str::to_owned),
            resolved_host: match_host.map(str::to_owned),
            username: username.map(str::to_owned),
            password: password.map(str::to_owned),
            token: token.map(str::to_owned),
            ..Default::default()
        }
    }

    // Ported: "returns an empty array if matchHost is missing" — bundler/host-rules.spec.ts line 55
    #[test]
    fn find_all_authenticatable_empty_if_no_match_host() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(crate::util::host_rules::HostRule {
            host_type: Some("bundler".to_owned()),
            resolved_host: None,
            username: Some("user".to_owned()),
            password: Some("pass".to_owned()),
            ..Default::default()
        })
        .unwrap();
        assert!(find_all_authenticatable("bundler").is_empty());
        crate::util::host_rules::clear();
    }

    // Ported: "returns an empty array if username is missing and password is present" — bundler/host-rules.spec.ts line 63
    #[test]
    fn find_all_authenticatable_empty_if_no_username() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(make_rule(
            "bundler",
            Some("example.com"),
            None,
            Some("pass"),
            None,
        ))
        .unwrap();
        assert!(find_all_authenticatable("bundler").is_empty());
        crate::util::host_rules::clear();
    }

    // Ported: "returns an empty array if password and token are missing" — bundler/host-rules.spec.ts line 73
    #[test]
    fn find_all_authenticatable_empty_if_no_credentials() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(make_rule(
            "bundler",
            Some("example.com"),
            Some("user"),
            None,
            None,
        ))
        .unwrap();
        assert!(find_all_authenticatable("bundler").is_empty());
        crate::util::host_rules::clear();
    }

    // Ported: "returns the hostRule if using matchHost and password" — bundler/host-rules.spec.ts line 83
    #[test]
    fn find_all_authenticatable_returns_rule_with_match_host_and_password() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(make_rule(
            "bundler",
            Some("example.com"),
            Some("user"),
            Some("pass"),
            None,
        ))
        .unwrap();
        let result = find_all_authenticatable("bundler");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].resolved_host.as_deref(), Some("example.com"));
        crate::util::host_rules::clear();
    }

    // Ported: "returns the hostRule if using matchHost and token" — bundler/host-rules.spec.ts line 92
    #[test]
    fn find_all_authenticatable_returns_rule_with_match_host_and_token() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(make_rule(
            "bundler",
            Some("example.com"),
            None,
            None,
            Some("token123"),
        ))
        .unwrap();
        let result = find_all_authenticatable("bundler");
        assert_eq!(result.len(), 1);
        crate::util::host_rules::clear();
    }

    // Ported: "returns the hostRule if using baseUrl and password" — bundler/host-rules.spec.ts line 101
    #[test]
    fn find_all_authenticatable_returns_rule_with_base_url_and_password() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(make_rule(
            "bundler",
            Some("https://example.com"),
            Some("user"),
            Some("pass"),
            None,
        ))
        .unwrap();
        let result = find_all_authenticatable("bundler");
        assert_eq!(result.len(), 1);
        crate::util::host_rules::clear();
    }

    // Ported: "returns the hostRule if using baseUrl and token" — bundler/host-rules.spec.ts line 110
    #[test]
    fn find_all_authenticatable_returns_rule_with_base_url_and_token() {
        crate::util::host_rules::clear();
        crate::util::host_rules::add(make_rule(
            "bundler",
            Some("https://example.com"),
            None,
            None,
            Some("token"),
        ))
        .unwrap();
        let result = find_all_authenticatable("bundler");
        assert_eq!(result.len(), 1);
        crate::util::host_rules::clear();
    }

    // Ported: "skips local gems in Gemfile" — bundler/extract.spec.ts line 284
    #[test]
    fn extract_skips_local_gems() {
        let gemfile = "gem 'foo', path: 'vendor/foo'\ngem 'bar'\n";
        let pkg = extract(gemfile).unwrap();
        assert_eq!(pkg.deps.len(), 2);
        let foo = pkg.deps.iter().find(|d| d.name == "foo").unwrap();
        assert_eq!(foo.skip_reason, Some(BundlerSkipReason::InternalPackage));
        let bar = pkg.deps.iter().find(|d| d.name == "bar").unwrap();
        assert!(bar.skip_reason.is_none());
    }
}
