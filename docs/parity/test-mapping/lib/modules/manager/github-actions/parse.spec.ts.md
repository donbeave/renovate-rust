# `lib/modules/manager/github-actions/parse.spec.ts`

[← `manager/github-actions`](../../../../_by-module/manager/github-actions.md) · [all modules](../../../../README.md)

**53/53 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns null for empty string | ported | `crates/renovate-core/src/extractors/github_actions.rs:1666` |
| 16 | returns null for empty docker reference | ported | `crates/renovate-core/src/extractors/github_actions.rs:1672` |
| 20 | parses docker image with digest | ported | `crates/renovate-core/src/extractors/github_actions.rs:1678` |
| 29 | parses docker image with tag | ported | `crates/renovate-core/src/extractors/github_actions.rs:1692` |
| 38 | parses docker image with registry port and tag | ported | `crates/renovate-core/src/extractors/github_actions.rs:1706` |
| 51 | parses docker image without tag or digest | ported | `crates/renovate-core/src/extractors/github_actions.rs:1720` |
| 59 | parses docker image with registry but no tag | ported | `crates/renovate-core/src/extractors/github_actions.rs:1734` |
| 69 | parses ./ local reference | ported | `crates/renovate-core/src/extractors/github_actions.rs:1748` |
| 76 | parses ../ local reference | ported | `crates/renovate-core/src/extractors/github_actions.rs:1759` |
| 85 | returns null for invalid format | ported | `crates/renovate-core/src/extractors/github_actions.rs:1770` |
| 90 | parses owner/repo@ref with default hostname | ported | `crates/renovate-core/src/extractors/github_actions.rs:1777` |
| 102 | parses owner/repo/path@ref | ported | `crates/renovate-core/src/extractors/github_actions.rs:1793` |
| 114 | parses https://host/owner/repo@ref with explicit hostname | ported | `crates/renovate-core/src/extractors/github_actions.rs:1809` |
| 128 | parses https://host/owner/repo/path@ref | ported | `crates/renovate-core/src/extractors/github_actions.rs:1825` |
| 147 | returns ratchetexclude for ratchet:exclude | ported | `crates/renovate-core/src/extractors/github_actions.rs:1841` |
| 154 | returns empty object for no match | ported | `crates/renovate-core/src/extractors/github_actions.rs:1854` |
| 159 | parses pinned version with tag= prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:1864` |
| 168 | parses pinned version with pin prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:1873` |
| 177 | parses pinned version with renovate: prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:1882` |
| 186 | parses pinned version with renovate:pin prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:1897` |
| 195 | parses bare version | ported | `crates/renovate-core/src/extractors/github_actions.rs:1912` |
| 204 | parses version with @ prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:1921` |
| 213 | parses ratchet pinned version | ported | `crates/renovate-core/src/extractors/github_actions.rs:1930` |
| 222 | parses version without v prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:1945` |
| 231 | parses version with leading whitespace | ported | `crates/renovate-core/src/extractors/github_actions.rs:1954` |
| 240 | parses prefixed version like node/v20 | ported | `crates/renovate-core/src/extractors/github_actions.rs:1963` |
| 249 | parses prerelease version like v2.2-rc.1 | ported | `crates/renovate-core/src/extractors/github_actions.rs:1972` |
| 258 | parses full semver prerelease version like v2.2.0-rc.1 | ported | `crates/renovate-core/src/extractors/github_actions.rs:1981` |
| 267 | parses bare non-semver ref | ported | `crates/renovate-core/src/extractors/github_actions.rs:1996` |
| 276 | parses bare branch name | ported | `crates/renovate-core/src/extractors/github_actions.rs:2011` |
| 285 | ignores multi-word comments | ported | `crates/renovate-core/src/extractors/github_actions.rs:2020` |
| 291 | returns empty quote for unquoted string | ported | `crates/renovate-core/src/extractors/github_actions.rs:2029` |
| 295 | returns empty quote for empty string | ported | `crates/renovate-core/src/extractors/github_actions.rs:2035` |
| 299 | returns empty quote for single char | ported | `crates/renovate-core/src/extractors/github_actions.rs:2041` |
| 303 | parses double quoted string | ported | `crates/renovate-core/src/extractors/github_actions.rs:2047` |
| 307 | parses single quoted string | ported | `crates/renovate-core/src/extractors/github_actions.rs:2053` |
| 311 | handles whitespace around quotes | ported | `crates/renovate-core/src/extractors/github_actions.rs:2059` |
| 315 | returns empty quote for mismatched quotes | ported | `crates/renovate-core/src/extractors/github_actions.rs:2068` |
| 320 | returns empty quote for only opening quote | ported | `crates/renovate-core/src/extractors/github_actions.rs:2075` |
| 326 | returns null for non-uses lines | ported | `crates/renovate-core/src/extractors/github_actions.rs:2081` |
| 333 | returns null when value is only a comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2090` |
| 337 | parses simple uses line without comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2096` |
| 359 | parses uses line with - prefix | ported | `crates/renovate-core/src/extractors/github_actions.rs:2114` |
| 381 | parses uses line with comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2132` |
| 407 | parses uses line with multiple spaces before comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2150` |
| 435 | parses double quoted value | ported | `crates/renovate-core/src/extractors/github_actions.rs:2168` |
| 457 | parses single quoted value | ported | `crates/renovate-core/src/extractors/github_actions.rs:2186` |
| 479 | parses quoted value with comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2204` |
| 505 | parses docker action | ported | `crates/renovate-core/src/extractors/github_actions.rs:2222` |
| 524 | parses local action | ported | `crates/renovate-core/src/extractors/github_actions.rs:2245` |
| 541 | handles ratchet:exclude comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2265` |
| 567 | handles unrecognized comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2283` |
| 591 | returns null actionref for invalid action | ported | `crates/renovate-core/src/extractors/github_actions.rs:2301` |

