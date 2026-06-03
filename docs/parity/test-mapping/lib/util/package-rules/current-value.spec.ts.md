# `lib/util/package-rules/current-value.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | return true for exact match | ported | `crates/renovate-core/src/package_rule.rs:1395` |
| 19 | return true for glob match | ported | `crates/renovate-core/src/package_rule.rs:1403` |
| 31 | return false for glob non match | ported | `crates/renovate-core/src/package_rule.rs:1411` |
| 43 | return false for regex version non match | ported | `crates/renovate-core/src/package_rule.rs:1419` |
| 55 | case insensitive match | ported | `crates/renovate-core/src/package_rule.rs:1427` |
| 67 | return true for regex version match | ported | `crates/renovate-core/src/package_rule.rs:1435` |
| 79 | return false for now value | ported | `crates/renovate-core/src/package_rule.rs:1443` |

