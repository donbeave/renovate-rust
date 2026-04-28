//! Renovate-compatible argv migration.
//!
//! Ports `migrateArgs` from
//! [`lib/workers/global/config/parse/cli.ts`](../../renovate/lib/workers/global/config/parse/cli.ts)
//! in the upstream Renovate reference. The function rewrites legacy CLI flag
//! forms (e.g. `--azure-auto-complete`) into their current equivalents
//! (`--platform-automerge`) and strips the deprecated `--git-fs*` family.
//!
//! ## Semantics worth knowing
//!
//! - JavaScript `String.prototype.replace(string, string)` replaces only the
//!   **first** occurrence. The Rust port uses [`replacen`](str::replacen)
//!   with `count = 1` to mirror that exactly. This matters for the JSON-key
//!   rewrites (e.g. `"platform":"`) where the same substring can appear
//!   multiple times inside a single `--host-rules=` value.
//! - Replacement order is significant: longer, more-specific patterns must
//!   run before their bare prefixes (e.g. `--include-forks=true` before
//!   `--include-forks`), otherwise the bare prefix would chew up the suffix
//!   and produce nonsense like `--fork-processing=enabled=true`.
//! - The two anchored regexes from upstream (`^--dry-run$`,
//!   `^--require-config$`) are expressed as full-string equality checks,
//!   which is the cleaner Rust idiom for the same intent.
//! - `--git-fs*` flags are dropped entirely (Renovate uses `.filter`).

/// Apply Renovate's legacy-flag migrations to a slice of CLI arguments.
///
/// Returns a fresh `Vec<String>` even when no migrations apply, so callers
/// can pass the result straight to a parser without further allocation
/// decisions. See the module docs for ordering and semantic notes.
#[must_use]
pub(crate) fn migrate_args(input: &[String]) -> Vec<String> {
    input
        .iter()
        .filter(|a| !a.starts_with("--git-fs"))
        .map(|arg| migrate_one(arg))
        .collect()
}

fn migrate_one(arg: &str) -> String {
    // Anchored upstream regexes — full-string equality is the natural Rust
    // expression of `/^…$/` without the `g` flag.
    if arg == "--dry-run" {
        return "--dry-run=true".to_owned();
    }
    if arg == "--require-config" {
        return "--require-config=true".to_owned();
    }

    // Substring rewrites in upstream's exact order. Each `replacen(_, _, 1)`
    // mirrors JS `String.prototype.replace(string, string)`.
    let mut out = arg.to_owned();
    for (needle, replacement) in MIGRATIONS {
        if out.contains(needle) {
            out = out.replacen(needle, replacement, 1);
        }
    }
    out
}

