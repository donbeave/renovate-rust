# `lib/config/presets/index.spec.ts`

[← `config/presets`](../../../_by-module/config/presets.md) · [all modules](../../../README.md)

**0/69 ported** (69 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 93 | returns same if no presets | pending | — |
| 102 | skips duplicate resolves | pending | — |
| 118 | throws if invalid preset file | pending | — |
| 139 | throws if invalid preset | pending | — |
| 157 | throws if path + invalid syntax | pending | — |
| 173 | throws if path + sub-preset | pending | — |
| 191 | throws if invalid preset json | pending | — |
| 208 | throws noconfig | pending | — |
| 226 | throws throw | pending | — |
| 244 | works with valid | pending | — |
| 258 | throws if valid and invalid | pending | — |
| 276 | resolves packagerule | pending | — |
| 306 | resolves eslint | pending | — |
| 314 | resolves linters | pending | — |
| 322 | resolves nested groups | pending | — |
| 331 | migrates automerge in presets | pending | — |
| 339 | ignores presets | pending | — |
| 348 | resolves self-hosted presets without baseconfig | pending | — |
| 361 | returns the presets which have been merged into the resulting config | pending | — |
| 378 | de-duplicates the presets which have been meregd into the resulting config | pending | — |
| 410 | resolves self-hosted preset with templating | pending | — |
| 430 | resolves self-hosted transitive presets without baseconfig | pending | — |
| 449 | resolves http presets | pending | — |
| 456 | resolves forgejo presets | pending | — |
| 463 | resolves gitea presets | pending | — |
| 470 | resolves gitlab presets | pending | — |
| 477 | gets preset value from cache when it has been seen | pending | — |
| 512 | default packagecache ttl should be 15 minutes | pending | — |
| 553 | use packagecache when presetcachepersistence is set | pending | — |
| 594 | throws | pending | — |
| 614 | merges `extends` | pending | — |
| 669 | does not return any unmerged presets | pending | — |
| 694 | merges `extends`, recursively | pending | — |
| 715 | does not return any unmerged presets | pending | — |
| 738 | merges internal `extends` | pending | — |
| 776 | does not return any unmerged presets | pending | — |
| 798 | merges internal `extends` | pending | — |
| 860 | does not return any unmerged presets | pending | — |
| 892 | does not merge `extends` | pending | — |
| 918 | returns the presets in the unmerged array | pending | — |
| 945 | does not merge `extends` | pending | — |
| 959 | returns the preset in the unmerged array | pending | — |
| 977 | does not merge `extends` | pending | — |
| 991 | returns the unmerged internal presets | pending | — |
| 1010 | does not merge `extends` | pending | — |
| 1030 | returns the unmerged internal presets | pending | — |
| 1052 | does not expand internal `extends` | pending | — |
| 1085 | returns the unmerged internal presets | pending | — |
| 1118 | returns the unmerged internal presets from a datasource | pending | — |
| 1142 | they are de-duplicated when returned as unmerged | pending | — |
| 1179 | replaces args in strings | pending | — |
| 1185 | replaces args twice in same string | pending | — |
| 1191 | replaces objects | pending | — |
| 1208 | replaces arrays | pending | — |
| 1220 | does not use cache for internal presets | pending | — |
| 1227 | handles removed presets with a migration | pending | — |
| 1250 | handles removed presets with no migration | pending | — |
| 1255 | handles renamed monorepos | pending | — |
| 1269 | handles renamed monorepo groups | pending | — |
| 1294 | handles renamed regexmanagers presets | pending | — |
| 1302 | gets linters | pending | — |
| 1310 | gets parameterised configs | pending | — |
| 1326 | handles missing params | pending | — |
| 1339 | ignores irrelevant params | pending | — |
| 1349 | substitutes {{args}} | pending | — |
| 1376 | handles 404 packages | pending | — |
| 1389 | handles no config | pending | — |
| 1402 | handles throw errors | pending | — |
| 1415 | handles preset not found | pending | — |

