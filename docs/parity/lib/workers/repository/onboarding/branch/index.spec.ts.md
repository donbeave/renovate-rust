# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** pending

### `workers/repository/onboarding/branch/index › checkOnboardingBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if no package files | 57 | pending | — | — | — |
| doesn't throw if there are no package files and onboardingNoDeps config option is set | 63 | pending | — | — | — |
| throws if fork | 73 | pending | — | — | — |
| throws if bot disabled | 80 | pending | — | — | — |
| has default onboarding config | 87 | pending | — | — | — |
| uses discovered onboarding config | 127 | pending | — | — | — |
| handles skipped onboarding combined with requireConfig = optional | 170 | pending | — | — | — |
| handles skipped onboarding, requireConfig=required, and a config file | 181 | pending | — | — | — |
| handles skipped onboarding, requireConfig=ignored | 192 | pending | — | — | — |
| handles skipped onboarding, requireConfig=required, and no config file | 203 | pending | — | — | — |
| detects repo is onboarded via file | 216 | pending | — | — | — |
| handles removed cached file name | 223 | pending | — | — | — |
| handles cached file name | 230 | pending | — | — | — |
| handles cached package.json | 253 | pending | — | — | — |
| detects repo is onboarded via package.json config | 279 | pending | — | — | — |
| detects repo is onboarded via PR | 286 | pending | — | — | — |
| throws if no required config | 297 | pending | — | — | — |
| rebases onboarding branch | 310 | pending | — | — | — |
| skips processing onboarding branch when main/onboarding SHAs have not changed | 347 | pending | — | — | — |
| processes modified onboarding branch and invalidates extract cache | 379 | pending | — | — | — |
| skips processing conflicted onboarding branch | 417 | pending | — | — | — |
| sets onboarding cache for existing onboarding branch | 440 | pending | — | — | — |

### `workers/repository/onboarding/branch/index › checkOnboardingBranch › tests onboarding rebase/retry checkbox handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects unsupported platfom | 474 | pending | — | — | — |
| detects missing rebase checkbox | 495 | pending | — | — | — |
| detects manual pr update requested | 511 | pending | — | — | — |
| handles unchecked rebase checkbox | 527 | pending | — | — | — |

---

