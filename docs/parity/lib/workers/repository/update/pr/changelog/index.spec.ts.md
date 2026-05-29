# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/index.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 56 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles unsupported changelog source | 65 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns null if no currentVersion | 74 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns null if currentVersion equals newVersion | 83 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| skips invalid repos | 93 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| works without Github | 102 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| uses GitHub tags | 140 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| filters unnecessary warns | 176 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| supports node engines | 206 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles no sourceUrl | 236 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles invalid sourceUrl | 245 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles missing Github token | 254 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles no releases | 264 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles not enough releases | 273 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| will call getInRangeReleases when releases is undefined | 282 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| supports github enterprise and github.com changelog | 291 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| supports github enterprise and github enterprise changelog | 325 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| supports github.com and github enterprise changelog | 364 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

---
