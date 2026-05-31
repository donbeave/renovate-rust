use std::collections::HashMap;

pub const BASIC_ENV_VARS: &[&str] = &[
    "HOME",
    "PATH",
    "LANG",
    "LC_ALL",
    "LC_CTYPE",
    "SSL_CERT_FILE",
    "NODE_EXTRA_CA_CERTS",
    "CI",
    "CONTAINERBASE",
    "HERMIT_ENV",
];

pub fn get_child_process_env(
    process_env: &HashMap<String, String>,
    custom_env_vars: &[String],
    expose_all: bool,
) -> HashMap<String, String> {
    let mut env = HashMap::new();

    for &key in BASIC_ENV_VARS {
        if let Some(val) = process_env.get(key) {
            env.insert(key.to_owned(), val.clone());
        }
    }

    for key in custom_env_vars {
        let key_upper = key.to_uppercase();
        if let Some(val) = process_env.get(key_upper.as_str()) {
            env.insert(key_upper, val.clone());
        } else if let Some(val) = process_env.get(key.as_str()) {
            env.insert(key.clone(), val.clone());
        }
    }

    if expose_all {
        for (key, val) in process_env {
            env.insert(key.clone(), val.clone());
        }
    }

    env
}

pub fn get_child_env(
    process_env: &HashMap<String, String>,
    custom_env: Option<&HashMap<String, String>>,
    extra_env: Option<&HashMap<String, String>>,
    custom_env_vars: &[String],
    expose_all: bool,
) -> HashMap<String, String> {
    let mut env = get_child_process_env(process_env, custom_env_vars, expose_all);

    if let Some(custom) = custom_env {
        for (key, val) in custom {
            env.insert(key.clone(), val.clone());
        }
    }

    if let Some(extra) = extra_env {
        for (key, val) in extra {
            env.insert(key.clone(), val.clone());
        }
    }

    env
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().cloned().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()
    }

    // Ported: "returns default environment variables" — util/exec/env.spec.ts line 35
    #[test]
    fn get_child_process_env_basic() {
        let env = make_env(&[("HOME", "/home/user"), ("PATH", "/usr/bin"), ("SECRET", "123")]);
        let result = get_child_process_env(&env, &[], false);
        assert_eq!(result.get("HOME").unwrap(), "/home/user");
        assert_eq!(result.get("PATH").unwrap(), "/usr/bin");
        assert!(!result.contains_key("SECRET"));
    }

    // Ported: "returns custom environment variables if passed and defined" — util/exec/env.spec.ts line 62
    #[test]
    fn get_child_process_env_custom_vars() {
        let env = make_env(&[("HOME", "/home"), ("MY_TOKEN", "abc")]);
        let result = get_child_process_env(&env, &["MY_TOKEN".to_owned()], false);
        assert_eq!(result.get("MY_TOKEN").unwrap(), "abc");
    }

    // Ported: "returns process.env if trustlevel set to high" — util/exec/env.spec.ts line 79
    #[test]
    fn get_child_process_env_expose_all() {
        let env = make_env(&[("HOME", "/home"), ("SECRET", "123")]);
        let result = get_child_process_env(&env, &[], true);
        assert_eq!(result.get("SECRET").unwrap(), "123");
    }

    // Ported: "returns environment variable only if defined" — util/exec/env.spec.ts line 57
    #[test]
    fn get_child_process_env_only_if_defined() {
        let env = make_env(&[("HOME", "/home")]);
        let result = get_child_process_env(&env, &[], false);
        assert!(!result.contains_key("PATH"));
        assert!(result.contains_key("HOME"));
    }

    // Rust-specific: env behavior test
    #[test]
    fn get_child_env_merges_custom_and_extra() {
        let process_env = make_env(&[("HOME", "/home"), ("PATH", "/usr/bin")]);
        let custom = make_env(&[("CUSTOM_VAR", "custom")]);
        let extra = make_env(&[("EXTRA_VAR", "extra")]);

        let result = get_child_env(&process_env, Some(&custom), Some(&extra), &[], false);
        assert_eq!(result.get("CUSTOM_VAR").unwrap(), "custom");
        assert_eq!(result.get("EXTRA_VAR").unwrap(), "extra");
        assert_eq!(result.get("HOME").unwrap(), "/home");
    }
}
