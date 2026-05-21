# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/common.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/exec/common › exec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| command exits with code 0 | 175 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| never extends the process environment | 194 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| throws if an error occurs, when using CommandWithOptions | 214 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| throws if an error occurs | 241 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| throws if an error occurs, and we specify ignoreFailure=false | 265 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| does not throw if an error occurs, but we specify ignoreFailure=true | 292 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify a shell | 320 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify a specific shell with CommandWithOptions | 343 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify shell=true with CommandWithOptions | 366 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify a command with spaces, with a shell | 389 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify a command with spaces, with no shell | 412 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| defaults to shell=false | 435 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| the command is provided as a string with no arguments when shell is a string | 455 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| the command is provided as a string with no arguments when shell=true | 475 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| the command is split into the command and arguments when shell=false | 495 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify shell=true | 515 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| can specify shell=false | 538 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| should invoke the output listeners | 561 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| command exits with code 1 | 602 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| process terminated with SIGTERM | 618 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| process does nothing when signaled with SIGSTOP and eventually times out | 632 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| process exits due to error | 644 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| process exits with error due to exceeded stdout maxBuffer | 659 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| process exits with error due to exceeded stderr maxBuffer | 683 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/common › rawExec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| command exits with code 0 | 708 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| never extends the process environment | 727 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/common › rawExec › is instrumented`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls instrument function | 753 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| command name and arguments are sanitized | 773 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/common › handle gpid`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls process.kill on the gpid | 806 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| handles process.kill call on non existent gpid | 826 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

---

