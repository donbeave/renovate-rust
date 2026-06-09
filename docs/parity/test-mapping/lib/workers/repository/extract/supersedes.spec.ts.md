# `lib/workers/repository/extract/supersedes.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | handles empty input | ported | [`crates/renovate-core/src/managers.rs:1963`](../../../../../../../crates/renovate-core/src/managers.rs#L1963) |
| 12 | ignores extracts without superseding managers | ported | [`crates/renovate-core/src/managers.rs:1971`](../../../../../../../crates/renovate-core/src/managers.rs#L1971) |
| 28 | removes superseded package files without lock files | ported | [`crates/renovate-core/src/managers.rs:1979`](../../../../../../../crates/renovate-core/src/managers.rs#L1979) |
| 52 | keeps superseded package files with lock files | ported | [`crates/renovate-core/src/managers.rs:1991`](../../../../../../../crates/renovate-core/src/managers.rs#L1991) |
| 88 | keeps non-superseded package files | ported | [`crates/renovate-core/src/managers.rs:2004`](../../../../../../../crates/renovate-core/src/managers.rs#L2004) |
| 115 | handles primary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2020`](../../../../../../../crates/renovate-core/src/managers.rs#L2020) |
| 137 | handles missing secondary extract manager | ported | [`crates/renovate-core/src/managers.rs:2029`](../../../../../../../crates/renovate-core/src/managers.rs#L2029) |
| 153 | handles secondary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2037`](../../../../../../../crates/renovate-core/src/managers.rs#L2037) |

