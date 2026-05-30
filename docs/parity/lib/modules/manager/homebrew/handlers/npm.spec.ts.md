# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/homebrew/handlers/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homebrew/handlers/npm.spec.ts
**Total tests:** 15 | **Ported:** 14 | **Actionable:** 0 | **Status:** done

### `parseUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty string | 8 | ported | `homebrew.rs` | `npm_parse_url_empty_string_returns_none` | — |
| returns null for non-string input: %s | 12 | not-applicable | — | — | TypeScript type-system test; it.each([null, undefined]) with input as never cannot occur in Rust's type system |
| returns null for non-npm registry URL | 19 | ported | `homebrew.rs` | `npm_parse_url_non_npm_registry_returns_none` | — |
| returns null for custom npm registry | 25 | ported | `homebrew.rs` | `npm_parse_url_custom_registry_returns_none` | — |
| parses scoped package URL | 33 | ported | `homebrew.rs` | `npm_parse_url_scoped_package` | — |
| parses unscoped package URL | 45 | ported | `homebrew.rs` | `npm_parse_url_unscoped_package` | — |
| parses version with prerelease | 57 | ported | `homebrew.rs` | `npm_parse_url_prerelease_version` | — |
| parses version with build metadata | 69 | ported | `homebrew.rs` | `npm_parse_url_build_metadata_version` | — |
| returns null for malformed URL | 81 | ported | `homebrew.rs` | `npm_parse_url_malformed_returns_none` | — |

### `createDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates dependency with npm datasource for scoped package | 89 | ported | `homebrew.rs` | `npm_create_dependency_scoped` | — |
| creates dependency with npm datasource for unscoped package | 116 | ported | `homebrew.rs` | `npm_create_dependency_unscoped` | — |

### `buildArchiveUrls`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| builds URL for scoped package | 145 | ported | `homebrew.rs` | `npm_build_archive_urls_scoped` | — |
| builds URL for unscoped package | 160 | ported | `homebrew.rs` | `npm_build_archive_urls_unscoped` | — |
| builds URL with prerelease version | 175 | ported | `homebrew.rs` | `npm_build_archive_urls_prerelease` | — |
| builds URL for deeply scoped package | 190 | ported | `homebrew.rs` | `npm_build_archive_urls_deeply_scoped` | — |

---

