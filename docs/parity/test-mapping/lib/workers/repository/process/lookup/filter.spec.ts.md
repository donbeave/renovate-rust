# `lib/workers/repository/process/lookup/filter.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**6/9 in-scope tests ported** (3 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | should filter versions allowed by semver syntax when allowedversions is not valid version, range or pypi syntax | pending | — |
| 60 | should filter versions when allowedversions templating is used | pending | — |
| 98 | allows unstable major upgrades | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:320`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L320) |
| 124 | ignores version insufficient prefixes | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:352`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L352) |
| 153 | single version range, but invalid current version (for coverage) | pending | — |
| 187 | filters versions with major increment greater than maxmajorincrement | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:177`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L177) |
| 216 | allows all versions when maxmajorincrement is 0 | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:249`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L249) |
| 243 | filters with maxmajorincrement set to 1 | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:213`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L213) |
| 272 | handles maxmajorincrement with 0.x versions | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:284`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L284) |

