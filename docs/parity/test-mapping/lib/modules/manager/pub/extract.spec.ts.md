# `lib/modules/manager/pub/extract.spec.ts`

[← `manager/pub`](../../../../_by-module/manager/pub.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null for invalid pubspec file | ported | `crates/renovate-core/src/extractors/pubspec.rs:687` |
| 16 | returns dart sdk only | ported | `crates/renovate-core/src/extractors/pubspec.rs:694` |
| 33 | returns valid dependencies | ported | `crates/renovate-core/src/extractors/pubspec.rs:706` |

