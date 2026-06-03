# `lib/modules/manager/composer/extract.spec.ts`

[← `manager/composer`](../../../../_by-module/manager/composer.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | returns null for invalid json | ported | `crates/renovate-core/src/extractors/composer.rs:1099` |
| 28 | returns null for empty deps | ported | `crates/renovate-core/src/extractors/composer.rs:1092` |
| 32 | extracts dependencies with no lock file | ported | `crates/renovate-core/src/extractors/composer.rs:698` |
| 38 | extracts registryurls | ported | `crates/renovate-core/src/extractors/composer.rs:876` |
| 81 | extracts object registryurls | ported | `crates/renovate-core/src/extractors/composer.rs:907` |
| 186 | extracts repositories and registryurls | ported | `crates/renovate-core/src/extractors/composer.rs:949` |
| 219 | extracts bitbucket repositories and registryurls | ported | `crates/renovate-core/src/extractors/composer.rs:1001` |
| 248 | extracts object repositories and registryurls with lock file | ported | `crates/renovate-core/src/extractors/composer.rs:1026` |
| 284 | skips path dependencies | ported | `crates/renovate-core/src/extractors/composer.rs:852` |
| 313 | extracts dependencies with lock file | ported | `crates/renovate-core/src/extractors/composer.rs:1083` |

