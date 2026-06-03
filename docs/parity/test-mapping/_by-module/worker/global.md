# Module: `worker/global`

[← all modules](../../README.md)

**Coverage:** 116/208 tests ported across 11 spec files.

| Spec file | it() | ported | pending | Rust test file(s) | Status |
|---|--:|--:|--:|---|---|
| [`lib/workers/global/autodiscover.spec.ts`](../../lib/workers/global/autodiscover.spec.ts.md) | 14 | 0 | 14 | — | pending |
| [`lib/workers/global/config/parse/additional-config-file.spec.ts`](../../lib/workers/global/config/parse/additional-config-file.spec.ts.md) | 15 | 0 | 15 | — | pending |
| [`lib/workers/global/config/parse/cli.spec.ts`](../../lib/workers/global/config/parse/cli.spec.ts.md) | 30 | 30 | 0 | [`crates/renovate-cli/src/config_builder.rs:631`](../../../../../crates/renovate-cli/src/config_builder.rs#L631)<br>[`crates/renovate-cli/tests/cli.rs:17`](../../../../../crates/renovate-cli/tests/cli.rs#L17)<br>[`crates/renovate-core/src/util.rs:5885`](../../../../../crates/renovate-core/src/util.rs#L5885) | ported |
| [`lib/workers/global/config/parse/env.spec.ts`](../../lib/workers/global/config/parse/env.spec.ts.md) | 45 | 44 | 1 | [`crates/renovate-cli/src/config_env.rs:683`](../../../../../crates/renovate-cli/src/config_env.rs#L683)<br>[`crates/renovate-core/src/util.rs:5860`](../../../../../crates/renovate-core/src/util.rs#L5860) | partial |
| [`lib/workers/global/config/parse/file.spec.ts`](../../lib/workers/global/config/parse/file.spec.ts.md) | 15 | 8 | 7 | [`crates/renovate-core/src/config/file.rs:463`](../../../../../crates/renovate-core/src/config/file.rs#L463) | partial |
| [`lib/workers/global/config/parse/host-rules-from-env.spec.ts`](../../lib/workers/global/config/parse/host-rules-from-env.spec.ts.md) | 12 | 12 | 0 | [`crates/renovate-core/src/config/host_rules_from_env.rs:261`](../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L261) | ported |
| [`lib/workers/global/config/parse/index.spec.ts`](../../lib/workers/global/config/parse/index.spec.ts.md) | 35 | 0 | 35 | — | pending |
| [`lib/workers/global/config/parse/util.spec.ts`](../../lib/workers/global/config/parse/util.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/config/migrate_validate.rs:4932`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4932) | ported |
| [`lib/workers/global/index.spec.ts`](../../lib/workers/global/index.spec.ts.md) | 15 | 2 | 13 | [`crates/renovate-core/src/util.rs:10122`](../../../../../crates/renovate-core/src/util.rs#L10122) | partial |
| [`lib/workers/global/initialize.spec.ts`](../../lib/workers/global/initialize.spec.ts.md) | 7 | 0 | 7 | — | pending |
| [`lib/workers/global/limits.spec.ts`](../../lib/workers/global/limits.spec.ts.md) | 19 | 19 | 0 | [`crates/renovate-core/src/limits.rs:233`](../../../../../crates/renovate-core/src/limits.rs#L233) | ported |

