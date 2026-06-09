# `lib/modules/datasource/npm/get.spec.ts`

[← `datasource/npm`](../../../../_by-module/datasource/npm.md) · [all modules](../../../../README.md)

**19/24 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 43 | _(it.each / template — verify manually)_ | ? | — |
| 76 | _(it.each / template — verify manually)_ | ? | — |
| 103 | _(it.each / template — verify manually)_ | ? | — |
| 118 | uses hostrules basic auth | ported | [`crates/renovate-core/src/datasources/npm.rs:1970`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1970) |
| 140 | uses hostrules token auth | ported | [`crates/renovate-core/src/datasources/npm.rs:2004`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L2004) |
| 161 | uses hostrules basic token auth | ported | [`crates/renovate-core/src/datasources/npm.rs:2035`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L2035) |
| 183 | cover all paths | ported | [`crates/renovate-core/src/datasources/npm.rs:1713`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1713) |
| 249 | throw externalhosterror when error happens on registry.npmjs.org | ported | [`crates/renovate-core/src/datasources/npm.rs:1689`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1689) |
| 260 | redact body for externalhosterror when error happens on registry.npmjs.org | ported | [`crates/renovate-core/src/datasources/npm.rs:2066`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L2066) |
| 276 | do not throw externalhosterror when error happens on custom host | ported | [`crates/renovate-core/src/datasources/npm.rs:1428`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1428) |
| 288 | do not throw externalhosterror when error happens on registry.npmjs.org when hostrules disables abortonerror | ported | [`crates/renovate-core/src/datasources/npm.rs:2093`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L2093) |
| 303 | do not throw externalhosterror when error happens on registry.npmjs.org when hostrules without protocol disables abortonerror | ported | [`crates/renovate-core/src/datasources/npm.rs:2115`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L2115) |
| 319 | throw externalhosterror when error happens on custom host when hostrules enables abortonerror | ported | [`crates/renovate-core/src/datasources/npm.rs:2143`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L2143) |
| 335 | massages non-compliant repository urls | ported | [`crates/renovate-core/src/datasources/npm.rs:1365`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1365) |
| 379 | handles missing dist-tags latest | ported | [`crates/renovate-core/src/datasources/npm.rs:702`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L702) |
| 402 | handles mixed sourceurls in releases | ported | [`crates/renovate-core/src/datasources/npm.rs:1185`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1185) |
| 443 | handles short sourceurls in releases | ported | [`crates/renovate-core/src/datasources/npm.rs:1235`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1235) |
| 484 | does not override sourcedirectory | ported | [`crates/renovate-core/src/datasources/npm.rs:1333`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1333) |
| 527 | handles full repository urls with release source directories | ported | [`crates/renovate-core/src/datasources/npm.rs:1295`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1295) |
| 553 | does not massage non-github non-compliant repository urls | ported | [`crates/renovate-core/src/datasources/npm.rs:1397`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1397) |
| 609 | stores a trimmed packument body in cache | ported | [`crates/renovate-core/src/datasources/npm.rs:1566`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1566) |
| 706 | returns unexpired cache | pending | — |
| 738 | returns soft expired cache if revalidated | pending | — |
| 772 | returns soft expired cache on npmjs error | pending | — |

