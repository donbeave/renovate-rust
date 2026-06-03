# `lib/modules/datasource/github-releases/index.spec.ts`

[← `datasource/github-releases`](../../../../_by-module/datasource/github-releases.md) · [all modules](../../../../README.md)

**5/5 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 20 | returns releases | ported | `crates/renovate-core/src/datasources/github_releases.rs:372` |
| 116 | should be independent of the current digest | ported | `crates/renovate-core/src/datasources/github_releases.rs:414` |
| 128 | should be independent of the current value | ported | `crates/renovate-core/src/datasources/github_releases.rs:436` |
| 136 | returns updated digest in new release | ported | `crates/renovate-core/src/datasources/github_releases.rs:458` |
| 149 | returns null if the new value/tag does not exist | ported | `crates/renovate-core/src/datasources/github_releases.rs:480` |

