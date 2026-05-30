# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/repository/error.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/error.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** done

### `workers/repository/error › handleError()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| errors ${err} | 77 | ported | `util.rs` | `classify_repo_error_known_constants_pass_through` | — |
| handles ExternalHostError | 83 | ported | `util.rs` | `classify_repo_error_external_host_error_constant` | — |
| rewrites git 5xx error | 91 | ported | `util.rs` | `classify_repo_error_rewrites_git_5xx` | — |
| rewrites git remote error | 99 | ported | `util.rs` | `classify_repo_error_rewrites_git_remote_error` | — |
| rewrites git fatal error | 107 | ported | `util.rs` | `classify_repo_error_rewrites_git_fatal` | — |
| handles unknown error | 115 | ported | `util.rs` | `classify_repo_error_unknown` | — |
| logs config validation errors as warnings by default | 120 | ported | `util.rs` | `config_validation_log_level_default_warn` | — |
| logs config validation errors as warnings when configValidationError is false | 130 | ported | `util.rs` | `config_validation_log_level_false_warn` | — |
| logs config validation errors as errors when configValidationError is true | 140 | ported | `util.rs` | `config_validation_log_level_true_error` | — |

---

