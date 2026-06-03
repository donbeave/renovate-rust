# `lib/workers/repository/process/lookup/filter.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**4/9 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | should filter versions allowed by semver syntax when allowedversions is not valid version, range or pypi syntax | pending | — |
| 60 | should filter versions when allowedversions templating is used | pending | — |
| 98 | allows unstable major upgrades | pending | — |
| 124 | ignores version insufficient prefixes | pending | — |
| 153 | single version range, but invalid current version (for coverage) | pending | — |
| 187 | filters versions with major increment greater than maxmajorincrement | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:207`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L207) |
| 216 | allows all versions when maxmajorincrement is 0 | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:227`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L227) |
| 243 | filters with maxmajorincrement set to 1 | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:245`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L245) |
| 272 | handles maxmajorincrement with 0.x versions | ported | [`crates/renovate-core/src/workers/repository/process/lookup/filter.rs:265`](../../../../../../../../crates/renovate-core/src/workers/repository/process/lookup/filter.rs#L265) |

