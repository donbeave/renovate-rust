# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ocb/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ocb/extract.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| run successfully with full example | 6 | ported | `ocb.rs` | `extracts_full_example` | — |
| return null for unknown content | 81 | ported | `ocb.rs` | `skips_unknown_content` | — |
| return null for content which is not YAML | 85 | ported | `ocb.rs` | `skips_arbitrary_yaml` | — |

---

