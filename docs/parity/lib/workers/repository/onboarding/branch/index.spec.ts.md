# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/index.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/onboarding/branch/index › checkOnboardingBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if no package files | 57 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| doesn't throw if there are no package files and onboardingNoDeps config option is set | 63 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| throws if fork | 73 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| throws if bot disabled | 80 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| has default onboarding config | 87 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| uses discovered onboarding config | 127 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles skipped onboarding combined with requireConfig = optional | 170 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles skipped onboarding, requireConfig=required, and a config file | 181 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles skipped onboarding, requireConfig=ignored | 192 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles skipped onboarding, requireConfig=required, and no config file | 203 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| detects repo is onboarded via file | 216 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles removed cached file name | 223 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles cached file name | 230 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles cached package.json | 253 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| detects repo is onboarded via package.json config | 279 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| detects repo is onboarded via PR | 286 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| throws if no required config | 297 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| rebases onboarding branch | 310 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| skips processing onboarding branch when main/onboarding SHAs have not changed | 347 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| processes modified onboarding branch and invalidates extract cache | 379 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| skips processing conflicted onboarding branch | 417 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| sets onboarding cache for existing onboarding branch | 440 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |

### `workers/repository/onboarding/branch/index › checkOnboardingBranch › tests onboarding rebase/retry checkbox handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects unsupported platfom | 474 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| detects missing rebase checkbox | 495 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| detects manual pr update requested | 511 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |
| handles unchecked rebase checkbox | 527 | not-applicable | — | — | tests onboarding branch creation via git/platform; requires git2 + platform layer |

---
