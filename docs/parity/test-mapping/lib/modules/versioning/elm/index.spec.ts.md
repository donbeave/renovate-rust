# `lib/modules/versioning/elm/index.spec.ts`

[← `versioning/elm`](../../../../_by-module/versioning/elm.md) · [all modules](../../../../README.md)

**28/31 in-scope tests ported** (3 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | _(it.each / template — verify manually)_ | ? | — |
| 23 | _(it.each / template — verify manually)_ | ? | — |
| 43 | _(it.each / template — verify manually)_ | ? | — |
| 55 | _(it.each / template — verify manually)_ | ? | — |
| 65 | returns false for invalid version | ported | [`crates/renovate-core/src/versioning/elm.rs:273`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L273) |
| 71 | _(it.each / template — verify manually)_ | ? | — |
| 81 | extracts version components | ported | [`crates/renovate-core/src/versioning/elm.rs:286`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L286) |
| 89 | _(it.each / template — verify manually)_ | ? | — |
| 100 | _(it.each / template — verify manually)_ | ? | — |
| 112 | sorts versions correctly | ported | [`crates/renovate-core/src/versioning/elm.rs:311`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L311) |
| 120 | _(it.each / template — verify manually)_ | ? | — |
| 139 | returns false for invalid version | ported | [`crates/renovate-core/src/versioning/elm.rs:273`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L273) |
| 143 | returns false for invalid range | ported | [`crates/renovate-core/src/versioning/elm.rs:349`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L349) |
| 147 | returns false for malformed range where lower > upper | ported | [`crates/renovate-core/src/versioning/elm.rs:355`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L355) |
| 153 | _(it.each / template — verify manually)_ | ? | — |
| 170 | returns false for invalid version | ported | [`crates/renovate-core/src/versioning/elm.rs:273`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L273) |
| 176 | returns false for invalid range | ported | [`crates/renovate-core/src/versioning/elm.rs:349`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L349) |
| 182 | _(it.each / template — verify manually)_ | ? | — |
| 199 | _(it.each / template — verify manually)_ | ? | — |
| 215 | replaces exact version with new version | ported | [`crates/renovate-core/src/versioning/elm.rs:449`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L449) |
| 225 | handles bump strategy for exact version | ported | [`crates/renovate-core/src/versioning/elm.rs:460`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L460) |
| 237 | _(it.each / template — verify manually)_ | ? | — |
| 266 | returns null for invalid new version | ported | [`crates/renovate-core/src/versioning/elm.rs:532`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L532) |
| 276 | returns null for invalid current value | ported | [`crates/renovate-core/src/versioning/elm.rs:543`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L543) |
| 286 | returns null for unknown range strategy | ported | [`crates/renovate-core/src/versioning/elm.rs:554`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L554) |
| 296 | handles widen when newversion equals upper bound exactly | ported | [`crates/renovate-core/src/versioning/elm.rs:565`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L565) |
| 307 | widens elm-version range for new compiler release | ported | [`crates/renovate-core/src/versioning/elm.rs:576`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L576) |
| 318 | keeps elm-version range unchanged when version is already satisfied | ported | [`crates/renovate-core/src/versioning/elm.rs:587`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L587) |
| 328 | replaces elm-version range when explicitly requested | ported | [`crates/renovate-core/src/versioning/elm.rs:598`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L598) |
| 341 | finds highest satisfying version for elm-version range | ported | [`crates/renovate-core/src/versioning/elm.rs:609`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L609) |
| 355 | returns null when no compiler version satisfies range | ported | [`crates/renovate-core/src/versioning/elm.rs:619`](../../../../../../../crates/renovate-core/src/versioning/elm.rs#L619) |

