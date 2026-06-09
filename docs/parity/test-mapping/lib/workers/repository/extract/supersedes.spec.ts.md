# `lib/workers/repository/extract/supersedes.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | handles empty input | ported | [`crates/renovate-core/src/managers.rs:2177`](../../../../../../../crates/renovate-core/src/managers.rs#L2177) |
| 12 | ignores extracts without superseding managers | ported | [`crates/renovate-core/src/managers.rs:2185`](../../../../../../../crates/renovate-core/src/managers.rs#L2185) |
| 28 | removes superseded package files without lock files | ported | [`crates/renovate-core/src/managers.rs:2193`](../../../../../../../crates/renovate-core/src/managers.rs#L2193) |
| 52 | keeps superseded package files with lock files | ported | [`crates/renovate-core/src/managers.rs:2205`](../../../../../../../crates/renovate-core/src/managers.rs#L2205) |
| 88 | keeps non-superseded package files | ported | [`crates/renovate-core/src/managers.rs:2218`](../../../../../../../crates/renovate-core/src/managers.rs#L2218) |
| 115 | handles primary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2234`](../../../../../../../crates/renovate-core/src/managers.rs#L2234) |
| 137 | handles missing secondary extract manager | ported | [`crates/renovate-core/src/managers.rs:2243`](../../../../../../../crates/renovate-core/src/managers.rs#L2243) |
| 153 | handles secondary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2251`](../../../../../../../crates/renovate-core/src/managers.rs#L2251) |

