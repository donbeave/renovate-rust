# `lib/util/interpolator.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | does nothing if not input | ported | [`crates/renovate-core/src/util.rs:10688`](../../../../../crates/renovate-core/src/util.rs#L10688) |
| 19 | does not throw error when keys and values are valid | ported | [`crates/renovate-core/src/util.rs:10694`](../../../../../crates/renovate-core/src/util.rs#L10694) |
| 25 | throws when input is not a valid object | ported | [`crates/renovate-core/src/util.rs:10702`](../../../../../crates/renovate-core/src/util.rs#L10702) |
| 31 | throws when keys do not follow specified regex patterns | ported | [`crates/renovate-core/src/util.rs:10710`](../../../../../crates/renovate-core/src/util.rs#L10710) |
| 40 | throws when values are not of type string | ported | [`crates/renovate-core/src/util.rs:10718`](../../../../../crates/renovate-core/src/util.rs#L10718) |
| 48 | replaces values and deletes secrets | ported | [`crates/renovate-core/src/config/secrets.rs:387`](../../../../../crates/renovate-core/src/config/secrets.rs#L387) |
| 97 | replaces values and keeps secrets | ported | [`crates/renovate-core/src/config/secrets.rs:413`](../../../../../crates/renovate-core/src/config/secrets.rs#L413) |
| 115 | does not resolve secrets in onboaringconfig | ported | [`crates/renovate-core/src/config/secrets.rs:425`](../../../../../crates/renovate-core/src/config/secrets.rs#L425) |
| 155 | throws error if secrets are used in disallowed options | ported | [`crates/renovate-core/src/config/secrets.rs:448`](../../../../../crates/renovate-core/src/config/secrets.rs#L448) |
| 175 | throws error if secret key is not present in config | ported | [`crates/renovate-core/src/config/secrets.rs:460`](../../../../../crates/renovate-core/src/config/secrets.rs#L460) |

