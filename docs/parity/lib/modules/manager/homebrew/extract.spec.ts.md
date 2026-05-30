# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/homebrew/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homebrew/extract.spec.ts
**Total tests:** 17 | **Ported:** 17 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips sourceforge dependency 1 | 10 | ported | `homebrew.rs` | `unsupported_url_skipped` | — |
| skips sourceforge dependency 2 | 32 | ported | `homebrew.rs` | `skips_sourceforge_dependency_2` | — |
| skips github dependency with wrong format | 54 | ported | `homebrew.rs` | `skips_github_dependency_wrong_format` | — |
| extracts "releases" github dependency | 77 | ported | `homebrew.rs` | `extracts_github_release` | — |
| extracts "archive" github dependency | 99 | ported | `homebrew.rs` | `extracts_github_archive_refs_tags` | — |
| handles old "archive" github url format | 121 | ported | `homebrew.rs` | `extracts_github_archive_old_form` | — |
| handles no space before class header | 152 | ported | `homebrew.rs` | `handles_no_space_before_class_header` | — |
| returns null for invalid class header 1 | 183 | ported | `homebrew.rs` | `no_class_header_returns_none` | — |
| returns null for invalid class header 2 | 198 | ported | `homebrew.rs` | `invalid_class_header_not_formula_returns_none` | — |
| skips if there is no url field | 213 | ported | `homebrew.rs` | `missing_url_skipped` | — |
| skips if invalid url protocol | 235 | ported | `homebrew.rs` | `skips_invalid_url_protocol` | — |
| skips if invalid url | 257 | ported | `homebrew.rs` | `skips_invalid_url` | — |
| skips if there is no sha256 field | 279 | ported | `homebrew.rs` | `skips_no_sha256_field` | — |
| skips if sha256 field is invalid | 301 | ported | `homebrew.rs` | `invalid_sha256_skipped` | — |
| extracts npm scoped package dependency | 323 | ported | `homebrew.rs` | `extracts_npm_scoped_package` | — |
| extracts npm unscoped package dependency | 354 | ported | `homebrew.rs` | `extracts_npm_unscoped_package` | — |
| skips npm package from custom registry | 385 | ported | `homebrew.rs` | `skips_npm_custom_registry` | — |

---

