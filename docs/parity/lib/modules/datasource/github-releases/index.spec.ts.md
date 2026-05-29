# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-releases/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/datasource/github-releases/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases | 20 | ported | `crates/renovate-core/src/datasources/github_releases.rs` | `returns_releases` | filters non-version strings; prerelease→is_stable=false; sourceUrl+registryUrl |

### `modules/datasource/github-releases/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be independent of the current digest | 116 | ported | `crates/renovate-core/src/datasources/github_releases.rs` | `digest_independent_of_current_digest` | tag hash lookup ignores currentDigest |
| should be independent of the current value | 128 | ported | `crates/renovate-core/src/datasources/github_releases.rs` | `digest_independent_of_current_value` | tag hash lookup ignores currentValue |
| returns updated digest in new release | 136 | ported | `crates/renovate-core/src/datasources/github_releases.rs` | `returns_updated_digest_in_new_release` | returns sha for newValue tag |
| returns null if the new value/tag does not exist | 149 | ported | `crates/renovate-core/src/datasources/github_releases.rs` | `returns_null_for_unknown_tag` | unknown tag → None |

---
