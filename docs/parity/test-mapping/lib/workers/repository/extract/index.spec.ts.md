# `lib/workers/repository/extract/index.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**1/4 in-scope tests ported** (3 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | runs | pending | — |
| 32 | skips non-enabled managers | ported | [`crates/renovate-core/src/managers.rs:2583`](../../../../../../../crates/renovate-core/src/managers.rs#L2583) |
| 43 | warns if no packages found for a enabled manager | opt-out | asserts logger.debug spy (with { manager: 'custom.regex' } and the exact 'Manager explicitly enabled in "enabledManagers" config, but found no results. Possible config error?' message) when getManagerPackageFiles returns [] for an enabled custom manager; the core business (empty packageFiles result for enabled manager with no results) may be covered by other extract tests (the 1/5 ported in the spec); the test is written around the spy, no direct Rust equivalent (tracing without spy harness). Opt as pure TS logger spy + enabled manager warn message shape. |
| 54 | warns if packagefiles is null | pending | — |
| 60 | checks custom managers | pending | — |

