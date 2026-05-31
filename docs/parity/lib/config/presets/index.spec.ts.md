# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/presets/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/index.spec.ts
**Total tests:** 69 | **Ported:** 0 | **Actionable:** 69 | **Status:** pending

### `config/presets/index › resolveConfigPresets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if no presets | 93 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| skips duplicate resolves | 102 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws if invalid preset file | 118 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws if invalid preset | 139 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws if path + invalid syntax | 157 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws if path + sub-preset | 173 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws if invalid preset json | 191 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws noconfig | 208 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws throw | 226 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| works with valid | 244 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws if valid and invalid | 258 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves packageRule | 276 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves eslint | 306 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves linters | 314 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves nested groups | 322 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| migrates automerge in presets | 331 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| ignores presets | 339 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves self-hosted presets without baseConfig | 348 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| returns the presets which have been merged into the resulting config | 361 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| de-duplicates the presets which have been meregd into the resulting config | 378 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves self-hosted preset with templating | 410 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves self-hosted transitive presets without baseConfig | 430 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves http presets | 449 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves forgejo presets | 456 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves gitea presets | 463 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| resolves gitlab presets | 470 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| gets preset value from cache when it has been seen | 477 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| default packageCache TTL should be 15 minutes | 512 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| use packageCache when presetCachePersistence is set | 553 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| throws | 594 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends` | 614 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| does not return any unmerged presets | 669 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends`, recursively | 694 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| does not return any unmerged presets | 715 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 738 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| does not return any unmerged presets | 776 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 798 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| does not return any unmerged presets | 860 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 892 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| returns the presets in the unmerged array | 918 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal, parameterised preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 945 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| returns the preset in the unmerged array | 959 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 977 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| returns the unmerged internal presets | 991 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 1010 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| returns the unmerged internal presets | 1030 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not expand internal `extends` | 1052 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| returns the unmerged internal presets | 1085 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset inside a nested object config value`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the unmerged internal presets from a datasource | 1118 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when duplicate internal presets are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| they are de-duplicated when returned as unmerged | 1142 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › replaceArgs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces args in strings | 1179 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| replaces args twice in same string | 1185 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| replaces objects | 1191 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| replaces arrays | 1208 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

### `config/presets/index › getPreset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not use cache for internal presets | 1220 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles removed presets with a migration | 1227 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles removed presets with no migration | 1249 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles renamed monorepos | 1254 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles renamed monorepo groups | 1268 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles renamed regexManagers presets | 1293 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| gets linters | 1301 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| gets parameterised configs | 1309 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles missing params | 1325 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| ignores irrelevant params | 1338 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| substitutes {{args}} | 1348 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles 404 packages | 1375 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles no config | 1388 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles throw errors | 1401 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |
| handles preset not found | 1414 | pending | — | — | Rust handles presets through compile-time expansion and effect extraction; no runtime preset resolution |

---

