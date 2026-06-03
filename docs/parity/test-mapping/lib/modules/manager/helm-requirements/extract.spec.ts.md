# `lib/modules/manager/helm-requirements/extract.spec.ts`

[← `manager/helm-requirements`](../../../../_by-module/manager/helm-requirements.md) · [all modules](../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | ensure that currentvalue is string | ported | `crates/renovate-core/src/extractors/helm.rs:615` |
| 34 | skips invalid registry urls | ported | `crates/renovate-core/src/extractors/helm.rs:602` |
| 64 | parses simple requirements.yaml correctly | ported | `crates/renovate-core/src/extractors/helm.rs:562` |
| 96 | parses simple requirements.yaml but skips if necessary fields missing | ported | `crates/renovate-core/src/extractors/helm.rs:775` |
| 112 | resolves aliased registry urls | ported | `crates/renovate-core/src/extractors/helm.rs:587` |
| 141 | skips local dependencies | ported | `crates/renovate-core/src/extractors/helm.rs:744` |
| 172 | returns null if no dependencies | ported | `crates/renovate-core/src/extractors/helm.rs:690` |
| 192 | returns null if requirements.yaml is invalid | ported | `crates/renovate-core/src/extractors/helm.rs:765` |
| 214 | returns null if chart.yaml is empty | ported | `crates/renovate-core/src/extractors/helm.rs:782` |
| 279 | validates ${params.fieldname} is required | ported | `crates/renovate-core/src/extractors/helm.rs:628` |
| 293 | skips only invalid dependences | ported | `crates/renovate-core/src/extractors/helm.rs:802` |

