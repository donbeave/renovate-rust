# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/index.spec.ts
**Total tests:** 39 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/exec/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| echo hello | 910 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| Supports image prefetch | 943 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| throws when an error is thrown | 985 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| rejects and throws if an error is thrown, even if we specify ignoreFailure=true | 995 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| does not reject and throw if rawExec returns an exit code, and we specify ignoreFailure=true | 1010 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| exec takes an array with both `string`s and `CommandWithOptions` as an argument | 1038 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| exec takes CommandWithOptions as an argument | 1059 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| Supports binarySource=install | 1076 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| Supports binarySource=install preCommands | 1087 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| only calls removeDockerContainer in catch block is useDocker is set | 1101 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| wraps error if removeDockerContainer throws an error | 1116 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| converts to TEMPORARY_ERROR | 1150 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for JVM settings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default values if no global or repo config | 1183 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns default values if empty repo config | 1194 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| returns default values if empty global config | 1205 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for JVM settings › does not allow floating point numbers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| in global config | 1219 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| in repo config | 1232 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for JVM settings › when using repo config to override memory limits`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when below global settings, repo settings are used | 1252 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo settings are the same as global settings, they are used | 1266 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo jvmMemory setting is higher than global setting, but lower than global jvmMaxMemory, the repo config is used | 1280 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo jvmMaxMemory setting is lower than global settings, it is applied | 1292 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo jvmMaxMemory setting is lower than global jvmMemory, jvmMemory is set to the same value | 1304 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo jvmMaxMemory setting is lower than repo jvmMemory, jvmMemory is set to the same value | 1317 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo jvmMaxMemory setting is higher than global settings, they are ignored | 1331 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo jvmMaxMemory setting is higher than global settings, a debug log is logged | 1344 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for JVM settings › a minimum of 512M is enforced`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when global settings are lower than 512M, they are overridden to 512M | 1364 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when global settings are lower than 512M, a debug log is logged | 1377 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo settings are lower than 512M, they are overridden to 512M | 1389 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo settings are lower than 512M, a debug log is logged | 1403 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for Node settings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not return a default value if no global or repo config | 1428 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| does not return default values if empty global config | 1438 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for Node settings › does not allow floating point numbers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| in global config | 1451 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| in repo config | 1463 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › getToolSettingsOptions() › for Node settings › when using repo config to override memory limits`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when below global settings, repo settings are used | 1479 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo settings are the same as global settings, they are used | 1491 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo nodeMaxMemory setting is lower than global settings, it is applied | 1503 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo nodeMaxMemory setting is higher than global settings, they are ignored | 1515 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |
| when repo nodeMaxMemory setting is higher than global settings, a debug log is logged | 1527 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

### `util/exec/index › gradleJvmArg()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| takes the values given to it, and returns the JVM arguments | 1547 | not-applicable | — | — | out of scope: tests Node.js child-process/Docker exec infrastructure not used by Rust CLI |

---

