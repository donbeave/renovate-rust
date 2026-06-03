# `lib/modules/manager/gitlabci-include/extract.spec.ts`

[← `manager/gitlabci-include`](../../../../_by-module/manager/gitlabci-include.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | returns null for empty | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:291`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L291) |
| 17 | returns null for include block without any actual includes | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:355`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L355) |
| 22 | extracts single include block | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:240`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L240) |
| 28 | extracts multiple include blocks | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:255`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L255) |
| 34 | extracts multiple embedded include blocks | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:297`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L297) |
| 51 | ignores includes without project and file keys | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:370`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L370) |
| 60 | normalizes configured endpoints | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:386`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L386) |
| 73 | supports multi-document files | ported | [`crates/renovate-core/src/extractors/gitlabci_include.rs:326`](../../../../../../../crates/renovate-core/src/extractors/gitlabci_include.rs#L326) |

