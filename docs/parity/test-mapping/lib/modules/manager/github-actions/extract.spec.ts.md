# `lib/modules/manager/github-actions/extract.spec.ts`

[← `manager/github-actions`](../../../../_by-module/manager/github-actions.md) · [all modules](../../../../README.md)

**28/28 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 46 | returns null for empty | ported | `crates/renovate-core/src/extractors/github_actions.rs:2573` |
| 52 | returns null for invalid yaml | ported | `crates/renovate-core/src/extractors/github_actions.rs:2579` |
| 58 | extracts multiple docker image lines from yaml configuration file | ported | `crates/renovate-core/src/extractors/github_actions.rs:2357` |
| 69 | extracts multiple action tag lines from yaml configuration file | ported | `crates/renovate-core/src/extractors/github_actions.rs:2319` |
| 83 | use github.com as registry when no settings provided | ported | `crates/renovate-core/src/extractors/github_actions.rs:3196` |
| 91 | use github.enterprise.com first and then github.com as registry running against github.enterprise.com | ported | `crates/renovate-core/src/extractors/github_actions.rs:3204` |
| 106 | use github.enterprise.com first and then github.com as registry running against github.enterprise.com/api/v3 | ported | `crates/renovate-core/src/extractors/github_actions.rs:3219` |
| 121 | use github.com only as registry when running against non-github | ported | `crates/renovate-core/src/extractors/github_actions.rs:3234` |
| 133 | use github.com only as registry when running against github.com | ported | `crates/renovate-core/src/extractors/github_actions.rs:3246` |
| 145 | use github.com only as registry when running against api.github.com | ported | `crates/renovate-core/src/extractors/github_actions.rs:3258` |
| 157 | returns undefined registryurls when endpoint is invalid url | ported | `crates/renovate-core/src/extractors/github_actions.rs:3270` |
| 169 | extracts multiple action tag lines with double quotes and comments | ported | `crates/renovate-core/src/extractors/github_actions.rs:2511` |
| 233 | maintains quotes | ported | `crates/renovate-core/src/extractors/github_actions.rs:2492` |
| 315 | maintains spaces between hash and comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2521` |
| 368 | extracts tags in different formats | ported | `crates/renovate-core/src/extractors/github_actions.rs:2413` |
| 500 | extracts non-semver ref automatically | ported | `crates/renovate-core/src/extractors/github_actions.rs:2470` |
| 520 | extracts pinned non-semver ref with digest | ported | `crates/renovate-core/src/extractors/github_actions.rs:2481` |
| 543 | disables naked sha pins without version comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2369` |
| 562 | disables naked short sha pins without version comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2378` |
| 581 | does not disable sha pins with version comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2390` |
| 606 | does not disable short sha pins with version comment | ported | `crates/renovate-core/src/extractors/github_actions.rs:2402` |
| 630 | extracts actions with fqdn | ported | `crates/renovate-core/src/extractors/github_actions.rs:2872` |
| 689 | extracts multiple action runners from yaml configuration file | ported | `crates/renovate-core/src/extractors/github_actions.rs:2765` |
| 773 | extracts x-version from actions/setup-x | ported | `crates/renovate-core/src/extractors/github_actions.rs:2945` |
| 905 | handles actions/setup-x without x-version field | ported | `crates/renovate-core/src/extractors/github_actions.rs:2832` |
| 923 | extracts x-version from actions/setup-x in composite action | ported | `crates/renovate-core/src/extractors/github_actions.rs:3004` |
| 1055 | logs unknown schema | ported | `crates/renovate-core/src/extractors/github_actions.rs:3282` |
| 1065 | _(it.each / template — verify manually)_ | ? | — |

