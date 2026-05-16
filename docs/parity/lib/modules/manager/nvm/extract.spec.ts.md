# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nvm/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nvm/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `nvmrc_plain_version` | — |
| supports ranges | 16 | ported | `version_file.rs` | `nvmrc_partial_version_range` | — |
| skips non ranges | 27 | ported | `version_file.rs` | `nvmrc_passes_through_latest_literal` | — |
| supports code comments | 38 | ported | `version_file.rs` | `nvmrc_skips_full_line_comments_and_inline_comment` | — |

---

