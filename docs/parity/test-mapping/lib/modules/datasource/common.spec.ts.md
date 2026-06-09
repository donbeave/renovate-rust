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
| 95 | should return the same release result if extractversion is not defined | ported | [`crates/renovate-core/src/util.rs:12504`](../../../../../../crates/renovate-core/src/util.rs#L12504) |
| 103 | should extract version from release using provided regex | ported | [`crates/renovate-core/src/util.rs:12512`](../../../../../../crates/renovate-core/src/util.rs#L12512) |
| 116 | should return null for releases with invalid version | ported | [`crates/renovate-core/src/util.rs:12523`](../../../../../../crates/renovate-core/src/util.rs#L12523) |
| 136 | should filter out invalid versions | ported | [`crates/renovate-core/src/util.rs:12532`](../../../../../../crates/renovate-core/src/util.rs#L12532) |
| 144 | should use default versioning if none is specified | ported | [`crates/renovate-core/src/util.rs:12558`](../../../../../../crates/renovate-core/src/util.rs#L12558) |
| 152 | should use specified versioning if provided | ported | [`crates/renovate-core/src/util.rs:12546`](../../../../../../crates/renovate-core/src/util.rs#L12546) |
| 162 | sorts releases by version and removes duplicates | ported | [`crates/renovate-core/src/util.rs:12664`](../../../../../../crates/renovate-core/src/util.rs#L12664) |
| 183 | uses default versioning if none is specified | ported | [`crates/renovate-core/src/util.rs:12584`](../../../../../../crates/renovate-core/src/util.rs#L12584) |
| 201 | should remove constraints from releases if constraintsfiltering is not strict | ported | [`crates/renovate-core/src/datasources.rs:1651`](../../../../../../crates/renovate-core/src/datasources.rs#L1651) |
| 230 | should filter releases based on constraints if constraintsfiltering is strict | ported | [`crates/renovate-core/src/datasources.rs:1680`](../../../../../../crates/renovate-core/src/datasources.rs#L1680) |
| 250 | should return all releases when no configconstraints | ported | [`crates/renovate-core/src/datasources.rs:1717`](../../../../../../crates/renovate-core/src/datasources.rs#L1717) |
| 268 | should match exact constraints | ported | [`crates/renovate-core/src/datasources.rs:1743`](../../../../../../crates/renovate-core/src/datasources.rs#L1743) |
| 287 | should handle config with a range constraint, and a release with an exact version | ported | [`crates/renovate-core/src/datasources.rs:1773`](../../../../../../crates/renovate-core/src/datasources.rs#L1773) |
| 306 | should handle config with an exact version, and a release with a range constraint | ported | [`crates/renovate-core/src/datasources.rs:1852`](../../../../../../crates/renovate-core/src/datasources.rs#L1852) |
| 325 | should allow constraintsversioning to override the datasource's default versioning | ported | [`crates/renovate-core/src/datasources.rs:1803`](../../../../../../crates/renovate-core/src/datasources.rs#L1803) |
| 378 | returns immediately if no versioncompatibility | ported | [`crates/renovate-core/src/util.rs:12596`](../../../../../../crates/renovate-core/src/util.rs#L12596) |
| 383 | filters out non-matching | ported | [`crates/renovate-core/src/util.rs:12604`](../../../../../../crates/renovate-core/src/util.rs#L12604) |
| 395 | filters out incompatible | ported | [`crates/renovate-core/src/util.rs:12620`](../../../../../../crates/renovate-core/src/util.rs#L12620) |
| 407 | does not override versionorig from extractversion | ported | [`crates/renovate-core/src/util.rs:12641`](../../../../../../crates/renovate-core/src/util.rs#L12641) |

