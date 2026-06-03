# `lib/config/presets/parse.spec.ts`

[← `config/presets`](../../../_by-module/config/presets.md) · [all modules](../../../README.md)

**46/46 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | returns default package name | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1117` |
| 17 | parses github | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1130` |
| 28 | handles special chars | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1143` |
| 39 | parses github subfiles | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1155` |
| 50 | parses github subfiles with preset name | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1167` |
| 61 | parses github file with preset name with .json extension | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1179` |
| 73 | parses github file with preset name with .json5 extension | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1192` |
| 85 | parses github subfiles with preset name with .json extension | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1205` |
| 97 | parses github subfiles with preset name with .json5 extension | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1218` |
| 111 | parses github subfiles with preset and sub-preset name | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1231` |
| 124 | parses github subdirectories | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1243` |
| 137 | parses github toplevel file using subdirectory syntax | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1255` |
| 148 | parses gitlab | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1267` |
| 159 | parses gitea | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1279` |
| 170 | parses forgejo | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1291` |
| 181 | parses local | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1303` |
| 192 | parses local with spaces | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1315` |
| 203 | parses local with subdirectory | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1327` |
| 216 | parses local with spaces and subdirectory | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1339` |
| 229 | parses local with sub preset and tag | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1351` |
| 243 | parses local with subdirectory and tag | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1364` |
| 257 | parses local with subdirectory and branch/tag with a slash | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1377` |
| 271 | parses local with sub preset and branch/tag with a slash | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1390` |
| 285 | parses local repo with presetpath with url-encoded characters | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1403` |
| 298 | parses local repo with url-encoded characters | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1415` |
| 309 | parses no prefix as local | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1427` |
| 320 | parses local bitbucket user repo with preset name | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1439` |
| 331 | parses local bitbucket user repo | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1451` |
| 342 | returns default package name with params | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1463` |
| 354 | returns simple scope | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1478` |
| 365 | returns simple scope and params | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1490` |
| 376 | returns scope with repo and default | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1502` |
| 387 | returns scope with repo and params and default | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1514` |
| 400 | returns scope with presetname | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1533` |
| 411 | returns scope with presetname and params | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1545` |
| 422 | returns scope with repo and presetname | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1557` |
| 433 | returns scope with repo and presetname and params | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1569` |
| 449 | returns non-scoped default | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1584` |
| 460 | returns non-scoped package name | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1596` |
| 471 | returns non-scoped package name full | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1608` |
| 482 | returns non-scoped package name with params | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1620` |
| 493 | parses https urls for gitea | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1632` |
| 508 | parses https urls for forgejo | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1648` |
| 523 | parses http urls | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1664` |
| 538 | parses https urls with parameters for gitea | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1681` |
| 553 | parses https urls with parameters for forgejo | ported | `crates/renovate-core/src/extractors/renovate_config_presets.rs:1698` |

