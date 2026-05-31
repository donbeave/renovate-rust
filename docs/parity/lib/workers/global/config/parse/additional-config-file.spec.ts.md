# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/additional-config-file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/additional-config-file.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `workers/global/config/parse/additional-config-file › .getConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| custom js config file exporting a function returning a Promise  | 26 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| migrates  | 55 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| warns if config is invalid  | 68 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| parse and returns empty config if there is no RENOVATE_ADDITIONAL_CONFIG_FILE in env  | 80 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| config.invalid.js  | 84 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| fatal error and exit if custom config file does not exist  | 112 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| fatal error and exit if config.js contains unresolved env var  | 125 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| fatal error and exit if %s  | 146 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| exports env variables to environment from processEnv object  | 160 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| does not export env variables to environment from processEnv object if key/value is invalid  | 183 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |

### `workers/global/config/parse/additional-config-file › deleteConfigFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip when RENOVATE_ADDITIONAL_CONFIG_FILE is not set ("%s")  | 213 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| skip when config file does not exist  | 225 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| skip if deleteConfigFile is not set ("%s")  | 238 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| removes the specified config file  | 254 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |
| fails silently when attempting to delete the config file  | 276 | not-applicable | — | — | TS module system — tests JS config file loading; Rust uses different config loading mechanism |

---
