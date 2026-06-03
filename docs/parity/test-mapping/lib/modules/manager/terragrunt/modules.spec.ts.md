# `lib/modules/manager/terragrunt/modules.spec.ts`

[← `manager/terragrunt`](../../../../_by-module/manager/terragrunt.md) · [all modules](../../../../README.md)

**4/5 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | should split project and tag from source | ported | [`crates/renovate-core/src/extractors/terragrunt.rs:488`](../../../../../../../crates/renovate-core/src/extractors/terragrunt.rs#L488) |
| 21 | should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | ported | [`crates/renovate-core/src/extractors/terragrunt.rs:497`](../../../../../../../crates/renovate-core/src/extractors/terragrunt.rs#L497) |
| 33 | should split host, path and tag from source | ported | [`crates/renovate-core/src/extractors/terragrunt.rs:506`](../../../../../../../crates/renovate-core/src/extractors/terragrunt.rs#L506) |
| 61 | should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | ported | [`crates/renovate-core/src/extractors/terragrunt.rs:497`](../../../../../../../crates/renovate-core/src/extractors/terragrunt.rs#L497) |
| 89 | sets skipreason for invalid git tags url | ported | [`crates/renovate-core/src/extractors/terragrunt.rs:555`](../../../../../../../crates/renovate-core/src/extractors/terragrunt.rs#L555) |

