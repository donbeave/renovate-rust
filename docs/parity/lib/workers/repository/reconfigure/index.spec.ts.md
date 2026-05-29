# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/reconfigure/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/reconfigure/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** not-applicable

### `workers/repository/reconfigure/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no effect when running with platform=local | 66 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| no effect on repo with no reconfigure branch | 75 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| skips if reconfigure branch unchanged | 85 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| skips if error while finding reconfigure config | 103 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| skips if reconfigure config is invalid | 115 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| validates reconfigure branch and skips extraction if no reconfigure pr | 124 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| extracts deps and adds comment when branch and reconfigure pr both exist | 133 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| skips pr comment if error during deps extraction | 144 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

---

