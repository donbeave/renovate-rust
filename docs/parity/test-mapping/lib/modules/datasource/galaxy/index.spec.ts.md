# `lib/modules/datasource/galaxy/index.spec.ts`

[← `datasource/galaxy`](../../../../_by-module/datasource/galaxy.md) · [all modules](../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns null for empty result | ported | `crates/renovate-core/src/datasources/galaxy.rs:134` |
| 24 | returns null for missing fields | ported | `crates/renovate-core/src/datasources/galaxy.rs:153` |
| 37 | returns null for empty list | ported | `crates/renovate-core/src/datasources/galaxy.rs:172` |
| 50 | returns null for 404 | ported | `crates/renovate-core/src/datasources/galaxy.rs:191` |
| 63 | returns null for unknown error | ported | `crates/renovate-core/src/datasources/galaxy.rs:207` |
| 76 | processes real data | ported | `crates/renovate-core/src/datasources/galaxy.rs:224` |
| 90 | handles multiple results when one user matches exactly | ported | `crates/renovate-core/src/datasources/galaxy.rs:246` |
| 103 | rejects multiple results when no user matches exactly | ported | `crates/renovate-core/src/datasources/galaxy.rs:269` |
| 115 | return null if searching random username and project name | ported | `crates/renovate-core/src/datasources/galaxy.rs:291` |
| 127 | throws for 5xx | ported | `crates/renovate-core/src/datasources/galaxy.rs:313` |
| 140 | throws for 404 | ported | `crates/renovate-core/src/datasources/galaxy.rs:327` |

