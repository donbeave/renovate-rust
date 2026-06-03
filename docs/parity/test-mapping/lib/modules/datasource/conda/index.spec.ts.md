# `lib/modules/datasource/conda/index.spec.ts`

[← `datasource/conda`](../../../../_by-module/datasource/conda.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | throws for error | ported | `crates/renovate-core/src/datasources/conda.rs:353` |
| 24 | returns null for 404 | ported | `crates/renovate-core/src/datasources/conda.rs:361` |
| 34 | returns null for empty result | ported | `crates/renovate-core/src/datasources/conda.rs:378` |
| 47 | throws for 5xx | ported | `crates/renovate-core/src/datasources/conda.rs:395` |
| 57 | processes real data | ported | `crates/renovate-core/src/datasources/conda.rs:410` |
| 70 | returns null without registryurl | ported | `crates/renovate-core/src/datasources/conda.rs:429` |
| 79 | supports multiple custom datasource urls | ported | `crates/renovate-core/src/datasources/conda.rs:437` |
| 118 | supports channel from prefix.dev with null response | ported | `crates/renovate-core/src/datasources/conda.rs:491` |
| 135 | supports channel from prefix.dev with multiple page responses | ported | `crates/renovate-core/src/datasources/conda.rs:514` |

