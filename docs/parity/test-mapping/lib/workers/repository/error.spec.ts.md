# `lib/workers/repository/error.spec.ts`

[← `worker/repository`](../../../_by-module/worker/repository.md) · [all modules](../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 77 | errors ${err} | ported | [`crates/renovate-core/src/util.rs:11809`](../../../../../../crates/renovate-core/src/util.rs#L11809) |
| 83 | handles externalhosterror | ported | [`crates/renovate-core/src/util.rs:11829`](../../../../../../crates/renovate-core/src/util.rs#L11829) |
| 91 | rewrites git 5xx error | ported | [`crates/renovate-core/src/util.rs:11764`](../../../../../../crates/renovate-core/src/util.rs#L11764) |
| 99 | rewrites git remote error | ported | [`crates/renovate-core/src/util.rs:11771`](../../../../../../crates/renovate-core/src/util.rs#L11771) |
| 107 | rewrites git fatal error | ported | [`crates/renovate-core/src/util.rs:11778`](../../../../../../crates/renovate-core/src/util.rs#L11778) |
| 115 | handles unknown error | ported | [`crates/renovate-core/src/util.rs:11785`](../../../../../../crates/renovate-core/src/util.rs#L11785) |
| 120 | logs config validation errors as warnings by default | ported | [`crates/renovate-core/src/util.rs:11791`](../../../../../../crates/renovate-core/src/util.rs#L11791) |
| 130 | logs config validation errors as warnings when configvalidationerror is false | ported | [`crates/renovate-core/src/util.rs:11797`](../../../../../../crates/renovate-core/src/util.rs#L11797) |
| 140 | logs config validation errors as errors when configvalidationerror is true | ported | [`crates/renovate-core/src/util.rs:11803`](../../../../../../crates/renovate-core/src/util.rs#L11803) |

