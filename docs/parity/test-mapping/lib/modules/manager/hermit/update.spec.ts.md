# `lib/modules/manager/hermit/update.spec.ts`

[← `manager/hermit`](../../../../_by-module/manager/hermit.md) · [all modules](../../../../README.md)

**2/2 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | should append a new marking line at the end to trigger the artifact update | ported | [`crates/renovate-core/src/extractors/hermit.rs:192`](../../../../../../../crates/renovate-core/src/extractors/hermit.rs#L192) |
| 19 | should not update again if the new line has been appended | ported | [`crates/renovate-core/src/extractors/hermit.rs:200`](../../../../../../../crates/renovate-core/src/extractors/hermit.rs#L200) |

