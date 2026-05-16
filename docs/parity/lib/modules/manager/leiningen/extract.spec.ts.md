# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/leiningen/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/leiningen/extract.spec.ts
**Total tests:** 4 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/manager/leiningen/extract`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| trimAtKey | 10 | not-applicable | — | — | TypeScript-internal helper; Rust extractor uses different parser structure |
| extractFromVectors | 22 | not-applicable | — | — | TypeScript-internal helper; Rust extractor uses different parser structure |
| extractPackageFile | 74 | ported | `leiningen.rs` | `extracts_dependencies` (+ extracts_managed_dependencies, extracts_plugins, dev_profile_dependencies_also_extracted) | — |
| extractVariables | 239 | not-applicable | — | — | TypeScript-internal helper; Rust handles variable expansion inline in extract() |

---

