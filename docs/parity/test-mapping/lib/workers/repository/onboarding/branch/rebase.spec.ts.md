# `lib/workers/repository/onboarding/branch/rebase.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**2/9 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 41 | does nothing if branch is up to date | pending | — |
| 48 | rebases onboarding branch | ported | [`crates/renovate-core/src/workers/repository/onboarding/branch/rebase.rs:68`](../../../../../../../../crates/renovate-core/src/workers/repository/onboarding/branch/rebase.rs#L68) |
| 56 | uses the onboardingconfigfilename if set | pending | — |
| 76 | falls back to "renovate.json" if onboardingconfigfilename is not set | pending | — |
| 95 | handles a missing previous config hash | pending | — |
| 103 | does nothing if config hashes match | pending | — |
| 110 | dryrun=full | pending | — |
| 120 | uses semantic commit pr title when semanticcommits is enabled | ported | [`crates/renovate-core/src/workers/repository/onboarding/common.rs:200`](../../../../../../../../crates/renovate-core/src/workers/repository/onboarding/common.rs#L200) |
| 140 | _(it.each / template — verify manually)_ | ? | — |

