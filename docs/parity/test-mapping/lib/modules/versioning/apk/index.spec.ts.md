# `lib/modules/versioning/apk/index.spec.ts`

[← `versioning/apk`](../../../../_by-module/versioning/apk.md) · [all modules](../../../../README.md)

**53/53 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | _(it.each / template — verify manually)_ | ? | — |
| 19 | _(it.each / template — verify manually)_ | ? | — |
| 41 | _(it.each / template — verify manually)_ | ? | — |
| 51 | _(it.each / template — verify manually)_ | ? | — |
| 61 | _(it.each / template — verify manually)_ | ? | — |
| 74 | _(it.each / template — verify manually)_ | ? | — |
| 102 | _(it.each / template — verify manually)_ | ? | — |
| 115 | _(it.each / template — verify manually)_ | ? | — |
| 136 | _(it.each / template — verify manually)_ | ? | — |
| 149 | _(it.each / template — verify manually)_ | ? | — |
| 164 | _(it.each / template — verify manually)_ | ? | — |
| 175 | should return null for invalid range operators | ported | [`crates/renovate-core/src/versioning/apk.rs:578`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L578) |
| 179 | should return null for empty versions array | ported | [`crates/renovate-core/src/versioning/apk.rs:592`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L592) |
| 183 | should filter out invalid versions | ported | [`crates/renovate-core/src/versioning/apk.rs:598`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L598) |
| 192 | _(it.each / template — verify manually)_ | ? | — |
| 202 | should return false for empty versions | ported | [`crates/renovate-core/src/versioning/apk.rs:617`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L617) |
| 210 | _(it.each / template — verify manually)_ | ? | — |
| 225 | should sort versions correctly | ported | [`crates/renovate-core/src/versioning/apk.rs:632`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L632) |
| 236 | should compare release numbers when version parts are equal | ported | [`crates/renovate-core/src/versioning/apk.rs:652`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L652) |
| 246 | _(it.each / template — verify manually)_ | ? | — |
| 261 | _(it.each / template — verify manually)_ | ? | — |
| 278 | _(it.each / template — verify manually)_ | ? | — |
| 295 | should handle invalid version parsing gracefully | ported | [`crates/renovate-core/src/versioning/apk.rs:699`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L699) |
| 305 | should handle null/undefined inputs | ported | [`crates/renovate-core/src/versioning/apk.rs:711`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L711) |
| 315 | should return false for unstable versions with prerelease | ported | [`crates/renovate-core/src/versioning/apk.rs:721`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L721) |
| 321 | should return false for empty versions in isstable | ported | [`crates/renovate-core/src/versioning/apk.rs:729`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L729) |
| 329 | should handle versions with different major versions in tilde range | ported | [`crates/renovate-core/src/versioning/apk.rs:735`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L735) |
| 335 | should handle versions with different minor versions in tilde range | ported | [`crates/renovate-core/src/versioning/apk.rs:749`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L749) |
| 340 | should handle invalid target versions in ranges | ported | [`crates/renovate-core/src/versioning/apk.rs:759`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L759) |
| 346 | should handle versions with prerelease identifiers in ranges | ported | [`crates/renovate-core/src/versioning/apk.rs:767`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L767) |
| 358 | should return null for versions with _p package fix suffix | ported | [`crates/renovate-core/src/versioning/apk.rs:781`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L781) |
| 364 | should return null for invalid versions | ported | [`crates/renovate-core/src/versioning/apk.rs:788`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L788) |
| 370 | should return patch version for non-_p patterns | ported | [`crates/renovate-core/src/versioning/apk.rs:795`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L795) |
| 376 | should handle versions with operators | ported | [`crates/renovate-core/src/versioning/apk.rs:803`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L803) |
| 384 | should strip revision from newversion when currentvalue has no revision | ported | [`crates/renovate-core/src/versioning/apk.rs:811`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L811) |
| 394 | should keep revision in newversion when currentvalue has revision | ported | [`crates/renovate-core/src/versioning/apk.rs:817`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L817) |
| 404 | should handle newversion without revision when currentvalue has no revision | ported | [`crates/renovate-core/src/versioning/apk.rs:823`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L823) |
| 414 | should handle newversion without revision when currentvalue has revision | ported | [`crates/renovate-core/src/versioning/apk.rs:829`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L829) |
| 426 | should handle complex prerelease identifier comparisons | ported | [`crates/renovate-core/src/versioning/apk.rs:835`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L835) |
| 438 | should handle versions with different prerelease patterns | ported | [`crates/renovate-core/src/versioning/apk.rs:843`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L843) |
| 445 | should handle unknown range operators | ported | [`crates/renovate-core/src/versioning/apk.rs:850`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L850) |
| 456 | should handle unhandled range operators that match regex | ported | [`crates/renovate-core/src/versioning/apk.rs:861`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L861) |
| 467 | should handle tilde range with invalid target version | ported | [`crates/renovate-core/src/versioning/apk.rs:871`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L871) |
| 474 | should handle tilde range with invalid version in list | ported | [`crates/renovate-core/src/versioning/apk.rs:878`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L878) |
| 485 | should handle major-only versions without minor/patch | ported | [`crates/renovate-core/src/versioning/apk.rs:888`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L888) |
| 494 | should handle letter vs number at same position in version parts | ported | [`crates/renovate-core/src/versioning/apk.rs:898`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L898) |
| 499 | should handle number vs letter comparison in version parts | ported | [`crates/renovate-core/src/versioning/apk.rs:905`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L905) |
| 504 | should handle extra numeric parts in remaining segments | ported | [`crates/renovate-core/src/versioning/apk.rs:912`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L912) |
| 509 | should handle lexicographic string comparison in version parts | ported | [`crates/renovate-core/src/versioning/apk.rs:919`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L919) |
| 514 | should handle equal letter parts continuing to next segment | ported | [`crates/renovate-core/src/versioning/apk.rs:926`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L926) |
| 519 | should handle trailing letter in remaining segments | ported | [`crates/renovate-core/src/versioning/apk.rs:933`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L933) |
| 524 | should return 0 for numerically equal but string-different versions | ported | [`crates/renovate-core/src/versioning/apk.rs:940`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L940) |
| 528 | should handle versions with different extra segment lengths | ported | [`crates/renovate-core/src/versioning/apk.rs:946`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L946) |

