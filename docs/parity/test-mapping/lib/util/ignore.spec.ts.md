# `lib/util/ignore.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | returns true for "renovate:ignore" comments | ported | [`crates/renovate-core/src/string_match.rs:588`](../../../../../crates/renovate-core/src/string_match.rs#L588) |
| 15 | returns false for comments not starting with "renovate:" or "pyup:" | ported | [`crates/renovate-core/src/string_match.rs:600`](../../../../../crates/renovate-core/src/string_match.rs#L600) |
| 19 | returns false for "renovate:" comments without "ignore" | ported | [`crates/renovate-core/src/string_match.rs:606`](../../../../../crates/renovate-core/src/string_match.rs#L606) |
| 23 | logs unknown command for "renovate:" comments without "ignore" | ported | [`crates/renovate-core/src/string_match.rs:612`](../../../../../crates/renovate-core/src/string_match.rs#L612) |
| 31 | returns false when comment is undefined | ported | [`crates/renovate-core/src/string_match.rs:619`](../../../../../crates/renovate-core/src/string_match.rs#L619) |

