# `lib/modules/manager/pep621/extract.spec.ts`

[← `manager/pep621`](../../../../_by-module/manager/pep621.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | should return null for empty content | ported | `crates/renovate-core/src/extractors/pep621.rs:1117` |
| 21 | should return null for invalid toml | ported | `crates/renovate-core/src/extractors/pep621.rs:1124` |
| 32 | should return dependencies for valid content | ported | `crates/renovate-core/src/extractors/pep621.rs:660` |
| 233 | should return dependencies with overwritten pypi registryurl | ported | `crates/renovate-core/src/extractors/pep621.rs:991` |
| 309 | should return dependencies with original pypi registryurl | ported | `crates/renovate-core/src/extractors/pep621.rs:963` |
| 340 | should skip dependencies with unsupported uv sources | ported | `crates/renovate-core/src/extractors/pep621.rs:787` |
| 412 | should handle ssh git urls correctly for github sources | ported | `crates/renovate-core/src/extractors/pep621.rs:860` |
| 446 | should extract dependencies from hatch environments | ported | `crates/renovate-core/src/extractors/pep621.rs:897` |
| 498 | should extract project version | ported | `crates/renovate-core/src/extractors/pep621.rs:1131` |
| 510 | should extract dependencies from build-system.requires | ported | `crates/renovate-core/src/extractors/pep621.rs:1148` |
| 551 | should resolve lockedversions from pdm.lock | ported | `crates/renovate-core/src/extractors/pep621.rs:1178` |
| 595 | should resolve lockedversions from uv.lock | ported | `crates/renovate-core/src/extractors/pep621.rs:1218` |
| 661 | should resolve dependencies without locked versions on invalid uv.lock | ported | `crates/renovate-core/src/extractors/pep621.rs:1254` |
| 694 | should resolve dependencies with template | ported | `crates/renovate-core/src/extractors/pep621.rs:1270` |

