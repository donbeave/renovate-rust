# `lib/modules/manager/helm-values/extract.spec.ts`

[← `manager/helm-values`](../../../../_by-module/manager/helm-values.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | returns null for invalid yaml file content | ported | [`crates/renovate-core/src/extractors/helm_values.rs:348`](../../../../../../../crates/renovate-core/src/extractors/helm_values.rs#L348) |
| 31 | returns null for empty yaml file content | ported | [`crates/renovate-core/src/extractors/helm_values.rs:342`](../../../../../../../crates/renovate-core/src/extractors/helm_values.rs#L342) |
| 36 | extracts from values.yaml correctly with same structure as "helm create" | ported | [`crates/renovate-core/src/extractors/helm_values.rs:265`](../../../../../../../crates/renovate-core/src/extractors/helm_values.rs#L265) |
| 52 | extracts from complex values file correctly" | ported | [`crates/renovate-core/src/extractors/helm_values.rs:293`](../../../../../../../crates/renovate-core/src/extractors/helm_values.rs#L293) |
| 62 | extract data from file with multiple documents | ported | [`crates/renovate-core/src/extractors/helm_values.rs:364`](../../../../../../../crates/renovate-core/src/extractors/helm_values.rs#L364) |
| 85 | extract data from file with registry aliases | ported | [`crates/renovate-core/src/extractors/helm_values.rs:329`](../../../../../../../crates/renovate-core/src/extractors/helm_values.rs#L329) |

