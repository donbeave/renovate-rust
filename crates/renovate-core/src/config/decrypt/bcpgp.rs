//! BcPGP decryption for encrypted config values.
//!
//! Renovate reference: `lib/config/decrypt/bcpgp.ts`.
//!
//! Note: Full PGP decryption requires a native crypto library. This module
//! provides the interface and a placeholder that returns `None` when the
//! required dependencies are not available.

/// Attempt to decrypt an encrypted config value using BcPGP.
///
/// Returns `Some(decrypted_string)` on success, `None` on failure.
///
/// Mirrors `tryDecryptBcPgp()` from `lib/config/decrypt/bcpgp.ts`.
pub fn decrypt_pgp(private_key: &str, encrypted_str: &str) -> Option<String> {
    let _ = (private_key, encrypted_str);
    None
}

/// Armor a raw encrypted string into PGP message format.
///
/// Adds PGP message headers if not already present.
pub fn armor_message(encrypted_str: &str) -> String {
    let start_block = "-----BEGIN PGP MESSAGE-----\n\n";
    let end_block = "\n-----END PGP MESSAGE-----";
    let mut armored = encrypted_str.trim().to_owned();

    let has_start = armored.starts_with(start_block);
    let has_end = armored.ends_with(end_block);

    if !has_start && !has_end && !armored.contains('=') && !armored.contains('\n') {
        let padding_needed = 4 - (armored.len() % 4);
        if padding_needed < 4 {
            for _ in 0..padding_needed {
                armored.push('=');
            }
        }
    }

    if !has_start {
        armored = format!("{start_block}{armored}");
    }
    if !has_end {
        armored = format!("{armored}{end_block}");
    }

    armored
}

/// Massage a private key by collapsing indented lines.
pub fn massage_private_key(key: &str) -> String {
    regex::Regex::new(r"\n[ \t]+")
        .map(|re| re.replace_all(key, "\n").into_owned())
        .unwrap_or_else(|_| key.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armor_message_adds_headers() {
        let raw = "YWJjZGVm";
        let armored = armor_message(raw);
        assert!(armored.starts_with("-----BEGIN PGP MESSAGE-----"));
        assert!(armored.ends_with("-----END PGP MESSAGE-----"));
    }

    #[test]
    fn armor_message_preserves_existing_headers() {
        let raw = "-----BEGIN PGP MESSAGE-----\n\nYWJj\n-----END PGP MESSAGE-----";
        let armored = armor_message(raw);
        assert_eq!(armored, raw);
    }

    #[test]
    fn massage_private_key_collapses_indentation() {
        let key = "-----BEGIN KEY-----\n  line1\n  line2\n-----END KEY-----";
        let massaged = massage_private_key(key);
        assert_eq!(massaged, "-----BEGIN KEY-----\nline1\nline2\n-----END KEY-----");
    }

    #[test]
    fn decrypt_pgp_returns_none_without_crypto() {
        let result = decrypt_pgp("key", "encrypted");
        assert!(result.is_none());
    }
}
