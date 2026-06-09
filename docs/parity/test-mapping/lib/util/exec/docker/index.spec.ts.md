# `lib/util/exec/docker/index.spec.ts`

[← `util/exec`](../../../../_by-module/util/exec.md) · [all modules](../../../../README.md)

**13/19 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 25 | runs prefetch command | pending | — |
| 31 | performs prefetch once for each image | pending | — |
| 47 | gracefully handles container list error | ported | [`crates/renovate-core/src/exec/docker.rs:374`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L374) |
| 52 | gracefully handles container removal error | ported | [`crates/renovate-core/src/exec/docker.rs:382`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L382) |
| 57 | gracefully handles empty container list | ported | [`crates/renovate-core/src/exec/docker.rs:389`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L389) |
| 62 | runs docker commands for container removal | ported | [`crates/renovate-core/src/exec/docker.rs:396`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L396) |
| 80 | short-circuits in non-docker environment | ported | [`crates/renovate-core/src/exec/docker.rs:456`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L456) |
| 87 | handles insufficient memory error | pending | — |
| 96 | handles missing docker daemon | pending | — |
| 108 | handles unknown error | ported | [`crates/renovate-core/src/exec/docker.rs:465`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L465) |
| 118 | handles empty container list | pending | — |
| 129 | removes containers | pending | — |
| 160 | returns executable command | ported | [`crates/renovate-core/src/exec/docker.rs:118`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L118) |
| 171 | adds `\|\| true` if ignorefailure is set on a pre-command | ported | [`crates/renovate-core/src/exec/docker.rs:171`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L171) |
| 201 | adds `\|\| true` if ignorefailure is set on a command | ported | [`crates/renovate-core/src/exec/docker.rs:206`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L206) |
| 231 | handles volumes | ported | [`crates/renovate-core/src/exec/docker.rs:238`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L238) |
| 255 | adds custom containerbasedir to volumes | ported | [`crates/renovate-core/src/exec/docker.rs:294`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L294) |
| 281 | adds dedupes default containerbasedir in volumes | ported | [`crates/renovate-core/src/exec/docker.rs:321`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L321) |
| 307 | add multiple docker cli option | ported | [`crates/renovate-core/src/exec/docker.rs:349`](../../../../../../../crates/renovate-core/src/exec/docker.rs#L349) |

