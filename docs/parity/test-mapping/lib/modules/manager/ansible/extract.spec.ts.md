# `lib/modules/manager/ansible/extract.spec.ts`

[← `manager/ansible`](../../../../_by-module/manager/ansible.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns null for empty | ported | [`crates/renovate-core/src/extractors/ansible.rs:173`](../../../../../../../crates/renovate-core/src/extractors/ansible.rs#L173) |
| 10 | extracts multiple image lines from docker_container | ported | [`crates/renovate-core/src/extractors/ansible.rs:147`](../../../../../../../crates/renovate-core/src/extractors/ansible.rs#L147) |
| 16 | extracts multiple image lines from docker_service | ported | [`crates/renovate-core/src/extractors/ansible.rs:186`](../../../../../../../crates/renovate-core/src/extractors/ansible.rs#L186) |
| 22 | extracts image and replaces registry | ported | [`crates/renovate-core/src/extractors/ansible.rs:214`](../../../../../../../crates/renovate-core/src/extractors/ansible.rs#L214) |
| 52 | extracts image but no replacement | ported | [`crates/renovate-core/src/extractors/ansible.rs:237`](../../../../../../../crates/renovate-core/src/extractors/ansible.rs#L237) |
| 82 | extracts image and no double replacement | ported | [`crates/renovate-core/src/extractors/ansible.rs:261`](../../../../../../../crates/renovate-core/src/extractors/ansible.rs#L261) |

