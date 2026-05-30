# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/utils.spec.ts
**Total tests:** 21 | **Ported:** 4 | **Actionable:** 21 | **Status:** ported

### `util/exec/utils › isCommandWithOptions › when command is an array of 1 command`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 7 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is an array of many command`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 17 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is an empty array`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 27 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is a string`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 37 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is a mixed array of strings booleans`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 47 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is an array of booleans`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 57 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and no ignoreFailure is present`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 67 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and ignoreFailure is not a boolean`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 77 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and ignoreFailure=false`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 88 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and ignoreFailure=true`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 99 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and no shell is present`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 110 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and shell is not a string or a boolean`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 120 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and shell is an empty string`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 131 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and shell is a string with only whitespace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 142 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and shell is a non-empty string`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is not a CommandWithOptions | 153 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and shell=false`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 165 | pending | — | — | — |

### `util/exec/utils › isCommandWithOptions › when command is valid, and shell=true`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is a CommandWithOptions | 176 | pending | — | — | — |

### `util/exec/utils › asRawCommands › with a string`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns array of strings | 189 | ported | `util.rs` | `test_as_raw_commands_single_string` | — |

### `util/exec/utils › asRawCommands › with an array of strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns array of strings | 198 | ported | `util.rs` | `test_as_raw_commands_array_of_strings` | — |

### `util/exec/utils › asRawCommands › with many commands`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an array of many strings | 207 | ported | `util.rs` | `test_as_raw_commands_many_strings` | — |

### `util/exec/utils › asRawCommands › with `CommandWithOptions``

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns commands from the `CommandWithOptions` | 220 | ported | `util.rs` | `test_as_raw_commands_with_opts` | — |

---
