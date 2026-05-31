# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/rules.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `RuleToBazelModulePackageDep`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .parse() with $msg | 145 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal RuleToBazelModulePackageDep class; Rust uses different internal types (BazelModuleDep) with same behavior covered by extract.spec.ts |

### `GitRepositoryToPackageDep`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| .parse() with $msg | 176 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal GitRepositoryToPackageDep class; Rust uses different internal structure with same behavior covered by extract.spec.ts |

### `.toPackageDependencies()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| with $msg | 239 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal toPackageDependencies() merge/override logic; Rust extract() produces same output via different internal pipeline |

### `.processModulePkgDeps`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array if the input is an empty array | 263 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal processModulePkgDeps helper; trivial empty-input edge case |
| returns the bazel_dep if more than one override is found | 267 | not-applicable | — | — | TS-library-specific schema internals — tests TypeScript internal multi-override dedup logic; Rust extract() handles same behavior |

---
