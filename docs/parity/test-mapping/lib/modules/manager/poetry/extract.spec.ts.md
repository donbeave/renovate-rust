# `lib/modules/manager/poetry/extract.spec.ts`

[← `manager/poetry`](../../../../_by-module/manager/poetry.md) · [all modules](../../../../README.md)

**34/34 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 43 | returns null for empty | ported | `crates/renovate-core/src/extractors/poetry.rs:1492` |
| 47 | returns null for parsed file without poetry section | ported | `crates/renovate-core/src/extractors/poetry.rs:1484` |
| 51 | extracts multiple dependencies | ported | `crates/renovate-core/src/extractors/poetry.rs:766` |
| 60 | extracts multiple dependencies (with dep = {version = "1.2.3"} case) | ported | `crates/renovate-core/src/extractors/poetry.rs:797` |
| 66 | handles case with no dependencies | ported | `crates/renovate-core/src/extractors/poetry.rs:1502` |
| 71 | handles multiple constraint dependencies | ported | `crates/renovate-core/src/extractors/poetry.rs:1372` |
| 77 | extracts build-system.requires dependencies | ported | `crates/renovate-core/src/extractors/poetry.rs:1336` |
| 112 | can parse toml v1 heterogeneous arrays | ported | `crates/renovate-core/src/extractors/poetry.rs:1446` |
| 118 | extracts mixed versioning types | ported | `crates/renovate-core/src/extractors/poetry.rs:849` |
| 160 | extracts dependencies from dependency groups | ported | `crates/renovate-core/src/extractors/poetry.rs:871` |
| 197 | resolves lockedversions from the lockfile | ported | `crates/renovate-core/src/extractors/poetry.rs:1413` |
| 209 | parses git dependencies long commit hashes on http urls | ported | `crates/renovate-core/src/extractors/poetry.rs:1528` |
| 234 | parses git dependencies short commit hashes on http urls | ported | `crates/renovate-core/src/extractors/poetry.rs:1529` |
| 259 | parses git dependencies long commit hashes on ssh urls | ported | `crates/renovate-core/src/extractors/poetry.rs:1530` |
| 284 | parses git dependencies long commit hashes on http urls with branch marker | ported | `crates/renovate-core/src/extractors/poetry.rs:1531` |
| 310 | parses github dependencies tags on ssh urls | ported | `crates/renovate-core/src/extractors/poetry.rs:1581` |
| 325 | parses github dependencies tags on http urls | ported | `crates/renovate-core/src/extractors/poetry.rs:1582` |
| 340 | parses git dependencies with tags that are not on github | ported | `crates/renovate-core/src/extractors/poetry.rs:1583` |
| 363 | skips git dependencies | ported | `crates/renovate-core/src/extractors/gemspec.rs:195` |
| 375 | skips git dependencies with version | ported | `crates/renovate-core/src/extractors/poetry.rs:1514` |
| 388 | skips path dependencies | ported | `crates/renovate-core/src/extractors/gemspec.rs:187` |
| 400 | skips path dependencies with version | ported | `crates/renovate-core/src/extractors/poetry.rs:1622` |
| 413 | does not include registry url for dependency python | ported | `crates/renovate-core/src/extractors/poetry.rs:910` |
| 436 | can parse empty registries | ported | `crates/renovate-core/src/extractors/poetry.rs:932` |
| 441 | can parse missing registries | ported | `crates/renovate-core/src/extractors/poetry.rs:948` |
| 446 | extracts registries | ported | `crates/renovate-core/src/extractors/poetry.rs:963` |
| 455 | dedupes registries | ported | `crates/renovate-core/src/extractors/poetry.rs:989` |
| 463 | source with priority="default" and implicit pypi priority="primary" | ported | `crates/renovate-core/src/extractors/poetry.rs:1017` |
| 483 | source with implicit priority and pypi with priority="explicit" | ported | `crates/renovate-core/src/extractors/poetry.rs:1042` |
| 500 | supports dependencies with explicit source | ported | `crates/renovate-core/src/extractors/poetry.rs:1064` |
| 535 | parses package file with template | ported | `crates/renovate-core/src/extractors/poetry.rs:1113` |
| 555 | extract dependencies from the project section | ported | `crates/renovate-core/src/extractors/poetry.rs:1138` |
| 616 | extracts dependencies from pep735 dependency-groups | ported | `crates/renovate-core/src/extractors/poetry.rs:1179` |
| 663 | enriches pep621/pep735 dependencies with poetry managerdata | ported | `crates/renovate-core/src/extractors/poetry.rs:1209` |

