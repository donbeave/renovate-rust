# `lib/modules/manager/maven/index.spec.ts`

[← `manager/maven`](../../../../_by-module/manager/maven.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | should update an existing dependency | ported | `crates/renovate-core/src/extractors/maven.rs:3232` |
| 43 | should update existing dependency defined via properties | ported | `crates/renovate-core/src/extractors/maven.rs:3358` |
| 67 | should not touch content if new and old versions are equal | ported | `crates/renovate-core/src/extractors/maven.rs:3257` |
| 79 | should update to version of the latest dep in implicit group | ported | `crates/renovate-core/src/extractors/maven.rs:3375` |
| 135 | should return null for ungrouped deps if content was updated outside | ported | `crates/renovate-core/src/extractors/maven.rs:3420` |
| 150 | should return null if current versions in content and upgrade are not same | ported | `crates/renovate-core/src/extractors/maven.rs:3276` |
| 162 | should update ranges | ported | `crates/renovate-core/src/extractors/maven.rs:3295` |
| 181 | should preserve ranges | ported | `crates/renovate-core/src/extractors/maven.rs:3320` |

