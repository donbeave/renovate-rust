# `lib/workers/repository/extract/supersedes.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | handles empty input | ported | [`crates/renovate-core/src/managers.rs:1962`](../../../../../../../crates/renovate-core/src/managers.rs#L1962) |
| 12 | ignores extracts without superseding managers | ported | [`crates/renovate-core/src/managers.rs:1970`](../../../../../../../crates/renovate-core/src/managers.rs#L1970) |
| 28 | removes superseded package files without lock files | ported | [`crates/renovate-core/src/managers.rs:1978`](../../../../../../../crates/renovate-core/src/managers.rs#L1978) |
| 52 | keeps superseded package files with lock files | ported | [`crates/renovate-core/src/managers.rs:1990`](../../../../../../../crates/renovate-core/src/managers.rs#L1990) |
| 88 | keeps non-superseded package files | ported | [`crates/renovate-core/src/managers.rs:2003`](../../../../../../../crates/renovate-core/src/managers.rs#L2003) |
| 115 | handles primary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2019`](../../../../../../../crates/renovate-core/src/managers.rs#L2019) |
| 137 | handles missing secondary extract manager | ported | [`crates/renovate-core/src/managers.rs:2028`](../../../../../../../crates/renovate-core/src/managers.rs#L2028) |
| 153 | handles secondary extract with undefined packagefiles | ported | [`crates/renovate-core/src/managers.rs:2036`](../../../../../../../crates/renovate-core/src/managers.rs#L2036) |

