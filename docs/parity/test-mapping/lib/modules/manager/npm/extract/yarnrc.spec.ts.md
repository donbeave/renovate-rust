# `lib/modules/manager/npm/extract/yarnrc.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/7 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | considers default registry | ported | [`crates/renovate-core/src/extractors/npm.rs:3354`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3354) |
| 17 | chooses matching scoped registry over default registry | ported | [`crates/renovate-core/src/extractors/npm.rs:3364`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3364) |
| 29 | ignores non matching scoped registry | ported | [`crates/renovate-core/src/extractors/npm.rs:3377`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3377) |
| 40 | ignores partial scope match | ported | [`crates/renovate-core/src/extractors/npm.rs:3387`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3387) |
| 51 | ignores missing scope registryserver | ported | [`crates/renovate-core/src/extractors/npm.rs:3394`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3394) |
| 63 | _(it.each / template — verify manually)_ | ? | — |
| 117 | _(it.each / template — verify manually)_ | ? | — |

