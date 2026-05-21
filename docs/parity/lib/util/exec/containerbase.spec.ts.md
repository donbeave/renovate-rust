# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/containerbase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/containerbase.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/exec/containerbase › isDynamicInstall()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if binarySource is not install | 22 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns false if not containerbase | 26 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns false if any unsupported tools | 31 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns true if supported tools | 42 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/containerbase › getToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config for a known tool | 51 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns undefined for an unknown tool | 60 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/containerbase › resolveConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from config | 67 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns highest stable | 73 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns highest unstable | 87 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| respects latest | 96 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| supports rust docker tags | 113 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| throws for unknown tools | 127 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| throws no releases | 133 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| falls back to latest version if no compatible release | 142 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| falls back to latest version if invalid constraint | 151 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| supports python ranges "$version" => "$expected" | 160 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| removes pep440 == | 184 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| supports flutter ranges "$version" => "$expected" | 193 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| supports dart ranges "$version" => "$expected" | 223 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/containerbase › generateInstallCommands()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns install commands | 269 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

---

