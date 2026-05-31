# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/package-tree.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/package-tree.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** pending-applicable-applicable

### `getDependentPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns self for single project | 32 | pending | — | — | NuGet package tree resolution |
| returns self for two projects with no references | 45 | pending | — | — | NuGet package tree resolution |
| returns projects for two projects with one reference | 60 | pending | — | — | NuGet package tree resolution |
| returns project for two projects with one reference and central versions | 77 | pending | — | — | NuGet package tree resolution |
| returns projects for two projects with one reference and Directory.Build.props | 99 | pending | — | — | NuGet package tree resolution |
| returns only projects under nested Directory.Build.props directory | 121 | pending | — | — | NuGet package tree resolution |
| returns project for two projects with one reference and global.json | 143 | pending | — | — | NuGet package tree resolution |
| returns projects for three projects with two linear references | 163 | pending | — | — | NuGet package tree resolution |
| returns projects for three projects with two tree-like references | 197 | pending | — | — | NuGet package tree resolution |
| throws error on circular reference | 229 | pending | — | — | NuGet package tree resolution |
| skips on invalid xml file | 245 | pending | — | — | NuGet package tree resolution |

---

