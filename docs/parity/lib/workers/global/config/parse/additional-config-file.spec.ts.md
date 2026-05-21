# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/additional-config-file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/additional-config-file.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** not-applicable

### `workers/global/config/parse/additional-config-file › .getConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| custom js config file exporting a function returning a Promise | 26 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| migrates | 55 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| warns if config is invalid | 68 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| parse and returns empty config if there is no RENOVATE_ADDITIONAL_CONFIG_FILE in env | 80 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| config.invalid.js | 84 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| fatal error and exit if custom config file does not exist | 112 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| fatal error and exit if config.js contains unresolved env var | 125 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| fatal error and exit if %s | 146 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| exports env variables to environment from processEnv object | 160 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| does not export env variables to environment from processEnv object if key/value is invalid | 183 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |

### `workers/global/config/parse/additional-config-file › deleteConfigFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip when RENOVATE_ADDITIONAL_CONFIG_FILE is not set ("%s") | 213 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| skip when config file does not exist | 225 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| skip if deleteConfigFile is not set ("%s") | 238 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| removes the specified config file | 254 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |
| fails silently when attempting to delete the config file | 276 | not-applicable | — | — | tests additional config file resolution via Node.js fs; Rust CLI has own config infrastructure |

---

