# `lib/modules/manager/gleam/extract.spec.ts`

[← `manager/gleam`](../../../../_by-module/manager/gleam.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | should extract dev and prod dependencies | ported | `crates/renovate-core/src/extractors/gleam.rs:261` |
| 41 | should extract dev only dependencies | ported | `crates/renovate-core/src/extractors/gleam.rs:275` |
| 65 | should return null when no dependencies are found | ported | `crates/renovate-core/src/extractors/gleam.rs:308` |
| 82 | should return null when gleam.toml is invalid | ported | `crates/renovate-core/src/extractors/gleam.rs:302` |
| 91 | should return locked versions | ported | `crates/renovate-core/src/extractors/gleam.rs:362` |
| 119 | should fail to extract locked version | ported | `crates/renovate-core/src/extractors/gleam.rs:375` |
| 138 | should fail to find locked version in range | ported | `crates/renovate-core/src/extractors/gleam.rs:386` |
| 166 | should handle invalid versions in lock file | ported | `crates/renovate-core/src/extractors/gleam.rs:395` |
| 193 | should handle lock file parsing and extracting errors | ported | `crates/renovate-core/src/extractors/gleam.rs:404` |

