# `lib/modules/manager/npm/post-update/yarn.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**21/29 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 62 | _(it.each / template — verify manually)_ | ? | — |
| 115 | if nodemaxmemory set on global config | ported | [`crates/renovate-core/src/extractors/npm_post_update/utils.rs:44`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/utils.rs#L44) |
| 157 | if nodemaxmemory set on repo config | ported | [`crates/renovate-core/src/extractors/npm_post_update/utils.rs:53`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/utils.rs#L53) |
| 201 | only skips build if skipinstalls is false | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:358`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L358) |
| 224 | allows and ignore scripts | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:367`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L367) |
| 252 | sets http proxy | pending | — |
| 288 | does not use global cache if zero install is detected | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:301`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L301) |
| 311 | _(it.each / template — verify manually)_ | ? | — |
| 350 | _(it.each / template — verify manually)_ | ? | — |
| 374 | _(it.each / template — verify manually)_ | ? | — |
| 410 | _(it.each / template — verify manually)_ | ? | — |
| 461 | _(it.each / template — verify manually)_ | ? | — |
| 494 | catches errors | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:319`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L319) |
| 504 | supports corepack | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:193`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L193) |
| 550 | supports packagemanager url corepack | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:291`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L291) |
| 597 | supports corepack on grouping | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:217`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L217) |
| 646 | supports customizing corepack version via config constraints | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:251`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L251) |
| 705 | uses slim yarn instead of corepack | ported | [`crates/renovate-core/src/extractors/npm_post_update/utils.rs:73`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/utils.rs#L73) |
| 744 | uses devengine.packagemanager(object) instead of corepack | ported | [`crates/renovate-core/src/extractors/npm_post_update.rs:216`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update.rs#L216) |
| 783 | uses devengine.packagemanager(array) instead of corepack | ported | [`crates/renovate-core/src/extractors/npm_post_update.rs:225`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update.rs#L225) |
| 822 | patches local yarn | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:337`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L337) |
| 872 | patches local yarn (docker) | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:385`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L385) |
| 919 | returns offline mirror and yarn path | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:393`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L393) |
| 939 | returns yarn path in subdir | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:233`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L233) |
| 953 | returns offline mirror | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:226`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L226) |
| 971 | returns no offline mirror and no absolute yarn path | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:243`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L243) |
| 990 | returns offline mirror and no yarn path for non-existant yarn-path binary | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:406`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L406) |
| 1008 | removes pure-lockfile and frozen-lockfile from .yarnrc | ported | [`crates/renovate-core/src/extractors/npm_post_update/yarn.rs:344`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/yarn.rs#L344) |
| 1026 | _(it.each / template — verify manually)_ | ? | — |

