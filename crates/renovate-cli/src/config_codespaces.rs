//! Mirror `lib/workers/global/config/parse/codespaces.ts` behavior for Codespaces runs.
//! @parity lib/workers/global/config/parse/codespaces.ts full — in GitHub Codespaces, infer token from `GITHUB_TOKEN` and prompt for repository when none are configured.

use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

use renovate_core::config::GlobalConfig;

/// Applies Codespaces-specific configuration adjustments.
pub(crate) fn apply_codespaces_config(
    env: &BTreeMap<String, String>,
    config: &mut GlobalConfig,
) -> io::Result<()> {
    apply_codespaces_config_with_stdin(env, config, &mut io::stdin().lock())
}

pub(crate) fn apply_codespaces_config_with_stdin<R: BufRead>(
    env: &BTreeMap<String, String>,
    config: &mut GlobalConfig,
    stdin: &mut R,
) -> io::Result<()> {
    if env.get("CODESPACES").is_none_or(|value| value != "true") {
        return Ok(());
    }

    if config.token.is_none()
        && let Some(token) = env.get("GITHUB_TOKEN")
    {
        config.token = Some(token.clone());
    }

    if config.repositories.is_empty() {
        let mut repo = String::new();
        print!("\n\nRepository name: ");
        io::stdout().flush()?;
        stdin.read_line(&mut repo)?;
        let repo = repo.trim_end_matches(&['\n', '\r'][..]).to_owned();
        config.repositories = vec![repo];
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::io::Cursor;

    use renovate_core::config::GlobalConfig;

    use super::apply_codespaces_config_with_stdin;

    #[test]
    fn codespaces_config_sets_token_and_repo() {
        let mut env = BTreeMap::new();
        env.insert("CODESPACES".into(), "true".into());
        env.insert("GITHUB_TOKEN".into(), "ghs-example".into());

        let mut config = GlobalConfig::default();
        let mut input = Cursor::new("owner/repo\n");

        apply_codespaces_config_with_stdin(&env, &mut config, &mut input).unwrap();

        assert_eq!(config.token, Some("ghs-example".into()));
        assert_eq!(config.repositories, vec!["owner/repo".to_owned()]);
    }
}
