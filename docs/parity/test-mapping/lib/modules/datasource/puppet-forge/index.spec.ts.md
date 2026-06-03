# `lib/modules/datasource/puppet-forge/index.spec.ts`

[← `datasource/puppet-forge`](../../../../_by-module/datasource/puppet-forge.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | should use default forge if no other provided | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:168`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L168) |
| 34 | parses real data | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:191`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L191) |
| 79 | has a deprecated for reason | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:235`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L235) |
| 107 | should return null if lookup fails 400 | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:260`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L260) |
| 123 | should return null if lookup fails | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:278`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L278) |
| 137 | should fetch package info from custom registry | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:296`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L296) |
| 182 | load all possible null values | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:326`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L326) |
| 208 | no releases available -> return null | ported | [`crates/renovate-core/src/datasources/puppet_forge.rs:352`](../../../../../../../crates/renovate-core/src/datasources/puppet_forge.rs#L352) |

