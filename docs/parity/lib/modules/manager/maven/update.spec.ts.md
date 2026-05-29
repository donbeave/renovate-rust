# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven/update.spec.ts
**Total tests:** 18 | **Ported:** 18 | **Actionable:** 18 | **Status:** done

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update version | 15 | ported | `extractors/maven.rs` | `maven_update_dep_version` | — |
| should do simple replacement | 36 | ported | `extractors/maven.rs` | `maven_update_dep_simple_replacement` | — |
| should do full replacement | 58 | ported | `extractors/maven.rs` | `maven_update_dep_full_replacement` | — |
| should do replacement if version is first | 90 | ported | `extractors/maven.rs` | `maven_update_dep_replacement_version_first` | — |
| should ignore replacement if name does not match | 134 | ported | `extractors/maven.rs` | `maven_update_dep_ignore_mismatched_name` | — |
| should update a cloud native buildpack version | 151 | ported | `extractors/maven.rs` | `maven_update_dep_cnb_version` | — |
| should update a cloud native buildpack digest | 173 | ported | `extractors/maven.rs` | `maven_update_dep_cnb_digest` | — |

### `bumpPackageVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bumps pom.xml version | 215 | ported | `extractors/maven.rs` | `maven_bump_version_patch` | — |
| bumps pom.xml version keeping SNAPSHOT | 226 | ported | `extractors/maven.rs` | `maven_bump_version_snapshot_patch` | — |
| bumps pom.xml minor version keeping SNAPSHOT | 237 | ported | `extractors/maven.rs` | `maven_bump_version_snapshot_minor` | — |
| bumps pom.xml major version keeping SNAPSHOT | 248 | ported | `extractors/maven.rs` | `maven_bump_version_snapshot_major` | — |
| bumps pom.xml version keeping qualifier with -SNAPSHOT | 259 | ported | `extractors/maven.rs` | `maven_bump_version_qualified_snapshot` | — |
| does not bump version twice | 273 | ported | `extractors/maven.rs` | `maven_bump_version_not_twice` | — |
| does not bump version if version is not a semantic version | 288 | ported | `extractors/maven.rs` | `maven_bump_version_non_semver` | — |
| does not bump version if pom.xml has no version | 299 | ported | `extractors/maven.rs` | `maven_bump_version_no_version` | — |
| returns content if bumping errors | 305 | ported | `extractors/maven.rs` | `maven_bump_version_error_returns_content` | — |
| bumps pom.xml version to SNAPSHOT with prerelease | 314 | ported | `extractors/maven.rs` | `maven_bump_version_prerelease_adds_snapshot` | — |
| bumps pom.xml version with prerelease semver level | 325 | ported | `extractors/maven.rs` | `maven_bump_version_prerelease_increment` | — |

---
