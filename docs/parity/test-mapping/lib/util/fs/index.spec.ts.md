# `lib/util/fs/index.spec.ts`

[← `util/fs`](../../../_by-module/util/fs.md) · [all modules](../../../README.md)

**24/56 ported** (32 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 78 | _(it.each / template — verify manually)_ | ? | — |
| 99 | _(it.each / template — verify manually)_ | ? | — |
| 113 | reads buffer | ported | [`crates/renovate-core/src/fs.rs:388`](../../../../../../crates/renovate-core/src/fs.rs#L388) |
| 119 | reads string | ported | [`crates/renovate-core/src/fs.rs:389`](../../../../../../crates/renovate-core/src/fs.rs#L389) |
| 125 | returns null if file is not found | ported | [`crates/renovate-core/src/fs.rs:390`](../../../../../../crates/renovate-core/src/fs.rs#L390) |
| 129 | logs a warning if hidden unciode characters are found | pending | — |
| 140 | does not log the same hidden unciode characters if found multiple times | pending | — |
| 153 | logs a trace message (not warning) if hidden unicode characters are found in a binary file | pending | — |
| 173 | but no other hidden characters, it logs a trace message | pending | — |
| 188 | as well as other hidden characters, it logs a warning | pending | — |
| 204 | outputs file | ported | [`crates/renovate-core/src/fs.rs:409`](../../../../../../crates/renovate-core/src/fs.rs#L409) |
| 214 | throws if platform is local | pending | — |
| 219 | deletes file | ported | [`crates/renovate-core/src/fs.rs:410`](../../../../../../crates/renovate-core/src/fs.rs#L410) |
| 230 | renames file | ported | [`crates/renovate-core/src/fs.rs:411`](../../../../../../crates/renovate-core/src/fs.rs#L411) |
| 244 | renames file and replaces existing target | pending | — |
| 259 | creates directory | ported | [`crates/renovate-core/src/fs.rs:429`](../../../../../../crates/renovate-core/src/fs.rs#L429) |
| 269 | creates local directory | ported | [`crates/renovate-core/src/fs.rs:430`](../../../../../../crates/renovate-core/src/fs.rs#L430) |
| 279 | prefers environment variables over global config | ported | [`crates/renovate-core/src/fs.rs:431`](../../../../../../crates/renovate-core/src/fs.rs#L431) |
| 288 | returns cache dir | ported | [`crates/renovate-core/src/fs.rs:432`](../../../../../../crates/renovate-core/src/fs.rs#L432) |
| 295 | returns true for file | ported | [`crates/renovate-core/src/fs.rs:454`](../../../../../../crates/renovate-core/src/fs.rs#L454) |
| 301 | returns true for directory | ported | [`crates/renovate-core/src/fs.rs:455`](../../../../../../crates/renovate-core/src/fs.rs#L455) |
| 305 | returns false | ported | [`crates/renovate-core/src/fs.rs:456`](../../../../../../crates/renovate-core/src/fs.rs#L456) |
| 311 | returns true for valid local path | ported | [`crates/renovate-core/src/fs.rs:457`](../../../../../../crates/renovate-core/src/fs.rs#L457) |
| 315 | returns false | ported | [`crates/renovate-core/src/fs.rs:456`](../../../../../../crates/renovate-core/src/fs.rs#L456) |
| 321 | reads symlink | ported | [`crates/renovate-core/src/fs.rs:474`](../../../../../../crates/renovate-core/src/fs.rs#L474) |
| 333 | return null when link not exists | ported | [`crates/renovate-core/src/fs.rs:475`](../../../../../../crates/renovate-core/src/fs.rs#L475) |
| 347 | returns path for file | ported | [`crates/renovate-core/src/fs.rs:504`](../../../../../../crates/renovate-core/src/fs.rs#L504) |
| 371 | immediately returns null when either path is absolute | ported | [`crates/renovate-core/src/fs.rs:505`](../../../../../../crates/renovate-core/src/fs.rs#L505) |
| 378 | returns dir content | ported | [`crates/renovate-core/src/fs.rs:541`](../../../../../../crates/renovate-core/src/fs.rs#L541) |
| 396 | return empty array for non existing directory | ported | [`crates/renovate-core/src/fs.rs:542`](../../../../../../crates/renovate-core/src/fs.rs#L542) |
| 400 | return empty array for a existing but empty directory | ported | [`crates/renovate-core/src/fs.rs:543`](../../../../../../crates/renovate-core/src/fs.rs#L543) |
| 409 | creates write stream | pending | — |
| 426 | creates read stream | pending | — |
| 449 | returns true for file | ported | [`crates/renovate-core/src/fs.rs:454`](../../../../../../crates/renovate-core/src/fs.rs#L454) |
| 455 | returns false for directory | pending | — |
| 461 | returns false for non-existing path | pending | — |
| 469 | returns false for file | pending | — |
| 475 | returns false for directory | pending | — |
| 481 | returns false for non-existing path | pending | — |
| 486 | returns true for symlink | ported | [`crates/renovate-core/src/fs.rs:476`](../../../../../../crates/renovate-core/src/fs.rs#L476) |
| 502 | returns relative path for file | pending | — |
| 508 | returns null if nothing found | pending | — |
| 514 | returns undefined if found a file outside of localdir | pending | — |
| 522 | changes file mode | pending | — |
| 539 | returns stat object | pending | — |
| 550 | returns stat object | pending | — |
| 561 | lists directory | pending | — |
| 568 | removes cache dir | pending | — |
| 577 | reads file | pending | — |
| 585 | returns false if does not exist | pending | — |
| 591 | reads file | pending | — |
| 601 | outputs file | ported | [`crates/renovate-core/src/fs.rs:409`](../../../../../../crates/renovate-core/src/fs.rs#L409) |
| 609 | reads file | pending | — |
| 618 | writes file | pending | — |
| 626 | reads list of files from local fs | pending | — |
| 638 | returns null as content if file is not found | pending | — |

