# `lib/config/secrets.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**13/13 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | works with default config | ported | [`crates/renovate-core/src/config/secrets.rs:204`](../../../../../crates/renovate-core/src/config/secrets.rs#L204) |
| 20 | returns if no secrets/variables | ported | [`crates/renovate-core/src/config/secrets.rs:210`](../../../../../crates/renovate-core/src/config/secrets.rs#L210) |
| 24 | throws for invalid secret name | ported | [`crates/renovate-core/src/config/secrets.rs:216`](../../../../../crates/renovate-core/src/config/secrets.rs#L216) |
| 32 | throws for invalid variable name | ported | [`crates/renovate-core/src/config/secrets.rs:225`](../../../../../crates/renovate-core/src/config/secrets.rs#L225) |
| 40 | throws for secrets in repositories | ported | [`crates/renovate-core/src/config/secrets.rs:234`](../../../../../crates/renovate-core/src/config/secrets.rs#L234) |
| 48 | throws for variables in repositories | ported | [`crates/renovate-core/src/config/secrets.rs:245`](../../../../../crates/renovate-core/src/config/secrets.rs#L245) |
| 58 | replaces both secrets and variables | ported | [`crates/renovate-core/src/config/secrets.rs:256`](../../../../../crates/renovate-core/src/config/secrets.rs#L256) |
| 75 | replaces all secrets and variables | ported | [`crates/renovate-core/src/config/secrets.rs:276`](../../../../../crates/renovate-core/src/config/secrets.rs#L276) |
| 94 | handles a mix of space characters around the curly braces | ported | [`crates/renovate-core/src/config/secrets.rs:299`](../../../../../crates/renovate-core/src/config/secrets.rs#L299) |
| 111 | does not handle non-space characters around the curly braces | ported | [`crates/renovate-core/src/config/secrets.rs:319`](../../../../../crates/renovate-core/src/config/secrets.rs#L319) |
| 128 | preserves secrets and variables if delete flags are false | ported | [`crates/renovate-core/src/config/secrets.rs:339`](../../../../../crates/renovate-core/src/config/secrets.rs#L339) |
| 151 | throws if secret is missing | ported | [`crates/renovate-core/src/config/secrets.rs:359`](../../../../../crates/renovate-core/src/config/secrets.rs#L359) |
| 160 | throws if variable is missing | ported | [`crates/renovate-core/src/config/secrets.rs:372`](../../../../../crates/renovate-core/src/config/secrets.rs#L372) |

