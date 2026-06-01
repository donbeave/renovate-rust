use std::collections::HashMap;

pub fn get_git_config(extra_config: &[(&str, &str)]) -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("core.autocrlf".to_owned(), "input".to_owned());
    config.insert("core.symlinks".to_owned(), "true".to_owned());
    for (key, value) in extra_config {
        config.insert(key.to_string(), value.to_string());
    }
    config
}

pub fn set_git_config(config: &mut HashMap<String, String>, key: &str, value: &str) {
    config.insert(key.to_owned(), value.to_owned());
}

pub fn git_config_args(config: &HashMap<String, String>) -> Vec<String> {
    config
        .iter()
        .map(|(k, v)| format!("-c{}={}", k, v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_git_config_defaults() {
        let config = get_git_config(&[]);
        assert_eq!(config.get("core.autocrlf"), Some(&"input".to_owned()));
        assert_eq!(config.get("core.symlinks"), Some(&"true".to_owned()));
    }

    #[test]
    fn get_git_config_with_extra() {
        let config = get_git_config(&[("user.name", "Renovate Bot")]);
        assert_eq!(config.get("user.name"), Some(&"Renovate Bot".to_owned()));
    }

    #[test]
    fn set_git_config_adds_entry() {
        let mut config = get_git_config(&[]);
        set_git_config(&mut config, "user.email", "bot@renovate.com");
        assert_eq!(
            config.get("user.email"),
            Some(&"bot@renovate.com".to_owned())
        );
    }

    #[test]
    fn git_config_args_produces_flags() {
        let mut config = HashMap::new();
        config.insert("user.name".to_owned(), "Bot".to_owned());
        let args = git_config_args(&config);
        assert!(args.contains(&"-cuser.name=Bot".to_owned()));
    }
}
