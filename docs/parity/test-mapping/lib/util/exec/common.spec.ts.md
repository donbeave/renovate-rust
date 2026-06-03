# `lib/util/exec/common.spec.ts`

[← `util/exec`](../../../_by-module/util/exec.md) · [all modules](../../../README.md)

**13/30 ported** (17 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 175 | command exits with code 0 | ported | `crates/renovate-core/src/exec/raw.rs:145` |
| 194 | never extends the process environment | ported | `crates/renovate-core/src/exec/raw.rs:166` |
| 214 | throws if an error occurs, when using commandwithoptions | ported | `crates/renovate-core/src/exec/error.rs:123` |
| 241 | throws if an error occurs | ported | `crates/renovate-core/src/exec/error.rs:115` |
| 265 | throws if an error occurs, and we specify ignorefailure=false | pending | — |
| 292 | does not throw if an error occurs, but we specify ignorefailure=true | pending | — |
| 320 | can specify a shell | ported | `crates/renovate-core/src/exec/raw.rs:182` |
| 343 | can specify a specific shell with commandwithoptions | pending | — |
| 366 | can specify shell=true with commandwithoptions | pending | — |
| 389 | can specify a command with spaces, with a shell | ported | `crates/renovate-core/src/exec/raw.rs:253` |
| 412 | can specify a command with spaces, with no shell | pending | — |
| 435 | defaults to shell=false | pending | — |
| 455 | the command is provided as a string with no arguments when shell is a string | ported | `crates/renovate-core/src/exec/raw.rs:265` |
| 475 | the command is provided as a string with no arguments when shell=true | ported | `crates/renovate-core/src/exec/raw.rs:277` |
| 495 | the command is split into the command and arguments when shell=false | pending | — |
| 515 | can specify shell=true | ported | `crates/renovate-core/src/exec/raw.rs:207` |
| 538 | can specify shell=false | pending | — |
| 561 | should invoke the output listeners | pending | — |
| 602 | command exits with code 1 | ported | `crates/renovate-core/src/exec/raw.rs:155` |
| 618 | process terminated with sigterm | ported | `crates/renovate-core/src/exec/error.rs:136` |
| 632 | process does nothing when signaled with sigstop and eventually times out | ported | `crates/renovate-core/src/exec/raw.rs:194` |
| 644 | process exits due to error | ported | `crates/renovate-core/src/exec/raw.rs:219` |
| 659 | process exits with error due to exceeded stdout maxbuffer | pending | — |
| 683 | process exits with error due to exceeded stderr maxbuffer | pending | — |
| 708 | command exits with code 0 | ported | `crates/renovate-core/src/exec/raw.rs:145` |
| 727 | never extends the process environment | ported | `crates/renovate-core/src/exec/raw.rs:166` |
| 753 | calls instrument function | pending | — |
| 773 | command name and arguments are sanitized | pending | — |
| 806 | calls process.kill on the gpid | pending | — |
| 826 | handles process.kill call on non existent gpid | pending | — |

