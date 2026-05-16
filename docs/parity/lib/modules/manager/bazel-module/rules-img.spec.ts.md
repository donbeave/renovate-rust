# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/rules-img.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/rules-img.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `transformRulesImgCalls()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores repo rule calls that are not rules_img | 5 | ported | `bazel_module.rs` | `rules_img_ignores_non_rules_img` | — |
| handles valid rules_img pull call | 32 | ported | `bazel_module.rs` | `rules_img_handles_valid_pull_call` | — |
| skips repo rule calls without corresponding use_repo_rule | 72 | ported | `bazel_module.rs` | `rules_img_skips_unknown_function` | — |
| skips malformed repo rule calls | 91 | ported | `bazel_module.rs` | `rules_img_skips_malformed_call` | — |

---

