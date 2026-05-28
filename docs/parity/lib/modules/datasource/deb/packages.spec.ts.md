# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/deb/packages.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/packages.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** partial

### `modules/datasource/deb/packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should ignore error when fetching the InRelease content fails | 67 | not-applicable | — | — | Requires httpMock + GlobalConfig + vi.spyOn for filesystem |
| should throw error when checksum validation fails | 93 | not-applicable | — | — | Requires httpMock + GlobalConfig + vi.spyOn for filesystem |
| should throw error for when extracting fails | 108 | not-applicable | — | — | Requires httpMock + GlobalConfig + vi.spyOn for filesystem |

---

