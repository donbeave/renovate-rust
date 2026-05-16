# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/bump-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/bump-versions.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 23 | **Status:** pending

### `workers/repository/update/branch/bump-versions › bumpVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be noop if bumpVersions is undefined | 11 | pending | — | — | — |
| should be noop if bumpVersions is empty array | 18 | pending | — | — | — |
| should be noop if no packageFiles or artifacts have been updated | 29 | pending | — | — | — |
| should catch template error in filePatterns | 49 | pending | — | — | — |
| should catch template error in matchString | 84 | pending | — | — | — |
| should be noop if no files are matching | 122 | pending | — | — | — |
| should log debug if no matchString could be applied | 165 | pending | — | — | — |
| should catch template error in bumpType | 201 | pending | — | — | — |
| should bump version in a non edited file and add to updatedArtifacts | 239 | pending | — | — | — |
| should bump version with patch by default | 271 | pending | — | — | — |
| should bump version in an already changed packageFiles | 302 | pending | — | — | — |
| should bump version in an already changed artifact file | 347 | pending | — | — | — |
| should bump version in deleted and recreated file changed artifact file | 392 | pending | — | — | — |
| should ignore deleted file | 445 | pending | — | — | — |
| should log if file is not readable | 474 | pending | — | — | — |
| should ignore not matched strings | 518 | pending | — | — | — |
| should bump major version | 568 | pending | — | — | — |
| should bump major/minor version | 600 | pending | — | — | — |
| should bump minor version | 632 | pending | — | — | — |
| throws for invalid bump type and short version | 664 | pending | — | — | — |
| should use matched version when bumpType is sync | 696 | pending | — | — | — |
| should log debug when no upgrades found for sync type | 736 | pending | — | — | — |
| should log debug when newVersion is not found in upgrades for sync type | 766 | pending | — | — | — |

---

