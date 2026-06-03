# `lib/util/package-rules/dep-names.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return false if packagefile is not defined | ported | [`crates/renovate-core/src/package_rule.rs:1359`](../../../../../../crates/renovate-core/src/package_rule.rs#L1359) |
| 19 | should return false if depname is excluded prefix | ported | [`crates/renovate-core/src/package_rule.rs:1367`](../../../../../../crates/renovate-core/src/package_rule.rs#L1367) |
| 42 | should return true if depname is included prefix | ported | [`crates/renovate-core/src/package_rule.rs:1377`](../../../../../../crates/renovate-core/src/package_rule.rs#L1377) |
| 65 | should return false if for wrong prefix | ported | [`crates/renovate-core/src/package_rule.rs:1387`](../../../../../../crates/renovate-core/src/package_rule.rs#L1387) |

