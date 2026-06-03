# `lib/modules/datasource/elm-package/index.spec.ts`

[← `datasource/elm-package`](../../../../_by-module/datasource/elm-package.md) · [all modules](../../../../README.md)

**4/10 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/elm_package.rs:150`](../../../../../../../crates/renovate-core/src/datasources/elm_package.rs#L150) |
| 32 | returns null for 404 | pending | — |
| 45 | throws for 5xx | pending | — |
| 58 | throws for 429 | pending | — |
| 71 | returns null for invalid json response | pending | — |
| 84 | returns null for unknown error | pending | — |
| 97 | processes real data | ported | [`crates/renovate-core/src/datasources/elm_package.rs:122`](../../../../../../../crates/renovate-core/src/datasources/elm_package.rs#L122) |
| 120 | returns null when registryurl is not provided | pending | — |
| 129 | returns null for invalid schema response | ported | [`crates/renovate-core/src/datasources/elm_package.rs:157`](../../../../../../../crates/renovate-core/src/datasources/elm_package.rs#L157) |
| 142 | handles package without slash in name | ported | [`crates/renovate-core/src/datasources/elm_package.rs:165`](../../../../../../../crates/renovate-core/src/datasources/elm_package.rs#L165) |

