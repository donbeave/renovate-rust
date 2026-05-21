# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/rebase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/rebase.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** not-applicable

### `workers/repository/onboarding/branch/rebase › rebaseOnboardingBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if branch is up to date | 41 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| rebases onboarding branch | 48 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| uses the onboardingConfigFileName if set | 56 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| falls back to "renovate.json" if onboardingConfigFileName is not set | 76 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| handles a missing previous config hash | 95 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| does nothing if config hashes match | 103 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| dryRun=full | 110 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| uses semantic commit PR title when semanticCommits is enabled | 120 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |
| returns null for $platform | 140 | not-applicable | — | — | tests onboarding branch rebase via git; requires git2 layer |

---

