# `lib/modules/manager/terragrunt/extract.spec.ts`

[← `manager/terragrunt`](../../../../_by-module/manager/terragrunt.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns null for empty | ported | `crates/renovate-core/src/extractors/terragrunt.rs:462` |
| 10 | extracts terragrunt sources using tfr protocol | ported | `crates/renovate-core/src/extractors/terragrunt.rs:295` |
| 51 | extracts terragrunt sources | ported | `crates/renovate-core/src/extractors/terragrunt.rs:275` |
| 269 | extracts terragrunt sources with depth specified after the branch | ported | `crates/renovate-core/src/extractors/terragrunt.rs:373` |
| 487 | extracts terragrunt sources with depth specified before the branch | ported | `crates/renovate-core/src/extractors/terragrunt.rs:402` |
| 698 | returns null if only local terragrunt deps | ported | `crates/renovate-core/src/extractors/terragrunt.rs:470` |
| 707 | returns empty deps if only local terragrunt includes | ported | `crates/renovate-core/src/extractors/terragrunt.rs:481` |

