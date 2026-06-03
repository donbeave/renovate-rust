# `lib/modules/datasource/terraform-module/utils.spec.ts`

[← `datasource/terraform-module`](../../../../_by-module/datasource/terraform-module.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | returns url with relative sd for modules | ported | [`crates/renovate-core/src/datasources/terraform.rs:496`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L496) |
| 21 | returns url with relative sd for providers | ported | [`crates/renovate-core/src/datasources/terraform.rs:511`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L511) |
| 35 | returns url with absolute sd for modules | ported | [`crates/renovate-core/src/datasources/terraform.rs:526`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L526) |
| 49 | returns url with absolute sd for providers and missing trailing slash | ported | [`crates/renovate-core/src/datasources/terraform.rs:541`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L541) |
| 63 | returns url with with empty sd | ported | [`crates/renovate-core/src/datasources/terraform.rs:556`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L556) |
| 75 | returns url with with missing sd | ported | [`crates/renovate-core/src/datasources/terraform.rs:568`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L568) |
| 87 | uses the configured registry url for standard package names | ported | [`crates/renovate-core/src/datasources/terraform.rs:580`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L580) |
| 99 | extracts the registry from packagename when it is embedded | ported | [`crates/renovate-core/src/datasources/terraform.rs:591`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L591) |
| 111 | normalizes an embedded registry without a scheme | ported | [`crates/renovate-core/src/datasources/terraform.rs:600`](../../../../../../../crates/renovate-core/src/datasources/terraform.rs#L600) |

