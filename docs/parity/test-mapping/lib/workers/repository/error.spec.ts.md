# `lib/workers/repository/error.spec.ts`

[← `worker/repository`](../../../_by-module/worker/repository.md) · [all modules](../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 77 | errors ${err} | ported | [`crates/renovate-core/src/util.rs:11727`](../../../../../../crates/renovate-core/src/util.rs#L11727) |
| 83 | handles externalhosterror | ported | [`crates/renovate-core/src/util.rs:11747`](../../../../../../crates/renovate-core/src/util.rs#L11747) |
| 91 | rewrites git 5xx error | ported | [`crates/renovate-core/src/util.rs:11682`](../../../../../../crates/renovate-core/src/util.rs#L11682) |
| 99 | rewrites git remote error | ported | [`crates/renovate-core/src/util.rs:11689`](../../../../../../crates/renovate-core/src/util.rs#L11689) |
| 107 | rewrites git fatal error | ported | [`crates/renovate-core/src/util.rs:11696`](../../../../../../crates/renovate-core/src/util.rs#L11696) |
| 115 | handles unknown error | ported | [`crates/renovate-core/src/util.rs:11703`](../../../../../../crates/renovate-core/src/util.rs#L11703) |
| 120 | logs config validation errors as warnings by default | ported | [`crates/renovate-core/src/util.rs:11709`](../../../../../../crates/renovate-core/src/util.rs#L11709) |
| 130 | logs config validation errors as warnings when configvalidationerror is false | ported | [`crates/renovate-core/src/util.rs:11715`](../../../../../../crates/renovate-core/src/util.rs#L11715) |
| 140 | logs config validation errors as errors when configvalidationerror is true | ported | [`crates/renovate-core/src/util.rs:11721`](../../../../../../crates/renovate-core/src/util.rs#L11721) |

