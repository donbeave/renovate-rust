# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/parser/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/parser/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable

### `parse`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty string if invalid content | 7 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds simple bazel_dep | 17 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds the git_override | 44 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds archive_override | 85 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds local_path_override | 119 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds single_version_override | 148 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds maven.artifact | 179 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds maven.install and maven.artifact | 248 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds oci.pull | 335 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds the git_repository | 376 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| finds use_repo_rule and repo rule call | 408 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |
| ignores use_repo_rule with insufficient args | 420 | not-applicable | — | — | TS-library-specific schema internals; AST parser APIs are TypeScript-specific |

---

