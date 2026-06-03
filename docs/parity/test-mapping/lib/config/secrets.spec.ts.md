# `lib/config/secrets.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**13/13 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | works with default config | ported | [`crates/renovate-core/src/config/secrets.rs:200`](../../../../../crates/renovate-core/src/config/secrets.rs#L200) |
| 20 | returns if no secrets/variables | ported | [`crates/renovate-core/src/config/secrets.rs:206`](../../../../../crates/renovate-core/src/config/secrets.rs#L206) |
| 24 | throws for invalid secret name | ported | [`crates/renovate-core/src/config/secrets.rs:212`](../../../../../crates/renovate-core/src/config/secrets.rs#L212) |
| 32 | throws for invalid variable name | ported | [`crates/renovate-core/src/config/secrets.rs:221`](../../../../../crates/renovate-core/src/config/secrets.rs#L221) |
| 40 | throws for secrets in repositories | ported | [`crates/renovate-core/src/config/secrets.rs:230`](../../../../../crates/renovate-core/src/config/secrets.rs#L230) |
| 48 | throws for variables in repositories | ported | [`crates/renovate-core/src/config/secrets.rs:241`](../../../../../crates/renovate-core/src/config/secrets.rs#L241) |
| 58 | replaces both secrets and variables | ported | [`crates/renovate-core/src/config/secrets.rs:252`](../../../../../crates/renovate-core/src/config/secrets.rs#L252) |
| 75 | replaces all secrets and variables | ported | [`crates/renovate-core/src/config/secrets.rs:272`](../../../../../crates/renovate-core/src/config/secrets.rs#L272) |
| 94 | handles a mix of space characters around the curly braces | ported | [`crates/renovate-core/src/config/secrets.rs:295`](../../../../../crates/renovate-core/src/config/secrets.rs#L295) |
| 111 | does not handle non-space characters around the curly braces | ported | [`crates/renovate-core/src/config/secrets.rs:315`](../../../../../crates/renovate-core/src/config/secrets.rs#L315) |
| 128 | preserves secrets and variables if delete flags are false | ported | [`crates/renovate-core/src/config/secrets.rs:335`](../../../../../crates/renovate-core/src/config/secrets.rs#L335) |
| 151 | throws if secret is missing | ported | [`crates/renovate-core/src/config/secrets.rs:355`](../../../../../crates/renovate-core/src/config/secrets.rs#L355) |
| 160 | throws if variable is missing | ported | [`crates/renovate-core/src/config/secrets.rs:368`](../../../../../crates/renovate-core/src/config/secrets.rs#L368) |

