# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/additional-config-file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/additional-config-file.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** pending-applicable

### `workers/global/config/parse/additional-config-file › .getConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| custom js config file exporting a function returning a Promise | 26 | pending | — | — | — |
| migrates | 55 | pending | — | — | — |
| warns if config is invalid | 68 | pending | — | — | — |
| parse and returns empty config if there is no RENOVATE_ADDITIONAL_CONFIG_FILE in env | 80 | pending | — | — | — |
| config.invalid.js | 84 | pending | — | — | — |
| fatal error and exit if custom config file does not exist | 112 | pending | — | — | — |
| fatal error and exit if config.js contains unresolved env var | 125 | pending | — | — | — |
| fatal error and exit if %s | 146 | pending | — | — | — |
| exports env variables to environment from processEnv object | 160 | pending | — | — | — |
| does not export env variables to environment from processEnv object if key/value is invalid | 183 | pending | — | — | — |

### `workers/global/config/parse/additional-config-file › deleteConfigFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip when RENOVATE_ADDITIONAL_CONFIG_FILE is not set ("%s") | 213 | pending | — | — | — |
| skip when config file does not exist | 225 | pending | — | — | — |
| skip if deleteConfigFile is not set ("%s") | 238 | pending | — | — | — |
| removes the specified config file | 254 | pending | — | — | — |
| fails silently when attempting to delete the config file | 276 | pending | — | — | — |

---
