# `lib/modules/manager/docker-compose/extract.spec.ts`

[← `manager/docker-compose`](../../../../_by-module/manager/docker-compose.md) · [all modules](../../../../README.md)

**13/13 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | returns null for empty | ported | `crates/renovate-core/src/extractors/docker_compose.rs:441` |
| 16 | returns null for non-object yaml | ported | `crates/renovate-core/src/extractors/docker_compose.rs:448` |
| 20 | returns null for malformed yaml | ported | `crates/renovate-core/src/extractors/docker_compose.rs:455` |
| 24 | extracts multiple image lines for version 1 | ported | `crates/renovate-core/src/extractors/docker_compose.rs:414` |
| 30 | extracts multiple image lines for version 3 | ported | `crates/renovate-core/src/extractors/docker_compose.rs:307` |
| 36 | extracts multiple image lines for version 3 without set version key | ported | `crates/renovate-core/src/extractors/docker_compose.rs:573` |
| 42 | extracts default variable values for version 3 | ported | `crates/renovate-core/src/extractors/docker_compose.rs:375` |
| 59 | extracts can parse yaml tags for version 3 | ported | `crates/renovate-core/src/extractors/docker_compose.rs:462` |
| 87 | extracts image and replaces registry | ported | `crates/renovate-core/src/extractors/docker_compose.rs:497` |
| 115 | extracts image but no replacement | ported | `crates/renovate-core/src/extractors/docker_compose.rs:521` |
| 143 | extracts image and no double replacement | ported | `crates/renovate-core/src/extractors/docker_compose.rs:546` |
| 172 | extracts image of templated compose file | ported | `crates/renovate-core/src/extractors/docker_compose.rs:480` |
| 198 | extract images from fragments | ported | `crates/renovate-core/src/extractors/docker_compose.rs:610` |

