# `lib/modules/manager/ansible-galaxy/extract.spec.ts`

[← `manager/ansible-galaxy`](../../../../_by-module/manager/ansible-galaxy.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 15 | returns null for empty | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:394`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L394) |
| 19 | extracts multiple dependencies from requirements.yml | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:330`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L330) |
| 25 | extracts dependencies from a not beautified requirements file | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:462`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L462) |
| 31 | extracts dependencies from requirements.yml with a space at the end of line | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:400`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L400) |
| 41 | extracts git@ dependencies | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:409`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L409) |
| 56 | check if an empty file returns null | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:431`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L431) |
| 61 | check if a requirements file of other systems returns null | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:423`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L423) |
| 66 | check collection style requirements file | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:381`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L381) |
| 73 | check collection style requirements file in reverse order and missing empty line | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:451`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L451) |
| 79 | check galaxy definition file | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:644`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L644) |
| 87 | negative start number returns -1 | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:658`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L658) |
| 92 | a start number bigger then number of lines return -1 | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:664`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L664) |
| 97 | choose first block | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:670`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L670) |
| 102 | choose second block | ported | [`crates/renovate-core/src/extractors/ansible_galaxy.rs:676`](../../../../../../../crates/renovate-core/src/extractors/ansible_galaxy.rs#L676) |

