# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/bump-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/bump-versions.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/branch/bump-versions › bumpVersions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be noop if bumpVersions is undefined | 11 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should be noop if bumpVersions is empty array | 18 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should be noop if no packageFiles or artifacts have been updated | 29 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should catch template error in filePatterns | 49 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should catch template error in matchString | 84 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should be noop if no files are matching | 122 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should log debug if no matchString could be applied | 165 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should catch template error in bumpType | 201 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump version in a non edited file and add to updatedArtifacts | 239 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump version with patch by default | 271 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump version in an already changed packageFiles | 302 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump version in an already changed artifact file | 347 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump version in deleted and recreated file changed artifact file | 392 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should ignore deleted file | 445 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should log if file is not readable | 474 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should ignore not matched strings | 518 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump major version | 568 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump major/minor version | 600 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should bump minor version | 632 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| throws for invalid bump type and short version | 664 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should use matched version when bumpType is sync | 696 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should log debug when no upgrades found for sync type | 736 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |
| should log debug when newVersion is not found in upgrades for sync type | 766 | not-applicable | — | — | tests version bumping in package files via manager updaters; needs full manager infrastructure |

---
