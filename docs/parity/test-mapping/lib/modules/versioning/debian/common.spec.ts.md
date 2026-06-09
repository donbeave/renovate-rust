# `lib/modules/versioning/debian/common.spec.ts`

[← `versioning/debian`](../../../../_by-module/versioning/debian.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | no rolling release data | opt-out | asserts TypeScript logger.debug spy behavior (toHaveBeenCalledTimes(1) + exact 'RollingReleasesData - data written' message) when dataFiles is overridden to empty distro json (no rolling releases); also checks RollingReleasesData.has() returns false for known codenames. The spy + test harness dataFiles injection + DistroInfo/RollingReleasesData construction has no direct Rust equivalent (Rust debian versioning uses embedded/parsed data without this spy surface or 'data written' debug side-effect); core fallback behavior for missing data is simple and exercised by other data-driven versioning tests. |
| 31 | _(it.each / template — verify manually)_ | ? | — |
| 48 | _(it.each / template — verify manually)_ | ? | — |
| 69 | _(it.each / template — verify manually)_ | ? | — |
| 87 | _(it.each / template — verify manually)_ | ? | — |

