# `lib/modules/manager/helmv3/utils.spec.ts`

[← `manager/helmv3`](../../../../_by-module/manager/helmv3.md) · [all modules](../../../../README.md)

**9/11 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | return alias with "alias:" | ported | `crates/renovate-core/src/extractors/helm.rs:871` |
| 14 | return alias with "@" | ported | `crates/renovate-core/src/extractors/helm.rs:878` |
| 22 | return null if alias repo is not defined | ported | `crates/renovate-core/src/extractors/helm.rs:885` |
| 29 | return resolved repository on oci registries | ported | `crates/renovate-core/src/extractors/helm.rs:899` |
| 36 | return repository parameter if it is not an alias | ported | `crates/renovate-core/src/extractors/helm.rs:906` |
| 44 | return repository parameter if repository is null | ported | `crates/renovate-core/src/extractors/helm.rs:914` |
| 52 | return repository parameter if repository is undefined | ported | `crates/renovate-core/src/extractors/helm.rs:921` |
| 62 | return false if repository is null | ported | `crates/renovate-core/src/extractors/helm.rs:927` |
| 68 | return false if repository is undefined | ported | `crates/renovate-core/src/extractors/helm.rs:933` |
| 76 | return false if repository is null | ported | `crates/renovate-core/src/extractors/helm.rs:927` |
| 81 | return false if repository is undefined | ported | `crates/renovate-core/src/extractors/helm.rs:933` |

