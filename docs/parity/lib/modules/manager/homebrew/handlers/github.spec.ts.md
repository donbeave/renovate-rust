# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/homebrew/handlers/github.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/homebrew/handlers/github.spec.ts
**Total tests:** 8 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `parseUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty string | 8 | ported | `homebrew.rs` | `github_parse_url_empty_string_returns_none` | — |
| returns null for non-string input: %s | 12 | not-applicable | — | — | TypeScript type-system test; it.each([null, undefined]) with input as never cannot occur in Rust's type system |
| parses valid releases URL | 19 | ported | `homebrew.rs` | `github_parse_url_releases` | — |
| parses valid archive URL | 33 | ported | `homebrew.rs` | `github_parse_url_archive` | — |

### `buildArchiveUrls`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses original version when semver.coerce fails | 49 | ported | `homebrew.rs` | `github_build_archive_urls_non_semver` | — |
| uses coerced version for filename when semver succeeds | 66 | ported | `homebrew.rs` | `github_build_archive_urls_semver_coerce` | — |

### `createDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates dependency with github-releases datasource for releases URL | 85 | ported | `homebrew.rs` | `github_create_dependency_releases` | — |
| creates dependency with github-tags datasource for archive URL | 107 | ported | `homebrew.rs` | `github_create_dependency_archive` | — |

---

