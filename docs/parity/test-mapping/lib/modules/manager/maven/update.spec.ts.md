# `lib/modules/manager/maven/update.spec.ts`

[← `manager/maven`](../../../../_by-module/manager/maven.md) · [all modules](../../../../README.md)

**18/18 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | should update version | ported | [`crates/renovate-core/src/extractors/maven.rs:2909`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2909) |
| 36 | should do simple replacement | ported | [`crates/renovate-core/src/extractors/maven.rs:2933`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2933) |
| 58 | should do full replacement | ported | [`crates/renovate-core/src/extractors/maven.rs:2958`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2958) |
| 90 | should do replacement if version is first | ported | [`crates/renovate-core/src/extractors/maven.rs:3005`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3005) |
| 134 | should ignore replacement if name does not match | ported | [`crates/renovate-core/src/extractors/maven.rs:3047`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3047) |
| 151 | should update a cloud native buildpack version | ported | [`crates/renovate-core/src/extractors/maven.rs:3076`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3076) |
| 173 | should update a cloud native buildpack digest | ported | [`crates/renovate-core/src/extractors/maven.rs:3091`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3091) |
| 215 | bumps pom.xml version | ported | [`crates/renovate-core/src/extractors/maven.rs:3114`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3114) |
| 226 | bumps pom.xml version keeping snapshot | ported | [`crates/renovate-core/src/extractors/maven.rs:3122`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3122) |
| 237 | bumps pom.xml minor version keeping snapshot | ported | [`crates/renovate-core/src/extractors/maven.rs:3130`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3130) |
| 248 | bumps pom.xml major version keeping snapshot | ported | [`crates/renovate-core/src/extractors/maven.rs:3138`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3138) |
| 259 | bumps pom.xml version keeping qualifier with -snapshot | ported | [`crates/renovate-core/src/extractors/maven.rs:3146`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3146) |
| 273 | does not bump version twice | ported | [`crates/renovate-core/src/extractors/maven.rs:3155`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3155) |
| 288 | does not bump version if version is not a semantic version | ported | [`crates/renovate-core/src/extractors/maven.rs:3165`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3165) |
| 299 | does not bump version if pom.xml has no version | ported | [`crates/renovate-core/src/extractors/maven.rs:3174`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3174) |
| 305 | returns content if bumping errors | ported | [`crates/renovate-core/src/extractors/maven.rs:3181`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3181) |
| 314 | bumps pom.xml version to snapshot with prerelease | ported | [`crates/renovate-core/src/extractors/maven.rs:3189`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3189) |
| 325 | bumps pom.xml version with prerelease semver level | ported | [`crates/renovate-core/src/extractors/maven.rs:3197`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L3197) |

