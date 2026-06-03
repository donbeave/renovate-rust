# `lib/modules/manager/quadlet/extract.spec.ts`

[← `manager/quadlet`](../../../../_by-module/manager/quadlet.md) · [all modules](../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 19 | returns null for invalid quadlet file content | ported | [`crates/renovate-core/src/extractors/quadlet.rs:208`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L208) |
| 24 | returns null for empty yaml file content | ported | [`crates/renovate-core/src/extractors/quadlet.rs:150`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L150) |
| 29 | extracts from quadlet container unit | ported | [`crates/renovate-core/src/extractors/quadlet.rs:106`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L106) |
| 47 | extracts from quadlet image unit | ported | [`crates/renovate-core/src/extractors/quadlet.rs:165`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L165) |
| 65 | extracts from quadlet volume unit | ported | [`crates/renovate-core/src/extractors/quadlet.rs:175`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L175) |
| 83 | handles docker prefix | ported | [`crates/renovate-core/src/extractors/quadlet.rs:116`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L116) |
| 101 | handles docker-daemon prefix | ported | [`crates/renovate-core/src/extractors/quadlet.rs:184`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L184) |
| 119 | does not extract an image file reference | ported | [`crates/renovate-core/src/extractors/quadlet.rs:194`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L194) |
| 129 | does not extract an build file reference | ported | [`crates/renovate-core/src/extractors/quadlet.rs:201`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L201) |
| 139 | extract data from file with registry aliases | ported | [`crates/renovate-core/src/extractors/quadlet.rs:221`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L221) |
| 158 | handles an unsuccessful parse | ported | [`crates/renovate-core/src/extractors/quadlet.rs:214`](../../../../../../../crates/renovate-core/src/extractors/quadlet.rs#L214) |

