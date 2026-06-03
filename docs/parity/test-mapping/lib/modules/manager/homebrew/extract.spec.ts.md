# `lib/modules/manager/homebrew/extract.spec.ts`

[← `manager/homebrew`](../../../../_by-module/manager/homebrew.md) · [all modules](../../../../README.md)

**17/17 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | skips sourceforge dependency 1 | ported | `crates/renovate-core/src/extractors/homebrew.rs:709` |
| 32 | skips sourceforge dependency 2 | ported | `crates/renovate-core/src/extractors/homebrew.rs:749` |
| 54 | skips github dependency with wrong format | ported | `crates/renovate-core/src/extractors/homebrew.rs:763` |
| 77 | extracts "releases" github dependency | ported | `crates/renovate-core/src/extractors/homebrew.rs:680` |
| 99 | extracts "archive" github dependency | ported | `crates/renovate-core/src/extractors/homebrew.rs:641` |
| 121 | handles old "archive" github url format | ported | `crates/renovate-core/src/extractors/homebrew.rs:662` |
| 152 | handles no space before class header | ported | `crates/renovate-core/src/extractors/homebrew.rs:779` |
| 183 | returns null for invalid class header 1 | ported | `crates/renovate-core/src/extractors/homebrew.rs:741` |
| 198 | returns null for invalid class header 2 | ported | `crates/renovate-core/src/extractors/homebrew.rs:727` |
| 213 | skips if there is no url field | ported | `crates/renovate-core/src/extractors/homebrew.rs:720` |
| 235 | skips if invalid url protocol | ported | `crates/renovate-core/src/extractors/homebrew.rs:800` |
| 257 | skips if invalid url | ported | `crates/renovate-core/src/extractors/homebrew.rs:809` |
| 279 | skips if there is no sha256 field | ported | `crates/renovate-core/src/extractors/homebrew.rs:820` |
| 301 | skips if sha256 field is invalid | ported | `crates/renovate-core/src/extractors/homebrew.rs:698` |
| 323 | extracts npm scoped package dependency | ported | `crates/renovate-core/src/extractors/homebrew.rs:831` |
| 354 | extracts npm unscoped package dependency | ported | `crates/renovate-core/src/extractors/homebrew.rs:851` |
| 385 | skips npm package from custom registry | ported | `crates/renovate-core/src/extractors/homebrew.rs:870` |

