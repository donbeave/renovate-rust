# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/rules.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `RuleToBazelModulePackageDep`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .parse() with $msg | 145 | not-applicable | — | — | tests Bazel MODULE.bazel rule validation; Rust bazel-module extractor uses own approach |

### `GitRepositoryToPackageDep`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .parse() with $msg | 176 | not-applicable | — | — | tests Bazel MODULE.bazel rule validation; Rust bazel-module extractor uses own approach |

### `.toPackageDependencies()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| with $msg | 239 | not-applicable | — | — | tests Bazel MODULE.bazel rule validation; Rust bazel-module extractor uses own approach |

### `.processModulePkgDeps`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array if the input is an empty array | 263 | not-applicable | — | — | tests Bazel MODULE.bazel rule validation; Rust bazel-module extractor uses own approach |
| returns the bazel_dep if more than one override is found | 267 | not-applicable | — | — | tests Bazel MODULE.bazel rule validation; Rust bazel-module extractor uses own approach |

---

