# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/additional-config-file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/additional-config-file.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** not-applicable

### `workers/global/config/parse/additional-config-file › .getConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| custom js config file exporting a function returning a Promise | 26 | not-applicable | — | — | Loads JS config files via Node.js dynamic import with process.exit spy — no Rust equivalent |
| migrates | 55 | not-applicable | — | — | Requires JS config migration via Node.js dynamic import |
| warns if config is invalid | 68 | not-applicable | — | — | Requires writing/reading JS config file from temp dir with vi.spyOn(fsExtra) |
| parse and returns empty config if there is no RENOVATE_ADDITIONAL_CONFIG_FILE in env | 80 | not-applicable | — | — | Tests RENOVATE_ADDITIONAL_CONFIG_FILE env var handling — covered equivalently in Rust env parsing |
| config.invalid.js | 84 | not-applicable | — | — | Requires loading invalid JS config via Node.js import with process.exit spy |
| fatal error and exit if custom config file does not exist | 112 | not-applicable | — | — | Uses vi.spyOn(process, 'exit') and fsExtra spies — process exit infrastructure |
| fatal error and exit if config.js contains unresolved env var | 125 | not-applicable | — | — | Requires JS config with env var substitution via Node.js import |
| fatal error and exit if %s | 146 | not-applicable | — | — | Requires unsupported extension loading with process.exit spy |
| exports env variables to environment from processEnv object | 160 | not-applicable | — | — | Tests processEnv export from JS config — JS runtime infrastructure |
| does not export env variables to environment from processEnv object if key/value is invalid | 183 | not-applicable | — | — | Tests processEnv validation in JS config — JS runtime infrastructure |

### `workers/global/config/parse/additional-config-file › deleteConfigFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skip when RENOVATE_ADDITIONAL_CONFIG_FILE is not set ("%s") | 213 | not-applicable | — | — | Uses vi.spyOn(fsExtra) file system spy — infrastructure |
| skip when config file does not exist | 225 | not-applicable | — | — | Uses vi.spyOn(fsExtra.pathExists) file system spy |
| skip if deleteConfigFile is not set ("%s") | 238 | not-applicable | — | — | Uses vi.spyOn(fsExtra) file system spy |
| removes the specified config file | 254 | not-applicable | — | — | Uses vi.spyOn(fsExtra.remove) file system spy |
| fails silently when attempting to delete the config file | 276 | not-applicable | — | — | Uses vi.spyOn(fsExtra.remove) file system spy |

---
