# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/parser/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/parser/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `parse`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if invalid content | 7 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds simple bazel_dep | 17 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds the git_override | 44 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds archive_override | 85 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds local_path_override | 119 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds single_version_override | 148 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds maven.artifact | 179 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds maven.install and maven.artifact | 248 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds oci.pull | 335 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds the git_repository | 376 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| finds use_repo_rule and repo rule call | 408 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |
| ignores use_repo_rule with insufficient args | 420 | not-applicable | — | — | tests Bazel MODULE.bazel parser; Rust bazel-module extractor uses different parsing approach |

---

