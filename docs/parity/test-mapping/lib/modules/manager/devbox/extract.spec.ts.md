# `lib/modules/manager/devbox/extract.spec.ts`

[← `manager/devbox`](../../../../_by-module/manager/devbox.md) · [all modules](../../../../README.md)

**13/13 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns null when the devbox json file is empty | ported | `crates/renovate-core/src/extractors/devbox.rs:151` |
| 11 | returns null when the devbox json file is malformed | ported | `crates/renovate-core/src/extractors/devbox.rs:145` |
| 16 | returns null when the devbox json file has no packages | ported | `crates/renovate-core/src/extractors/devbox.rs:138` |
| 21 | returns a package dependency when the devbox json file has a single package | ported | `crates/renovate-core/src/extractors/devbox.rs:106` |
| 42 | returns a package dependency when the devbox json file has a single package with a version object | ported | `crates/renovate-core/src/extractors/devbox.rs:129` |
| 65 | returns invalid-version when the devbox json file has a single package with an invalid version | ported | `crates/renovate-core/src/extractors/devbox.rs:157` |
| 89 | returns a package dependency when the devbox json file has multiple packages | ported | `crates/renovate-core/src/extractors/devbox.rs:168` |
| 115 | returns a package dependency when the devbox json file has multiple packages with in a packages object | ported | `crates/renovate-core/src/extractors/devbox.rs:119` |
| 144 | returns a package dependency when the devbox json file has multiple packages with package objects | ported | `crates/renovate-core/src/extractors/devbox.rs:191` |
| 177 | returns invalid dependencies | ported | `crates/renovate-core/src/extractors/devbox.rs:177` |
| 213 | returns invalid dependencies with package objects | ported | `crates/renovate-core/src/extractors/devbox.rs:205` |
| 251 | returns invalid dependencies from the packages array | ported | `crates/renovate-core/src/extractors/devbox.rs:216` |
| 288 | returns null if there are no dependencies | ported | `crates/renovate-core/src/extractors/devbox.rs:240` |

