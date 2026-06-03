# `lib/modules/datasource/repology/index.spec.ts`

[← `datasource/repology`](../../../../_by-module/datasource/repology.md) · [all modules](../../../../README.md)

**18/19 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 69 | returns null for empty result | ported | `crates/renovate-core/src/datasources/repology.rs:242` |
| 88 | returns null for missing repository or package | ported | `crates/renovate-core/src/datasources/repology.rs:272` |
| 105 | throws error on unexpected api response | ported | `crates/renovate-core/src/datasources/repology.rs:286` |
| 124 | throws error on unexpected resolver response with binary package | ported | `crates/renovate-core/src/datasources/repology.rs:307` |
| 138 | throws error on unexpected resolver response with source package | ported | `crates/renovate-core/src/datasources/repology.rs:318` |
| 156 | throws error on api request timeout | ported | `crates/renovate-core/src/datasources/repology.rs:338` |
| 175 | throws error on resolver request timeout | ported | `crates/renovate-core/src/datasources/repology.rs:360` |
| 189 | returns null on resolver ambiguous binary package | ported | `crates/renovate-core/src/datasources/repology.rs:371` |
| 204 | throws without repository and package name | ported | `crates/renovate-core/src/datasources/repology.rs:384` |
| 214 | throws on disabled host | pending | — |
| 225 | returns correct version for binary package | ported | `crates/renovate-core/src/datasources/repology.rs:392` |
| 241 | returns correct version for source package | ported | `crates/renovate-core/src/datasources/repology.rs:416` |
| 260 | returns correct version for api package | ported | `crates/renovate-core/src/datasources/repology.rs:449` |
| 276 | returns correct version for multi-package project with same name | ported | `crates/renovate-core/src/datasources/repology.rs:474` |
| 292 | returns correct version for multi-package project with different name | ported | `crates/renovate-core/src/datasources/repology.rs:498` |
| 308 | returns multiple versions if they are present in repository | ported | `crates/renovate-core/src/datasources/repology.rs:522` |
| 328 | returns null for scenario when repo is not in package results | ported | `crates/renovate-core/src/datasources/repology.rs:548` |
| 354 | returns correct package types for api_call | ported | `crates/renovate-core/src/datasources/repology.rs:563` |
| 443 | returns correct package versions for multi-package project | ported | `crates/renovate-core/src/datasources/repology.rs:596` |

