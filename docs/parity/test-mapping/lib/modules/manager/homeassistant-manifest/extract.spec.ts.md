# `lib/modules/manager/homeassistant-manifest/extract.spec.ts`

[← `manager/homeassistant-manifest`](../../../../_by-module/manager/homeassistant-manifest.md) · [all modules](../../../../README.md)

**16/16 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | returns null for invalid json | ported | `crates/renovate-core/src/extractors/homeassistant.rs:166` |
| 14 | returns null for non-home assistant manifest (missing domain) | ported | `crates/renovate-core/src/extractors/homeassistant.rs:186` |
| 24 | returns null for non-home assistant manifest (missing name) | ported | `crates/renovate-core/src/extractors/homeassistant.rs:193` |
| 34 | returns null for chrome extension manifest | ported | `crates/renovate-core/src/extractors/homeassistant.rs:201` |
| 45 | returns null for empty requirements | ported | `crates/renovate-core/src/extractors/homeassistant.rs:172` |
| 55 | returns null when no requirements field | ported | `crates/renovate-core/src/extractors/homeassistant.rs:179` |
| 64 | extracts single requirement with exact version | ported | `crates/renovate-core/src/extractors/homeassistant.rs:208` |
| 84 | extracts multiple requirements | ported | `crates/renovate-core/src/extractors/homeassistant.rs:150` |
| 118 | handles requirements with extras | ported | `crates/renovate-core/src/extractors/homeassistant.rs:230` |
| 138 | extracts git+https requirements | ported | `crates/renovate-core/src/extractors/homeassistant.rs:240` |
| 168 | supports requirements with other operators | ported | `crates/renovate-core/src/extractors/homeassistant.rs:158` |
| 211 | handles requirements without version | ported | `crates/renovate-core/src/extractors/homeassistant.rs:262` |
| 237 | extracts from real-world asuswrt manifest | ported | `crates/renovate-core/src/extractors/homeassistant.rs:279` |
| 272 | handles invalid requirement types in array | ported | `crates/renovate-core/src/extractors/homeassistant.rs:295` |
| 299 | returns null when requirements is not an array | ported | `crates/renovate-core/src/extractors/homeassistant.rs:306` |
| 313 | handles unparseable requirement strings with skipreason | ported | `crates/renovate-core/src/extractors/homeassistant.rs:313` |

