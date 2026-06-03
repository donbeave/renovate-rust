# Module: `manager/terraform`

[← all modules](../../README.md)

**Coverage:** 83/88 tests ported across 13 spec files.

| Spec file | it() | ported | pending | Rust test file(s) | Status |
|---|--:|--:|--:|---|---|
| [`lib/modules/manager/terraform/extract.spec.ts`](../../lib/modules/manager/terraform/extract.spec.ts.md) | 18 | 18 | 0 | [`crates/renovate-core/src/extractors/renovate_config_presets.rs:767`](../../../../../crates/renovate-core/src/extractors/renovate_config_presets.rs#L767)<br>[`crates/renovate-core/src/extractors/terraform.rs:1853`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L1853) | ported |
| [`lib/modules/manager/terraform/extractors/others/modules.spec.ts`](../../lib/modules/manager/terraform/extractors/others/modules.spec.ts.md) | 13 | 9 | 4 | [`crates/renovate-core/src/extractors/terraform.rs:3219`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3219) | partial |
| [`lib/modules/manager/terraform/extractors/others/providers.spec.ts`](../../lib/modules/manager/terraform/extractors/others/providers.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3075`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3075) | ported |
| [`lib/modules/manager/terraform/extractors/resources/generic-docker-image-ref.spec.ts`](../../lib/modules/manager/terraform/extractors/resources/generic-docker-image-ref.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:2937`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L2937) | ported |
| [`lib/modules/manager/terraform/extractors/resources/helm-release.spec.ts`](../../lib/modules/manager/terraform/extractors/resources/helm-release.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3086`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3086) | ported |
| [`lib/modules/manager/terraform/extractors/resources/terraform-workspaces.spec.ts`](../../lib/modules/manager/terraform/extractors/resources/terraform-workspaces.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3097`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3097) | ported |
| [`lib/modules/manager/terraform/extractors/terraform-block/required-provider.spec.ts`](../../lib/modules/manager/terraform/extractors/terraform-block/required-provider.spec.ts.md) | 3 | 3 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3119`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3119) | ported |
| [`lib/modules/manager/terraform/extractors/terraform-block/terraform-version.spec.ts`](../../lib/modules/manager/terraform/extractors/terraform-block/terraform-version.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3108`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3108) | ported |
| [`lib/modules/manager/terraform/hcl/index.spec.ts`](../../lib/modules/manager/terraform/hcl/index.spec.ts.md) | 4 | 4 | 0 | [`crates/renovate-core/src/extractors/terraform_hcl.rs:326`](../../../../../crates/renovate-core/src/extractors/terraform_hcl.rs#L326) | ported |
| [`lib/modules/manager/terraform/lockfile/hash.spec.ts`](../../lib/modules/manager/terraform/lockfile/hash.spec.ts.md) | 11 | 10 | 1 | [`crates/renovate-core/src/extractors/terraform.rs:3785`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3785) | partial |
| [`lib/modules/manager/terraform/lockfile/index.spec.ts`](../../lib/modules/manager/terraform/lockfile/index.spec.ts.md) | 26 | 26 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3467`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3467) | ported |
| [`lib/modules/manager/terraform/lockfile/update-locked.spec.ts`](../../lib/modules/manager/terraform/lockfile/update-locked.spec.ts.md) | 5 | 5 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:3021`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L3021) | ported |
| [`lib/modules/manager/terraform/lockfile/util.spec.ts`](../../lib/modules/manager/terraform/lockfile/util.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/extractors/terraform.rs:2956`](../../../../../crates/renovate-core/src/extractors/terraform.rs#L2956) | ported |

