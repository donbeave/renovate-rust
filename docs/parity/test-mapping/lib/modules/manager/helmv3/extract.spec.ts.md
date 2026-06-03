# `lib/modules/manager/helmv3/extract.spec.ts`

[← `manager/helmv3`](../../../../_by-module/manager/helmv3.md) · [all modules](../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | skips invalid registry urls | ported | [`crates/renovate-core/src/extractors/helm.rs:959`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L959) |
| 40 | parses simple chart.yaml correctly | ported | [`crates/renovate-core/src/extractors/helm.rs:967`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L967) |
| 67 | extract correctly oci references | ported | [`crates/renovate-core/src/extractors/helm.rs:980`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L980) |
| 100 | resolves aliased registry urls | ported | [`crates/renovate-core/src/extractors/helm.rs:991`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L991) |
| 131 | doesn't fail if chart.yaml is invalid | ported | [`crates/renovate-core/src/extractors/helm.rs:1006`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1006) |
| 142 | skips local dependencies | ported | [`crates/renovate-core/src/extractors/helm.rs:1013`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1013) |
| 167 | returns null if no dependencies key | ported | [`crates/renovate-core/src/extractors/helm.rs:1022`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1022) |
| 183 | returns null if dependencies are an empty list | ported | [`crates/renovate-core/src/extractors/helm.rs:1029`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1029) |
| 199 | returns null if dependencies key is invalid | ported | [`crates/renovate-core/src/extractors/helm.rs:1036`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1036) |
| 215 | returns null if chart.yaml is empty | ported | [`crates/renovate-core/src/extractors/helm.rs:1043`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1043) |
| 222 | returns null if chart.yaml uses an unsupported apiversion | ported | [`crates/renovate-core/src/extractors/helm.rs:1049`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1049) |
| 235 | returns null if name and version are missing for all dependencies | ported | [`crates/renovate-core/src/extractors/helm.rs:1056`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L1056) |

