# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/rules.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `RuleToBazelModulePackageDep`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .parse() with $msg | 145 | not-applicable | — | — | Tests internal RuleToBazelModulePackageDep class; Rust bazel_module.rs uses different internal types (BazelModuleDep) but same behavioral output covered by extract.spec.ts tests |

### `GitRepositoryToPackageDep`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .parse() with $msg | 176 | not-applicable | — | — | Tests internal GitRepositoryToPackageDep class; Rust uses BazelGitRepositoryDep with different internal structure, same behavior covered by extract.spec.ts |

### `.toPackageDependencies()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| with $msg | 239 | not-applicable | — | — | Tests internal toPackageDependencies() merge/override logic; Rust extract() produces same output via different internal pipeline, covered by extract.spec.ts override tests |

### `.processModulePkgDeps`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array if the input is an empty array | 263 | not-applicable | — | — | Tests internal processModulePkgDeps helper; trivial empty-input edge case for internal type |
| returns the bazel_dep if more than one override is found | 267 | not-applicable | — | — | Tests internal multi-override dedup logic; Rust extract() handles same behavior, covered by extract.spec.ts override tests |

---
