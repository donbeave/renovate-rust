# `lib/workers/repository/extract/supersedes.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | handles empty input | ported | [`crates/renovate-core/src/managers.rs:2175`](../../../../../../../crates/renovate-core/src/managers.rs#L2175) |
| 12 | ignores extracts without superseding managers | ported | [`crates/renovate-core/src/managers.rs:2183`](../../../../../../../crates/renovate-core/src/managers.rs#L2183) |
| 28 | removes superseded package files without lock files | ported | [`crates/renovate-core/src/managers.rs:2191`](../../../../../../../crates/renovate-core/src/managers.rs#L2191) |
| 52 | keeps superseded package files with lock files | ported | [`crates/renovate-core/src/managers.rs:2203`](../../../../../../../crates/renovate-core/src/managers.rs#L2203) |
| 88 | keeps non-superseded package files | ported | [`crates/renovate-core/src/managers.rs:2216`](../../../../../../../crates/renovate-core/src/managers.rs#L2216) |
| 115 | handles primary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2232`](../../../../../../../crates/renovate-core/src/managers.rs#L2232) |
| 137 | handles missing secondary extract manager | ported | [`crates/renovate-core/src/managers.rs:2241`](../../../../../../../crates/renovate-core/src/managers.rs#L2241) |
| 153 | handles secondary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2249`](../../../../../../../crates/renovate-core/src/managers.rs#L2249) |

