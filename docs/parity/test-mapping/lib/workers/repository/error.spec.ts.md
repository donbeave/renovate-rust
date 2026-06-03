# `lib/workers/repository/error.spec.ts`

[← `worker/repository`](../../../_by-module/worker/repository.md) · [all modules](../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 77 | errors ${err} | ported | [`crates/renovate-core/src/util.rs:10091`](../../../../../../crates/renovate-core/src/util.rs#L10091) |
| 83 | handles externalhosterror | ported | [`crates/renovate-core/src/util.rs:10111`](../../../../../../crates/renovate-core/src/util.rs#L10111) |
| 91 | rewrites git 5xx error | ported | [`crates/renovate-core/src/util.rs:10046`](../../../../../../crates/renovate-core/src/util.rs#L10046) |
| 99 | rewrites git remote error | ported | [`crates/renovate-core/src/util.rs:10053`](../../../../../../crates/renovate-core/src/util.rs#L10053) |
| 107 | rewrites git fatal error | ported | [`crates/renovate-core/src/util.rs:10060`](../../../../../../crates/renovate-core/src/util.rs#L10060) |
| 115 | handles unknown error | ported | [`crates/renovate-core/src/util.rs:10067`](../../../../../../crates/renovate-core/src/util.rs#L10067) |
| 120 | logs config validation errors as warnings by default | ported | [`crates/renovate-core/src/util.rs:10073`](../../../../../../crates/renovate-core/src/util.rs#L10073) |
| 130 | logs config validation errors as warnings when configvalidationerror is false | ported | [`crates/renovate-core/src/util.rs:10079`](../../../../../../crates/renovate-core/src/util.rs#L10079) |
| 140 | logs config validation errors as errors when configvalidationerror is true | ported | [`crates/renovate-core/src/util.rs:10085`](../../../../../../crates/renovate-core/src/util.rs#L10085) |

