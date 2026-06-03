# `lib/modules/datasource/docker/common.spec.ts`

[← `datasource/docker`](../../../../_by-module/datasource/docker.md) · [all modules](../../../../README.md)

**1/15 ported** (14 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | handles local registries | pending | — |
| 35 | supports registryurls | pending | — |
| 46 | supports http registryurls | pending | — |
| 57 | supports schemeless registryurls | pending | — |
| 68 | supports insecure registryurls | pending | — |
| 80 | _(it.each / template — verify manually)_ | ? | — |
| 117 | returns raw registryhost and dockerrepository when fullurl is invalid | pending | — |
| 136 | throw page not found exception | pending | — |
| 152 | returns "authtype token" if both provided | pending | — |
| 177 | returns "bearer token" if only token provided | pending | — |
| 201 | fails | pending | — |
| 223 | use resources url and resolve scope in www-authenticate header | pending | — |
| 251 | supports multiple challenges in www-authenticate header | pending | — |
| 279 | findlateststable works | ported | `crates/renovate-core/src/datasources/docker_hub.rs:532` |
| 283 | findhelmsourceurl works | pending | — |

