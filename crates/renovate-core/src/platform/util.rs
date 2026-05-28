//! Platform utility functions.
//!
//! Ports `lib/modules/platform/util.ts`.

use sha2::{Digest, Sha512};

fn sha512_hex(data: &[u8]) -> String {
    let mut h = Sha512::new();
    h.update(data);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Compute a repository fingerprint (SHA-512 hex) from a repo ID and optional
/// base URL endpoint.
///
/// Mirrors `repoFingerprint` from `lib/modules/platform/util.ts`.
pub fn repo_fingerprint(repo_id: &str, endpoint: Option<&str>) -> String {
    let input = match endpoint {
        Some(ep) if !ep.is_empty() => format!("{ep}::{repo_id}"),
        _ => repo_id.to_owned(),
    };
    sha512_hex(input.as_bytes())
}

/// Ensure a branch name is prefixed with `refs/heads/`.
///
/// Mirrors `getNewBranchName` from `lib/modules/platform/util.ts`.
pub fn get_new_branch_name(branch_name: &str) -> String {
    if branch_name.starts_with("refs/heads/") {
        branch_name.to_owned()
    } else {
        format!("refs/heads/{branch_name}")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: '("$repoId", "$endpoint") === $fingerprint' — modules/platform/util.spec.ts line 8
    #[test]
    fn platform_util_repo_fingerprint() {
        assert_eq!(
            repo_fingerprint("some-id", None),
            "361b1bf27a0c0ef8fa5d270f588aa5747ba9497b16de64a44f186253295bc80a3891ecfee768f5c88734a6a738eacca69ccca7e50b16529cfc50dca77226a760"
        );
        assert_eq!(
            repo_fingerprint("some-id", Some("https://github.com")),
            "423e527a4f88a1b6aae8b70e72a4ae80b44fe83f11b90851f5bc654f39a3272c76b57d7ad30cabd727c04c254a3e7ea16109d05e398a228701ac805460344815"
        );
    }

    // Ported: "should add refs/heads" — modules/platform/util.spec.ts line 21
    #[test]
    fn platform_util_get_new_branch_name_adds_prefix() {
        assert_eq!(get_new_branch_name("testBB"), "refs/heads/testBB");
    }

    // Ported: "should be the same" — modules/platform/util.spec.ts line 26
    #[test]
    fn platform_util_get_new_branch_name_keeps_prefix() {
        assert_eq!(
            get_new_branch_name("refs/heads/testBB"),
            "refs/heads/testBB"
        );
    }
}
