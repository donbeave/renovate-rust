# `lib/modules/datasource/repology/index.spec.ts`

[← `datasource/repology`](../../../../_by-module/datasource/repology.md) · [all modules](../../../../README.md)

**18/19 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 69 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/repology.rs:242`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L242) |
| 88 | returns null for missing repository or package | ported | [`crates/renovate-core/src/datasources/repology.rs:272`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L272) |
| 105 | throws error on unexpected api response | ported | [`crates/renovate-core/src/datasources/repology.rs:286`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L286) |
| 124 | throws error on unexpected resolver response with binary package | ported | [`crates/renovate-core/src/datasources/repology.rs:307`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L307) |
| 138 | throws error on unexpected resolver response with source package | ported | [`crates/renovate-core/src/datasources/repology.rs:318`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L318) |
| 156 | throws error on api request timeout | ported | [`crates/renovate-core/src/datasources/repology.rs:338`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L338) |
| 175 | throws error on resolver request timeout | ported | [`crates/renovate-core/src/datasources/repology.rs:360`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L360) |
| 189 | returns null on resolver ambiguous binary package | ported | [`crates/renovate-core/src/datasources/repology.rs:371`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L371) |
| 204 | throws without repository and package name | ported | [`crates/renovate-core/src/datasources/repology.rs:384`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L384) |
| 214 | throws on disabled host | pending | — |
| 225 | returns correct version for binary package | ported | [`crates/renovate-core/src/datasources/repology.rs:392`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L392) |
| 241 | returns correct version for source package | ported | [`crates/renovate-core/src/datasources/repology.rs:416`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L416) |
| 260 | returns correct version for api package | ported | [`crates/renovate-core/src/datasources/repology.rs:449`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L449) |
| 276 | returns correct version for multi-package project with same name | ported | [`crates/renovate-core/src/datasources/repology.rs:474`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L474) |
| 292 | returns correct version for multi-package project with different name | ported | [`crates/renovate-core/src/datasources/repology.rs:498`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L498) |
| 308 | returns multiple versions if they are present in repository | ported | [`crates/renovate-core/src/datasources/repology.rs:522`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L522) |
| 328 | returns null for scenario when repo is not in package results | ported | [`crates/renovate-core/src/datasources/repology.rs:548`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L548) |
| 354 | returns correct package types for api_call | ported | [`crates/renovate-core/src/datasources/repology.rs:563`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L563) |
| 443 | returns correct package versions for multi-package project | ported | [`crates/renovate-core/src/datasources/repology.rs:596`](../../../../../../../crates/renovate-core/src/datasources/repology.rs#L596) |

