# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/common.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/exec/common › exec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| command exits with code 0 | 175 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| never extends the process environment | 194 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| throws if an error occurs, when using CommandWithOptions | 214 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| throws if an error occurs | 241 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| throws if an error occurs, and we specify ignoreFailure=false | 265 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| does not throw if an error occurs, but we specify ignoreFailure=true | 292 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify a shell | 320 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify a specific shell with CommandWithOptions | 343 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify shell=true with CommandWithOptions | 366 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify a command with spaces, with a shell | 389 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify a command with spaces, with no shell | 412 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| defaults to shell=false | 435 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| the command is provided as a string with no arguments when shell is a string | 455 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| the command is provided as a string with no arguments when shell=true | 475 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| the command is split into the command and arguments when shell=false | 495 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify shell=true | 515 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| can specify shell=false | 538 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| should invoke the output listeners | 561 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| command exits with code 1 | 602 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| process terminated with SIGTERM | 618 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| process does nothing when signaled with SIGSTOP and eventually times out | 632 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| process exits due to error | 644 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| process exits with error due to exceeded stdout maxBuffer | 659 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| process exits with error due to exceeded stderr maxBuffer | 683 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |

### `util/exec/common › rawExec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| command exits with code 0 | 708 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| never extends the process environment | 727 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |

### `util/exec/common › rawExec › is instrumented`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls instrument function | 753 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| command name and arguments are sanitized | 773 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |

### `util/exec/common › handle gpid`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| calls process.kill on the gpid | 806 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |
| handles process.kill call on non existent gpid | 826 | not-applicable | — | — | Tests TS-specific `rawExec` child-process wrapper (maxBuffer, shell splitting, instrumentation, gpid); Rust uses `std::process::Command` / `tokio::process::Command` directly |

---
