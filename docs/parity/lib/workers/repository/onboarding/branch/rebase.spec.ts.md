# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/rebase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/rebase.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** not-applicable

### `workers/repository/onboarding/branch/rebase › rebaseOnboardingBranch()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if branch is up to date | 41 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| rebases onboarding branch | 48 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| uses the onboardingConfigFileName if set | 56 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| falls back to "renovate.json" if onboardingConfigFileName is not set | 76 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| handles a missing previous config hash | 95 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| does nothing if config hashes match | 103 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| dryRun=full | 110 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| uses semantic commit PR title when semanticCommits is enabled | 120 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| returns null for $platform | 140 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

---

