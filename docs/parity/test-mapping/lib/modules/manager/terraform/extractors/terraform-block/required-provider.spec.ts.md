# `lib/modules/manager/terraform/extractors/terraform-block/required-provider.spec.ts`

[← `manager/terraform`](../../../../../../_by-module/manager/terraform.md) · [all modules](../../../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | return empty array if no terraform block is found | ported | `crates/renovate-core/src/extractors/terraform.rs:3119` |
| 13 | return empty array if no required_providers block is found | ported | `crates/renovate-core/src/extractors/terraform.rs:3130` |
| 18 | extract provider with version and registry url | ported | `crates/renovate-core/src/extractors/terraform.rs:3142` |

