# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/typst/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/typst/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty deps for empty content | 5 | ported | `typst.rs` | `empty_content_returns_empty` | — |
| returns empty deps when no imports found | 10 | ported | `typst.rs` | `no_imports_returns_empty` | — |
| extracts single import | 21 | ported | `typst.rs` | `extracts_preview_import` (+ extracts_import_with_trailing_colon_import) | — |
| extracts multiple imports | 36 | ported | `typst.rs` | `multiple_imports` | — |
| handles imports with different version formats | 67 | ported | `typst.rs` | `prerelease_version_formats_extracted` | — |
| strips JSON comments before parsing | 98 | ported | `typst.rs` | `comment_line_skipped` | — |
| handles multiple imports on same line | 125 | ported | `typst.rs` | `multiple_imports_on_same_line` | — |
| ignores invalid import formats | 147 | ported | `typst.rs` | `ignores_invalid_import_formats` | — |
| adds skipReason for non-preview namespaces | 167 | ported | `typst.rs` | `local_namespace_skipped` (+ unknown_namespace_skipped, non_preview_namespaces_get_skip_reasons) | — |

---

