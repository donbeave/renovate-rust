# `lib/modules/manager/terraform/lockfile/index.spec.ts`

[← `manager/terraform`](../../../../../_by-module/manager/terraform.md) · [all modules](../../../../../README.md)

**26/26 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 36 | returns artifact error | ported | `crates/renovate-core/src/extractors/terraform.rs:3654` |
| 56 | returns null if no .terraform.lock.hcl found | ported | `crates/renovate-core/src/extractors/terraform.rs:3598` |
| 67 | returns null if .terraform.lock.hcl is empty | ported | `crates/renovate-core/src/extractors/terraform.rs:3616` |
| 81 | returns null if .terraform.lock.hcl is invalid | ported | `crates/renovate-core/src/extractors/terraform.rs:3635` |
| 95 | update single dependency with exact constraint and deptype provider | ported | `crates/renovate-core/src/extractors/terraform.rs:3939` |
| 151 | update single dependency with exact constraint and and deptype required_provider | ported | `crates/renovate-core/src/extractors/terraform.rs:3972` |
| 209 | does not update dependency with exact constraint during lockfile update | ported | `crates/renovate-core/src/extractors/terraform.rs:3704` |
| 249 | does not update dependency with exact constraint within multiple during lockfile update | ported | `crates/renovate-core/src/extractors/terraform.rs:3738` |
| 289 | do not update dependency with deptype module | ported | `crates/renovate-core/src/extractors/terraform.rs:3674` |
| 307 | update single dependency with range constraint and minor update from private registry | ported | `crates/renovate-core/src/extractors/terraform.rs:4005` |
| 366 | update single dependency with range constraint and major update | ported | `crates/renovate-core/src/extractors/terraform.rs:4040` |
| 424 | update single dependency in subfolder | ported | `crates/renovate-core/src/extractors/terraform.rs:4074` |
| 484 | update multiple dependencies which are not ordered | ported | `crates/renovate-core/src/extractors/terraform.rs:4113` |
| 621 | do full lock file maintenance | ported | `crates/renovate-core/src/extractors/terraform.rs:4216` |
| 757 | do full lock file maintenance with lockfile in subfolder | ported | `crates/renovate-core/src/extractors/terraform.rs:4301` |
| 873 | do full lock file maintenance without necessary changes | ported | `crates/renovate-core/src/extractors/terraform.rs:4280` |
| 933 | return null if hashing fails | ported | `crates/renovate-core/src/extractors/terraform.rs:3907` |
| 1023 | return null if experimental flag is not set | ported | `crates/renovate-core/src/extractors/terraform.rs:4198` |
| 1037 | preserves constraints when current value and new value are same | ported | `crates/renovate-core/src/extractors/terraform.rs:3548` |
| 1097 | replaces current value to new version within a constraint | ported | `crates/renovate-core/src/extractors/terraform.rs:3564` |
| 1157 | replaces current version to new version within a constraint | ported | `crates/renovate-core/src/extractors/terraform.rs:3580` |
| 1217 | correctly calculate new constraint on pinning | ported | `crates/renovate-core/src/extractors/terraform.rs:3467` |
| 1230 | update constraint with multiple elements | ported | `crates/renovate-core/src/extractors/terraform.rs:3483` |
| 1243 | update constraint when current version is matched multiple times | ported | `crates/renovate-core/src/extractors/terraform.rs:3499` |
| 1256 | update constraint when current version is in a complicated constraint | ported | `crates/renovate-core/src/extractors/terraform.rs:3515` |
| 1269 | create constraint with full version | ported | `crates/renovate-core/src/extractors/terraform.rs:3531` |

