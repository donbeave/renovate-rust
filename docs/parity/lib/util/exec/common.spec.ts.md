# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/common.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** pending

### `util/exec/common › exec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| command exits with code 0 | 175 | pending | — | — | —|
| never extends the process environment | 194 | pending | — | — | —|
| throws if an error occurs, when using CommandWithOptions | 214 | pending | — | — | —|
| throws if an error occurs | 241 | pending | — | — | —|
| throws if an error occurs, and we specify ignoreFailure=false | 265 | pending | — | — | —|
| does not throw if an error occurs, but we specify ignoreFailure=true | 292 | pending | — | — | —|
| can specify a shell | 320 | pending | — | — | —|
| can specify a specific shell with CommandWithOptions | 343 | pending | — | — | —|
| can specify shell=true with CommandWithOptions | 366 | pending | — | — | —|
| can specify a command with spaces, with a shell | 389 | pending | — | — | —|
| can specify a command with spaces, with no shell | 412 | pending | — | — | —|
| defaults to shell=false | 435 | pending | — | — | —|
| the command is provided as a string with no arguments when shell is a string | 455 | pending | — | — | —|
| the command is provided as a string with no arguments when shell=true | 475 | pending | — | — | —|
| the command is split into the command and arguments when shell=false | 495 | pending | — | — | —|
| can specify shell=true | 515 | pending | — | — | —|
| can specify shell=false | 538 | pending | — | — | —|
| should invoke the output listeners | 561 | pending | — | — | —|
| command exits with code 1 | 602 | pending | — | — | —|
| process terminated with SIGTERM | 618 | pending | — | — | —|
| process does nothing when signaled with SIGSTOP and eventually times out | 632 | pending | — | — | —|
| process exits due to error | 644 | pending | — | — | —|
| process exits with error due to exceeded stdout maxBuffer | 659 | pending | — | — | —|
| process exits with error due to exceeded stderr maxBuffer | 683 | pending | — | — | —|

### `util/exec/common › rawExec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| command exits with code 0 | 708 | pending | — | — | —|
| never extends the process environment | 727 | pending | — | — | —|

### `util/exec/common › rawExec › is instrumented`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls instrument function | 753 | pending | — | — | —|
| command name and arguments are sanitized | 773 | pending | — | — | —|

### `util/exec/common › handle gpid`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls process.kill on the gpid | 806 | pending | — | — | —|
| handles process.kill call on non existent gpid | 826 | pending | — | — | —|

---

