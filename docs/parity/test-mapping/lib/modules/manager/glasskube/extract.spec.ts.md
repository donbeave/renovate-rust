# `lib/modules/manager/glasskube/extract.spec.ts`

[← `manager/glasskube`](../../../../_by-module/manager/glasskube.md) · [all modules](../../../../README.md)

**3/5 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 43 | should extract version and registryurl | ported | [`crates/renovate-core/src/extractors/glasskube.rs:111`](../../../../../../../crates/renovate-core/src/extractors/glasskube.rs#L111) |
| 62 | should return null for empty packagefiles | ported | [`crates/renovate-core/src/extractors/glasskube.rs:131`](../../../../../../../crates/renovate-core/src/extractors/glasskube.rs#L131) |
| 67 | should skip package with non-existing repo | ported | [`crates/renovate-core/src/extractors/glasskube.rs:137`](../../../../../../../crates/renovate-core/src/extractors/glasskube.rs#L137) |
| 85 | should extract registryurl from repo in other file | pending | — |
| 107 | should extract registryurl from default repo in other file | pending | — |

