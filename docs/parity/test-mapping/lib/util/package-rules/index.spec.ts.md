# `lib/util/package-rules/index.spec.ts`

[← `util/package-rules`](../../../_by-module/util/package-rules.md) · [all modules](../../../README.md)

**73/73 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 38 | applies | ported | `crates/renovate-core/src/repo_config.rs:8545` |
| 71 | applies both rules for a | ported | `crates/renovate-core/src/repo_config.rs:8581` |
| 81 | applies both rules for b | ported | `crates/renovate-core/src/repo_config.rs:8582` |
| 91 | applies the second rule | ported | `crates/renovate-core/src/repo_config.rs:8583` |
| 101 | applies matchpackagenames | ported | `crates/renovate-core/src/repo_config.rs:8584` |
| 109 | applies the second second rule | ported | `crates/renovate-core/src/repo_config.rs:8585` |
| 118 | excludes package name | ported | `crates/renovate-core/src/repo_config.rs:8586` |
| 127 | excludes package pattern | ported | `crates/renovate-core/src/repo_config.rs:8587` |
| 136 | ignores patterns if lock file maintenance | ported | `crates/renovate-core/src/repo_config.rs:8630` |
| 152 | do apply rule with matchpackagename | ported | `crates/renovate-core/src/repo_config.rs:8644` |
| 169 | sets skipreason=package-rules if enabled=false | ported | `crates/renovate-core/src/repo_config.rs:7756` |
| 184 | unsets skipreason=package-rules if enabled=true | ported | `crates/renovate-core/src/repo_config.rs:8934` |
| 202 | does not set skipreason=package-rules if the last packagerule has force.enabled=true | ported | `crates/renovate-core/src/repo_config.rs:7767` |
| 223 | does not set skipreason=package-rules if the last packagerule has force.enabled=true (if config.enabled=false) | ported | `crates/renovate-core/src/repo_config.rs:7818` |
| 245 | does not set skipreason=package-rules if the last packagerule has enabled=true (if config.force.enabled=false) | ported | `crates/renovate-core/src/repo_config.rs:7858` |
| 267 | sets skipreason=package-rules if the last packagerule has force.enabled=false (if config.force.enabled=false) | ported | `crates/renovate-core/src/repo_config.rs:7837` |
| 292 | sets skipreason=package-rules if the last packagerule has force.enabled=false | ported | `crates/renovate-core/src/repo_config.rs:7791` |
| 312 | skips skipreason=package-rules if enabled=true | ported | `crates/renovate-core/src/repo_config.rs:7878` |
| 326 | matches anything if missing inclusive rules | ported | `crates/renovate-core/src/repo_config.rs:8040` |
| 348 | supports inclusive or | ported | `crates/renovate-core/src/repo_config.rs:8051` |
| 370 | filters requested deptype | ported | `crates/renovate-core/src/repo_config.rs:8367` |
| 389 | filters from list of requested deptypes | ported | `crates/renovate-core/src/repo_config.rs:8466` |
| 408 | returns false if no deptypes | ported | `crates/renovate-core/src/repo_config.rs:8386` |
| 426 | filters managers with matching manager | ported | `crates/renovate-core/src/repo_config.rs:8197` |
| 446 | filters managers with non-matching manager | ported | `crates/renovate-core/src/repo_config.rs:8212` |
| 468 | filters categories with matching category | ported | `crates/renovate-core/src/repo_config.rs:12635` |
| 489 | filters categories with non-matching category | ported | `crates/renovate-core/src/repo_config.rs:12656` |
| 510 | filters categories with undefined category | ported | `crates/renovate-core/src/repo_config.rs:12615` |
| 529 | filters datasources with matching datasource | ported | `crates/renovate-core/src/repo_config.rs:8691` |
| 554 | filters branches with matching branch | ported | `crates/renovate-core/src/repo_config.rs:12550` |
| 573 | filters datasources with non-matching datasource | ported | `crates/renovate-core/src/repo_config.rs:8712` |
| 591 | filters branches with non-matching branch | ported | `crates/renovate-core/src/repo_config.rs:12551` |
| 609 | filters branches with matching branch regex | ported | `crates/renovate-core/src/repo_config.rs:12563` |
| 628 | filters branches with non-matching branch regex | ported | `crates/renovate-core/src/repo_config.rs:12564` |
| 647 | filters updatetype | ported | `crates/renovate-core/src/repo_config.rs:8848` |
| 672 | matches matchsourceurls with glob | ported | `crates/renovate-core/src/repo_config.rs:12114` |
| 695 | non-matches matchsourceurls with globs | ported | `crates/renovate-core/src/repo_config.rs:12115` |
| 718 | handles matchsourceurls when missing sourceurl | ported | `crates/renovate-core/src/repo_config.rs:12140` |
| 740 | matches matchsourceurls | ported | `crates/renovate-core/src/repo_config.rs:12088` |
| 763 | non-matches matchsourceurls | ported | `crates/renovate-core/src/repo_config.rs:12089` |
| 786 | handles matchregistryurls when missing registryurls | ported | `crates/renovate-core/src/repo_config.rs:12749` |
| 808 | matches matchregistryurls | ported | `crates/renovate-core/src/repo_config.rs:12703` |
| 831 | non-matches matchregistryurls | ported | `crates/renovate-core/src/repo_config.rs:12704` |
| 865 | matches matchconfidence | ported | `crates/renovate-core/src/repo_config.rs:12179` |
| 884 | non-matches matchconfidence | ported | `crates/renovate-core/src/repo_config.rs:12193` |
| 903 | does not match matchconfidence when there is no mergeconfidencelevel | ported | `crates/renovate-core/src/repo_config.rs:12207` |
| 922 | throws when unauthenticated | ported | `crates/renovate-core/src/repo_config.rs:12221` |
| 950 | filters naked deptype | ported | `crates/renovate-core/src/repo_config.rs:8507` |
| 968 | filters out unrequested deptype | ported | `crates/renovate-core/src/repo_config.rs:8526` |
| 987 | checks if matchcurrentversion selector is valid and satisfies the condition on range overlap | ported | `crates/renovate-core/src/repo_config.rs:9114` |
| 1020 | checks if matchcurrentversion selector is valid and satisfies the condition on pinned to range overlap | ported | `crates/renovate-core/src/repo_config.rs:9386` |
| 1041 | checks if matchcurrentversion selector is a version and matches if currentvalue is a range | ported | `crates/renovate-core/src/repo_config.rs:9355` |
| 1067 | checks if matchcurrentversion selector works with static values | ported | `crates/renovate-core/src/repo_config.rs:9338` |
| 1087 | checks if matchcurrentversion selector works with regular expressions | ported | `crates/renovate-core/src/repo_config.rs:9290` |
| 1114 | checks if matchcurrentversion selector works with negated regular expressions | ported | `crates/renovate-core/src/repo_config.rs:9314` |
| 1141 | matches packagefiles | ported | `crates/renovate-core/src/repo_config.rs:9434` |
| 1165 | matches lock files | ported | `crates/renovate-core/src/repo_config.rs:12392` |
| 1181 | matches paths | ported | `crates/renovate-core/src/repo_config.rs:12412` |
| 1211 | empty rules | ported | `crates/renovate-core/src/repo_config.rs:7728` |
| 1220 | creates groupslug if necessary | ported | `crates/renovate-core/src/repo_config.rs:13988` |
| 1239 | matches matchsourceurls with patterns (case-insensitive) | ported | `crates/renovate-core/src/repo_config.rs:12128` |
| 1262 | matches matchsourceurls(case-insensitive) | ported | `crates/renovate-core/src/repo_config.rs:12129` |
| 1285 | needs language to match | ported | `crates/renovate-core/src/repo_config.rs:12616` |
| 1303 | needs basebranch to match | ported | `crates/renovate-core/src/repo_config.rs:12577` |
| 1321 | needs manager to match | ported | `crates/renovate-core/src/repo_config.rs:12596` |
| 1339 | matches matchdepnames(depname) | ported | `crates/renovate-core/src/repo_config.rs:9582` |
| 1364 | matches if there are no matchers | ported | `crates/renovate-core/src/repo_config.rs:9597` |
| 1382 | overrides | ported | `crates/renovate-core/src/repo_config.rs:15352` |
| 1425 | overrides with templates | ported | `crates/renovate-core/src/repo_config.rs:15395` |
| 1442 | propagates fetchchangelogs from matching packagerule | ported | `crates/renovate-core/src/repo_config.rs:9605` |
| 1457 | does not set fetchchangelogs when packagerule does not match | ported | `crates/renovate-core/src/repo_config.rs:9622` |
| 1472 | compiles sourceurl with template helper functions | ported | `crates/renovate-core/src/repo_config.rs:9636` |
| 1491 | compiles sourceurl with template variables | ported | `crates/renovate-core/src/repo_config.rs:9657` |

