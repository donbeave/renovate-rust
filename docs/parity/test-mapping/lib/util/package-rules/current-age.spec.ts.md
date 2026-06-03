# `lib/util/package-rules/current-age.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | returns false if release is older | ported | `crates/renovate-core/src/package_rule.rs:1550` |
| 30 | returns false if release is younger | ported | `crates/renovate-core/src/package_rule.rs:1558` |
| 42 | returns null if release invalid | ported | `crates/renovate-core/src/package_rule.rs:1566` |
| 54 | returns false if release undefined | ported | `crates/renovate-core/src/package_rule.rs:1574` |
| 66 | returns true if age matches | ported | `crates/renovate-core/src/package_rule.rs:1582` |

