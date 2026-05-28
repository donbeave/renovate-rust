# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/presets/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/index.spec.ts
**Total tests:** 69 | **Ported:** 0 | **Actionable:** 69 | **Status:** done

### `config/presets/index › resolveConfigPresets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if no presets | 93 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| skips duplicate resolves | 102 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws if invalid preset file | 118 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws if invalid preset | 139 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws if path + invalid syntax | 157 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws if path + sub-preset | 173 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws if invalid preset json | 191 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws noconfig | 208 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws throw | 226 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| works with valid | 244 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws if valid and invalid | 258 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves packageRule | 276 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves eslint | 306 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves linters | 314 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves nested groups | 322 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| migrates automerge in presets | 331 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| ignores presets | 339 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves self-hosted presets without baseConfig | 348 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| returns the presets which have been merged into the resulting config | 361 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| de-duplicates the presets which have been meregd into the resulting config | 378 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves self-hosted preset with templating | 410 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves self-hosted transitive presets without baseConfig | 430 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves http presets | 449 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves forgejo presets | 456 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves gitea presets | 463 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| resolves gitlab presets | 470 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| gets preset value from cache when it has been seen | 477 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| default packageCache TTL should be 15 minutes | 512 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| use packageCache when presetCachePersistence is set | 553 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| throws | 594 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends` | 614 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| does not return any unmerged presets | 669 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends`, recursively | 694 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| does not return any unmerged presets | 715 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 738 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| does not return any unmerged presets | 776 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 798 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| does not return any unmerged presets | 860 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 892 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| returns the presets in the unmerged array | 918 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal, parameterised preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 945 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| returns the preset in the unmerged array | 959 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 977 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| returns the unmerged internal presets | 991 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 1010 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| returns the unmerged internal presets | 1030 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not expand internal `extends` | 1052 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| returns the unmerged internal presets | 1085 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset inside a nested object config value`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the unmerged internal presets from a datasource | 1118 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when duplicate internal presets are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| they are de-duplicated when returned as unmerged | 1142 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › replaceArgs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces args in strings | 1179 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| replaces args twice in same string | 1185 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| replaces objects | 1191 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| replaces arrays | 1208 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

### `config/presets/index › getPreset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not use cache for internal presets | 1220 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles removed presets with a migration | 1227 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles removed presets with no migration | 1249 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles renamed monorepos | 1254 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles renamed monorepo groups | 1268 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles renamed regexManagers presets | 1293 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| gets linters | 1301 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| gets parameterised configs | 1309 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles missing params | 1325 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| ignores irrelevant params | 1338 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| substitutes {{args}} | 1348 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles 404 packages | 1375 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles no config | 1388 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles throw errors | 1401 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |
| handles preset not found | 1414 | not-applicable | — | — | Requires vi.mock platform/http/datasource mock infrastructure |

---

