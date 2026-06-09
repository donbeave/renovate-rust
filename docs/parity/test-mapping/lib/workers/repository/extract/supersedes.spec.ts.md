# `lib/workers/repository/extract/supersedes.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | handles empty input | ported | [`crates/renovate-core/src/managers.rs:2170`](../../../../../../../crates/renovate-core/src/managers.rs#L2170) |
| 12 | ignores extracts without superseding managers | ported | [`crates/renovate-core/src/managers.rs:2178`](../../../../../../../crates/renovate-core/src/managers.rs#L2178) |
| 28 | removes superseded package files without lock files | ported | [`crates/renovate-core/src/managers.rs:2186`](../../../../../../../crates/renovate-core/src/managers.rs#L2186) |
| 52 | keeps superseded package files with lock files | ported | [`crates/renovate-core/src/managers.rs:2198`](../../../../../../../crates/renovate-core/src/managers.rs#L2198) |
| 88 | keeps non-superseded package files | ported | [`crates/renovate-core/src/managers.rs:2211`](../../../../../../../crates/renovate-core/src/managers.rs#L2211) |
| 115 | handles primary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2227`](../../../../../../../crates/renovate-core/src/managers.rs#L2227) |
| 137 | handles missing secondary extract manager | ported | [`crates/renovate-core/src/managers.rs:2236`](../../../../../../../crates/renovate-core/src/managers.rs#L2236) |
| 153 | handles secondary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2244`](../../../../../../../crates/renovate-core/src/managers.rs#L2244) |

