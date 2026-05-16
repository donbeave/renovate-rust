# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/nuget/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/nuget/common.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/nuget/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sortNugetVersions("$version", "$other") === $result | 4 | not-applicable | — | — | Renovate's NuGet datasource comparator helper is not exposed as a Rust API; Rust NuGet update summaries compare versions internally. |

---

