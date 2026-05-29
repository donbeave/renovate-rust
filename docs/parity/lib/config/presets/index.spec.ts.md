# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/presets/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/index.spec.ts
**Total tests:** 69 | **Ported:** 0 | **Actionable:** 69 | **Status:** pending

### `config/presets/index › resolveConfigPresets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if no presets | 93 | pending | — | — | —|
| skips duplicate resolves | 102 | pending | — | — | —|
| throws if invalid preset file | 118 | pending | — | — | —|
| throws if invalid preset | 139 | pending | — | — | —|
| throws if path + invalid syntax | 157 | pending | — | — | —|
| throws if path + sub-preset | 173 | pending | — | — | —|
| throws if invalid preset json | 191 | pending | — | — | —|
| throws noconfig | 208 | pending | — | — | —|
| throws throw | 226 | pending | — | — | —|
| works with valid | 244 | pending | — | — | —|
| throws if valid and invalid | 258 | pending | — | — | —|
| resolves packageRule | 276 | pending | — | — | —|
| resolves eslint | 306 | pending | — | — | —|
| resolves linters | 314 | pending | — | — | —|
| resolves nested groups | 322 | pending | — | — | —|
| migrates automerge in presets | 331 | pending | — | — | —|
| ignores presets | 339 | pending | — | — | —|
| resolves self-hosted presets without baseConfig | 348 | pending | — | — | —|
| returns the presets which have been merged into the resulting config | 361 | pending | — | — | —|
| de-duplicates the presets which have been meregd into the resulting config | 378 | pending | — | — | —|
| resolves self-hosted preset with templating | 410 | pending | — | — | —|
| resolves self-hosted transitive presets without baseConfig | 430 | pending | — | — | —|
| resolves http presets | 449 | pending | — | — | —|
| resolves forgejo presets | 456 | pending | — | — | —|
| resolves gitea presets | 463 | pending | — | — | —|
| resolves gitlab presets | 470 | pending | — | — | —|
| gets preset value from cache when it has been seen | 477 | pending | — | — | —|
| default packageCache TTL should be 15 minutes | 512 | pending | — | — | —|
| use packageCache when presetCachePersistence is set | 553 | pending | — | — | —|
| throws | 594 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends` | 614 | pending | — | — | —|
| does not return any unmerged presets | 669 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges `extends`, recursively | 694 | pending | — | — | —|
| does not return any unmerged presets | 715 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 738 | pending | — | — | —|
| does not return any unmerged presets | 776 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=true › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges internal `extends` | 798 | pending | — | — | —|
| does not return any unmerged presets | 860 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 892 | pending | — | — | —|
| returns the presets in the unmerged array | 918 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal, parameterised preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 945 | pending | — | — | —|
| returns the preset in the unmerged array | 959 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset which includes many other internal presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 977 | pending | — | — | —|
| returns the unmerged internal presets | 991 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an external preset which references an internal preset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not merge `extends` | 1010 | pending | — | — | —|
| returns the unmerged internal presets | 1030 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving mixed internal and external presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not expand internal `extends` | 1052 | pending | — | — | —|
| returns the unmerged internal presets | 1085 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when resolving an internal preset inside a nested object config value`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the unmerged internal presets from a datasource | 1118 | pending | — | — | —|

### `config/presets/index › resolveConfigPresets › when using mergeInternalPresets=false › when duplicate internal presets are found`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| they are de-duplicated when returned as unmerged | 1142 | pending | — | — | —|

### `config/presets/index › replaceArgs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces args in strings | 1179 | pending | — | — | —|
| replaces args twice in same string | 1185 | pending | — | — | —|
| replaces objects | 1191 | pending | — | — | —|
| replaces arrays | 1208 | pending | — | — | —|

### `config/presets/index › getPreset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not use cache for internal presets | 1220 | pending | — | — | —|
| handles removed presets with a migration | 1227 | pending | — | — | —|
| handles removed presets with no migration | 1249 | pending | — | — | —|
| handles renamed monorepos | 1254 | pending | — | — | —|
| handles renamed monorepo groups | 1268 | pending | — | — | —|
| handles renamed regexManagers presets | 1293 | pending | — | — | —|
| gets linters | 1301 | pending | — | — | —|
| gets parameterised configs | 1309 | pending | — | — | —|
| handles missing params | 1325 | pending | — | — | —|
| ignores irrelevant params | 1338 | pending | — | — | —|
| substitutes {{args}} | 1348 | pending | — | — | —|
| handles 404 packages | 1375 | pending | — | — | —|
| handles no config | 1388 | pending | — | — | —|
| handles throw errors | 1401 | pending | — | — | —|
| handles preset not found | 1414 | pending | — | — | —|

---

