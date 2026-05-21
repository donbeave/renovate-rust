# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/presets/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/index.spec.ts
**Total tests:** 69 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/presets/index › resolveConfigPresets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if no presets | 93 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| skips duplicate resolves | 102 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws if invalid preset file | 118 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws if invalid preset | 139 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws if path + invalid syntax | 157 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws if path + sub-preset | 173 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws if invalid preset json | 191 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws noconfig | 208 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws throw | 226 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| works with valid | 244 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws if valid and invalid | 258 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves packageRule | 276 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves eslint | 306 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves linters | 314 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves nested groups | 322 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| migrates automerge in presets | 331 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| ignores presets | 339 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves self-hosted presets without baseConfig | 348 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| returns the presets which have been merged into the resulting config | 361 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| de-duplicates the presets which have been meregd into the resulting config | 378 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves self-hosted preset with templating | 410 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves self-hosted transitive presets without baseConfig | 430 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves http presets | 449 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves forgejo presets | 456 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves gitea presets | 463 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| resolves gitlab presets | 470 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| gets preset value from cache when it has been seen | 477 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| default packageCache TTL should be 15 minutes | 512 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| use packageCache when presetCachePersistence is set | 553 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| throws | 594 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends` | 614 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| does not return any unmerged presets | 669 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends`, recursively | 694 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| does not return any unmerged presets | 715 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 738 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| does not return any unmerged presets | 776 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 798 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| does not return any unmerged presets | 860 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 892 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| returns the presets in the unmerged array | 918 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal, parameterised preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 945 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| returns the preset in the unmerged array | 959 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 977 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| returns the unmerged internal presets | 991 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 1010 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| returns the unmerged internal presets | 1030 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not expand internal `extends` | 1052 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| returns the unmerged internal presets | 1085 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset inside a nested object config value`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the unmerged internal presets from a datasource | 1118 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when duplicate internal presets are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| they are de-duplicated when returned as unmerged | 1142 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › replaceArgs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces args in strings | 1179 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| replaces args twice in same string | 1185 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| replaces objects | 1191 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| replaces arrays | 1208 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

### `config/presets/index › getPreset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not use cache for internal presets | 1220 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles removed presets with a migration | 1227 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles removed presets with no migration | 1249 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles renamed monorepos | 1254 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles renamed monorepo groups | 1268 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles renamed regexManagers presets | 1293 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| gets linters | 1301 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| gets parameterised configs | 1309 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles missing params | 1325 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| ignores irrelevant params | 1338 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| substitutes {{args}} | 1348 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles 404 packages | 1375 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles no config | 1388 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles throw errors | 1401 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |
| handles preset not found | 1414 | not-applicable | — | — | tests preset resolution system requiring platform file fetching and npm package loading |

---

