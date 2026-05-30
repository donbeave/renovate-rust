# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/file.spec.ts
**Total tests:** 15 | **Ported:** 8 | **Actionable:** 8 | **Status:** partial

### `workers/global/config/parse/file › .getConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses %s > %s | 27 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |
| migrates | 56 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |
| warns if config is invalid | 68 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |
| parse and returns empty config if there is no RENOVATE_CONFIG_FILE in env | 80 | ported | `file.rs` | `resolve_returns_none_when_env_not_set` | — |
| fatal error and exit if error in parsing %s | 84 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |
| fatal error and exit if custom config file does not exist | 112 | ported | `file.rs` | `resolve_errors_when_explicit_file_missing` | — |
| fatal error and exit if config.js contains unresolved env var | 126 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |
| fatal error and exit if %s | 147 | ported | `file.rs` | `load_rejects_unsupported_or_missing_extension` | — |
| exports env variables to environment from processEnv object | 161 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |
| does not export env variables to environment from processEnv object if key/value is invalid | 184 | not-applicable | — | — | JS config files / env var resolution (TS-specific) |

### `workers/global/config/parse/file › deleteConfigFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip when RENOVATE_CONFIG_FILE is not set ("%s") | 214 | ported | `file.rs` | `delete_non_default_config_skips_when_env_not_set` | — |
| skip when config file does not exist | 226 | ported | `file.rs` | `delete_non_default_config_skips_missing_file` | — |
| skip if deleteConfigFile is not set ("%s") | 239 | ported | `file.rs` | `delete_non_default_config_skips_when_flag_is_false` | — |
| removes the specified config file | 255 | ported | `file.rs` | `delete_non_default_config_removes_file` | — |
| fails silently when attempting to delete the config file | 278 | ported | `file.rs` | `delete_non_default_config_fails_silently` | — |

---
