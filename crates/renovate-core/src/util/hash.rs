//! Hash utilities — mirrors `lib/util/hash.ts`.

use sha2::{Digest, Sha256, Sha512};

/// Compute SHA-256 hex digest of the input data.
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Compute SHA-512 hex digest of the input data.
pub fn sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

mod hex {
    pub(crate) fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_empty() {
        let result = sha256(b"");
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sha256_hello() {
        let result = sha256(b"hello");
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn sha256_foobar() {
        let result = sha256(b"foobar");
        assert_eq!(
            result,
            "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2"
        );
    }

    #[test]
    fn sha256_returns_lowercase_hex() {
        let result = sha256(b"test");
        assert_eq!(result, result.to_lowercase());
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn sha512_empty() {
        let result = sha512(b"");
        assert_eq!(
            result,
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        );
    }

    #[test]
    fn sha512_hello() {
        let result = sha512(b"hello");
        assert_eq!(
            result,
            "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca7\
             2323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
        );
    }

    #[test]
    fn sha512_returns_lowercase_hex() {
        let result = sha512(b"test");
        assert_eq!(result, result.to_lowercase());
        assert_eq!(result.len(), 128);
    }

    #[test]
    fn sha256_deterministic() {
        let data = b"renovate";
        assert_eq!(sha256(data), sha256(data));
    }

    #[test]
    fn sha512_deterministic() {
        let data = b"renovate";
        assert_eq!(sha512(data), sha512(data));
    }

    #[test]
    fn sha256_different_inputs() {
        assert_ne!(sha256(b"a"), sha256(b"b"));
    }

    #[test]
    fn sha512_different_inputs() {
        assert_ne!(sha512(b"a"), sha512(b"b"));
    }
}
