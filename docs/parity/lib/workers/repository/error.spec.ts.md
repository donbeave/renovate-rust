# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/repository/error.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/error.spec.ts
**Total tests:** 9 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `workers/repository/error › handleError()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| errors ${err} | 77 | not-applicable | — | — | mocking framework internals — vi.mock on error-config.ts; tests TypeScript error constant pass-through in module pipeline |
| handles ExternalHostError | 83 | not-applicable | — | — | TypeScript error-class hierarchy test; ExternalHostError is a TypeScript class with no direct Rust equivalent |
| rewrites git 5xx error | 91 | ported | `util.rs` | `classify_repo_error_rewrites_git_5xx` | — |
| rewrites git remote error | 99 | ported | `util.rs` | `classify_repo_error_rewrites_git_remote_error` | — |
| rewrites git fatal error | 107 | ported | `util.rs` | `classify_repo_error_rewrites_git_fatal` | — |
| handles unknown error | 115 | ported | `util.rs` | `classify_repo_error_unknown` | — |
| logs config validation errors as warnings by default | 120 | not-applicable | — | — | mocking framework internals — tests logger.logger.warn spy for CONFIG_VALIDATION error |
| logs config validation errors as warnings when configValidationError is false | 130 | not-applicable | — | — | mocking framework internals — tests logger.logger.warn spy |
| logs config validation errors as errors when configValidationError is true | 140 | not-applicable | — | — | mocking framework internals — tests logger.logger.error spy |

---

