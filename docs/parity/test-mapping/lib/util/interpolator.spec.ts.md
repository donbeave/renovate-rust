# `lib/util/interpolator.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | does nothing if not input | ported | [`crates/renovate-core/src/util.rs:9105`](../../../../../crates/renovate-core/src/util.rs#L9105) |
| 19 | does not throw error when keys and values are valid | ported | [`crates/renovate-core/src/util.rs:9111`](../../../../../crates/renovate-core/src/util.rs#L9111) |
| 25 | throws when input is not a valid object | ported | [`crates/renovate-core/src/util.rs:9119`](../../../../../crates/renovate-core/src/util.rs#L9119) |
| 31 | throws when keys do not follow specified regex patterns | ported | [`crates/renovate-core/src/util.rs:9127`](../../../../../crates/renovate-core/src/util.rs#L9127) |
| 40 | throws when values are not of type string | ported | [`crates/renovate-core/src/util.rs:9135`](../../../../../crates/renovate-core/src/util.rs#L9135) |
| 48 | replaces values and deletes secrets | ported | [`crates/renovate-core/src/config/secrets.rs:383`](../../../../../crates/renovate-core/src/config/secrets.rs#L383) |
| 97 | replaces values and keeps secrets | ported | [`crates/renovate-core/src/config/secrets.rs:409`](../../../../../crates/renovate-core/src/config/secrets.rs#L409) |
| 115 | does not resolve secrets in onboaringconfig | ported | [`crates/renovate-core/src/config/secrets.rs:421`](../../../../../crates/renovate-core/src/config/secrets.rs#L421) |
| 155 | throws error if secrets are used in disallowed options | ported | [`crates/renovate-core/src/config/secrets.rs:444`](../../../../../crates/renovate-core/src/config/secrets.rs#L444) |
| 175 | throws error if secret key is not present in config | ported | [`crates/renovate-core/src/config/secrets.rs:456`](../../../../../crates/renovate-core/src/config/secrets.rs#L456) |

