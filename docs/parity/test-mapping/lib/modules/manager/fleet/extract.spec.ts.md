# `lib/modules/manager/fleet/extract.spec.ts`

[← `manager/fleet`](../../../../_by-module/manager/fleet.md) · [all modules](../../../../README.md)

**8/10 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | should return null if empty content | ported | `crates/renovate-core/src/extractors/fleet.rs:682` |
| 30 | should return null if a unknown manifest is supplied | ported | `crates/renovate-core/src/extractors/fleet.rs:497` |
| 37 | should return null if content is a malformed yaml | ported | `crates/renovate-core/src/extractors/fleet.rs:699` |
| 49 | should parse valid configuration | ported | `crates/renovate-core/src/extractors/fleet.rs:506` |
| 88 | should support registryalias configuration | ported | `crates/renovate-core/src/extractors/fleet.rs:524` |
| 132 | should parse valid configuration with target customization | ported | `crates/renovate-core/src/extractors/fleet.rs:576` |
| 208 | should parse parse invalid configurations | ported | `crates/renovate-core/src/extractors/fleet.rs:602` |
| 242 | should return null if content is a malformed yaml | ported | `crates/renovate-core/src/extractors/fleet.rs:699` |
| 254 | should parse valid configuration | ported | `crates/renovate-core/src/extractors/fleet.rs:506` |
| 276 | should parse invalid configuration | ported | `crates/renovate-core/src/extractors/fleet.rs:653` |

