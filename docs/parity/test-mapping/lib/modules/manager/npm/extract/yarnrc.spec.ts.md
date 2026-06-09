# `lib/modules/manager/npm/extract/yarnrc.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/7 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | considers default registry | ported | [`crates/renovate-core/src/extractors/npm.rs:3361`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3361) |
| 17 | chooses matching scoped registry over default registry | ported | [`crates/renovate-core/src/extractors/npm.rs:3371`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3371) |
| 29 | ignores non matching scoped registry | ported | [`crates/renovate-core/src/extractors/npm.rs:3384`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3384) |
| 40 | ignores partial scope match | ported | [`crates/renovate-core/src/extractors/npm.rs:3394`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3394) |
| 51 | ignores missing scope registryserver | ported | [`crates/renovate-core/src/extractors/npm.rs:3401`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3401) |
| 63 | _(it.each / template — verify manually)_ | ? | — |
| 117 | _(it.each / template — verify manually)_ | ? | — |

