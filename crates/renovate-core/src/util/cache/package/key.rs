use sha2::{Digest, Sha256};

pub fn generate_cache_key(namespace: &str, key: &str) -> String {
    if namespace.is_empty() {
        key.to_owned()
    } else {
        format!("{namespace}:{key}")
    }
}

pub fn hash_key(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_cache_key_with_namespace() {
        assert_eq!(generate_cache_key("npm", "lodash"), "npm:lodash");
    }

    #[test]
    fn generate_cache_key_empty_namespace() {
        assert_eq!(generate_cache_key("", "lodash"), "lodash");
    }

    #[test]
    fn generate_cache_key_both_empty() {
        assert_eq!(generate_cache_key("", ""), "");
    }

    #[test]
    fn hash_key_deterministic() {
        let h1 = hash_key("test-key");
        let h2 = hash_key("test-key");
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_key_different_inputs() {
        assert_ne!(hash_key("a"), hash_key("b"));
    }

    #[test]
    fn hash_key_returns_hex() {
        let h = hash_key("test");
        assert_eq!(h.len(), 64);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_key_empty_string() {
        let h = hash_key("");
        assert_eq!(h.len(), 64);
    }
}
