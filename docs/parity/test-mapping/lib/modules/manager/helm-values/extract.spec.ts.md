# `lib/modules/manager/helm-values/extract.spec.ts`

[← `manager/helm-values`](../../../../_by-module/manager/helm-values.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | returns null for invalid yaml file content | ported | `crates/renovate-core/src/extractors/helm_values.rs:348` |
| 31 | returns null for empty yaml file content | ported | `crates/renovate-core/src/extractors/helm_values.rs:342` |
| 36 | extracts from values.yaml correctly with same structure as "helm create" | ported | `crates/renovate-core/src/extractors/helm_values.rs:265` |
| 52 | extracts from complex values file correctly" | ported | `crates/renovate-core/src/extractors/helm_values.rs:293` |
| 62 | extract data from file with multiple documents | ported | `crates/renovate-core/src/extractors/helm_values.rs:364` |
| 85 | extract data from file with registry aliases | ported | `crates/renovate-core/src/extractors/helm_values.rs:329` |

