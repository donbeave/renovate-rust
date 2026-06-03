# `lib/modules/manager/vendir/extract.spec.ts`

[← `manager/vendir`](../../../../_by-module/manager/vendir.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns null for invalid yaml file content | ported | `crates/renovate-core/src/extractors/vendir.rs:167` |
| 15 | returns null for empty yaml file content | ported | `crates/renovate-core/src/extractors/vendir.rs:154` |
| 20 | returns null for empty directories key | ported | `crates/renovate-core/src/extractors/vendir.rs:160` |
| 30 | returns null for nonhelmchart key | ported | `crates/renovate-core/src/extractors/vendir.rs:174` |
| 35 | multiple charts - extracts helm-chart from vendir.yml correctly | ported | `crates/renovate-core/src/extractors/vendir.rs:133` |

