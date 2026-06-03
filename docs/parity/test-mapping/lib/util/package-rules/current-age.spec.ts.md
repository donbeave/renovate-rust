# `lib/util/package-rules/current-age.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | returns false if release is older | ported | [`crates/renovate-core/src/package_rule.rs:1550`](../../../../../../crates/renovate-core/src/package_rule.rs#L1550) |
| 30 | returns false if release is younger | ported | [`crates/renovate-core/src/package_rule.rs:1558`](../../../../../../crates/renovate-core/src/package_rule.rs#L1558) |
| 42 | returns null if release invalid | ported | [`crates/renovate-core/src/package_rule.rs:1566`](../../../../../../crates/renovate-core/src/package_rule.rs#L1566) |
| 54 | returns false if release undefined | ported | [`crates/renovate-core/src/package_rule.rs:1574`](../../../../../../crates/renovate-core/src/package_rule.rs#L1574) |
| 66 | returns true if age matches | ported | [`crates/renovate-core/src/package_rule.rs:1582`](../../../../../../crates/renovate-core/src/package_rule.rs#L1582) |

