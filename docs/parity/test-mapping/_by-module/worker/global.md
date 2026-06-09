# Module: `worker/global`

[← all modules](../../README.md)

**Coverage:** 118/208 in-scope tests ported (opt-out=0) across 11 spec files.

| Spec file | it() | ported | pending | opt-out | Rust test file(s) | Status |
|---|--:|--:|--:|--:|---|---|
| [`lib/workers/global/autodiscover.spec.ts`](../../lib/workers/global/autodiscover.spec.ts.md) | 14 | 0 | 14 | 0 | — | pending |
| [`lib/workers/global/config/parse/additional-config-file.spec.ts`](../../lib/workers/global/config/parse/additional-config-file.spec.ts.md) | 15 | 0 | 15 | 0 | — | pending |
| [`lib/workers/global/config/parse/cli.spec.ts`](../../lib/workers/global/config/parse/cli.spec.ts.md) | 30 | 30 | 0 | 0 | [`crates/renovate-cli/src/config_builder.rs:637`](../../../../../crates/renovate-cli/src/config_builder.rs#L637)<br>[`crates/renovate-cli/tests/cli.rs:17`](../../../../../crates/renovate-cli/tests/cli.rs#L17)<br>[`crates/renovate-core/src/util.rs:6946`](../../../../../crates/renovate-core/src/util.rs#L6946) | ported |
| [`lib/workers/global/config/parse/env.spec.ts`](../../lib/workers/global/config/parse/env.spec.ts.md) | 45 | 45 | 0 | 0 | [`crates/renovate-cli/src/config_env.rs:740`](../../../../../crates/renovate-cli/src/config_env.rs#L740)<br>[`crates/renovate-core/src/util.rs:6921`](../../../../../crates/renovate-core/src/util.rs#L6921) | ported |
| [`lib/workers/global/config/parse/file.spec.ts`](../../lib/workers/global/config/parse/file.spec.ts.md) | 15 | 8 | 7 | 0 | [`crates/renovate-core/src/config/file.rs:540`](../../../../../crates/renovate-core/src/config/file.rs#L540) | partial |
| [`lib/workers/global/config/parse/host-rules-from-env.spec.ts`](../../lib/workers/global/config/parse/host-rules-from-env.spec.ts.md) | 12 | 12 | 0 | 0 | [`crates/renovate-core/src/config/host_rules_from_env.rs:188`](../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L188) | ported |
| [`lib/workers/global/config/parse/index.spec.ts`](../../lib/workers/global/config/parse/index.spec.ts.md) | 35 | 1 | 34 | 0 | [`crates/renovate-core/src/workers/global/config/parse/index.rs:220`](../../../../../crates/renovate-core/src/workers/global/config/parse/index.rs#L220) | partial |
| [`lib/workers/global/config/parse/util.spec.ts`](../../lib/workers/global/config/parse/util.spec.ts.md) | 1 | 1 | 0 | 0 | [`crates/renovate-core/src/config/migrate_validate.rs:4934`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4934) | ported |
| [`lib/workers/global/index.spec.ts`](../../lib/workers/global/index.spec.ts.md) | 15 | 2 | 13 | 0 | [`crates/renovate-core/src/util.rs:11840`](../../../../../crates/renovate-core/src/util.rs#L11840) | partial |
| [`lib/workers/global/initialize.spec.ts`](../../lib/workers/global/initialize.spec.ts.md) | 7 | 0 | 7 | 0 | — | pending |
| [`lib/workers/global/limits.spec.ts`](../../lib/workers/global/limits.spec.ts.md) | 19 | 19 | 0 | 0 | [`crates/renovate-core/src/limits.rs:234`](../../../../../crates/renovate-core/src/limits.rs#L234) | ported |

