# `lib/modules/manager/copier/update.spec.ts`

[← `manager/copier`](../../../../_by-module/manager/copier.md) · [all modules](../../../../README.md)

**2/2 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | should append a new marking line at the end to trigger the artifact update | ported | [`crates/renovate-core/src/extractors/copier.rs:206`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L206) |
| 19 | should not update again if the new line has been appended | ported | [`crates/renovate-core/src/extractors/copier.rs:214`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L214) |

