# `lib/modules/manager/jsonnet-bundler/extract.spec.ts`

[← `manager/jsonnet-bundler`](../../../../_by-module/manager/jsonnet-bundler.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | returns null for invalid jsonnetfile | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:236`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L236) |
| 30 | returns null for jsonnetfile with no dependencies | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:242`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L242) |
| 36 | returns null for local dependencies | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:248`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L248) |
| 42 | returns null for vendored dependencies | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:255`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L255) |
| 48 | returns null for dependencies with empty git source | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:262`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L262) |
| 57 | extracts dependency | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:203`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L203) |
| 79 | extracts dependency with custom name | ported | [`crates/renovate-core/src/extractors/jsonnet_bundler.rs:269`](../../../../../../../crates/renovate-core/src/extractors/jsonnet_bundler.rs#L269) |