/// Substring-replace migrations, in the exact order Renovate applies them.
///
/// Order is load-bearing: see module docs.
const MIGRATIONS: &[(&str, &str)] = &[
    (
        "--allow-post-upgrade-command-templating",
        "--allow-command-templating",
    ),
    ("--allowed-post-upgrade-commands", "--allowed-commands"),
    ("--endpoints=", "--host-rules="),
    ("--expose-env=true", "--trust-level=high"),
    ("--expose-env", "--trust-level=high"),
    ("--renovate-fork", "--include-forks"),
    // JSON-key rewrites that show up inside complex flag values such as
    // `--host-rules='[{"platform":"github"}]'`.
    (r#""platform":""#, r#""hostType":""#),
    (r#""endpoint":""#, r#""matchHost":""#),
    (r#""host":""#, r#""matchHost":""#),
    // azureAutoComplete / gitLabAutomerge folded into platformAutomerge.
    ("--azure-auto-complete", "--platform-automerge"),
    ("--git-lab-automerge", "--platform-automerge"),
    ("--aliases", "--registry-aliases"),
    // `=true` form must precede the bare form, otherwise the bare match
    // would produce `--fork-processing=enabled=true`.
    ("--include-forks=true", "--fork-processing=enabled"),
    ("--include-forks", "--fork-processing=enabled"),
    // Same ordering rule for recreateClosed.
    ("--recreate-closed=false", "--recreate-when=auto"),
    ("--recreate-closed=true", "--recreate-when=always"),
    ("--recreate-closed", "--recreate-when=always"),
];

#[cfg(test)]
mod tests {
    use super::migrate_args;

    fn migrate(args: &[&str]) -> Vec<String> {
        let owned: Vec<String> = args.iter().map(|s| (*s).to_owned()).collect();
        migrate_args(&owned)
    }

    #[test]
    fn passes_through_unrelated_args() {
        let out = migrate(&["renovate", "myrepo", "--token=abc"]);
        assert_eq!(out, vec!["renovate", "myrepo", "--token=abc"]);
    }

    #[test]
    fn empty_input_returns_empty() {
        let out = migrate(&[]);
        assert!(out.is_empty());
    }

    #[test]
    fn rewrites_allow_post_upgrade_command_templating() {
        let out = migrate(&["--allow-post-upgrade-command-templating"]);
        assert_eq!(out, vec!["--allow-command-templating"]);
    }

    #[test]
    fn rewrites_allowed_post_upgrade_commands() {
        let out = migrate(&["--allowed-post-upgrade-commands=foo,bar"]);
        assert_eq!(out, vec!["--allowed-commands=foo,bar"]);
    }

    #[test]
    fn rewrites_endpoints_to_host_rules() {
        let out = migrate(&["--endpoints=[]"]);
        assert_eq!(out, vec!["--host-rules=[]"]);
    }

    #[test]
    fn rewrites_expose_env_true_to_trust_level_high() {
        let out = migrate(&["--expose-env=true"]);
        assert_eq!(out, vec!["--trust-level=high"]);
    }

    #[test]
    fn rewrites_bare_expose_env_to_trust_level_high() {
        let out = migrate(&["--expose-env"]);
        assert_eq!(out, vec!["--trust-level=high"]);
    }

    #[test]
    fn rewrites_renovate_fork_through_to_fork_processing_enabled() {
        // --renovate-fork → --include-forks → --fork-processing=enabled
        // exercises the chained substitutions.
        let out = migrate(&["--renovate-fork"]);
        assert_eq!(out, vec!["--fork-processing=enabled"]);
    }

    #[test]
    fn rewrites_json_platform_key_to_host_type() {
        let out = migrate(&[r#"--host-rules=[{"platform":"github.com"}]"#]);
        assert_eq!(out, vec![r#"--host-rules=[{"hostType":"github.com"}]"#],);
    }

    #[test]
    fn rewrites_json_endpoint_key_to_match_host() {
        let out = migrate(&[r#"--host-rules=[{"endpoint":"https://example.com"}]"#]);
        assert_eq!(
            out,
            vec![r#"--host-rules=[{"matchHost":"https://example.com"}]"#],
        );
    }

    #[test]
    fn rewrites_json_host_key_to_match_host() {
        let out = migrate(&[r#"--host-rules=[{"host":"example.com"}]"#]);
        assert_eq!(out, vec![r#"--host-rules=[{"matchHost":"example.com"}]"#],);
    }

    #[test]
    fn json_key_rewrite_uses_first_occurrence_only() {
        // Faithful to JS String.prototype.replace(string, string), which
        // only replaces the first occurrence.
        let out = migrate(&[r#"--host-rules=[{"platform":"github"},{"platform":"gitlab"}]"#]);
        assert_eq!(
            out,
            vec![r#"--host-rules=[{"hostType":"github"},{"platform":"gitlab"}]"#.to_owned(),],
        );
    }

    #[test]
    fn rewrites_azure_auto_complete_variants_to_platform_automerge() {
        assert_eq!(
            migrate(&["--azure-auto-complete=false"]),
            vec!["--platform-automerge=false"],
        );
        assert_eq!(
            migrate(&["--azure-auto-complete=true"]),
            vec!["--platform-automerge=true"],
        );
        assert_eq!(
            migrate(&["--azure-auto-complete"]),
            vec!["--platform-automerge"],
        );
    }

    #[test]
    fn rewrites_git_lab_automerge_variants_to_platform_automerge() {
        assert_eq!(
            migrate(&["--git-lab-automerge=false"]),
            vec!["--platform-automerge=false"],
        );
        assert_eq!(
            migrate(&["--git-lab-automerge=true"]),
            vec!["--platform-automerge=true"],
        );
        assert_eq!(
            migrate(&["--git-lab-automerge"]),
            vec!["--platform-automerge"],
        );
    }

    #[test]
    fn bare_dry_run_becomes_dry_run_true() {
        // Only the exact `--dry-run` token is rewritten; `--dry-run=false`
        // must pass through untouched.
        assert_eq!(migrate(&["--dry-run"]), vec!["--dry-run=true"]);
        assert_eq!(migrate(&["--dry-run=false"]), vec!["--dry-run=false"]);
        assert_eq!(migrate(&["--dry-run=null"]), vec!["--dry-run=null"]);
    }

    #[test]
    fn bare_require_config_becomes_require_config_true() {
        assert_eq!(
            migrate(&["--require-config"]),
            vec!["--require-config=true"],
        );
        assert_eq!(
            migrate(&["--require-config=false"]),
            vec!["--require-config=false"],
        );
    }

    #[test]
    fn rewrites_aliases_to_registry_aliases() {
        let out = migrate(&["--aliases=foo=bar"]);
        assert_eq!(out, vec!["--registry-aliases=foo=bar"]);
    }

    #[test]
    fn include_forks_true_rewrites_to_fork_processing_enabled() {
        let out = migrate(&["--include-forks=true"]);
        assert_eq!(out, vec!["--fork-processing=enabled"]);
    }

    #[test]
    fn bare_include_forks_rewrites_to_fork_processing_enabled() {
        let out = migrate(&["--include-forks"]);
        assert_eq!(out, vec!["--fork-processing=enabled"]);
    }

    #[test]
    fn recreate_closed_false_becomes_recreate_when_auto() {
        let out = migrate(&["--recreate-closed=false"]);
        assert_eq!(out, vec!["--recreate-when=auto"]);
    }

    #[test]
    fn recreate_closed_true_becomes_recreate_when_always() {
        let out = migrate(&["--recreate-closed=true"]);
        assert_eq!(out, vec!["--recreate-when=always"]);
    }

    #[test]
    fn bare_recreate_closed_becomes_recreate_when_always() {
        let out = migrate(&["--recreate-closed"]);
        assert_eq!(out, vec!["--recreate-when=always"]);
    }

    #[test]
    fn drops_git_fs_family_entirely() {
        let out = migrate(&["--git-fs", "--git-fs=foo", "--git-fs-something", "keep-me"]);
        assert_eq!(out, vec!["keep-me"]);
    }

    #[test]
    fn migrations_compose_across_an_argv() {
        let out = migrate(&[
            "renovate",
            "--dry-run",
            "--include-forks=true",
            "--azure-auto-complete=false",
            "--git-fs=ignored",
            "myrepo",
        ]);
        assert_eq!(
            out,
            vec![
                "renovate",
                "--dry-run=true",
                "--fork-processing=enabled",
                "--platform-automerge=false",
                "myrepo",
            ],
        );
    }
}
