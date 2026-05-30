# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ruby-version/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ruby-version/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a result | 5 | ported | `version_file.rs` | `ruby_version_file` | — |
| supports ranges | 16 | ported | `version_file.rs` | `ruby_version_partial_range` | — |
| skips non ranges | 27 | ported | `version_file.rs` | `ruby_version_passes_through_non_alias_literal` | — |

---

