# `lib/modules/datasource/kubernetes-api/index.spec.ts`

[← `datasource/kubernetes-api`](../../../../_by-module/datasource/kubernetes-api.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null for an unknown kubernetes api type | ported | [`crates/renovate-core/src/datasources/kubernetes_api.rs:56`](../../../../../../../crates/renovate-core/src/datasources/kubernetes_api.rs#L56) |
| 13 | returns for a known kubernetes api type | ported | [`crates/renovate-core/src/datasources/kubernetes_api.rs:62`](../../../../../../../crates/renovate-core/src/datasources/kubernetes_api.rs#L62) |
| 27 | is case sensitive | ported | [`crates/renovate-core/src/datasources/kubernetes_api.rs:71`](../../../../../../../crates/renovate-core/src/datasources/kubernetes_api.rs#L71) |

