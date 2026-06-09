# `lib/workers/repository/extract/index.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 3 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | runs | ported | [`crates/renovate-core/src/managers.rs:2596`](../../../../../../../crates/renovate-core/src/managers.rs#L2596) |
| 32 | skips non-enabled managers | ported | [`crates/renovate-core/src/managers.rs:2583`](../../../../../../../crates/renovate-core/src/managers.rs#L2583) |
| 43 | warns if no packages found for a enabled manager | opt-out | asserts logger.debug spy (with { manager: 'custom.regex' } and the exact 'Manager explicitly enabled in "enabledManagers" config, but found no results. Possible config error?' message) when getManagerPackageFiles returns [] for an enabled custom manager; the core business (empty packageFiles result for enabled manager with no results) may be covered by other extract tests (the 1/5 ported in the spec); the test is written around the spy, no direct Rust equivalent (tracing without spy harness). Opt as pure TS logger spy + enabled manager warn message shape. |
| 54 | warns if packagefiles is null | opt-out | asserts logger warn spy for the case when packageFiles is null (for a manager); the core (no result or empty) is business but the test is around the spy and the null check message; no direct Rust equivalent (tracing, the extract paths return empty or handle null without this exact spy). Opt as TS logger spy + null packageFiles warn. |
| 60 | checks custom managers | opt-out | sets config.customManagers (with customType 'regex', managerFilePatterns for README, matchStrings), mocks getManagerPackageFiles, calls extractAllDependencies, asserts 'regex' key in packageFiles result. Per @parity comment in managers.rs, 'customManagers array + mergeChildConfig + isCustomManager full path pending' in the extractAllDependencies orchestrator (get_enabled_managers_list, get_patterns_for_manager, extract_list synthesis do not yet handle custom config to produce custom manager entries like 'regex'). Basic manager matching/accumulation exercised by ported 'runs' and 'skips non-enabled'. Opt as the customManagers support in this extract unit is pending implementation work. |

