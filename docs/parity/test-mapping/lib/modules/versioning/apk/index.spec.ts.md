# `lib/modules/versioning/apk/index.spec.ts`

[← `versioning/apk`](../../../../_by-module/versioning/apk.md) · [all modules](../../../../README.md)

**53/53 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
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
| 175 | should return null for invalid range operators | ported | [`crates/renovate-core/src/versioning/apk.rs:579`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L579) |
| 179 | should return null for empty versions array | ported | [`crates/renovate-core/src/versioning/apk.rs:593`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L593) |
| 183 | should filter out invalid versions | ported | [`crates/renovate-core/src/versioning/apk.rs:599`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L599) |
| 192 | _(it.each / template — verify manually)_ | ? | — |
| 202 | should return false for empty versions | ported | [`crates/renovate-core/src/versioning/apk.rs:618`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L618) |
| 210 | _(it.each / template — verify manually)_ | ? | — |
| 225 | should sort versions correctly | ported | [`crates/renovate-core/src/versioning/apk.rs:633`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L633) |
| 236 | should compare release numbers when version parts are equal | ported | [`crates/renovate-core/src/versioning/apk.rs:653`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L653) |
| 246 | _(it.each / template — verify manually)_ | ? | — |
| 261 | _(it.each / template — verify manually)_ | ? | — |
| 278 | _(it.each / template — verify manually)_ | ? | — |
| 295 | should handle invalid version parsing gracefully | ported | [`crates/renovate-core/src/versioning/apk.rs:700`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L700) |
| 305 | should handle null/undefined inputs | ported | [`crates/renovate-core/src/versioning/apk.rs:712`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L712) |
| 315 | should return false for unstable versions with prerelease | ported | [`crates/renovate-core/src/versioning/apk.rs:722`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L722) |
| 321 | should return false for empty versions in isstable | ported | [`crates/renovate-core/src/versioning/apk.rs:730`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L730) |
| 329 | should handle versions with different major versions in tilde range | ported | [`crates/renovate-core/src/versioning/apk.rs:736`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L736) |
| 335 | should handle versions with different minor versions in tilde range | ported | [`crates/renovate-core/src/versioning/apk.rs:750`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L750) |
| 340 | should handle invalid target versions in ranges | ported | [`crates/renovate-core/src/versioning/apk.rs:760`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L760) |
| 346 | should handle versions with prerelease identifiers in ranges | ported | [`crates/renovate-core/src/versioning/apk.rs:768`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L768) |
| 358 | should return null for versions with _p package fix suffix | ported | [`crates/renovate-core/src/versioning/apk.rs:782`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L782) |
| 364 | should return null for invalid versions | ported | [`crates/renovate-core/src/versioning/apk.rs:789`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L789) |
| 370 | should return patch version for non-_p patterns | ported | [`crates/renovate-core/src/versioning/apk.rs:796`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L796) |
| 376 | should handle versions with operators | ported | [`crates/renovate-core/src/versioning/apk.rs:804`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L804) |
| 384 | should strip revision from newversion when currentvalue has no revision | ported | [`crates/renovate-core/src/versioning/apk.rs:812`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L812) |
| 394 | should keep revision in newversion when currentvalue has revision | ported | [`crates/renovate-core/src/versioning/apk.rs:818`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L818) |
| 404 | should handle newversion without revision when currentvalue has no revision | ported | [`crates/renovate-core/src/versioning/apk.rs:824`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L824) |
| 414 | should handle newversion without revision when currentvalue has revision | ported | [`crates/renovate-core/src/versioning/apk.rs:830`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L830) |
| 426 | should handle complex prerelease identifier comparisons | ported | [`crates/renovate-core/src/versioning/apk.rs:836`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L836) |
| 438 | should handle versions with different prerelease patterns | ported | [`crates/renovate-core/src/versioning/apk.rs:844`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L844) |
| 445 | should handle unknown range operators | ported | [`crates/renovate-core/src/versioning/apk.rs:851`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L851) |
| 456 | should handle unhandled range operators that match regex | ported | [`crates/renovate-core/src/versioning/apk.rs:862`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L862) |
| 467 | should handle tilde range with invalid target version | ported | [`crates/renovate-core/src/versioning/apk.rs:872`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L872) |
| 474 | should handle tilde range with invalid version in list | ported | [`crates/renovate-core/src/versioning/apk.rs:879`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L879) |
| 485 | should handle major-only versions without minor/patch | ported | [`crates/renovate-core/src/versioning/apk.rs:889`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L889) |
| 494 | should handle letter vs number at same position in version parts | ported | [`crates/renovate-core/src/versioning/apk.rs:899`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L899) |
| 499 | should handle number vs letter comparison in version parts | ported | [`crates/renovate-core/src/versioning/apk.rs:906`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L906) |
| 504 | should handle extra numeric parts in remaining segments | ported | [`crates/renovate-core/src/versioning/apk.rs:913`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L913) |
| 509 | should handle lexicographic string comparison in version parts | ported | [`crates/renovate-core/src/versioning/apk.rs:920`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L920) |
| 514 | should handle equal letter parts continuing to next segment | ported | [`crates/renovate-core/src/versioning/apk.rs:927`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L927) |
| 519 | should handle trailing letter in remaining segments | ported | [`crates/renovate-core/src/versioning/apk.rs:934`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L934) |
| 524 | should return 0 for numerically equal but string-different versions | ported | [`crates/renovate-core/src/versioning/apk.rs:941`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L941) |
| 528 | should handle versions with different extra segment lengths | ported | [`crates/renovate-core/src/versioning/apk.rs:947`](../../../../../../../crates/renovate-core/src/versioning/apk.rs#L947) |

