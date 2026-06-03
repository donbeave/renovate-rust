# `lib/modules/manager/ansible/extract.spec.ts`

[← `manager/ansible`](../../../../_by-module/manager/ansible.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns null for empty | ported | `crates/renovate-core/src/extractors/ansible.rs:173` |
| 10 | extracts multiple image lines from docker_container | ported | `crates/renovate-core/src/extractors/ansible.rs:147` |
| 16 | extracts multiple image lines from docker_service | ported | `crates/renovate-core/src/extractors/ansible.rs:186` |
| 22 | extracts image and replaces registry | ported | `crates/renovate-core/src/extractors/ansible.rs:214` |
| 52 | extracts image but no replacement | ported | `crates/renovate-core/src/extractors/ansible.rs:237` |
| 82 | extracts image and no double replacement | ported | `crates/renovate-core/src/extractors/ansible.rs:261` |

