# `lib/modules/manager/dockerfile/extract.spec.ts`

[← `manager/dockerfile`](../../../../_by-module/manager/dockerfile.md) · [all modules](../../../../README.md)

**76/76 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | handles no from | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:810`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L810) |
| 19 | handles naked dep | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:836`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L836) |
| 36 | handles run --mount=from | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1419`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1419) |
| 72 | is case insensitive | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:817`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L817) |
| 89 | handles tag | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:826`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L826) |
| 106 | handles digest | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:844`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L844) |
| 129 | handles tag and digest | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:858`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L858) |
| 152 | handles from as | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:926`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L926) |
| 173 | handles comments | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1256`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1256) |
| 194 | handles custom hosts | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:875`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L875) |
| 215 | handles custom hosts and suffix | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:883`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L883) |
| 236 | handles custom hosts with port | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:907`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L907) |
| 257 | handles custom hosts with port without tag | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:891`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L891) |
| 278 | handles quay hosts with port | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1756`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1756) |
| 295 | handles namespaced images | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:899`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L899) |
| 312 | handles custom hosts with namespace | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:867`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L867) |
| 333 | handles abnormal spacing | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:916`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L916) |
| 354 | extracts multiple from tags | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1246`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1246) |
| 386 | extracts tags from dockerfile which begins with a bom marker | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1234`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1234) |
| 407 | skips scratches | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:951`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L951) |
| 412 | skips named multistage from tags | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:936`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L936) |
| 433 | handles copy --from | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1440`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1440) |
| 454 | handles copy --from with digest | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1453`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1453) |
| 481 | handles copy --link --from | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1466`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1466) |
| 507 | skips named multistage copy --from tags | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1475`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1475) |
| 528 | skips index reference copy --from tags | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1485`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1485) |
| 549 | detects ["stage"] and ["final"] deps of docker multi-stage build. | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1494`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1494) |
| 598 | extracts images on adjacent lines | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1686`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1686) |
| 628 | extracts images from all sorts of (maybe multiline) from and copy --from statements | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1701`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1701) |
| 733 | handles calico/node | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1276`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1276) |
| 750 | handles ubuntu | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1285`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1285) |
| 768 | handles debian with codename | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1293`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1293) |
| 786 | handles debian with regular tag | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1301`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1301) |
| 803 | handles debian with prefixes | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1309`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1309) |
| 821 | handles debian with prefixes and registries | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1325`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1325) |
| 843 | handles prefixes | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1333`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1333) |
| 861 | handles prefixes with registries | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1349`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1349) |
| 883 | handles implausible line continuation | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1022`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1022) |
| 904 | handles multi-line from with space after escape character | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1033`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1033) |
| 921 | handles from without arg default value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1063`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1063) |
| 939 | handles from with empty arg default value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1071`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1071) |
| 960 | handles from with version in arg value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1081`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1081) |
| 981 | handles from with version in arg default value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1091`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1091) |
| 1002 | handles from with digest in arg default value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1103`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1103) |
| 1026 | handles from with overwritten arg value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1116`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1116) |
| 1058 | handles from with multiple arg values | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1129`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1129) |
| 1079 | skips scratch if provided in arg value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1141`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1141) |
| 1088 | extracts images from multi-line arg statements | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1149`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1149) |
| 1131 | ignores parser directives in wrong order | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1357`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1357) |
| 1152 | handles an alternative escape character | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1369`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1369) |
| 1227 | handles from with version in arg default value and quotes | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1180`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1180) |
| 1249 | handles version in arg and digest in from with crlf linefeed | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1193`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1193) |
| 1272 | handles updates of multiple arg values | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1206`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1206) |
| 1308 | handles same argument multiple times | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1222`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1222) |
| 1329 | handles empty optional parameters | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1766`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1766) |
| 1352 | handles registry alias | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1511`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1511) |
| 1380 | replaces registry alias from start only | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1538`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1538) |
| 1407 | handles empty registry | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1650`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1650) |
| 1435 | handles # syntax statements | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1662`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1662) |
| 1469 | ignores # syntax statements after first line | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1674`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1674) |
| 1493 | rejects null | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1778`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1778) |
| 1497 | rejects empty or whitespace | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1785`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1785) |
| 1501 | handles default environment variable values | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:973`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L973) |
| 1563 | skips tag containing a variable | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:999`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L999) |
| 1574 | skips depname containing a non default variable at start | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:958`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L958) |
| 1585 | skips depname containing a non default variable with brackets at start | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:965`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L965) |
| 1596 | skips depname containing a non default variable | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1006`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1006) |
| 1607 | skips depname containing a non default variable with brackets | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1013`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1013) |
| 1623 | _(it.each / template — verify manually)_ | ? | — |
| 1651 | handles no variable | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1794`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1794) |
| 1655 | handles simple variable | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1800`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1800) |
| 1661 | handles escaped variable | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1808`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1808) |
| 1667 | handles complex variable | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1816`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1816) |
| 1673 | handles complex variable with static default value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1827`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1827) |
| 1679 | handles complex variable with other variable as default value | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1835`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1835) |
| 1685 | handles multiple variables | ported | [`crates/renovate-core/src/extractors/dockerfile.rs:1843`](../../../../../../../crates/renovate-core/src/extractors/dockerfile.rs#L1843) |

