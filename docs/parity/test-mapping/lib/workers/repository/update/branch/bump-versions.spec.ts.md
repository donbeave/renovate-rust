# `lib/workers/repository/update/branch/bump-versions.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/23 in-scope tests ported** (23 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | should be noop if bumpversions is undefined | pending | — |
| 18 | should be noop if bumpversions is empty array | pending | — |
| 29 | should be noop if no packagefiles or artifacts have been updated | pending | — |
| 49 | should catch template error in filepatterns | pending | — |
| 84 | should catch template error in matchstring | pending | — |
| 122 | should be noop if no files are matching | pending | — |
| 165 | should log debug if no matchstring could be applied | pending | — |
| 201 | should catch template error in bumptype | pending | — |
| 239 | should bump version in a non edited file and add to updatedartifacts | pending | — |
| 271 | should bump version with patch by default | pending | — |
| 302 | should bump version in an already changed packagefiles | pending | — |
| 347 | should bump version in an already changed artifact file | pending | — |
| 392 | should bump version in deleted and recreated file changed artifact file | pending | — |
| 445 | should ignore deleted file | pending | — |
| 474 | should log if file is not readable | pending | — |
| 518 | should ignore not matched strings | pending | — |
| 568 | should bump major version | pending | — |
| 600 | should bump major/minor version | pending | — |
| 632 | should bump minor version | pending | — |
| 664 | throws for invalid bump type and short version | pending | — |
| 696 | should use matched version when bumptype is sync | pending | — |
| 736 | should log debug when no upgrades found for sync type | pending | — |
| 766 | should log debug when newversion is not found in upgrades for sync type | pending | — |

