use regex::Regex;

static SEMANTIC_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^(?P<type>[a-z]+)(?:\((?P<scope>[^)]*)\))?(?P<breaking>!)?:\s*(?P<subject>.+)$")
        .expect("valid semantic commit regex")
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCommit {
    pub commit_type: String,
    pub scope: Option<String>,
    pub breaking: bool,
    pub subject: String,
}

pub fn is_semantic_commit(message: &str) -> bool {
    let first_line = message.lines().next().unwrap_or("");
    SEMANTIC_RE.is_match(first_line)
}

pub fn parse_semantic_commit(message: &str) -> Option<SemanticCommit> {
    let first_line = message.lines().next()?;
    let caps = SEMANTIC_RE.captures(first_line)?;
    Some(SemanticCommit {
        commit_type: caps.name("type")?.as_str().to_owned(),
        scope: caps.name("scope").map(|m| m.as_str().to_owned()),
        breaking: caps.name("breaking").is_some(),
        subject: caps.name("subject")?.as_str().to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_semantic_simple() {
        assert!(is_semantic_commit("feat: add new feature"));
    }

    #[test]
    fn is_semantic_with_scope() {
        assert!(is_semantic_commit("fix(core): resolve bug"));
    }

    #[test]
    fn is_semantic_breaking() {
        assert!(is_semantic_commit("feat!: breaking change"));
    }

    #[test]
    fn is_semantic_breaking_with_scope() {
        assert!(is_semantic_commit("feat(api)!: breaking api change"));
    }

    #[test]
    fn not_semantic_plain() {
        assert!(!is_semantic_commit("update dependencies"));
    }

    #[test]
    fn not_semantic_no_colon() {
        assert!(!is_semantic_commit("feat add feature"));
    }

    #[test]
    fn parse_simple() {
        let commit = parse_semantic_commit("feat: add feature").unwrap();
        assert_eq!(commit.commit_type, "feat");
        assert_eq!(commit.scope, None);
        assert!(!commit.breaking);
        assert_eq!(commit.subject, "add feature");
    }

    #[test]
    fn parse_with_scope() {
        let commit = parse_semantic_commit("fix(core): resolve bug").unwrap();
        assert_eq!(commit.commit_type, "fix");
        assert_eq!(commit.scope, Some("core".to_owned()));
        assert_eq!(commit.subject, "resolve bug");
    }

    #[test]
    fn parse_breaking() {
        let commit = parse_semantic_commit("feat!: breaking change").unwrap();
        assert!(commit.breaking);
    }

    #[test]
    fn parse_multiline() {
        let commit = parse_semantic_commit("feat: title\n\nBody text").unwrap();
        assert_eq!(commit.subject, "title");
    }

    #[test]
    fn parse_returns_none_for_non_semantic() {
        assert!(parse_semantic_commit("just a regular message").is_none());
    }
}
