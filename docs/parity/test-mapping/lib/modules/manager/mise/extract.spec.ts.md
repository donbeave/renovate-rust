# `lib/modules/manager/mise/extract.spec.ts`

[← `manager/mise`](../../../../_by-module/manager/mise.md) · [all modules](../../../../README.md)

**41/42 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | returns null for empty | ported | [`crates/renovate-core/src/extractors/mise.rs:2152`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2152) |
| 18 | returns null for invalid toml | ported | [`crates/renovate-core/src/extractors/mise.rs:2202`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2202) |
| 22 | returns null for empty tools section | ported | [`crates/renovate-core/src/extractors/mise.rs:2208`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2208) |
| 29 | extracts tools - mise core plugins | ported | [`crates/renovate-core/src/extractors/mise.rs:1759`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1759) |
| 52 | extracts tools - mise registry tools | ported | [`crates/renovate-core/src/extractors/mise.rs:1807`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L1807) |
| 394 | extracts tools - asdf plugins | ported | [`crates/renovate-core/src/extractors/mise.rs:2112`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2112) |
| 410 | extracts tools with multiple versions | ported | [`crates/renovate-core/src/extractors/mise.rs:2141`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2141) |
| 433 | extracts tools with plugin options | ported | [`crates/renovate-core/src/extractors/mise.rs:2311`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2311) |
| 449 | extracts tools in the default registry with backends | ported | [`crates/renovate-core/src/extractors/mise.rs:2321`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2321) |
| 488 | extracts aqua backend tool | ported | [`crates/renovate-core/src/extractors/mise.rs:2357`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2357) |
| 515 | extracts cargo backend tools | ported | [`crates/renovate-core/src/extractors/mise.rs:2384`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2384) |
| 554 | extracts dotnet backend tool | ported | [`crates/renovate-core/src/extractors/mise.rs:2426`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2426) |
| 572 | extracts gem backend tool | ported | [`crates/renovate-core/src/extractors/mise.rs:2436`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2436) |
| 590 | extracts go backend tool | ported | [`crates/renovate-core/src/extractors/mise.rs:2446`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2446) |
| 608 | extracts npm backend tool | ported | [`crates/renovate-core/src/extractors/mise.rs:2459`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2459) |
| 626 | extracts pipx backend tools | ported | [`crates/renovate-core/src/extractors/mise.rs:2469`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2469) |
| 658 | extracts spm backend tools | ported | [`crates/renovate-core/src/extractors/mise.rs:2501`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2501) |
| 683 | extracts ubi backend tools | ported | [`crates/renovate-core/src/extractors/mise.rs:2526`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2526) |
| 741 | extracts github backend tools | ported | [`crates/renovate-core/src/extractors/mise.rs:2594`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2594) |
| 782 | provides skipreason for lines with unsupported tooling | ported | [`crates/renovate-core/src/extractors/mise.rs:2124`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2124) |
| 803 | provides skipreason for missing version - empty string | ported | [`crates/renovate-core/src/extractors/mise.rs:2214`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2214) |
| 819 | provides skipreason for missing version - missing version in object | ported | [`crates/renovate-core/src/extractors/mise.rs:2223`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2223) |
| 835 | provides skipreason for missing version - empty array | ported | [`crates/renovate-core/src/extractors/mise.rs:2232`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2232) |
| 856 | complete mise.toml example | ported | [`crates/renovate-core/src/extractors/mise.rs:2258`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2258) |
| 879 | complete example with skip | ported | [`crates/renovate-core/src/extractors/mise.rs:2288`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2288) |
| 912 | core java plugin function | ported | [`crates/renovate-core/src/extractors/mise.rs:2158`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2158) |
| 1035 | _(it.each / template — verify manually)_ | ? | — |
| 1062 | _(it.each / template — verify manually)_ | ? | — |
| 1087 | resolves tools from the mise registry data file via aqua backend | ported | [`crates/renovate-core/src/extractors/mise.rs:2620`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2620) |
| 1105 | resolves tools from the mise registry data file via cargo backend | ported | [`crates/renovate-core/src/extractors/mise.rs:2631`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2631) |
| 1123 | resolves tools from the mise registry data file via github backend | ported | [`crates/renovate-core/src/extractors/mise.rs:2642`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2642) |
| 1141 | resolves a tool from the mise registry, prioritising the github backend over others | ported | [`crates/renovate-core/src/extractors/mise.rs:2656`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2656) |
| 1170 | extracts lockedversion when lock file present | ported | [`crates/renovate-core/src/extractors/mise.rs:3006`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3006) |
| 1195 | sets lockfiles array when lock file present | ported | [`crates/renovate-core/src/extractors/mise.rs:3027`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3027) |
| 1205 | handles missing lock file gracefully | ported | [`crates/renovate-core/src/extractors/mise.rs:3036`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3036) |
| 1216 | handles malformed lock file gracefully | ported | [`crates/renovate-core/src/extractors/mise.rs:3045`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3045) |
| 1227 | works with environment-specific lock files | ported | [`crates/renovate-core/src/extractors/mise.rs:3054`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3054) |
| 1246 | extracts lockedversion for tools with backend prefix | ported | [`crates/renovate-core/src/extractors/mise.rs:3064`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3064) |
| 1260 | skips lockedversion when tool not in lock file | ported | [`crates/renovate-core/src/extractors/mise.rs:3082`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3082) |
| 1276 | extracts first lockedversion when multiple versions exist | ported | [`crates/renovate-core/src/extractors/mise.rs:3095`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L3095) |
| 1297 | skips kafka tool when version has no apache- prefix | ported | [`crates/renovate-core/src/extractors/mise.rs:2245`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2245) |
| 1317 | _(it.each / template — verify manually)_ | ? | — |

