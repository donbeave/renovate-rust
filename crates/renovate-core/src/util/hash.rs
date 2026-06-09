//! Hash utilities — mirrors `lib/util/hash.ts`.

/// @parity lib/util/hash.ts full
use sha1::Digest as Sha1Digest;
use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

/// Compute a hex digest using the requested algorithm.
///
/// Mirrors `hash(data, algorithm)` from `lib/util/hash.ts`.
///
/// Defaults to SHA-512 when `algorithm` is not provided.
pub fn hash(data: impl AsRef<[u8]>, algorithm: Option<&str>) -> String {
    match algorithm.unwrap_or("sha512") {
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(data.as_ref());
            hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        }
        "sha224" => {
            let mut hasher = Sha224::new();
            hasher.update(data.as_ref());
            hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data.as_ref());
            hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        }
        "sha384" => {
            let mut hasher = Sha384::new();
            hasher.update(data.as_ref());
            hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        }
        _ => {
            let mut hasher = Sha512::new();
            hasher.update(data.as_ref());
            hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        }
    }
}

/// Return the SHA-256 hex digest.
pub fn to_sha256(data: impl AsRef<[u8]>) -> String {
    hash(data, Some("sha256"))
}

/// Compute SHA digest from a readable stream.
pub async fn hash_stream(
    mut input: impl tokio::io::AsyncRead + Unpin,
    algorithm: Option<&str>,
) -> Result<String, std::io::Error> {
    use tokio::io::AsyncReadExt;

    let mut bytes = Vec::new();
    input.read_to_end(&mut bytes).await?;
    Ok(hash(bytes, algorithm))
}

/// Compute SHA-256 hex digest of the input data.
pub fn sha256(data: &[u8]) -> String {
    hash(data, Some("sha256"))
}

/// Compute SHA-512 hex digest of the input data.
pub fn sha512(data: &[u8]) -> String {
    hash(data, Some("sha512"))
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

    // Ported: "hashes data with sha256" — lib/util/hash.spec.ts line 6
    #[tokio::test]
    async fn test_hash_streams_and_to_sha256() {
        use std::io::Cursor;
        let content = b"This is some test content.";
        let content_hash = hash(content, Some("sha256"));
        let cursor = Cursor::new(content);
        let stream_hash = hash_stream(cursor, Some("sha256"))
            .await
            .expect("hash_stream should return a hash");
        assert_eq!(content_hash, stream_hash);
        assert_eq!(to_sha256(content), content_hash);
        assert_eq!(hash(content, None), hash(content, Some("sha512")));
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
