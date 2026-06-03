# `lib/modules/manager/buildkite/extract.spec.ts`

[← `manager/buildkite`](../../../../_by-module/manager/buildkite.md) · [all modules](../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns null for empty | ported | `crates/renovate-core/src/extractors/buildkite.rs:273` |
| 11 | extracts simple single plugin | ported | `crates/renovate-core/src/extractors/buildkite.rs:229` |
| 22 | extracts multiple plugins in same file | ported | `crates/renovate-core/src/extractors/buildkite.rs:210` |
| 47 | adds skipreason | ported | `crates/renovate-core/src/extractors/buildkite.rs:265` |
| 70 | extracts arrays of plugins | ported | `crates/renovate-core/src/extractors/buildkite.rs:288` |
| 92 | extracts git-based plugins | ported | `crates/renovate-core/src/extractors/buildkite.rs:247` |
| 105 | extracts git-based plugin with .git at the end of its name | ported | `crates/renovate-core/src/extractors/buildkite.rs:296` |
| 121 | extracts plugins outside plugins sections | ported | `crates/renovate-core/src/extractors/buildkite.rs:313` |
| 140 | extracts plugin with preceding ? | ported | `crates/renovate-core/src/extractors/buildkite.rs:331` |
| 155 | extracts plugin tags from bitbucket | ported | `crates/renovate-core/src/extractors/buildkite.rs:342` |
| 178 | extracts plugin tags with quotes | ported | `crates/renovate-core/src/extractors/buildkite.rs:365` |

