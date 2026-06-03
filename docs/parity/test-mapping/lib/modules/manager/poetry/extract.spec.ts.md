# `lib/modules/manager/poetry/extract.spec.ts`

[← `manager/poetry`](../../../../_by-module/manager/poetry.md) · [all modules](../../../../README.md)

**34/34 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 43 | returns null for empty | ported | [`crates/renovate-core/src/extractors/poetry.rs:1492`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1492) |
| 47 | returns null for parsed file without poetry section | ported | [`crates/renovate-core/src/extractors/poetry.rs:1484`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1484) |
| 51 | extracts multiple dependencies | ported | [`crates/renovate-core/src/extractors/poetry.rs:766`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L766) |
| 60 | extracts multiple dependencies (with dep = {version = "1.2.3"} case) | ported | [`crates/renovate-core/src/extractors/poetry.rs:797`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L797) |
| 66 | handles case with no dependencies | ported | [`crates/renovate-core/src/extractors/poetry.rs:1502`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1502) |
| 71 | handles multiple constraint dependencies | ported | [`crates/renovate-core/src/extractors/poetry.rs:1372`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1372) |
| 77 | extracts build-system.requires dependencies | ported | [`crates/renovate-core/src/extractors/poetry.rs:1336`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1336) |
| 112 | can parse toml v1 heterogeneous arrays | ported | [`crates/renovate-core/src/extractors/poetry.rs:1446`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1446) |
| 118 | extracts mixed versioning types | ported | [`crates/renovate-core/src/extractors/poetry.rs:849`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L849) |
| 160 | extracts dependencies from dependency groups | ported | [`crates/renovate-core/src/extractors/poetry.rs:871`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L871) |
| 197 | resolves lockedversions from the lockfile | ported | [`crates/renovate-core/src/extractors/poetry.rs:1413`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1413) |
| 209 | parses git dependencies long commit hashes on http urls | ported | [`crates/renovate-core/src/extractors/poetry.rs:1528`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1528) |
| 234 | parses git dependencies short commit hashes on http urls | ported | [`crates/renovate-core/src/extractors/poetry.rs:1529`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1529) |
| 259 | parses git dependencies long commit hashes on ssh urls | ported | [`crates/renovate-core/src/extractors/poetry.rs:1530`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1530) |
| 284 | parses git dependencies long commit hashes on http urls with branch marker | ported | [`crates/renovate-core/src/extractors/poetry.rs:1531`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1531) |
| 310 | parses github dependencies tags on ssh urls | ported | [`crates/renovate-core/src/extractors/poetry.rs:1581`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1581) |
| 325 | parses github dependencies tags on http urls | ported | [`crates/renovate-core/src/extractors/poetry.rs:1582`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1582) |
| 340 | parses git dependencies with tags that are not on github | ported | [`crates/renovate-core/src/extractors/poetry.rs:1583`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1583) |
| 363 | skips git dependencies | ported | [`crates/renovate-core/src/extractors/gemspec.rs:195`](../../../../../../../crates/renovate-core/src/extractors/gemspec.rs#L195) |
| 375 | skips git dependencies with version | ported | [`crates/renovate-core/src/extractors/poetry.rs:1514`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1514) |
| 388 | skips path dependencies | ported | [`crates/renovate-core/src/extractors/gemspec.rs:187`](../../../../../../../crates/renovate-core/src/extractors/gemspec.rs#L187) |
| 400 | skips path dependencies with version | ported | [`crates/renovate-core/src/extractors/poetry.rs:1622`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1622) |
| 413 | does not include registry url for dependency python | ported | [`crates/renovate-core/src/extractors/poetry.rs:910`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L910) |
| 436 | can parse empty registries | ported | [`crates/renovate-core/src/extractors/poetry.rs:932`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L932) |
| 441 | can parse missing registries | ported | [`crates/renovate-core/src/extractors/poetry.rs:948`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L948) |
| 446 | extracts registries | ported | [`crates/renovate-core/src/extractors/poetry.rs:963`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L963) |
| 455 | dedupes registries | ported | [`crates/renovate-core/src/extractors/poetry.rs:989`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L989) |
| 463 | source with priority="default" and implicit pypi priority="primary" | ported | [`crates/renovate-core/src/extractors/poetry.rs:1017`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1017) |
| 483 | source with implicit priority and pypi with priority="explicit" | ported | [`crates/renovate-core/src/extractors/poetry.rs:1042`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1042) |
| 500 | supports dependencies with explicit source | ported | [`crates/renovate-core/src/extractors/poetry.rs:1064`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1064) |
| 535 | parses package file with template | ported | [`crates/renovate-core/src/extractors/poetry.rs:1113`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1113) |
| 555 | extract dependencies from the project section | ported | [`crates/renovate-core/src/extractors/poetry.rs:1138`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1138) |
| 616 | extracts dependencies from pep735 dependency-groups | ported | [`crates/renovate-core/src/extractors/poetry.rs:1179`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1179) |
| 663 | enriches pep621/pep735 dependencies with poetry managerdata | ported | [`crates/renovate-core/src/extractors/poetry.rs:1209`](../../../../../../../crates/renovate-core/src/extractors/poetry.rs#L1209) |

