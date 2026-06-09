# Module: `worker/global`

[← all modules](../../README.md)

**Coverage:** 126/199 in-scope tests ported (opt-out=9) across 11 spec files.

| Spec file | it() | ported | pending | opt-out | Rust test file(s) | Status |
|---|--:|--:|--:|--:|---|---|
| [`lib/workers/global/autodiscover.spec.ts`](../../lib/workers/global/autodiscover.spec.ts.md) | 14 | 0 | 14 | 0 | — | pending |
| [`lib/workers/global/config/parse/additional-config-file.spec.ts`](../../lib/workers/global/config/parse/additional-config-file.spec.ts.md) | 15 | 4 | 5 | 6 | [`crates/renovate-core/src/config/file.rs:610`](../../../../../crates/renovate-core/src/config/file.rs#L610) | partial |
| [`lib/workers/global/config/parse/cli.spec.ts`](../../lib/workers/global/config/parse/cli.spec.ts.md) | 30 | 30 | 0 | 0 | [`crates/renovate-cli/src/config_builder.rs:631`](../../../../../crates/renovate-cli/src/config_builder.rs#L631)<br>[`crates/renovate-cli/tests/cli.rs:17`](../../../../../crates/renovate-cli/tests/cli.rs#L17)<br>[`crates/renovate-core/src/util.rs:6850`](../../../../../crates/renovate-core/src/util.rs#L6850) | ported |
| [`lib/workers/global/config/parse/env.spec.ts`](../../lib/workers/global/config/parse/env.spec.ts.md) | 45 | 45 | 0 | 0 | [`crates/renovate-cli/src/config_env.rs:702`](../../../../../crates/renovate-cli/src/config_env.rs#L702)<br>[`crates/renovate-core/src/util.rs:6825`](../../../../../crates/renovate-core/src/util.rs#L6825) | ported |
| [`lib/workers/global/config/parse/file.spec.ts`](../../lib/workers/global/config/parse/file.spec.ts.md) | 15 | 10 | 2 | 3 | [`crates/renovate-core/src/config/file.rs:540`](../../../../../crates/renovate-core/src/config/file.rs#L540)<br>[`crates/renovate-core/src/repo_config.rs:13576`](../../../../../crates/renovate-core/src/repo_config.rs#L13576) | partial |
| [`lib/workers/global/config/parse/host-rules-from-env.spec.ts`](../../lib/workers/global/config/parse/host-rules-from-env.spec.ts.md) | 12 | 12 | 0 | 0 | [`crates/renovate-core/src/config/host_rules_from_env.rs:262`](../../../../../crates/renovate-core/src/config/host_rules_from_env.rs#L262) | ported |
| [`lib/workers/global/config/parse/index.spec.ts`](../../lib/workers/global/config/parse/index.spec.ts.md) | 35 | 3 | 32 | 0 | [`crates/renovate-cli/src/config_builder.rs:661`](../../../../../crates/renovate-cli/src/config_builder.rs#L661)<br>[`crates/renovate-cli/src/config_env.rs:849`](../../../../../crates/renovate-cli/src/config_env.rs#L849) | partial |
| [`lib/workers/global/config/parse/util.spec.ts`](../../lib/workers/global/config/parse/util.spec.ts.md) | 1 | 1 | 0 | 0 | [`crates/renovate-core/src/config/migrate_validate.rs:4974`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4974) | ported |
| [`lib/workers/global/index.spec.ts`](../../lib/workers/global/index.spec.ts.md) | 15 | 2 | 13 | 0 | [`crates/renovate-core/src/util.rs:11745`](../../../../../crates/renovate-core/src/util.rs#L11745) | partial |
| [`lib/workers/global/initialize.spec.ts`](../../lib/workers/global/initialize.spec.ts.md) | 7 | 0 | 7 | 0 | — | pending |
| [`lib/workers/global/limits.spec.ts`](../../lib/workers/global/limits.spec.ts.md) | 19 | 19 | 0 | 0 | [`crates/renovate-core/src/limits.rs:237`](../../../../../crates/renovate-core/src/limits.rs#L237) | ported |

