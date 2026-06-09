# `lib/workers/repository/init/index.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**1/1 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 40 | runs | ported | [`crates/renovate-core/src/workers/repository/init/config.rs:40`](../../../../../../../crates/renovate-core/src/workers/repository/init/config.rs#L40) |
| 52 | warns on unsupported options | opt-out | asserts exact logger.logger.warn spy call (with { platform: 'github' } and the message naming 'filterUnavailableUsers' / 'expandCodeOwnersGroups' as not supported on the platform) after initRepo with those in the repo config result; core init continues (options ignored for platform); no direct Rust equivalent for the spy assertion or the exact unsupported-options-for-platform message shape (tracing in init or config layer may log but without this test's spy harness). Opt as pure TS logger spy + platform-specific unsupported message. |

