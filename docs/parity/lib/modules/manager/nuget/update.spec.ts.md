# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/update.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `bumpPackageVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bumps csproj version | 17 | ported | `nuget.rs` | `nuget_bumps_csproj_version` | — |
| does not bump version twice | 28 | ported | `nuget.rs` | `nuget_does_not_bump_twice` | — |
| issue 23526 does not bump version incorrectly | 43 | ported | `nuget.rs` | `nuget_issue_23526_minor_bump` | — |
| does not bump version if version is not a semantic version | 58 | ported | `nuget.rs` | `nuget_does_not_bump_non_semver` | — |
| does not bump version if extract found no version | 69 | ported | `nuget.rs` | `nuget_does_not_bump_empty_current_value` | — |
| does not bump version if csproj has no version | 75 | ported | `nuget.rs` | `nuget_does_not_bump_when_no_version_tag` | — |
| returns content if bumping errors | 87 | ported | `nuget.rs` | `nuget_returns_content_on_invalid_bump_type` | — |
| bumps csproj version with prerelease semver level | 96 | ported | `nuget.rs` | `nuget_bumps_prerelease_version` | — |
| bumps csproj version prefix | 107 | ported | `nuget.rs` | `nuget_bumps_version_prefix` | — |

---

