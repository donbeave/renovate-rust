//! Platform type.
//!
//! Source: `PLATFORM_HOST_TYPES` in `lib/constants/platforms.ts`.

/// Identifies the type of source-code platform hosting the target repositories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    Azure,
    Bitbucket,
    BitbucketServer,
    Codecommit,
    Forgejo,
    Gerrit,
    Gitea,
    Github,
    Gitlab,
    Local,
    ScmManager,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Azure => "azure",
            Self::Bitbucket => "bitbucket",
            Self::BitbucketServer => "bitbucket-server",
            Self::Codecommit => "codecommit",
            Self::Forgejo => "forgejo",
            Self::Gerrit => "gerrit",
            Self::Gitea => "gitea",
            Self::Github => "github",
            Self::Gitlab => "gitlab",
            Self::Local => "local",
            Self::ScmManager => "scm-manager",
        };
        f.write_str(s)
    }
}
