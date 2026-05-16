# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/haskell-cabal/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/haskell-cabal/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `countPackageNameLength`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| matches $input | 20 | ported | `cabal.rs` | `cabal_count_package_name_length` | it.each; all 17 cases covered |

### `countPrecedingIndentation()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| countPrecedingIndentation($content, $index) | 45 | ported | `cabal.rs` | `cabal_count_preceding_indentation` | it.each; 6 cases covered |

### `findExtents()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findExtents($indent, $content) | 62 | ported | `cabal.rs` | `cabal_find_extents` | it.each; 5 cases covered |

### `splitSingleDependency()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| splitSingleDependency($depLine) | 75 | ported | `cabal.rs` | `cabal_split_single_dependency` | it.each + 1 inline case |

### `extractNamesAndRanges()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| trims replaceString | 95 | ported | `cabal.rs` | `cabal_extract_names_and_ranges` | — |

### `findDepends()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips comments | 105 | ported | `cabal.rs` | `cabal_find_depends_strips_comments` | — |

---

