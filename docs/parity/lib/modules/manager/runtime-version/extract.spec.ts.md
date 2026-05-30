# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/runtime-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/runtime-version/extract.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result - python | 5 | ported | `runtime_version.rs` | `extracts_python_version` (+ extracts_with_trailing_newline) | — |
| returns no result | 16 | ported | `runtime_version.rs` | `ignores_partial_version` (+ returns_none_for_empty) | — |

---

