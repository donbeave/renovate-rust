//! Swift Package Manager `Package.swift` dependency extractor.
//!
//! Implements the same state-machine parser as the TypeScript reference:
//! - `lib/modules/manager/swift/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/swift/range.ts`   — `getRangeStrategy`
//! - `lib/modules/manager/swift/index.ts`   — pattern `/(^|/)Package\\.swift/`

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Swift Package dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpmDep {
    /// Dependency name (`owner/repo` for GitHub/GitLab, full URL for git-tags).
    pub dep_name: String,
    /// Current value string (verbatim from Package.swift, e.g. `from: "1.0.0"`, `"1.0.0"..."2.0.0"`).
    pub current_value: String,
    /// Renovate datasource (`github-tags`, `gitlab-tags`, `git-tags`).
    pub datasource: &'static str,
    /// Set for self-hosted GitHub/GitLab instances.
    pub registry_urls: Option<Vec<String>>,
}

// ── Token regexes (mirror the TypeScript regExps map) ─────────────────────────

static RE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)(\s+|//[^\n]*|/\*.*?\*/)+").unwrap());
static RE_DEPS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"dependencies").unwrap());
static RE_COLON: LazyLock<Regex> = LazyLock::new(|| Regex::new(r":").unwrap());
static RE_BEGIN_SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[").unwrap());
static RE_END_SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"],?").unwrap());
static RE_PACKAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s*.\s*package\s*\(\s*").unwrap());
static RE_URL_KEY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"url").unwrap());
static RE_STRING_LITERAL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""[^"]+""#).unwrap());
static RE_COMMA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r",").unwrap());
static RE_FROM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"from").unwrap());
static RE_RANGE_OP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.\.[.<]").unwrap());
static RE_EXACT_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\.\s*exact\s*\(\s*").unwrap());
static RE_EXACT_VERSION_LABEL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s*exact:").unwrap());
static RE_TRAITS_LABEL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*traits\s*:").unwrap());
// Line-scoped greedy consume: matches up to (and including) the next `.package(` on the SAME line.
// Mirrors TypeScript: `regEx(/.*\.\s*package\s*\(\s*/)` — NO dotAll flag, so `.*` stops at newlines.
static RE_TRAITS_CONSUME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r".*\.\s*package\s*\(\s*").unwrap());

/// Token labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Space,
    Deps,
    Colon,
    BeginSection,
    EndSection,
    Package,
    UrlKey,
    StringLiteral,
    Comma,
    From,
    RangeOp,
    ExactVersion,
    ExactVersionLabel,
    TraitsLabel,
    TraitsConsume,
}

/// Parser state string (mirrors TypeScript state strings).
#[derive(Debug, Clone, PartialEq)]
enum State {
    Scan,
    Dependencies,
    DependenciesColon,
    DependenciesSection,
    PackageCall,
    PackageUrl,
    PackageUrlColon,
    PackageDepName,
    PackageDepNameComma,
    PackageExact,
    PackageExactLabel,
    PackageFrom,
    PackageFromColon,
    PackageValue,
    PackageRangeFromRangeOp,
    Traits,
}

/// A candidate regex match.
struct Match<'a> {
    idx: usize,
    len: usize,
    token: Token,
    substr: &'a str,
}

/// Try each (token, regex) pair against `s`. Return the earliest match.
/// If a match is at idx=0 it is returned immediately (mirrors TypeScript early-return).
fn get_match<'a>(s: &'a str, candidates: &[(Token, &LazyLock<Regex>)]) -> Option<Match<'a>> {
    let mut best: Option<Match<'a>> = None;
    for &(token, re) in candidates {
        if let Some(m) = re.find(s) {
            let idx = m.start();
            let len = m.end() - m.start();
            if idx == 0 {
                return Some(Match {
                    idx,
                    len,
                    token,
                    substr: &s[..len],
                });
            }
            if best.as_ref().is_none_or(|b| idx < b.idx) {
                best = Some(Match {
                    idx,
                    len,
                    token,
                    substr: &s[idx..idx + len],
                });
            }
        }
    }
    best
}

