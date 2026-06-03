# `lib/modules/datasource/conda/index.spec.ts`

[← `datasource/conda`](../../../../_by-module/datasource/conda.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | throws for error | ported | [`crates/renovate-core/src/datasources/conda.rs:353`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L353) |
| 24 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/conda.rs:361`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L361) |
| 34 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/conda.rs:378`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L378) |
| 47 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/conda.rs:395`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L395) |
| 57 | processes real data | ported | [`crates/renovate-core/src/datasources/conda.rs:410`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L410) |
| 70 | returns null without registryurl | ported | [`crates/renovate-core/src/datasources/conda.rs:429`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L429) |
| 79 | supports multiple custom datasource urls | ported | [`crates/renovate-core/src/datasources/conda.rs:437`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L437) |
| 118 | supports channel from prefix.dev with null response | ported | [`crates/renovate-core/src/datasources/conda.rs:491`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L491) |
| 135 | supports channel from prefix.dev with multiple page responses | ported | [`crates/renovate-core/src/datasources/conda.rs:514`](../../../../../../../crates/renovate-core/src/datasources/conda.rs#L514) |

