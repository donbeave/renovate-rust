# `lib/modules/datasource/node-version/index.spec.ts`

[← `datasource/node-version`](../../../../_by-module/datasource/node-version.md) · [all modules](../../../../README.md)

**2/4 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | throws for 500 | pending | — |
| 19 | returns null for error | pending | — |
| 32 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/node_version.rs:121`](../../../../../../../crates/renovate-core/src/datasources/node_version.rs#L121) |
| 42 | processes real data | ported | [`crates/renovate-core/src/datasources/node_version.rs:129`](../../../../../../../crates/renovate-core/src/datasources/node_version.rs#L129) |

