# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/package-tree.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/package-tree.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `getDependentPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns self for single project | 32 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns self for two projects with no references | 45 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns projects for two projects with one reference | 60 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns project for two projects with one reference and central versions | 77 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns projects for two projects with one reference and Directory.Build.props | 99 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns only projects under nested Directory.Build.props directory | 121 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns project for two projects with one reference and global.json | 143 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns projects for three projects with two linear references | 163 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| returns projects for three projects with two tree-like references | 197 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| throws error on circular reference | 229 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |
| skips on invalid xml file | 245 | not-applicable | — | — | Uses vi.mock('fs-extra') + scm mock + GlobalConfig + Fixtures.mock; fs/scm mock infrastructure not portable to Rust |

---