fn candidates_for(state: &State) -> Vec<(Token, &'static LazyLock<Regex>)> {
    match state {
        State::Scan => vec![(Token::Deps, &RE_DEPS)],
        State::Dependencies => vec![(Token::Space, &RE_SPACE), (Token::Colon, &RE_COLON)],
        State::DependenciesColon => vec![
            (Token::Space, &RE_SPACE),
            (Token::BeginSection, &RE_BEGIN_SECTION),
        ],
        State::DependenciesSection => vec![
            (Token::Space, &RE_SPACE),
            (Token::Package, &RE_PACKAGE),
            (Token::Comma, &RE_COMMA),
            (Token::TraitsLabel, &RE_TRAITS_LABEL),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageCall => vec![
            (Token::Space, &RE_SPACE),
            (Token::UrlKey, &RE_URL_KEY),
            (Token::Package, &RE_PACKAGE),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageUrl => vec![
            (Token::Space, &RE_SPACE),
            (Token::Colon, &RE_COLON),
            (Token::Package, &RE_PACKAGE),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageUrlColon => vec![
            (Token::Space, &RE_SPACE),
            (Token::StringLiteral, &RE_STRING_LITERAL),
            (Token::Package, &RE_PACKAGE),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageDepName => vec![
            (Token::Space, &RE_SPACE),
            (Token::Comma, &RE_COMMA),
            (Token::Package, &RE_PACKAGE),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageDepNameComma => vec![
            (Token::Space, &RE_SPACE),
            (Token::From, &RE_FROM),
            (Token::StringLiteral, &RE_STRING_LITERAL),
            (Token::RangeOp, &RE_RANGE_OP),
            (Token::ExactVersion, &RE_EXACT_VERSION),
            (Token::ExactVersionLabel, &RE_EXACT_VERSION_LABEL),
            (Token::Package, &RE_PACKAGE),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageExact | State::PackageExactLabel => vec![
            (Token::Space, &RE_SPACE),
            (Token::StringLiteral, &RE_STRING_LITERAL),
            (Token::Package, &RE_PACKAGE),
            (Token::Comma, &RE_COMMA),
            (Token::TraitsLabel, &RE_TRAITS_LABEL),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageFrom => vec![
            (Token::Space, &RE_SPACE),
            (Token::Colon, &RE_COLON),
            (Token::Package, &RE_PACKAGE),
            (Token::Comma, &RE_COMMA),
            (Token::TraitsLabel, &RE_TRAITS_LABEL),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageFromColon => vec![
            (Token::Space, &RE_SPACE),
            (Token::StringLiteral, &RE_STRING_LITERAL),
            (Token::Package, &RE_PACKAGE),
            (Token::Comma, &RE_COMMA),
            (Token::TraitsLabel, &RE_TRAITS_LABEL),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageValue => vec![
            (Token::Space, &RE_SPACE),
            (Token::RangeOp, &RE_RANGE_OP),
            (Token::Package, &RE_PACKAGE),
            (Token::Comma, &RE_COMMA),
            (Token::TraitsLabel, &RE_TRAITS_LABEL),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::PackageRangeFromRangeOp => vec![
            (Token::Space, &RE_SPACE),
            (Token::StringLiteral, &RE_STRING_LITERAL),
            (Token::Package, &RE_PACKAGE),
            (Token::Comma, &RE_COMMA),
            (Token::TraitsLabel, &RE_TRAITS_LABEL),
            (Token::EndSection, &RE_END_SECTION),
        ],
        State::Traits => vec![(Token::TraitsConsume, &RE_TRAITS_CONSUME)],
    }
}

// ── URL parsing (mirrors TypeScript `parseUrl`) ────────────────────────────────

struct ParsedUrl {
    dep_name: String,
    datasource: &'static str,
    registry_urls: Option<Vec<String>>,
}

fn parse_url(url: &str) -> Option<ParsedUrl> {
    let (host, path_owned, scheme_str);

    if let Some(rest) = url.strip_prefix("git@") {
        // SCP-style: git@host:owner/repo.git
        let (h, p) = rest.split_once(':')?;
        host = h.to_owned();
        path_owned = format!("/{p}");
        scheme_str = "https".to_owned();
    } else if let Some(rest) = url.strip_prefix("ssh://") {
        // ssh://[user@]host/path
        let after_at = rest.find('@').map_or(rest, |i| &rest[i + 1..]);
        if after_at.is_empty() {
            return None;
        }
        let slash = after_at.find('/')?;
        let h = &after_at[..slash];
        if h.is_empty() {
            return None;
        }
        host = h.to_owned();
        path_owned = after_at[slash..].to_owned();
        scheme_str = "https".to_owned();
    } else if let Some(r) = url.strip_prefix("https://") {
        let slash = r.find('/')?;
        host = r[..slash].to_owned();
        path_owned = r[slash..].to_owned();
        scheme_str = "https".to_owned();
    } else if let Some(r) = url.strip_prefix("http://") {
        let slash = r.find('/')?;
        host = r[..slash].to_owned();
        path_owned = r[slash..].to_owned();
        scheme_str = "http".to_owned();
    } else {
        return None;
    }

    let path = &path_owned;
    let scheme = scheme_str.as_str();

    let (platform, is_public) = if host == "github.com" {
        ("github", true)
    } else if host == "gitlab.com" {
        ("gitlab", true)
    } else if host.contains("github") {
        ("github", false)
    } else if host.contains("gitlab") {
        ("gitlab", false)
    } else {
        return Some(ParsedUrl {
            dep_name: url.to_owned(),
            datasource: "git-tags",
            registry_urls: None,
        });
    };

    let stripped = path.trim_start_matches('/').trim_end_matches('/');
    let parts: Vec<&str> = stripped.splitn(3, '/').collect();
    if parts.len() < 2 || parts[0].is_empty() {
        return None;
    }
    let owner = parts[0];
    let repo = parts[1].trim_end_matches(".git").trim_end_matches('/');
    if repo.is_empty() {
        return None;
    }

    let dep_name = format!("{owner}/{repo}");
    let datasource = if platform == "github" {
        "github-tags"
    } else {
        "gitlab-tags"
    };
    let registry_urls = if !is_public {
        Some(vec![format!("{scheme}://{host}")])
    } else {
        None
    };

    Some(ParsedUrl {
        dep_name,
        datasource,
        registry_urls,
    })
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Package.swift` file and extract all package dependencies.
///
/// Returns `None` for empty content or files with no extractable dependencies.
/// Mirrors TypeScript `extractPackageFile` from `lib/modules/manager/swift/extract.ts`.
// pkg_url/current_value are reset to None after each yield even when the next
// write happens before the next read — intentional to mirror TypeScript closure resets.
#[allow(unused_assignments)]
pub fn extract_package_file(content: &str) -> Option<Vec<SpmDep>> {
    if content.is_empty() {
        return None;
    }

    let mut deps: Vec<SpmDep> = Vec::new();
    // Mirrors TypeScript closure vars `packageName` and `currentValue`.
    let mut pkg_url: Option<String> = None;
    let mut current_value: Option<String> = None;

    let mut state = State::Scan;
    let mut rest = content;

    // Mirrors TypeScript `yieldDep()`.
    let yield_dep = |url: &Option<String>, val: &Option<String>, deps: &mut Vec<SpmDep>| {
        if let (Some(u), Some(v)) = (url.as_deref(), val.as_deref())
            && let Some(parsed) = parse_url(u)
            && !v.is_empty()
        {
            deps.push(SpmDep {
                dep_name: parsed.dep_name,
                current_value: v.to_owned(),
                datasource: parsed.datasource,
                registry_urls: parsed.registry_urls,
            });
        }
    };

    loop {
        // TypeScript: if (deps.length) break in null/Scan state.
        if state == State::Scan && !deps.is_empty() {
            break;
        }

        let candidates = candidates_for(&state);
        let Some(m) = get_match(rest, &candidates) else {
            // No match: attempt recovery or stop.
            match state {
                State::Dependencies | State::DependenciesColon => {
                    state = State::Scan;
                    if rest.is_empty() {
                        break;
                    }
                    let next = rest.char_indices().nth(1).map_or(rest.len(), |(i, _)| i);
                    rest = &rest[next..];
                }
                _ => break,
            }
            continue;
        };

        rest = &rest[m.idx + m.len..];

        match state {
            State::Scan => {
                // Only DEPS token is in candidates.
                if m.token == Token::Deps {
                    state = State::Dependencies;
                }
            }

            State::Dependencies => match m.token {
                Token::Colon => state = State::DependenciesColon,
                Token::Space => {}
                _ => state = State::Scan,
            },

            State::DependenciesColon => match m.token {
                Token::BeginSection => state = State::DependenciesSection,
                Token::Space => {}
                _ => state = State::Scan,
            },

            State::DependenciesSection => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                Token::TraitsLabel => {
                    state = State::Traits;
                }
                _ => {}
            },

            State::PackageCall => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::UrlKey => state = State::PackageUrl,
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                }
                _ => {}
            },

            State::PackageUrl => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::Colon => state = State::PackageUrlColon,
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageUrlColon => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::StringLiteral => {
                    // Strip surrounding quotes.
                    pkg_url = Some(m.substr[1..m.substr.len() - 1].to_owned());
                    state = State::PackageDepName;
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageDepName => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::Comma => state = State::PackageDepNameComma,
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageDepNameComma => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::From => {
                    current_value = Some(m.substr.to_owned());
                    state = State::PackageFrom;
                }
                Token::StringLiteral => {
                    current_value = Some(m.substr.to_owned());
                    state = State::PackageValue;
                }
                Token::RangeOp => {
                    current_value = Some(m.substr.to_owned());
                    state = State::PackageRangeFromRangeOp;
                }
                Token::ExactVersion => {
                    state = State::PackageExact;
                }
                Token::ExactVersionLabel => {
                    state = State::PackageExactLabel;
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageExact => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::StringLiteral => {
                    current_value = Some(m.substr[1..m.substr.len() - 1].to_owned());
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    // Stay in PackageExact (TypeScript doesn't change state here).
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageExactLabel => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::StringLiteral => {
                    current_value = Some(m.substr[1..m.substr.len() - 1].to_owned());
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::DependenciesSection;
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageFrom => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::Colon => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                    state = State::PackageFromColon;
                }
                Token::Space => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageFromColon => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::StringLiteral => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::DependenciesSection;
                }
                Token::Space => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                _ => {}
            },

            State::PackageValue => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::RangeOp => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                    state = State::PackageRangeFromRangeOp;
                }
                Token::Space => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                Token::Comma | Token::TraitsLabel => {
                    // For comma: if next is traits, we'll handle it in DepsSection
                    // For traits: go to Traits state (pending dep carried via pkg_url/current_value)
                    if m.token == Token::TraitsLabel {
                        state = State::Traits;
                    } else {
                        state = State::DependenciesSection;
                    }
                }
                _ => {}
            },

            State::PackageRangeFromRangeOp => match m.token {
                Token::EndSection => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    break;
                }
                Token::StringLiteral => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                    // Go to DepsSection with pending dep (yield on next PACKAGE or END_SECTION).
                    state = State::DependenciesSection;
                }
                Token::Space => {
                    if let Some(v) = current_value.as_mut() {
                        v.push_str(m.substr);
                    }
                }
                Token::Package => {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
                Token::Comma | Token::TraitsLabel => {
                    if m.token == Token::TraitsLabel {
                        state = State::Traits;
                    } else {
                        state = State::DependenciesSection;
                    }
                }
                _ => {}
            },

            State::Traits => {
                if m.token == Token::TraitsConsume {
                    yield_dep(&pkg_url, &current_value, &mut deps);
                    pkg_url = None;
                    current_value = None;
                    state = State::PackageCall;
                }
            }
        }
    }

    if deps.is_empty() { None } else { Some(deps) }
}

// ── Determine the effective Swift (SPM) range strategy ────────────────────────

/// Mirrors `lib/modules/manager/swift/range.ts` `getRangeStrategy()`.
pub fn get_range_strategy(range_strategy: &str) -> &str {
    if range_strategy == "auto" {
        "bump"
    } else {
        range_strategy
    }
}

// ── Legacy API (kept for existing non-ported tests) ───────────────────────────

/// Why a Swift dep is skipped (legacy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpmSkipReason {
    LocalPath,
    NonGitHost,
}

/// Which Git hosting service (legacy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHost {
    GitHub,
    GitLab,
}

/// Legacy dep struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpmExtractedDep {
    pub owner_repo: String,
    pub current_value: String,
    pub git_host: Option<GitHost>,
    pub skip_reason: Option<SpmSkipReason>,
}

/// Legacy extract function.
pub fn extract(content: &str) -> Vec<SpmExtractedDep> {
    extract_package_file(content).map_or_else(Vec::new, |deps| {
        deps.into_iter()
            .map(|d| {
                let (git_host, skip_reason) = match d.datasource {
                    "github-tags" => (Some(GitHost::GitHub), None),
                    "gitlab-tags" => (Some(GitHost::GitLab), None),
                    _ => (None, Some(SpmSkipReason::NonGitHost)),
                };
                SpmExtractedDep {
                    owner_repo: d.dep_name,
                    current_value: d.current_value,
                    git_host,
                    skip_reason,
                }
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Ported from swift/extract.spec.ts ────────────────────────────────────

    // Ported: "returns null for empty content" — swift/extract.spec.ts line 7
    #[test]
    fn returns_null_for_empty_content() {
        assert!(extract_package_file("").is_none());
    }

    // Ported: "returns null for content without dependencies" — swift/extract.spec.ts line 11
    #[test]
    fn returns_null_for_content_without_dependencies() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          products: [
            .library(name: "MyLibrary", targets: ["MyLibrary"])
          ],
          targets: [.target(name: "MyLibrary")]
        )
      "#;
        assert!(extract_package_file(content).is_none());
    }

    // Ported: "extracts GitHub dependencies with github-tags datasource" — swift/extract.spec.ts line 31
    #[test]
    fn extracts_github_dependencies_with_github_tags_datasource() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/repo", from: "1.0.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, r#"from: "1.0.0""#);
        assert!(deps[0].registry_urls.is_none());
    }

    // Ported: "extracts GitLab dependencies with gitlab-tags datasource" — swift/extract.spec.ts line 52
    #[test]
    fn extracts_gitlab_dependencies_with_gitlab_tags_datasource() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://gitlab.com/example/repo", from: "2.0.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "gitlab-tags");
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, r#"from: "2.0.0""#);
    }

    // Ported: "extracts self-hosted GitHub dependencies with registryUrls" — swift/extract.spec.ts line 73
    #[test]
    fn extracts_self_hosted_github_with_registry_urls() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.example.com/org/repo", from: "1.0.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].dep_name, "org/repo");
        assert_eq!(
            deps[0].registry_urls,
            Some(vec!["https://github.example.com".to_owned()])
        );
    }

    // Ported: "extracts self-hosted GitLab dependencies with registryUrls" — swift/extract.spec.ts line 95
    #[test]
    fn extracts_self_hosted_gitlab_with_registry_urls() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://gitlab.mycompany.com/group/project.git", from: "2.5.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps[0].datasource, "gitlab-tags");
        assert_eq!(deps[0].dep_name, "group/project");
        assert_eq!(
            deps[0].registry_urls,
            Some(vec!["https://gitlab.mycompany.com".to_owned()])
        );
    }

    // Ported: "extracts other dependencies with git-tags datasource" — swift/extract.spec.ts line 192
    #[test]
    fn extracts_other_dependencies_with_git_tags_datasource() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://example.com/repo.git", from: "3.0.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps[0].datasource, "git-tags");
        assert_eq!(deps[0].dep_name, "https://example.com/repo.git");
        assert_eq!(deps[0].current_value, r#"from: "3.0.0""#);
    }

    // Ported: "extracts exact version dependencies" — swift/extract.spec.ts line 213
    #[test]
    fn extracts_exact_version_dependencies() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/repo", .exact("1.2.3"))
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, "1.2.3");
    }

    // Ported: "extracts exact version with label syntax" — swift/extract.spec.ts line 234
    #[test]
    fn extracts_exact_version_with_label_syntax() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/repo", exact: "1.2.1")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, "1.2.1");
    }

    // Ported: "extracts range version dependencies" — swift/extract.spec.ts line 255
    #[test]
    fn extracts_range_version_dependencies() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/repo", "1.0.0"..."2.0.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, r#""1.0.0"..."2.0.0""#);
    }

    // Ported: "extracts dependencies from sample package file" — swift/extract.spec.ts line 276
    #[test]
    fn extracts_dependencies_from_sample_package_file() {
        let content = include_str!("../../tests/fixtures/spm/SamplePackage.swift");
        let deps = extract_package_file(content).unwrap();

        assert_eq!(
            deps.iter()
                .filter(|d| d.datasource == "github-tags")
                .count(),
            10
        );
        assert_eq!(deps.len(), 10);

        // CountedSet uses .branch("master") → currentValue starts with `"master"`
        assert!(deps
            .iter()
            .any(|d| d.dep_name == "0x7fs/CountedSet" && d.current_value.trim() == "\"master\""));

        assert!(
            deps.iter()
                .any(|d| d.dep_name == "avito-tech/GraphiteClient" && d.current_value == "0.1.0")
        );

        assert!(
            deps.iter()
                .any(|d| d.dep_name == "apple/swift-argument-parser" && d.current_value == "1.2.1")
        );

        // ZIPFoundation uses `from : "0.9.6"` with space and block comment
        assert!(deps.iter().any(|d| d.dep_name == "weichsel/ZIPFoundation"
            && d.current_value.starts_with("from")
            && d.current_value.contains("0.9.6")));
    }

    // Ported: "handles malformed URLs gracefully" — swift/extract.spec.ts line 311
    #[test]
    fn handles_malformed_urls_gracefully() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "not-a-valid-url", from: "1.0.0")
          ]
        )
      "#;
        assert!(extract_package_file(content).is_none());
    }

    // Ported: "handles dependencies without version" — swift/extract.spec.ts line 324
    #[test]
    fn handles_dependencies_without_version() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/repo")
          ]
        )
      "#;
        assert!(extract_package_file(content).is_none());
    }

    // Ported: "handles dependencies with local package" — swift/extract.spec.ts line 337
    #[test]
    fn handles_dependencies_with_local_package() {
        let content = r#"let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(path: "../LocalPackage")
          ]
        )"#;
        assert!(extract_package_file(content).is_none());
    }

    // Ported: "handles dependencies with name (deprecated args)" — swift/extract.spec.ts line 350
    #[test]
    fn handles_dependencies_with_name_deprecated_args() {
        let content = r#"let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(name: "repo", url: "https://github.com/example/repo", from: "1.0.0")
          ]
        )"#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, r#"from: "1.0.0""#);
    }

    // Ported: "extracts multiple dependencies with different datasources" — swift/extract.spec.ts line 365
    #[test]
    fn extracts_multiple_dependencies_with_different_datasources() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/github-repo", from: "1.0.0"),
            .package(url: "https://gitlab.com/example/gitlab-repo", from: "2.0.0"),
            .package(url: "https://example.com/other-repo.git", from: "3.0.0")
          ]
        )
      "#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[1].datasource, "gitlab-tags");
        assert_eq!(deps[2].datasource, "git-tags");
    }

    // Ported: "extracts multiple dependencies with traits arguments" — swift/extract.spec.ts line 383
    #[test]
    fn extracts_multiple_dependencies_with_traits_arguments() {
        let content = r#"let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "https://github.com/example/repo1", from: "1.0.0", traits: []),
            .package(url: "https://github.com/example/repo2", "2.0.0"..<"3.0.0", traits: [.defaults]),
            .package(
              url: "https://github.com/example/repo3",
              .exact("4.0.0"),
              traits: [
                .trait(name: "CUSTOM_TRAIT"),
                .trait(name: "ANOTHER_TRAIT")
              ]
            )
          ]
        )"#;
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "example/repo1");
        assert_eq!(deps[0].current_value, r#"from: "1.0.0""#);
        assert_eq!(deps[1].dep_name, "example/repo2");
        assert_eq!(deps[1].current_value, r#""2.0.0"..<"3.0.0""#);
        assert_eq!(deps[2].dep_name, "example/repo3");
        assert_eq!(deps[2].current_value, "4.0.0");
    }

    // ── Range strategy (ported from swift/range.spec.ts) ─────────────────────

    // Ported: "returns same if not auto" — modules/manager/swift/range.spec.ts line 6
    #[test]
    fn swift_range_returns_same_if_not_auto() {
        assert_eq!(get_range_strategy("widen"), "widen");
    }

    // Ported: "defaults to update-lockfile" — modules/manager/swift/range.spec.ts line 11
    #[test]
    fn swift_range_defaults_to_bump() {
        assert_eq!(get_range_strategy("auto"), "bump");
    }

    // ── Legacy non-ported tests ───────────────────────────────────────────────

    // Rust-specific: spm behavior test
    #[test]
    fn from_version_legacy() {
        let content = r#"
let package = Package(
    dependencies: [
        .package(url: "https://github.com/apple/swift-log.git", from: "1.4.4"),
    ]
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].owner_repo, "apple/swift-log");
        assert_eq!(deps[0].git_host, Some(GitHost::GitHub));
        assert!(deps[0].skip_reason.is_none());
    }

    // Rust-specific: spm behavior test
    #[test]
    fn non_github_url_returns_git_tags_legacy() {
        let content = r#"
let package = Package(
    dependencies: [
        .package(url: "https://example.com/repo.git", from: "1.0.0"),
    ]
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(SpmSkipReason::NonGitHost));
    }

    // Rust-specific: spm behavior test
    #[test]
    fn gitlab_url_legacy() {
        let content = r#"
let package = Package(
    dependencies: [
        .package(url: "https://gitlab.com/myorg/mypackage.git", from: "2.0.0"),
    ]
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].git_host, Some(GitHost::GitLab));
        assert_eq!(deps[0].owner_repo, "myorg/mypackage");
        assert!(deps[0].skip_reason.is_none());
    }

    // Rust-specific: spm behavior test
    #[test]
    fn no_packages_returns_empty_legacy() {
        let content = "// just a comment\nlet x = 1\n";
        assert!(extract(content).is_empty());
    }

    // ── Ported from swift/index.spec.ts ──────────────────────────────────────

    // Ported: "returns null for empty content" — swift/index.spec.ts line 6
    #[test]
    fn index_returns_null_for_empty_and_no_package_calls() {
        assert!(extract_package_file("").is_none());
        assert!(extract_package_file("dependencies:[]").is_none());
        assert!(extract_package_file(r#"dependencies:["foobar"]"#).is_none());
    }

    // Ported: "returns null for invalid content" — swift/index.spec.ts line 12
    #[test]
    fn index_returns_null_for_invalid_content() {
        assert!(extract_package_file("dependen").is_none());
        assert!(extract_package_file("dependencies!: ").is_none());
        assert!(extract_package_file("dependencies :").is_none());
        assert!(extract_package_file("dependencies...").is_none());
        assert!(extract_package_file("dependencies:!").is_none());
        assert!(extract_package_file("dependencies:[").is_none());
        assert!(extract_package_file("dependencies:[...").is_none());
        assert!(extract_package_file("dependencies:[]").is_none());
        assert!(extract_package_file("dependencies:[.package").is_none());
        assert!(extract_package_file("dependencies:[.package.package(").is_none());
        assert!(extract_package_file("dependencies:[.package(asdf").is_none());
        assert!(extract_package_file("dependencies:[.package]").is_none());
        assert!(extract_package_file("dependencies:[.package(]").is_none());
        assert!(extract_package_file("dependencies:[.package(.package(").is_none());
        assert!(extract_package_file("dependencies:[.package(").is_none());
        assert!(extract_package_file("dependencies:[.package(]").is_none());
        assert!(extract_package_file("dependencies:[.package(url],").is_none());
        assert!(extract_package_file("dependencies:[.package(url.package(]").is_none());
        assert!(extract_package_file("dependencies:[.package(url:.package(").is_none());
        assert!(extract_package_file("dependencies:[.package(url:]").is_none());
        assert!(extract_package_file(r#"dependencies:[.package(url:"fo"#).is_none());
        assert!(extract_package_file(r#"dependencies:[.package(url:"fo]"#).is_none());
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://example.com/something.git"]"#
            )
            .is_none()
        );
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git"]"#
            )
            .is_none()
        );
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git".package(]"#
            )
            .is_none()
        );
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git", ]"#
            )
            .is_none()
        );
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git", .package(]"#
            )
            .is_none()
        );
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git", .exact(]"#
            )
            .is_none()
        );
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git", exact:]"#
            )
            .is_none()
        );
        assert!(extract_package_file(
            r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git", exact:.package()]"#
        )
        .is_none());
    }

    // Ported: "parses packages with invalid versions" — swift/index.spec.ts line 81
    #[test]
    fn index_parses_packages_with_invalid_versions() {
        let base = r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git", "#;
        assert!(extract_package_file(&format!("{base}from]")).is_some());
        assert!(extract_package_file(&format!("{base}from.package(")).is_some());
        assert!(extract_package_file(&format!("{base}from:]")).is_some());
        assert!(extract_package_file(&format!("{base}from:.package(")).is_some());
        assert!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git","1.2.3")]"#
            )
            .is_some()
        );
    }

    // Ported: "parses package descriptions" — swift/index.spec.ts line 109
    #[test]
    fn index_parses_package_descriptions() {
        let base = r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git","#;
        assert_eq!(
            extract_package_file(
                r#"dependencies:[.package(url:"https://github.com/vapor/vapor.git",from:"1.2.3")]"#
            )
            .unwrap()[0]
                .current_value,
            r#"from:"1.2.3""#
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}"1.2.3"...)]"#)).unwrap()[0].current_value,
            r#""1.2.3"..."#
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}"1.2.3"..."1.2.4")]"#)).unwrap()[0]
                .current_value,
            r#""1.2.3"..."1.2.4""#
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}"1.2.3"..<"1.2.4")]"#)).unwrap()[0]
                .current_value,
            r#""1.2.3"..<"1.2.4""#
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}..."1.2.3")]"#)).unwrap()[0].current_value,
            r#"..."1.2.3""#
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}..<"1.2.3")]"#)).unwrap()[0].current_value,
            r#"..<"1.2.3""#
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}.exact("1.2.3"))]"#)).unwrap()[0].current_value,
            "1.2.3"
        );
        assert_eq!(
            extract_package_file(&format!(r#"{base}exact:"1.2.3"))]"#)).unwrap()[0].current_value,
            "1.2.3"
        );
    }

    // Ported: "parses multiple packages" — swift/index.spec.ts line 152
    // TypeScript uses toMatchSnapshot(); Rust checks key invariants instead.
    #[test]
    fn index_parses_multiple_packages() {
        let content = include_str!("../../tests/fixtures/spm/SamplePackage.swift");
        let result = extract_package_file(content);
        assert!(result.is_some());
        let deps = result.unwrap();
        assert!(!deps.is_empty());
        // All deps from SamplePackage.swift that have github.com URLs should be github-tags.
        assert!(deps.iter().all(|d| d.datasource == "github-tags"));
    }

    // Ported: "extracts GitHub dependencies from SCP-style SSH URL" — swift/extract.spec.ts line 117
    #[test]
    fn extracts_github_dependencies_from_scp_style_ssh_url() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "git@github.com:example/repo.git", from: "1.0.0")
          ]
        )
      "#;
        let result = extract_package_file(content);
        let deps = result.unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, r#"from: "1.0.0""#);
    }

    // Ported: "extracts GitLab dependencies from SCP-style SSH URL" — swift/extract.spec.ts line 138
    #[test]
    fn extracts_gitlab_dependencies_from_scp_style_ssh_url() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "git@gitlab.com:group/project.git", from: "2.0.0")
          ]
        )
      "#;
        let result = extract_package_file(content);
        let deps = result.unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "gitlab-tags");
        assert_eq!(deps[0].dep_name, "group/project");
        assert_eq!(deps[0].current_value, r#"from: "2.0.0""#);
    }

    // Ported: "extracts dependencies from ssh:// URL" — swift/extract.spec.ts line 159
    #[test]
    fn extracts_dependencies_from_ssh_url() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "ssh://git@github.com/example/repo.git", from: "1.0.0")
          ]
        )
      "#;
        let result = extract_package_file(content);
        let deps = result.unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].dep_name, "example/repo");
        assert_eq!(deps[0].current_value, r#"from: "1.0.0""#);
    }

    // Ported: "returns null for unparseable SSH URL" — swift/extract.spec.ts line 180
    #[test]
    fn returns_null_for_unparseable_ssh_url() {
        let content = r#"
        let package = Package(
          name: "MyPackage",
          dependencies: [
            .package(url: "ssh://", from: "1.0.0")
          ]
        )
      "#;
        assert!(extract_package_file(content).is_none());
    }
}
