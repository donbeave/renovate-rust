# `lib/util/exec/docker/index.spec.ts`

[← `util/exec`](../../../../_by-module/util/exec.md) · [all modules](../../../../README.md)

**11/19 ported** (8 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 25 | runs prefetch command | pending | — |
| 31 | performs prefetch once for each image | pending | — |
| 47 | gracefully handles container list error | ported | [`crates/renovate-core/src/exec/docker.rs:307`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L307) |
| 52 | gracefully handles container removal error | ported | [`crates/renovate-core/src/exec/docker.rs:315`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L315) |
| 57 | gracefully handles empty container list | ported | [`crates/renovate-core/src/exec/docker.rs:322`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L322) |
| 62 | runs docker commands for container removal | ported | [`crates/renovate-core/src/exec/docker.rs:329`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L329) |
| 80 | short-circuits in non-docker environment | ported | [`crates/renovate-core/src/exec/docker.rs:389`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L389) |
| 87 | handles insufficient memory error | pending | — |
| 96 | handles missing docker daemon | pending | — |
| 108 | handles unknown error | ported | [`crates/renovate-core/src/exec/docker.rs:398`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L398) |
| 118 | handles empty container list | pending | — |
| 129 | removes containers | pending | — |
| 160 | returns executable command | ported | [`crates/renovate-core/src/exec/docker.rs:118`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L118) |
| 171 | adds `\|\| true` if ignorefailure is set on a pre-command | pending | — |
| 201 | adds `\|\| true` if ignorefailure is set on a command | pending | — |
| 231 | handles volumes | ported | [`crates/renovate-core/src/exec/docker.rs:171`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L171) |
| 255 | adds custom containerbasedir to volumes | ported | [`crates/renovate-core/src/exec/docker.rs:227`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L227) |
| 281 | adds dedupes default containerbasedir in volumes | ported | [`crates/renovate-core/src/exec/docker.rs:254`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L254) |
| 307 | add multiple docker cli option | ported | [`crates/renovate-core/src/exec/docker.rs:282`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L282) |

