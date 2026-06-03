# `lib/util/package-rules/current-version.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**9/10 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns true for null versioning | ported | [`crates/renovate-core/src/package_rule.rs:1712`](../../../../../../crates/renovate-core/src/package_rule.rs#L1712) |
| 22 | return false on version exception | pending | — |
| 39 | return true for a valid match | ported | [`crates/renovate-core/src/package_rule.rs:1720`](../../../../../../crates/renovate-core/src/package_rule.rs#L1720) |
| 52 | return false if no version could be found | ported | [`crates/renovate-core/src/package_rule.rs:1728`](../../../../../../crates/renovate-core/src/package_rule.rs#L1728) |
| 66 | case insensitive match | ported | [`crates/renovate-core/src/package_rule.rs:1736`](../../../../../../crates/renovate-core/src/package_rule.rs#L1736) |
| 79 | return false for regex version non match | ported | [`crates/renovate-core/src/package_rule.rs:1744`](../../../../../../crates/renovate-core/src/package_rule.rs#L1744) |
| 93 | return true for regex version match | ported | [`crates/renovate-core/src/package_rule.rs:1752`](../../../../../../crates/renovate-core/src/package_rule.rs#L1752) |
| 107 | return false for regex value match | ported | [`crates/renovate-core/src/package_rule.rs:1760`](../../../../../../crates/renovate-core/src/package_rule.rs#L1760) |
| 120 | return true for same-major verisioning if version lies in expected range | ported | [`crates/renovate-core/src/package_rule.rs:1768`](../../../../../../crates/renovate-core/src/package_rule.rs#L1768) |
| 133 | return false for same-major verisioning if version lies outside of expected range | ported | [`crates/renovate-core/src/package_rule.rs:1777`](../../../../../../crates/renovate-core/src/package_rule.rs#L1777) |

