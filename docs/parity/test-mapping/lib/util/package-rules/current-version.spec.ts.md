# `lib/util/package-rules/current-version.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns true for null versioning | ported | [`crates/renovate-core/src/package_rule.rs:1712`](../../../../../../crates/renovate-core/src/package_rule.rs#L1712) |
| 22 | return false on version exception | ported | [`crates/renovate-core/src/package_rule.rs:1736`](../../../../../../crates/renovate-core/src/package_rule.rs#L1736) |
| 39 | return true for a valid match | ported | [`crates/renovate-core/src/package_rule.rs:1720`](../../../../../../crates/renovate-core/src/package_rule.rs#L1720) |
| 52 | return false if no version could be found | ported | [`crates/renovate-core/src/package_rule.rs:1728`](../../../../../../crates/renovate-core/src/package_rule.rs#L1728) |
| 66 | case insensitive match | ported | [`crates/renovate-core/src/package_rule.rs:1748`](../../../../../../crates/renovate-core/src/package_rule.rs#L1748) |
| 79 | return false for regex version non match | ported | [`crates/renovate-core/src/package_rule.rs:1756`](../../../../../../crates/renovate-core/src/package_rule.rs#L1756) |
| 93 | return true for regex version match | ported | [`crates/renovate-core/src/package_rule.rs:1764`](../../../../../../crates/renovate-core/src/package_rule.rs#L1764) |
| 107 | return false for regex value match | ported | [`crates/renovate-core/src/package_rule.rs:1772`](../../../../../../crates/renovate-core/src/package_rule.rs#L1772) |
| 120 | return true for same-major verisioning if version lies in expected range | ported | [`crates/renovate-core/src/package_rule.rs:1780`](../../../../../../crates/renovate-core/src/package_rule.rs#L1780) |
| 133 | return false for same-major verisioning if version lies outside of expected range | ported | [`crates/renovate-core/src/package_rule.rs:1789`](../../../../../../crates/renovate-core/src/package_rule.rs#L1789) |

