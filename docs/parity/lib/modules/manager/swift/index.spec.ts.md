# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/swift/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/swift/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty content | 6 | ported | `spm.rs` | `index_returns_null_for_empty_and_no_package_calls` | — |
| returns null for invalid content | 12 | ported | `spm.rs` | `index_returns_null_for_invalid_content` | — |
| parses packages with invalid versions | 81 | ported | `spm.rs` | `index_parses_packages_with_invalid_versions` | — |
| parses package descriptions | 109 | ported | `spm.rs` | `index_parses_package_descriptions` | — |
| parses multiple packages | 152 | ported | `spm.rs` | `index_parses_multiple_packages` | Snapshot test adapted to invariant checks |

---

