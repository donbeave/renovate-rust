# `lib/workers/repository/finalize/repository-statistics.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**2/2 in-scope tests ported** (0 pending, 2 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 41 | calls runrenovaterepostats | ported | [`crates/renovate-core/src/workers/repository/finalize/index.rs:81`](../../../../../../../crates/renovate-core/src/workers/repository/finalize/index.rs#L81) |
| 63 | processes cache with basebranches only | ported | [`crates/renovate-core/src/workers/repository/finalize/repository_statistics.rs:148`](../../../../../../../crates/renovate-core/src/workers/repository/finalize/repository_statistics.rs#L148) |
| 94 | processes cache with basebranches and branches | opt-out | sets up baseCache/branchCaches with sha/baseBranch etc, RepoCacheData with scan+branches, spies getCache/isCacheModified, calls runBranchSummary, asserts exact logger.debug call with processed {baseBranches, branches (with meta), cacheModified, defaultBranch, inactiveBranches} + 'Branch summary'. Spy-heavy (getCacheSpy, isCacheModifiedSpy, logger.debug exact payload); core cache/branch processing may exist in repository_statistics but test is written around the spies and summary object shape. Opt as pure TS logger spy + cache summary debug (no direct Rust equivalent for the spy asserts). |
| 159 | logs extended branch info if branchsummaryextended | opt-out | sets branchSummaryExtended true, branchCache with result/upgrades, calls runBranchSummary, asserts extended logger debug/info. Pure spy on logger + the extended flag path. Opt as TS logger spy (similar to other stats 'prints report' / extended logs opted previously); no direct equivalent. |

