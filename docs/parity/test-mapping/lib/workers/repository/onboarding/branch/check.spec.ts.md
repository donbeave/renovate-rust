# `lib/workers/repository/onboarding/branch/check.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/11 ported** (11 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 31 | returns true if in silent mode | pending | — |
| 36 | skips normal onboarding check if onboardingcache is valid | pending | — |
| 56 | continues with normal logic if onboardingcache is invalid | pending | — |
| 72 | continues with normal logic if closedpr exists - adds closing comment | pending | — |
| 97 | adds closing comment if exactly at onboardingautocloseage | pending | — |
| 119 | skips closing comment if onboarding pr is slightly older than onboardingautocloseage | pending | — |
| 141 | skips closing comment if onboarding pr is 1 day older than onboardingautocloseage | pending | — |
| 162 | skips closing comment if onboarding pr is significantly older than onboardingautocloseage | pending | — |
| 179 | prefers inherited onboardingautocloseage over global config | pending | — |
| 203 | does not allow inherited onboardingautocloseage to be higher than global config | pending | — |
| 228 | checks git file list for config file when in fork mode | pending | — |

