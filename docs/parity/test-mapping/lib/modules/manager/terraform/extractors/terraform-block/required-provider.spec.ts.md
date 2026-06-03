# `lib/modules/manager/terraform/extractors/terraform-block/required-provider.spec.ts`

[← `manager/terraform`](../../../../../../_by-module/manager/terraform.md) · [all modules](../../../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | return empty array if no terraform block is found | ported | [`crates/renovate-core/src/extractors/terraform.rs:3119`](../../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3119) |
| 13 | return empty array if no required_providers block is found | ported | [`crates/renovate-core/src/extractors/terraform.rs:3130`](../../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3130) |
| 18 | extract provider with version and registry url | ported | [`crates/renovate-core/src/extractors/terraform.rs:3142`](../../../../../../../../../crates/renovate-core/src/extractors/terraform.rs#L3142) |

