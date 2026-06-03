# `lib/modules/manager/terraform/lockfile/update-locked.spec.ts`

[← `manager/terraform`](../../../../../_by-module/manager/terraform.md) · [all modules](../../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 35 | detects already updated | ported | `crates/renovate-core/src/extractors/terraform.rs:3021` |
| 47 | returns unsupported if dependency is undefined | ported | `crates/renovate-core/src/extractors/terraform.rs:3032` |
| 59 | returns unsupported if lockfilecontent is undefined | ported | `crates/renovate-core/src/extractors/terraform.rs:3039` |
| 70 | returns unsupported | ported | `crates/renovate-core/src/extractors/terraform.rs:3047` |
| 82 | returns update-failed for errors | ported | `crates/renovate-core/src/extractors/terraform.rs:3058` |

