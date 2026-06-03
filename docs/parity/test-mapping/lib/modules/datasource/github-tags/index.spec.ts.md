# `lib/modules/datasource/github-tags/index.spec.ts`

[← `datasource/github-tags`](../../../../_by-module/datasource/github-tags.md) · [all modules](../../../../README.md)

**11/12 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 25 | returns commit digest | ported | `crates/renovate-core/src/datasources/github_tags.rs:469` |
| 36 | returns null for missing commit | ported | `crates/renovate-core/src/datasources/github_tags.rs:486` |
| 45 | returns untagged commit digest | ported | `crates/renovate-core/src/datasources/github_tags.rs:501` |
| 54 | returns tagged commit digest | ported | `crates/renovate-core/src/datasources/github_tags.rs:518` |
| 73 | returns null for missing hash | pending | — |
| 91 | returns null for missing tagged commit digest | ported | `crates/renovate-core/src/datasources/github_tags.rs:536` |
| 110 | returns null for error | ported | `crates/renovate-core/src/datasources/github_tags.rs:551` |
| 120 | returns tags | ported | `crates/renovate-core/src/datasources/github_tags.rs:566` |
| 184 | if it is newer than tag timestamp | ported | `crates/renovate-core/src/datasources/github_tags.rs:631` |
| 213 | keeps tag timestamp when release timestamp is older | ported | `crates/renovate-core/src/datasources/github_tags.rs:668` |
| 242 | keeps tag timestamp when release timestamp is equal | ported | `crates/renovate-core/src/datasources/github_tags.rs:705` |
| 271 | keeps tag timestamp when no corresponding release exists | ported | `crates/renovate-core/src/datasources/github_tags.rs:742` |

