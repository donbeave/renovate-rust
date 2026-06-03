# `lib/modules/manager/pip_requirements/extract.spec.ts`

[← `manager/pip_requirements`](../../../../_by-module/manager/pip_requirements.md) · [all modules](../../../../README.md)

**22/22 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 39 | returns null for empty | ported | `crates/renovate-core/src/extractors/pip.rs:572` |
| 43 | extracts dependencies | ported | `crates/renovate-core/src/extractors/pip.rs:416` |
| 50 | extracts dependencies with --index-url short code | ported | `crates/renovate-core/src/extractors/pip.rs:579` |
| 68 | extracts --requirement short code option | ported | `crates/renovate-core/src/extractors/pip.rs:497` |
| 79 | extracts --constraints short code option | ported | `crates/renovate-core/src/extractors/pip.rs:505` |
| 90 | extracts multiple dependencies | ported | `crates/renovate-core/src/extractors/pip.rs:473` |
| 96 | handles comments and commands | ported | `crates/renovate-core/src/extractors/pip.rs:443` |
| 102 | handles extras and complex index url | ported | `crates/renovate-core/src/extractors/pip.rs:466` |
| 111 | handles extra index url | ported | `crates/renovate-core/src/extractors/pip.rs:604` |
| 123 | handles extra index url and defaults without index to config | ported | `crates/renovate-core/src/extractors/pip.rs:627` |
| 132 | handles extra index url and defaults without index to pypi | ported | `crates/renovate-core/src/extractors/pip.rs:646` |
| 141 | handles extra spaces around pinned dependency equal signs | ported | `crates/renovate-core/src/extractors/pip.rs:705` |
| 155 | should not replace env vars in low trust mode | ported | `crates/renovate-core/src/extractors/pip.rs:665` |
| 166 | should replace env vars in high trust mode | ported | `crates/renovate-core/src/extractors/pip.rs:685` |
| 178 | should handle hashes | ported | `crates/renovate-core/src/extractors/pip.rs:725` |
| 184 | should handle package with extras and no version specifiers | ported | `crates/renovate-core/src/extractors/pip.rs:458` |
| 198 | should handle dependency and ignore env markers | ported | `crates/renovate-core/src/extractors/pip.rs:450` |
| 213 | should handle git packages | ported | `crates/renovate-core/src/extractors/pip.rs:483` |
| 258 | extracts a file with only --index-url flags | ported | `crates/renovate-core/src/extractors/npm.rs:4055` |
| 266 | extracts a file with only --extra-index-url flags | ported | `crates/renovate-core/src/extractors/pip.rs:536` |
| 276 | extracts a file with only -r flags | ported | `crates/renovate-core/src/extractors/pip.rs:550` |
| 286 | extracts a file with only -c flags | ported | `crates/renovate-core/src/extractors/pip.rs:561` |

