# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven/index.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** done

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update an existing dependency | 26 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should update existing dependency defined via properties | 43 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should not touch content if new and old versions are equal | 67 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should update to version of the latest dep in implicit group | 79 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should return null for ungrouped deps if content was updated outside | 135 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should return null if current versions in content and upgrade are not same | 150 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should update ranges | 162 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should preserve ranges | 181 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |

---

