use std::sync::LazyLock;

use regex::Regex;

static RE_QUOTED_EMAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^"([^"]+)"\s*<([^>]*)>"#).unwrap());

static RE_UNQUOTED_EMAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([^<]+?)\s*<([^>]*)>").unwrap());

static RE_BARE_EMAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^\s@<>]+@[^\s@<>]+\.[^\s@<>.]+$").unwrap());

/// Parsed git author as returned by `parseGitAuthor`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitAuthor {
    pub name: String,
    pub address: String,
}

/// Wraps the name part of a `Name <email>` string in double-quotes so that
/// special characters (e.g. `[`, `]`) are accepted by standard email parsers.
fn massage_name_brackets(input: &str) -> String {
    let lt_pos = input
        .find(" <")
        .or_else(|| input.find('<'))
        .unwrap_or(input.len());
    let name_part = input[..lt_pos].trim();
    let rest = input[lt_pos..].trim_start_matches(' ');
    format!("\"{}\" {}", name_part, rest)
}

fn try_name_email(s: &str) -> Option<(String, String)> {
    if let Some(caps) = RE_QUOTED_EMAIL.captures(s) {
        let name = caps[1].trim().to_owned();
        let addr = caps[2].trim().to_owned();
        if addr.contains('@') {
            return Some((name, addr));
        }
    }
    if let Some(caps) = RE_UNQUOTED_EMAIL.captures(s) {
        let name = caps[1].trim().to_owned();
        let addr = caps[2].trim().to_owned();
        if addr.contains('@') {
            return Some((name, addr));
        }
    }
    None
}

/// Parses a git author string into a `GitAuthor`.
///
/// Mirrors `parseGitAuthor` from `lib/util/git/author.ts`. Handles:
/// - Bare emails: `user@domain.com`
/// - Standard RFC 5322: `Name <user@domain.com>`
/// - GitHub bot emails: `renovate[bot]@users.noreply.github.com`
/// - Names with brackets: `Name [tag] <user@domain.com>`
pub fn parse_git_author(input: &str) -> Option<GitAuthor> {
    if input.is_empty() {
        return None;
    }

    let has_bot_at = input.contains("[bot]@");
    let has_angle = input.contains('<') && input.contains('>');

    if has_angle {
        // Massage: quote the name part so brackets are valid, strip [bot]@ for parsing.
        let massaged = {
            let mut s = massage_name_brackets(input);
            if has_bot_at {
                s = s.replace("[bot]@", "@");
            }
            s
        };

        if let Some((name, raw_addr)) = try_name_email(&massaged) {
            let address = if has_bot_at && !raw_addr.contains("[bot]@") {
                raw_addr.replacen('@', "[bot]@", 1)
            } else {
                raw_addr
            };
            return Some(GitAuthor { name, address });
        }
        return None;
    }

    // Bare email (possibly with [bot]@ substitution)
    if input.contains('@') {
        let trimmed = input.trim();
        let check = if has_bot_at {
            trimmed.replace("[bot]@", "@")
        } else {
            trimmed.to_owned()
        };

        if RE_BARE_EMAIL.is_match(&check) {
            let name = if has_bot_at {
                trimmed.split('@').next().unwrap_or("").to_owned()
            } else {
                String::new()
            };
            return Some(GitAuthor {
                name,
                address: trimmed.to_owned(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns null if empty email given" — lib/util/git/author.spec.ts line 8
    #[test]
    fn parse_git_author_returns_none_for_empty() {
        assert_eq!(parse_git_author(""), None);
    }

    // Ported: "catches errors" — lib/util/git/author.spec.ts line 12
    // In Rust there are no exceptions; the function returns None for unparseable input.
    #[test]
    fn parse_git_author_returns_none_for_unparseable() {
        assert_eq!(parse_git_author("not-an-email"), None);
    }

    // Ported: "handles a normal address" — lib/util/git/author.spec.ts line 19
    #[test]
    fn parse_git_author_handles_normal_address() {
        assert!(parse_git_author("renovate@whitesourcesoftware.com").is_some());
    }

    // Ported: "parses bot email" — lib/util/git/author.spec.ts line 23
    #[test]
    fn parse_git_author_parses_bot_email() {
        assert_eq!(
            parse_git_author("renovate[bot]@users.noreply.github.com"),
            Some(GitAuthor {
                name: "renovate[bot]".to_owned(),
                address: "renovate[bot]@users.noreply.github.com".to_owned(),
            })
        );
    }

    // Ported: "parses bot name and email" — lib/util/git/author.spec.ts line 30
    #[test]
    fn parse_git_author_parses_bot_name_and_email() {
        assert_eq!(
            parse_git_author("renovate[bot] <renovate[bot]@users.noreply.github.com>"),
            Some(GitAuthor {
                name: "renovate[bot]".to_owned(),
                address: "renovate[bot]@users.noreply.github.com".to_owned(),
            })
        );
    }

    // Ported: "escapes names" — lib/util/git/author.spec.ts line 41
    #[test]
    fn parse_git_author_handles_name_with_brackets() {
        let result = parse_git_author("name [what] <name@what.com>").unwrap();
        assert_eq!(result.name, "name [what]");
    }

    // Ported: "tries again and fails" — lib/util/git/author.spec.ts line 47
    #[test]
    fn parse_git_author_returns_none_for_invalid_email_in_brackets() {
        assert_eq!(parse_git_author("foo<foo>"), None);
    }

    // Ported: "gives up" — lib/util/git/author.spec.ts line 51
    #[test]
    fn parse_git_author_gives_up_on_non_email() {
        assert_eq!(parse_git_author("a.b.c"), None);
    }
}
