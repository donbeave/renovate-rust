# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/package-tree.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/package-tree.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `getDependentPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns self for single project | 32 | not-applicable | — | — | NuGet package tree resolution |
| returns self for two projects with no references | 45 | not-applicable | — | — | NuGet package tree resolution |
| returns projects for two projects with one reference | 60 | not-applicable | — | — | NuGet package tree resolution |
| returns project for two projects with one reference and central versions | 77 | not-applicable | — | — | NuGet package tree resolution |
| returns projects for two projects with one reference and Directory.Build.props | 99 | not-applicable | — | — | NuGet package tree resolution |
| returns only projects under nested Directory.Build.props directory | 121 | not-applicable | — | — | NuGet package tree resolution |
| returns project for two projects with one reference and global.json | 143 | not-applicable | — | — | NuGet package tree resolution |
| returns projects for three projects with two linear references | 163 | not-applicable | — | — | NuGet package tree resolution |
| returns projects for three projects with two tree-like references | 197 | not-applicable | — | — | NuGet package tree resolution |
| throws error on circular reference | 229 | not-applicable | — | — | NuGet package tree resolution |
| skips on invalid xml file | 245 | not-applicable | — | — | NuGet package tree resolution |

---

