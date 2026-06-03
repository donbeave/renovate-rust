# `lib/util/package-rules/repositories.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | should return null if match repositories is not defined | ported | `crates/renovate-core/src/package_rule.rs:1590` |
| 19 | should return false if repository is not defined | ported | `crates/renovate-core/src/package_rule.rs:1598` |
| 31 | should return true if repository matches regex pattern | ported | `crates/renovate-core/src/package_rule.rs:1607` |
| 43 | should return false if repository has invalid regex pattern | ported | `crates/renovate-core/src/package_rule.rs:1615` |
| 55 | should return false if repository does not match regex pattern | ported | `crates/renovate-core/src/package_rule.rs:1623` |
| 67 | should return true if repository matches minimatch pattern | ported | `crates/renovate-core/src/package_rule.rs:1631` |
| 79 | should return false if repository does not match minimatch pattern | ported | `crates/renovate-core/src/package_rule.rs:1639` |
| 91 | should return true if repository matches at least one pattern | ported | `crates/renovate-core/src/package_rule.rs:1647` |
| 105 | should return false if exclude repository is not defined | ported | `crates/renovate-core/src/package_rule.rs:1655` |
| 117 | should return false if exclude repository matches regex pattern | ported | `crates/renovate-core/src/package_rule.rs:1664` |
| 129 | should return true if exclude repository has invalid regex pattern | ported | `crates/renovate-core/src/package_rule.rs:1672` |
| 141 | should return true if exclude repository does not match regex pattern | ported | `crates/renovate-core/src/package_rule.rs:1680` |
| 153 | should return false if exclude repository matches minimatch pattern | ported | `crates/renovate-core/src/package_rule.rs:1688` |
| 165 | should return true if exclude repository does not match minimatch pattern | ported | `crates/renovate-core/src/package_rule.rs:1696` |
| 177 | should return false if exclude repository matches at least one pattern | ported | `crates/renovate-core/src/package_rule.rs:1704` |

