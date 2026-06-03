# `lib/modules/manager/devcontainer/extract.spec.ts`

[← `manager/devcontainer`](../../../../_by-module/manager/devcontainer.md) · [all modules](../../../../README.md)

**15/15 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns null when the dev container json file is empty | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:265`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L265) |
| 22 | returns null when the dev container json file contents are malformed | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:162`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L162) |
| 34 | tests if jsonc can be parsed | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:282`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L282) |
| 72 | returns feature image deps when only the features property is defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:170`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L170) |
| 124 | returns image and feature image deps when both image and features properties are defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:225`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L225) |
| 174 | returns image dep when only the image property is defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:141`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L141) |
| 207 | returns null when the only feature property is malformed and no image property is defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:300`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L300) |
| 227 | returns null when the features property is malformed and no image property is defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:310`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L310) |
| 245 | returns null when the image property is malformed and no features are defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:319`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L319) |
| 263 | returns null when no image or features properties are defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:239`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L239) |
| 278 | returns null when the features property is null and no image property is defined in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:247`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L247) |
| 296 | returns null when the features property is not defined and the image property is null in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:154`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L154) |
| 314 | returns null when both the image and features properties are null | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:273`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L273) |
| 333 | returns only docker dependencies when non-docker feature types are defined beneath the features property in dev container json file | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:212`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L212) |
| 372 | parses known tool versions | ported | [`crates/renovate-core/src/extractors/devcontainer.rs:198`](../../../../../../../crates/renovate-core/src/extractors/devcontainer.rs#L198) |

