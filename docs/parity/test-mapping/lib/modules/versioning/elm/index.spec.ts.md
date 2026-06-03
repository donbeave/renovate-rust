# `lib/modules/versioning/elm/index.spec.ts`

[← `versioning/elm`](../../../../_by-module/versioning/elm.md) · [all modules](../../../../README.md)

**28/31 in-scope tests ported** (3 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | _(it.each / template — verify manually)_ | ? | — |
| 23 | _(it.each / template — verify manually)_ | ? | — |
| 43 | _(it.each / template — verify manually)_ | ? | — |
| 55 | _(it.each / template — verify manually)_ | ? | — |
| 65 | returns false for invalid version | ported | [`crates/renovate-core/src/versioning/elm.rs:271`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L271) |
| 71 | _(it.each / template — verify manually)_ | ? | — |
| 81 | extracts version components | ported | [`crates/renovate-core/src/versioning/elm.rs:284`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L284) |
| 89 | _(it.each / template — verify manually)_ | ? | — |
| 100 | _(it.each / template — verify manually)_ | ? | — |
| 112 | sorts versions correctly | ported | [`crates/renovate-core/src/versioning/elm.rs:309`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L309) |
| 120 | _(it.each / template — verify manually)_ | ? | — |
| 139 | returns false for invalid version | ported | [`crates/renovate-core/src/versioning/elm.rs:271`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L271) |
| 143 | returns false for invalid range | ported | [`crates/renovate-core/src/versioning/elm.rs:347`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L347) |
| 147 | returns false for malformed range where lower > upper | ported | [`crates/renovate-core/src/versioning/elm.rs:353`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L353) |
| 153 | _(it.each / template — verify manually)_ | ? | — |
| 170 | returns false for invalid version | ported | [`crates/renovate-core/src/versioning/elm.rs:271`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L271) |
| 176 | returns false for invalid range | ported | [`crates/renovate-core/src/versioning/elm.rs:347`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L347) |
| 182 | _(it.each / template — verify manually)_ | ? | — |
| 199 | _(it.each / template — verify manually)_ | ? | — |
| 215 | replaces exact version with new version | ported | [`crates/renovate-core/src/versioning/elm.rs:447`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L447) |
| 225 | handles bump strategy for exact version | ported | [`crates/renovate-core/src/versioning/elm.rs:458`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L458) |
| 237 | _(it.each / template — verify manually)_ | ? | — |
| 266 | returns null for invalid new version | ported | [`crates/renovate-core/src/versioning/elm.rs:530`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L530) |
| 276 | returns null for invalid current value | ported | [`crates/renovate-core/src/versioning/elm.rs:541`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L541) |
| 286 | returns null for unknown range strategy | ported | [`crates/renovate-core/src/versioning/elm.rs:552`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L552) |
| 296 | handles widen when newversion equals upper bound exactly | ported | [`crates/renovate-core/src/versioning/elm.rs:563`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L563) |
| 307 | widens elm-version range for new compiler release | ported | [`crates/renovate-core/src/versioning/elm.rs:574`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L574) |
| 318 | keeps elm-version range unchanged when version is already satisfied | ported | [`crates/renovate-core/src/versioning/elm.rs:585`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L585) |
| 328 | replaces elm-version range when explicitly requested | ported | [`crates/renovate-core/src/versioning/elm.rs:596`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L596) |
| 341 | finds highest satisfying version for elm-version range | ported | [`crates/renovate-core/src/versioning/elm.rs:607`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L607) |
| 355 | returns null when no compiler version satisfies range | ported | [`crates/renovate-core/src/versioning/elm.rs:617`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L617) |

