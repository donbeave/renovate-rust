# `lib/workers/repository/onboarding/pr/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/29 in-scope tests ported** (29 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 47 | returns if onboarded | pending | — |
| 56 | returns if onboarded cache is valid | pending | — |
| 65 | _(it.each / template — verify manually)_ | ? | — |
| 86 | creates pr | pending | — |
| 91 | creates semantic pr | pending | — |
| 108 | creates pr with labels | pending | — |
| 125 | _(it.each / template — verify manually)_ | ? | — |
| 149 | _(it.each / template — verify manually)_ | ? | — |
| 174 | _(it.each / template — verify manually)_ | ? | — |
| 208 | _(it.each / template — verify manually)_ | ? | — |
| 232 | ensures comment, when pr is conflicted | pending | — |
| 252 | ensures comment, if onboarding cache is up-to-date, but when onboarding pr is over onboardingautocloseage | pending | — |
| 279 | does not comment, when onboarding pr is exactly at onboardingautocloseage | pending | — |
| 300 | ensures comment, when onboarding pr is partially over onboardingautocloseage | pending | — |
| 327 | ensures comment, when onboarding pr is 1 day older than onboardingautocloseage | pending | — |
| 354 | ensures comment,when onboarding pr is significantly older than onboardingautocloseage | pending | — |
| 376 | prefers inherited onboardingautocloseage over global config | pending | — |
| 405 | does not allow inherited onboardingautocloseage to be higher than global config | pending | — |
| 435 | does nothing in dry run when pr is conflicted | pending | — |
| 454 | updates pr when modified | pending | — |
| 467 | creates pr (no require config) | pending | — |
| 478 | creates pr (require config) | pending | — |
| 485 | when set | pending | — |
| 496 | when not set, falls back to "renovate.json" | pending | — |
| 504 | when set, but not a valid filename, falls back to "renovate.json" | pending | — |
| 513 | dryrun of creates pr | pending | — |
| 528 | dryrun of updates pr | pending | — |
| 559 | throws when trying to create a new pr | pending | — |
| 567 | deletes branch when pr already exists but cannot find it | pending | — |

