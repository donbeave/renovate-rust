# `lib/modules/manager/helm-requirements/extract.spec.ts`

[← `manager/helm-requirements`](../../../../_by-module/manager/helm-requirements.md) · [all modules](../../../../README.md)

**11/11 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | ensure that currentvalue is string | ported | [`crates/renovate-core/src/extractors/helm.rs:615`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L615) |
| 34 | skips invalid registry urls | ported | [`crates/renovate-core/src/extractors/helm.rs:602`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L602) |
| 64 | parses simple requirements.yaml correctly | ported | [`crates/renovate-core/src/extractors/helm.rs:562`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L562) |
| 96 | parses simple requirements.yaml but skips if necessary fields missing | ported | [`crates/renovate-core/src/extractors/helm.rs:775`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L775) |
| 112 | resolves aliased registry urls | ported | [`crates/renovate-core/src/extractors/helm.rs:587`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L587) |
| 141 | skips local dependencies | ported | [`crates/renovate-core/src/extractors/helm.rs:744`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L744) |
| 172 | returns null if no dependencies | ported | [`crates/renovate-core/src/extractors/helm.rs:690`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L690) |
| 192 | returns null if requirements.yaml is invalid | ported | [`crates/renovate-core/src/extractors/helm.rs:765`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L765) |
| 214 | returns null if chart.yaml is empty | ported | [`crates/renovate-core/src/extractors/helm.rs:782`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L782) |
| 279 | validates ${params.fieldname} is required | ported | [`crates/renovate-core/src/extractors/helm.rs:628`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L628) |
| 293 | skips only invalid dependences | ported | [`crates/renovate-core/src/extractors/helm.rs:802`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L802) |

