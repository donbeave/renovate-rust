# `lib/modules/manager/homebrew/handlers/index.spec.ts`

[← `manager/homebrew`](../../../../../_by-module/manager/homebrew.md) · [all modules](../../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | _(it.each / template — verify manually)_ | ? | — |
| 9 | returns github handler for github type | ported | `crates/renovate-core/src/extractors/homebrew.rs:1181` |
| 16 | returns null for null url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1189` |
| 20 | returns null for unsupported url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1195` |
| 24 | returns handler and parsed result for github url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1201` |

