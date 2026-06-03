# `lib/modules/manager/ansible-galaxy/extract.spec.ts`

[← `manager/ansible-galaxy`](../../../../_by-module/manager/ansible-galaxy.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 15 | returns null for empty | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:394` |
| 19 | extracts multiple dependencies from requirements.yml | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:330` |
| 25 | extracts dependencies from a not beautified requirements file | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:462` |
| 31 | extracts dependencies from requirements.yml with a space at the end of line | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:400` |
| 41 | extracts git@ dependencies | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:409` |
| 56 | check if an empty file returns null | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:431` |
| 61 | check if a requirements file of other systems returns null | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:423` |
| 66 | check collection style requirements file | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:381` |
| 73 | check collection style requirements file in reverse order and missing empty line | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:451` |
| 79 | check galaxy definition file | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:644` |
| 87 | negative start number returns -1 | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:658` |
| 92 | a start number bigger then number of lines return -1 | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:664` |
| 97 | choose first block | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:670` |
| 102 | choose second block | ported | `crates/renovate-core/src/extractors/ansible_galaxy.rs:676` |

