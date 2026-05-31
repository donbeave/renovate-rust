use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitUrlParts {
    pub protocol: String,
    pub host: String,
    pub owner: String,
    pub repo: String,
}

pub fn parse_git_url(input: &str) -> Option<GitUrlParts> {
    let input = input.trim().trim_end_matches(".git");

    if let Some(rest) = input.strip_prefix("git@") {
        let (host, path) = if let Some(colon) = rest.find(':') {
            (&rest[..colon], &rest[colon + 1..])
        } else {
            return None;
        };
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if segments.len() >= 2 {
            return Some(GitUrlParts {
                protocol: "https".to_owned(),
                host: host.to_owned(),
                owner: segments[0].to_owned(),
                repo: segments[1].to_owned(),
            });
        }
        return None;
    }

    if let Ok(parsed) = Url::parse(input) {
        let host = parsed.host_str()?.to_owned();
        let path = parsed.path().trim_start_matches('/').trim_end_matches('/');
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if segments.len() >= 2 {
            return Some(GitUrlParts {
                protocol: parsed.scheme().to_owned(),
                host,
                owner: segments[0].to_owned(),
                repo: segments[1].to_owned(),
            });
        }
    }

    None
}

pub fn normalize_git_url(url: &str) -> String {
    let trimmed = url.trim();
    if let Some(parts) = parse_git_url(trimmed) {
        format!(
            "{}://{}/{}/{}.git",
            parts.protocol, parts.host, parts.owner, parts.repo
        )
    } else {
        trimmed.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_https_url() {
        let parts = parse_git_url("https://github.com/owner/repo").unwrap();
        assert_eq!(parts.protocol, "https");
        assert_eq!(parts.host, "github.com");
        assert_eq!(parts.owner, "owner");
        assert_eq!(parts.repo, "repo");
    }

    #[test]
    fn parse_https_url_with_git_suffix() {
        let parts = parse_git_url("https://github.com/owner/repo.git").unwrap();
        assert_eq!(parts.owner, "owner");
        assert_eq!(parts.repo, "repo");
    }

    #[test]
    fn parse_ssh_url() {
        let parts = parse_git_url("git@github.com:owner/repo").unwrap();
        assert_eq!(parts.protocol, "https");
        assert_eq!(parts.host, "github.com");
        assert_eq!(parts.owner, "owner");
        assert_eq!(parts.repo, "repo");
    }

    #[test]
    fn parse_git_protocol() {
        let parts = parse_git_url("git://github.com/owner/repo.git").unwrap();
        assert_eq!(parts.protocol, "git");
        assert_eq!(parts.owner, "owner");
        assert_eq!(parts.repo, "repo");
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert_eq!(parse_git_url(""), None);
        assert_eq!(parse_git_url("not-a-url"), None);
    }

    #[test]
    fn normalize_https_url() {
        assert_eq!(
            normalize_git_url("https://github.com/owner/repo"),
            "https://github.com/owner/repo.git"
        );
    }

    #[test]
    fn normalize_ssh_url() {
        assert_eq!(
            normalize_git_url("git@github.com:owner/repo"),
            "https://github.com/owner/repo.git"
        );
    }

    #[test]
    fn normalize_already_normalized() {
        assert_eq!(
            normalize_git_url("https://github.com/owner/repo.git"),
            "https://github.com/owner/repo.git"
        );
    }
}
