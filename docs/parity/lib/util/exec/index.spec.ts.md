# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/index.spec.ts
**Total tests:** 40 | **Ported:** 0 | **Actionable:** 40 | **Status:** pending

### `util/exec/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| echo hello | 910 | pending | — | — | — |
| Supports image prefetch | 943 | pending | — | — | — |
| throws when an error is thrown | 985 | pending | — | — | — |
| rejects and throws if an error is thrown, even if we specify ignoreFailure=true | 995 | pending | — | — | — |
| does not reject and throw if rawExec returns an exit code, and we specify ignoreFailure=true | 1010 | pending | — | — | — |
| exec takes an array with both `string`s and `CommandWithOptions` as an argument | 1038 | pending | — | — | — |
| exec takes CommandWithOptions as an argument | 1059 | pending | — | — | — |
| Supports binarySource=install | 1076 | pending | — | — | — |
| Supports binarySource=install preCommands | 1087 | pending | — | — | — |
| only calls removeDockerContainer in catch block is useDocker is set | 1101 | pending | — | — | — |
| wraps error if removeDockerContainer throws an error | 1116 | pending | — | — | — |
| converts to TEMPORARY_ERROR | 1150 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for JVM settings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default values if no global or repo config | 1183 | pending | — | — | — |
| returns default values if empty repo config | 1194 | pending | — | — | — |
| returns default values if empty global config | 1205 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for JVM settings › does not allow floating point numbers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| in global config | 1219 | pending | — | — | — |
| in repo config | 1232 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for JVM settings › when using repo config to override memory limits`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when below global settings, repo settings are used | 1252 | pending | — | — | — |
| when repo settings are the same as global settings, they are used | 1266 | pending | — | — | — |
| when repo jvmMemory setting is higher than global setting, but lower than global jvmMaxMemory, the repo config is used | 1280 | pending | — | — | — |
| when repo jvmMaxMemory setting is lower than global settings, it is applied | 1292 | pending | — | — | — |
| when repo jvmMaxMemory setting is lower than global jvmMemory, jvmMemory is set to the same value | 1304 | pending | — | — | — |
| when repo jvmMaxMemory setting is lower than repo jvmMemory, jvmMemory is set to the same value | 1317 | pending | — | — | — |
| when repo jvmMaxMemory setting is higher than global settings, they are ignored | 1331 | pending | — | — | — |
| when repo jvmMaxMemory setting is higher than global settings, a debug log is logged | 1344 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for JVM settings › a minimum of 512M is enforced`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when global settings are lower than 512M, they are overridden to 512M | 1364 | pending | — | — | — |
| when global settings are lower than 512M, a debug log is logged | 1377 | pending | — | — | — |
| when repo settings are lower than 512M, they are overridden to 512M | 1389 | pending | — | — | — |
| when repo settings are lower than 512M, a debug log is logged | 1403 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for Node settings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not return a default value if no global or repo config | 1428 | pending | — | — | — |
| does not return default values if empty global config | 1438 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for Node settings › does not allow floating point numbers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| in global config | 1451 | pending | — | — | — |
| in repo config | 1463 | pending | — | — | — |

### `util/exec/index › getToolSettingsOptions() › for Node settings › when using repo config to override memory limits`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when below global settings, repo settings are used | 1479 | pending | — | — | — |
| when repo settings are the same as global settings, they are used | 1491 | pending | — | — | — |
| when repo nodeMaxMemory setting is lower than global settings, it is applied | 1503 | pending | — | — | — |
| when repo nodeMaxMemory setting is higher than global settings, they are ignored | 1515 | pending | — | — | — |
| when repo nodeMaxMemory setting is higher than global settings, a debug log is logged | 1527 | pending | — | — | — |

### `util/exec/index › gradleJvmArg()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| takes the values given to it, and returns the JVM arguments | 1547 | pending | — | — | — |

| (parametrized test at line 910) | 910 | pending | — | — | — |
---

