# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/apis.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/apis.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** not-applicable

### `workers/repository/init/apis › initApis`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 24 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| throws for disabled | 34 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| throws for forked | 49 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| does not throw for includeForks=true | 66 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| does not throw for forkProcessing=enabled | 79 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| ignores platform.getJsonFile() failures | 92 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| throws for fork with platform.getJsonFile() failures | 109 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| uses the onboardingConfigFileName if set | 124 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| falls back to "renovate.json" if onboardingConfigFileName is not set | 151 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| falls back to "renovate.json" if onboardingConfigFileName is not valid | 172 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| checks for re-enablement and continues | 191 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| checks for re-enablement and skips | 211 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

---
