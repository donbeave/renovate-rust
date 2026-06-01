use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitError {
    AuthFailed(String),
    NotFound(String),
    Conflict(String),
    LockConflict(String),
    IntegrationError(String),
    PushRejected(String),
    BranchModified(String),
    HttpError(String),
    Unknown(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthFailed(msg) => write!(f, "Authentication failed: {msg}"),
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::Conflict(msg) => write!(f, "Conflict: {msg}"),
            Self::LockConflict(msg) => write!(f, "Lock conflict: {msg}"),
            Self::IntegrationError(msg) => write!(f, "Integration error: {msg}"),
            Self::PushRejected(msg) => write!(f, "Push rejected: {msg}"),
            Self::BranchModified(msg) => write!(f, "Branch modified: {msg}"),
            Self::HttpError(msg) => write!(f, "HTTP error: {msg}"),
            Self::Unknown(msg) => write!(f, "Git error: {msg}"),
        }
    }
}

impl std::error::Error for GitError {}

struct ErrorPattern {
    pattern: &'static str,
    kind: GitErrorKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GitErrorKind {
    AuthFailed,
    NotFound,
    Conflict,
    LockConflict,
    IntegrationError,
    PushRejected,
    BranchModified,
}

static ERROR_PATTERNS: &[ErrorPattern] = &[
    ErrorPattern {
        pattern: "Authentication failed",
        kind: GitErrorKind::AuthFailed,
    },
    ErrorPattern {
        pattern: "fatal: Authentication failed",
        kind: GitErrorKind::AuthFailed,
    },
    ErrorPattern {
        pattern: "could not read Username",
        kind: GitErrorKind::AuthFailed,
    },
    ErrorPattern {
        pattern: "not found",
        kind: GitErrorKind::NotFound,
    },
    ErrorPattern {
        pattern: "fatal: repository not found",
        kind: GitErrorKind::NotFound,
    },
    ErrorPattern {
        pattern: "CONFLICT",
        kind: GitErrorKind::Conflict,
    },
    ErrorPattern {
        pattern: "merge conflict",
        kind: GitErrorKind::Conflict,
    },
    ErrorPattern {
        pattern: "lock conflict",
        kind: GitErrorKind::LockConflict,
    },
    ErrorPattern {
        pattern: "Lock conflict",
        kind: GitErrorKind::LockConflict,
    },
    ErrorPattern {
        pattern: "failed to push some refs",
        kind: GitErrorKind::PushRejected,
    },
    ErrorPattern {
        pattern: "push rejected",
        kind: GitErrorKind::PushRejected,
    },
    ErrorPattern {
        pattern: "branch was modified",
        kind: GitErrorKind::BranchModified,
    },
    ErrorPattern {
        pattern: "Integration failed",
        kind: GitErrorKind::IntegrationError,
    },
];

fn make_error(kind: GitErrorKind, msg: &str) -> GitError {
    match kind {
        GitErrorKind::AuthFailed => GitError::AuthFailed(msg.to_owned()),
        GitErrorKind::NotFound => GitError::NotFound(msg.to_owned()),
        GitErrorKind::Conflict => GitError::Conflict(msg.to_owned()),
        GitErrorKind::LockConflict => GitError::LockConflict(msg.to_owned()),
        GitErrorKind::IntegrationError => GitError::IntegrationError(msg.to_owned()),
        GitErrorKind::PushRejected => GitError::PushRejected(msg.to_owned()),
        GitErrorKind::BranchModified => GitError::BranchModified(msg.to_owned()),
    }
}

fn kind_discriminant(error: &GitError) -> GitErrorKind {
    match error {
        GitError::AuthFailed(_) => GitErrorKind::AuthFailed,
        GitError::NotFound(_) => GitErrorKind::NotFound,
        GitError::Conflict(_) => GitErrorKind::Conflict,
        GitError::LockConflict(_) => GitErrorKind::LockConflict,
        GitError::IntegrationError(_) => GitErrorKind::IntegrationError,
        GitError::PushRejected(_) => GitErrorKind::PushRejected,
        GitError::BranchModified(_) => GitErrorKind::BranchModified,
        _ => GitErrorKind::AuthFailed,
    }
}

pub fn is_git_error(stderr: &str, error_kind: &GitError) -> bool {
    let stderr_lower = stderr.to_lowercase();
    let target_kind = kind_discriminant(error_kind);
    for ep in ERROR_PATTERNS {
        if stderr_lower.contains(&ep.pattern.to_lowercase()) && ep.kind == target_kind {
            return true;
        }
    }
    false
}

pub fn classify_git_error(stderr: &str) -> GitError {
    let stderr_lower = stderr.to_lowercase();
    for ep in ERROR_PATTERNS {
        if stderr_lower.contains(&ep.pattern.to_lowercase()) {
            return make_error(ep.kind, stderr);
        }
    }
    GitError::Unknown(stderr.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_auth_failed() {
        let err = classify_git_error("fatal: Authentication failed for repo");
        assert_eq!(
            err,
            GitError::AuthFailed("fatal: Authentication failed for repo".to_owned())
        );
    }

    #[test]
    fn classify_not_found() {
        let err = classify_git_error("fatal: repository not found");
        assert_eq!(
            err,
            GitError::NotFound("fatal: repository not found".to_owned())
        );
    }

    #[test]
    fn classify_push_rejected() {
        let err = classify_git_error("error: failed to push some refs");
        assert_eq!(
            err,
            GitError::PushRejected("error: failed to push some refs".to_owned())
        );
    }

    #[test]
    fn classify_unknown() {
        let err = classify_git_error("some random error");
        assert_eq!(err, GitError::Unknown("some random error".to_owned()));
    }

    #[test]
    fn is_git_error_matches_kind() {
        assert!(is_git_error(
            "fatal: Authentication failed",
            &GitError::AuthFailed(String::new())
        ));
        assert!(!is_git_error(
            "not found",
            &GitError::AuthFailed(String::new())
        ));
    }

    #[test]
    fn git_error_display() {
        assert!(
            GitError::AuthFailed("test".to_owned())
                .to_string()
                .contains("Authentication failed")
        );
    }
}
