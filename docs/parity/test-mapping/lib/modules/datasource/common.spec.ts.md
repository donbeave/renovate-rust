# `lib/modules/datasource/common.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 21 | returns null for unknown datasource | ported | [`crates/renovate-core/src/datasources.rs:875`](../../../../../../crates/renovate-core/src/datasources.rs#L875) |
| 25 | supports custom datasource | ported | [`crates/renovate-core/src/datasources.rs:881`](../../../../../../crates/renovate-core/src/datasources.rs#L881) |
| 31 | returns datasource for known datasource | ported | [`crates/renovate-core/src/datasources.rs:890`](../../../../../../crates/renovate-core/src/datasources.rs#L890) |
| 39 | returns default versioning for undefined datasource | ported | [`crates/renovate-core/src/datasources.rs:897`](../../../../../../crates/renovate-core/src/datasources.rs#L897) |
| 43 | returns default versioning for unknown datasource | ported | [`crates/renovate-core/src/datasources.rs:903`](../../../../../../crates/renovate-core/src/datasources.rs#L903) |
| 52 | returns default versioning for datasource with missing default versioning configuration | ported | [`crates/renovate-core/src/datasources.rs:912`](../../../../../../crates/renovate-core/src/datasources.rs#L912) |
| 56 | returns datasource-defined default versioning | ported | [`crates/renovate-core/src/datasources.rs:922`](../../../../../../crates/renovate-core/src/datasources.rs#L922) |
| 62 | returns true for valid input | ported | [`crates/renovate-core/src/datasources.rs:929`](../../../../../../crates/renovate-core/src/datasources.rs#L929) |
| 70 | returns false for invalid input | ported | [`crates/renovate-core/src/datasources.rs:936`](../../../../../../crates/renovate-core/src/datasources.rs#L936) |
| 78 | returns false for input with missing properties | ported | [`crates/renovate-core/src/datasources.rs:943`](../../../../../../crates/renovate-core/src/datasources.rs#L943) |
| 85 | returns false for input with non-string properties | ported | [`crates/renovate-core/src/datasources.rs:950`](../../../../../../crates/renovate-core/src/datasources.rs#L950) |
| 95 | should return the same release result if extractversion is not defined | ported | [`crates/renovate-core/src/util.rs:12498`](../../../../../../crates/renovate-core/src/util.rs#L12498) |
| 103 | should extract version from release using provided regex | ported | [`crates/renovate-core/src/util.rs:12506`](../../../../../../crates/renovate-core/src/util.rs#L12506) |
| 116 | should return null for releases with invalid version | ported | [`crates/renovate-core/src/util.rs:12517`](../../../../../../crates/renovate-core/src/util.rs#L12517) |
| 136 | should filter out invalid versions | ported | [`crates/renovate-core/src/util.rs:12526`](../../../../../../crates/renovate-core/src/util.rs#L12526) |
| 144 | should use default versioning if none is specified | ported | [`crates/renovate-core/src/util.rs:12552`](../../../../../../crates/renovate-core/src/util.rs#L12552) |
| 152 | should use specified versioning if provided | ported | [`crates/renovate-core/src/util.rs:12540`](../../../../../../crates/renovate-core/src/util.rs#L12540) |
| 162 | sorts releases by version and removes duplicates | ported | [`crates/renovate-core/src/util.rs:12658`](../../../../../../crates/renovate-core/src/util.rs#L12658) |
| 183 | uses default versioning if none is specified | ported | [`crates/renovate-core/src/util.rs:12578`](../../../../../../crates/renovate-core/src/util.rs#L12578) |
| 201 | should remove constraints from releases if constraintsfiltering is not strict | ported | [`crates/renovate-core/src/datasources.rs:1452`](../../../../../../crates/renovate-core/src/datasources.rs#L1452) |
| 230 | should filter releases based on constraints if constraintsfiltering is strict | ported | [`crates/renovate-core/src/datasources.rs:1481`](../../../../../../crates/renovate-core/src/datasources.rs#L1481) |
| 250 | should return all releases when no configconstraints | ported | [`crates/renovate-core/src/datasources.rs:1518`](../../../../../../crates/renovate-core/src/datasources.rs#L1518) |
| 268 | should match exact constraints | ported | [`crates/renovate-core/src/datasources.rs:1544`](../../../../../../crates/renovate-core/src/datasources.rs#L1544) |
| 287 | should handle config with a range constraint, and a release with an exact version | ported | [`crates/renovate-core/src/datasources.rs:1574`](../../../../../../crates/renovate-core/src/datasources.rs#L1574) |
| 306 | should handle config with an exact version, and a release with a range constraint | ported | [`crates/renovate-core/src/datasources.rs:1653`](../../../../../../crates/renovate-core/src/datasources.rs#L1653) |
| 325 | should allow constraintsversioning to override the datasource's default versioning | ported | [`crates/renovate-core/src/datasources.rs:1604`](../../../../../../crates/renovate-core/src/datasources.rs#L1604) |
| 378 | returns immediately if no versioncompatibility | ported | [`crates/renovate-core/src/util.rs:12590`](../../../../../../crates/renovate-core/src/util.rs#L12590) |
| 383 | filters out non-matching | ported | [`crates/renovate-core/src/util.rs:12598`](../../../../../../crates/renovate-core/src/util.rs#L12598) |
| 395 | filters out incompatible | ported | [`crates/renovate-core/src/util.rs:12614`](../../../../../../crates/renovate-core/src/util.rs#L12614) |
| 407 | does not override versionorig from extractversion | ported | [`crates/renovate-core/src/util.rs:12635`](../../../../../../crates/renovate-core/src/util.rs#L12635) |

