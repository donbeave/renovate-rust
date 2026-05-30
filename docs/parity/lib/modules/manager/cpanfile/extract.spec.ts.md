# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cpanfile/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cpanfile/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 0 | **Status:** done

### `extractPackageFile() › parse perl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $version | 12 | ported | `cpanfile.rs` | `parse_perl_core_dependency` | — |
| returns null for empty | 6 | ported | `cpanfile.rs` | `empty_input_returns_no_deps` | — |
| parse modules with requires | 39 | ported | `cpanfile.rs` | `extracts_basic_requires` (+ extracts_fat_arrow_form) | — |
| parse modules with recommends | 113 | ported | `cpanfile.rs` | `parse_modules_with_recommends` | — |
| parse modules with suggests | 138 | ported | `cpanfile.rs` | `parse_modules_with_suggests` | — |

### `extractPackageFile() › parse modules with phases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| configure phase | 164 | ported | `cpanfile.rs` | `parse_phase_configure` | — |
| build phase | 186 | ported | `cpanfile.rs` | `parse_phase_build_bareword` | — |
| phase | 208 | ported | `cpanfile.rs` | `extracts_test_phase_block` | — |
| runtime phase | 237 | ported | `cpanfile.rs` | `parse_phase_runtime_bareword_suggests` | — |
| develop phase | 266 | ported | `cpanfile.rs` | `parse_phase_develop` | — |

### `extractPackageFile() › parse modules with phase shortcuts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $shortcut (configure_requires/build_requires/test_requires/author_requires) | 296 | ported | `cpanfile.rs` | `extracts_phase_shortcut_keywords` | — |

---

