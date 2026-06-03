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
| 175 | should return null for invalid range operators | ported | `crates/renovate-core/src/versioning/apk.rs:578` |
| 179 | should return null for empty versions array | ported | `crates/renovate-core/src/versioning/apk.rs:592` |
| 183 | should filter out invalid versions | ported | `crates/renovate-core/src/versioning/apk.rs:598` |
| 192 | _(it.each / template — verify manually)_ | ? | — |
| 202 | should return false for empty versions | ported | `crates/renovate-core/src/versioning/apk.rs:617` |
| 210 | _(it.each / template — verify manually)_ | ? | — |
| 225 | should sort versions correctly | ported | `crates/renovate-core/src/versioning/apk.rs:632` |
| 236 | should compare release numbers when version parts are equal | ported | `crates/renovate-core/src/versioning/apk.rs:652` |
| 246 | _(it.each / template — verify manually)_ | ? | — |
| 261 | _(it.each / template — verify manually)_ | ? | — |
| 278 | _(it.each / template — verify manually)_ | ? | — |
| 295 | should handle invalid version parsing gracefully | ported | `crates/renovate-core/src/versioning/apk.rs:699` |
| 305 | should handle null/undefined inputs | ported | `crates/renovate-core/src/versioning/apk.rs:711` |
| 315 | should return false for unstable versions with prerelease | ported | `crates/renovate-core/src/versioning/apk.rs:721` |
| 321 | should return false for empty versions in isstable | ported | `crates/renovate-core/src/versioning/apk.rs:729` |
| 329 | should handle versions with different major versions in tilde range | ported | `crates/renovate-core/src/versioning/apk.rs:735` |
| 335 | should handle versions with different minor versions in tilde range | ported | `crates/renovate-core/src/versioning/apk.rs:749` |
| 340 | should handle invalid target versions in ranges | ported | `crates/renovate-core/src/versioning/apk.rs:759` |
| 346 | should handle versions with prerelease identifiers in ranges | ported | `crates/renovate-core/src/versioning/apk.rs:767` |
| 358 | should return null for versions with _p package fix suffix | ported | `crates/renovate-core/src/versioning/apk.rs:781` |
| 364 | should return null for invalid versions | ported | `crates/renovate-core/src/versioning/apk.rs:788` |
| 370 | should return patch version for non-_p patterns | ported | `crates/renovate-core/src/versioning/apk.rs:795` |
| 376 | should handle versions with operators | ported | `crates/renovate-core/src/versioning/apk.rs:803` |
| 384 | should strip revision from newversion when currentvalue has no revision | ported | `crates/renovate-core/src/versioning/apk.rs:811` |
| 394 | should keep revision in newversion when currentvalue has revision | ported | `crates/renovate-core/src/versioning/apk.rs:817` |
| 404 | should handle newversion without revision when currentvalue has no revision | ported | `crates/renovate-core/src/versioning/apk.rs:823` |
| 414 | should handle newversion without revision when currentvalue has revision | ported | `crates/renovate-core/src/versioning/apk.rs:829` |
| 426 | should handle complex prerelease identifier comparisons | ported | `crates/renovate-core/src/versioning/apk.rs:835` |
| 438 | should handle versions with different prerelease patterns | ported | `crates/renovate-core/src/versioning/apk.rs:843` |
| 445 | should handle unknown range operators | ported | `crates/renovate-core/src/versioning/apk.rs:850` |
| 456 | should handle unhandled range operators that match regex | ported | `crates/renovate-core/src/versioning/apk.rs:861` |
| 467 | should handle tilde range with invalid target version | ported | `crates/renovate-core/src/versioning/apk.rs:871` |
| 474 | should handle tilde range with invalid version in list | ported | `crates/renovate-core/src/versioning/apk.rs:878` |
| 485 | should handle major-only versions without minor/patch | ported | `crates/renovate-core/src/versioning/apk.rs:888` |
| 494 | should handle letter vs number at same position in version parts | ported | `crates/renovate-core/src/versioning/apk.rs:898` |
| 499 | should handle number vs letter comparison in version parts | ported | `crates/renovate-core/src/versioning/apk.rs:905` |
| 504 | should handle extra numeric parts in remaining segments | ported | `crates/renovate-core/src/versioning/apk.rs:912` |
| 509 | should handle lexicographic string comparison in version parts | ported | `crates/renovate-core/src/versioning/apk.rs:919` |
| 514 | should handle equal letter parts continuing to next segment | ported | `crates/renovate-core/src/versioning/apk.rs:926` |
| 519 | should handle trailing letter in remaining segments | ported | `crates/renovate-core/src/versioning/apk.rs:933` |
| 524 | should return 0 for numerically equal but string-different versions | ported | `crates/renovate-core/src/versioning/apk.rs:940` |
| 528 | should handle versions with different extra segment lengths | ported | `crates/renovate-core/src/versioning/apk.rs:946` |

