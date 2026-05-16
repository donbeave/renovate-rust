# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/cargo/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/cargo/update.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `bumpPackageVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| increments | 12 | ported | `cargo.rs` | `bump_package_version_increments_patch` | — |
| no ops | 22 | ported | `cargo.rs` | `bump_package_version_no_ops_when_current_value_mismatch` | — |
| updates | 31 | ported | `cargo.rs` | `bump_package_version_updates_minor` | — |
| returns content if bumping errors | 41 | ported | `cargo.rs` | `bump_package_version_returns_content_on_invalid_bump_type` | — |
| does not bump version if version is not a semantic version | 50 | ported | `cargo.rs` | `bump_package_version_no_bump_if_not_semver` | — |

---

