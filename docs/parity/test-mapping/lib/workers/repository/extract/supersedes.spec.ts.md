# `lib/workers/repository/extract/supersedes.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | handles empty input | ported | [`crates/renovate-core/src/managers.rs:1968`](../../../../../../../crates/renovate-core/src/managers.rs#L1968) |
| 12 | ignores extracts without superseding managers | ported | [`crates/renovate-core/src/managers.rs:1976`](../../../../../../../crates/renovate-core/src/managers.rs#L1976) |
| 28 | removes superseded package files without lock files | ported | [`crates/renovate-core/src/managers.rs:1984`](../../../../../../../crates/renovate-core/src/managers.rs#L1984) |
| 52 | keeps superseded package files with lock files | ported | [`crates/renovate-core/src/managers.rs:1996`](../../../../../../../crates/renovate-core/src/managers.rs#L1996) |
| 88 | keeps non-superseded package files | ported | [`crates/renovate-core/src/managers.rs:2009`](../../../../../../../crates/renovate-core/src/managers.rs#L2009) |
| 115 | handles primary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2025`](../../../../../../../crates/renovate-core/src/managers.rs#L2025) |
| 137 | handles missing secondary extract manager | ported | [`crates/renovate-core/src/managers.rs:2034`](../../../../../../../crates/renovate-core/src/managers.rs#L2034) |
| 153 | handles secondary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2042`](../../../../../../../crates/renovate-core/src/managers.rs#L2042) |

