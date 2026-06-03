# `lib/util/package-rules/repositories.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**15/15 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | should return null if match repositories is not defined | ported | [`crates/renovate-core/src/package_rule.rs:1590`](../../../../../../crates/renovate-core/src/package_rule.rs#L1590) |
| 19 | should return false if repository is not defined | ported | [`crates/renovate-core/src/package_rule.rs:1598`](../../../../../../crates/renovate-core/src/package_rule.rs#L1598) |
| 31 | should return true if repository matches regex pattern | ported | [`crates/renovate-core/src/package_rule.rs:1607`](../../../../../../crates/renovate-core/src/package_rule.rs#L1607) |
| 43 | should return false if repository has invalid regex pattern | ported | [`crates/renovate-core/src/package_rule.rs:1615`](../../../../../../crates/renovate-core/src/package_rule.rs#L1615) |
| 55 | should return false if repository does not match regex pattern | ported | [`crates/renovate-core/src/package_rule.rs:1623`](../../../../../../crates/renovate-core/src/package_rule.rs#L1623) |
| 67 | should return true if repository matches minimatch pattern | ported | [`crates/renovate-core/src/package_rule.rs:1631`](../../../../../../crates/renovate-core/src/package_rule.rs#L1631) |
| 79 | should return false if repository does not match minimatch pattern | ported | [`crates/renovate-core/src/package_rule.rs:1639`](../../../../../../crates/renovate-core/src/package_rule.rs#L1639) |
| 91 | should return true if repository matches at least one pattern | ported | [`crates/renovate-core/src/package_rule.rs:1647`](../../../../../../crates/renovate-core/src/package_rule.rs#L1647) |
| 105 | should return false if exclude repository is not defined | ported | [`crates/renovate-core/src/package_rule.rs:1655`](../../../../../../crates/renovate-core/src/package_rule.rs#L1655) |
| 117 | should return false if exclude repository matches regex pattern | ported | [`crates/renovate-core/src/package_rule.rs:1664`](../../../../../../crates/renovate-core/src/package_rule.rs#L1664) |
| 129 | should return true if exclude repository has invalid regex pattern | ported | [`crates/renovate-core/src/package_rule.rs:1672`](../../../../../../crates/renovate-core/src/package_rule.rs#L1672) |
| 141 | should return true if exclude repository does not match regex pattern | ported | [`crates/renovate-core/src/package_rule.rs:1680`](../../../../../../crates/renovate-core/src/package_rule.rs#L1680) |
| 153 | should return false if exclude repository matches minimatch pattern | ported | [`crates/renovate-core/src/package_rule.rs:1688`](../../../../../../crates/renovate-core/src/package_rule.rs#L1688) |
| 165 | should return true if exclude repository does not match minimatch pattern | ported | [`crates/renovate-core/src/package_rule.rs:1696`](../../../../../../crates/renovate-core/src/package_rule.rs#L1696) |
| 177 | should return false if exclude repository matches at least one pattern | ported | [`crates/renovate-core/src/package_rule.rs:1704`](../../../../../../crates/renovate-core/src/package_rule.rs#L1704) |

