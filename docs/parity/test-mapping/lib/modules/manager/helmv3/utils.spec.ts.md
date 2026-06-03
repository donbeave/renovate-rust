# `lib/modules/manager/helmv3/utils.spec.ts`

[← `manager/helmv3`](../../../../_by-module/manager/helmv3.md) · [all modules](../../../../README.md)

**9/11 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | return alias with "alias:" | ported | [`crates/renovate-core/src/extractors/helm.rs:871`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L871) |
| 14 | return alias with "@" | ported | [`crates/renovate-core/src/extractors/helm.rs:878`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L878) |
| 22 | return null if alias repo is not defined | ported | [`crates/renovate-core/src/extractors/helm.rs:885`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L885) |
| 29 | return resolved repository on oci registries | ported | [`crates/renovate-core/src/extractors/helm.rs:899`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L899) |
| 36 | return repository parameter if it is not an alias | ported | [`crates/renovate-core/src/extractors/helm.rs:906`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L906) |
| 44 | return repository parameter if repository is null | ported | [`crates/renovate-core/src/extractors/helm.rs:914`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L914) |
| 52 | return repository parameter if repository is undefined | ported | [`crates/renovate-core/src/extractors/helm.rs:921`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L921) |
| 62 | return false if repository is null | ported | [`crates/renovate-core/src/extractors/helm.rs:927`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L927) |
| 68 | return false if repository is undefined | ported | [`crates/renovate-core/src/extractors/helm.rs:933`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L933) |
| 76 | return false if repository is null | ported | [`crates/renovate-core/src/extractors/helm.rs:927`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L927) |
| 81 | return false if repository is undefined | ported | [`crates/renovate-core/src/extractors/helm.rs:933`](../../../../../../../crates/renovate-core/src/extractors/helm.rs#L933) |

