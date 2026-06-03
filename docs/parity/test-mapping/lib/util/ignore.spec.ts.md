# `lib/util/ignore.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**4/5 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns true for "renovate:ignore" comments | ported | `crates/renovate-core/src/string_match.rs:579` |
| 15 | returns false for comments not starting with "renovate:" or "pyup:" | ported | `crates/renovate-core/src/string_match.rs:591` |
| 19 | returns false for "renovate:" comments without "ignore" | ported | `crates/renovate-core/src/string_match.rs:597` |
| 23 | logs unknown command for "renovate:" comments without "ignore" | pending | — |
| 31 | returns false when comment is undefined | ported | `crates/renovate-core/src/string_match.rs:603` |

