# `lib/modules/manager/dockerfile/extract.spec.ts`

[← `manager/dockerfile`](../../../../_by-module/manager/dockerfile.md) · [all modules](../../../../README.md)

**76/76 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | handles no from | ported | `crates/renovate-core/src/extractors/dockerfile.rs:810` |
| 19 | handles naked dep | ported | `crates/renovate-core/src/extractors/dockerfile.rs:836` |
| 36 | handles run --mount=from | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1419` |
| 72 | is case insensitive | ported | `crates/renovate-core/src/extractors/dockerfile.rs:817` |
| 89 | handles tag | ported | `crates/renovate-core/src/extractors/dockerfile.rs:826` |
| 106 | handles digest | ported | `crates/renovate-core/src/extractors/dockerfile.rs:844` |
| 129 | handles tag and digest | ported | `crates/renovate-core/src/extractors/dockerfile.rs:858` |
| 152 | handles from as | ported | `crates/renovate-core/src/extractors/dockerfile.rs:926` |
| 173 | handles comments | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1256` |
| 194 | handles custom hosts | ported | `crates/renovate-core/src/extractors/dockerfile.rs:875` |
| 215 | handles custom hosts and suffix | ported | `crates/renovate-core/src/extractors/dockerfile.rs:883` |
| 236 | handles custom hosts with port | ported | `crates/renovate-core/src/extractors/dockerfile.rs:907` |
| 257 | handles custom hosts with port without tag | ported | `crates/renovate-core/src/extractors/dockerfile.rs:891` |
| 278 | handles quay hosts with port | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1756` |
| 295 | handles namespaced images | ported | `crates/renovate-core/src/extractors/dockerfile.rs:899` |
| 312 | handles custom hosts with namespace | ported | `crates/renovate-core/src/extractors/dockerfile.rs:867` |
| 333 | handles abnormal spacing | ported | `crates/renovate-core/src/extractors/dockerfile.rs:916` |
| 354 | extracts multiple from tags | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1246` |
| 386 | extracts tags from dockerfile which begins with a bom marker | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1234` |
| 407 | skips scratches | ported | `crates/renovate-core/src/extractors/dockerfile.rs:951` |
| 412 | skips named multistage from tags | ported | `crates/renovate-core/src/extractors/dockerfile.rs:936` |
| 433 | handles copy --from | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1440` |
| 454 | handles copy --from with digest | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1453` |
| 481 | handles copy --link --from | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1466` |
| 507 | skips named multistage copy --from tags | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1475` |
| 528 | skips index reference copy --from tags | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1485` |
| 549 | detects ["stage"] and ["final"] deps of docker multi-stage build. | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1494` |
| 598 | extracts images on adjacent lines | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1686` |
| 628 | extracts images from all sorts of (maybe multiline) from and copy --from statements | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1701` |
| 733 | handles calico/node | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1276` |
| 750 | handles ubuntu | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1285` |
| 768 | handles debian with codename | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1293` |
| 786 | handles debian with regular tag | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1301` |
| 803 | handles debian with prefixes | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1309` |
| 821 | handles debian with prefixes and registries | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1325` |
| 843 | handles prefixes | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1333` |
| 861 | handles prefixes with registries | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1349` |
| 883 | handles implausible line continuation | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1022` |
| 904 | handles multi-line from with space after escape character | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1033` |
| 921 | handles from without arg default value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1063` |
| 939 | handles from with empty arg default value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1071` |
| 960 | handles from with version in arg value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1081` |
| 981 | handles from with version in arg default value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1091` |
| 1002 | handles from with digest in arg default value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1103` |
| 1026 | handles from with overwritten arg value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1116` |
| 1058 | handles from with multiple arg values | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1129` |
| 1079 | skips scratch if provided in arg value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1141` |
| 1088 | extracts images from multi-line arg statements | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1149` |
| 1131 | ignores parser directives in wrong order | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1357` |
| 1152 | handles an alternative escape character | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1369` |
| 1227 | handles from with version in arg default value and quotes | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1180` |
| 1249 | handles version in arg and digest in from with crlf linefeed | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1193` |
| 1272 | handles updates of multiple arg values | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1206` |
| 1308 | handles same argument multiple times | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1222` |
| 1329 | handles empty optional parameters | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1766` |
| 1352 | handles registry alias | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1511` |
| 1380 | replaces registry alias from start only | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1538` |
| 1407 | handles empty registry | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1650` |
| 1435 | handles # syntax statements | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1662` |
| 1469 | ignores # syntax statements after first line | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1674` |
| 1493 | rejects null | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1778` |
| 1497 | rejects empty or whitespace | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1785` |
| 1501 | handles default environment variable values | ported | `crates/renovate-core/src/extractors/dockerfile.rs:973` |
| 1563 | skips tag containing a variable | ported | `crates/renovate-core/src/extractors/dockerfile.rs:999` |
| 1574 | skips depname containing a non default variable at start | ported | `crates/renovate-core/src/extractors/dockerfile.rs:958` |
| 1585 | skips depname containing a non default variable with brackets at start | ported | `crates/renovate-core/src/extractors/dockerfile.rs:965` |
| 1596 | skips depname containing a non default variable | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1006` |
| 1607 | skips depname containing a non default variable with brackets | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1013` |
| 1623 | _(it.each / template — verify manually)_ | ? | — |
| 1651 | handles no variable | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1794` |
| 1655 | handles simple variable | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1800` |
| 1661 | handles escaped variable | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1808` |
| 1667 | handles complex variable | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1816` |
| 1673 | handles complex variable with static default value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1827` |
| 1679 | handles complex variable with other variable as default value | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1835` |
| 1685 | handles multiple variables | ported | `crates/renovate-core/src/extractors/dockerfile.rs:1843` |

