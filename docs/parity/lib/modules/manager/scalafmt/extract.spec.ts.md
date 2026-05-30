# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/scalafmt/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/scalafmt/extract.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts version correctly | 6 | ported | `scalafmt.rs` | `extracts_version` | — |
| extracts version correctly if enclosed in quotes | 25 | ported | `scalafmt.rs` | `version_without_quotes` | — |
| ignore file if no version specified | 44 | ported | `scalafmt.rs` | `no_version_returns_none` | — |
| should return empty packagefiles is no content is provided | 52 | ported | `scalafmt.rs` | `empty_returns_none` | — |

---

