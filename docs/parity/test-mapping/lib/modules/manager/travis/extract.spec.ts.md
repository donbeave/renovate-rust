# `lib/modules/manager/travis/extract.spec.ts`

[← `manager/travis`](../../../../_by-module/manager/travis.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | returns empty if fails to parse | ported | `crates/renovate-core/src/extractors/travis.rs:142` |
| 18 | returns results | ported | `crates/renovate-core/src/extractors/travis.rs:110` |
| 24 | should handle invalid yaml | ported | `crates/renovate-core/src/extractors/travis.rs:196` |
| 29 | handles matrix node_js syntax with node_js string | ported | `crates/renovate-core/src/extractors/travis.rs:161` |
| 42 | handles matrix node_js syntax with node_js array | ported | `crates/renovate-core/src/extractors/travis.rs:203` |
| 60 | handles matrix node_js syntax with node_js array 2 | ported | `crates/renovate-core/src/extractors/travis.rs:170` |
| 78 | handles matrix node_js syntax with alias | ported | `crates/renovate-core/src/extractors/travis.rs:180` |
| 91 | handles invalid matrix node_js syntax | ported | `crates/renovate-core/src/extractors/travis.rs:189` |

