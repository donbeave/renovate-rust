# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gitlabci-include/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gitlabci-include/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 13 | ported | `gitlabci_include.rs` | `empty_returns_empty` | — |
| returns null for include block without any actual includes | 17 | ported | `gitlabci_include.rs` | `empty_include_block_returns_no_deps` | — |
| extracts single include block | 22 | ported | `gitlabci_include.rs` | `extracts_include_with_ref` | — |
| extracts multiple include blocks | 28 | ported | `gitlabci_include.rs` | `multiple_includes` | — |
| extracts multiple embedded include blocks | 34 | ported | `gitlabci_include.rs` | `extracts_multiple_embedded_include_blocks` | — |
| ignores includes without project and file keys | 51 | ported | `gitlabci_include.rs` | `ignores_includes_without_project_and_file_keys` | — |
| normalizes configured endpoints | 60 | ported | `gitlabci_include.rs` | `normalizes_configured_endpoints` | — |
| supports multi-document files | 73 | ported | `gitlabci_include.rs` | `supports_multi_document_files` | — |

---

