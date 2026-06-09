# Module: `util/exec`

[← all modules](../../README.md)

**Coverage:** 79/104 in-scope tests ported (opt-out=34) across 7 spec files.

| Spec file | it() | ported | pending | opt-out | Rust test file(s) | Status |
|---|--:|--:|--:|--:|---|---|
| [`lib/util/exec/common.spec.ts`](../../lib/util/exec/common.spec.ts.md) | 30 | 18 | 2 | 10 | [`crates/renovate-core/src/exec/error.rs:118`](../../../../../crates/renovate-core/src/exec/error.rs#L118)<br>[`crates/renovate-core/src/exec/orchestrator.rs:321`](../../../../../crates/renovate-core/src/exec/orchestrator.rs#L321)<br>[`crates/renovate-core/src/exec/raw.rs:145`](../../../../../crates/renovate-core/src/exec/raw.rs#L145) | partial |
| [`lib/util/exec/containerbase.spec.ts`](../../lib/util/exec/containerbase.spec.ts.md) | 20 | 9 | 11 | 0 | [`crates/renovate-core/src/exec/containerbase.rs:291`](../../../../../crates/renovate-core/src/exec/containerbase.rs#L291) | partial |
| [`lib/util/exec/docker/index.spec.ts`](../../lib/util/exec/docker/index.spec.ts.md) | 19 | 13 | 6 | 0 | [`crates/renovate-core/src/exec/docker.rs:118`](../../../../../crates/renovate-core/src/exec/docker.rs#L118) | partial |
| [`lib/util/exec/env.spec.ts`](../../lib/util/exec/env.spec.ts.md) | 4 | 4 | 0 | 0 | [`crates/renovate-core/src/exec/env.rs:83`](../../../../../crates/renovate-core/src/exec/env.rs#L83)<br>[`crates/renovate-core/src/util.rs:13737`](../../../../../crates/renovate-core/src/util.rs#L13737) | ported |
| [`lib/util/exec/hermit.spec.ts`](../../lib/util/exec/hermit.spec.ts.md) | 4 | 4 | 0 | 0 | [`crates/renovate-core/src/exec/hermit.rs:63`](../../../../../crates/renovate-core/src/exec/hermit.rs#L63) | ported |
| [`lib/util/exec/index.spec.ts`](../../lib/util/exec/index.spec.ts.md) | 40 | 28 | 5 | 7 | [`crates/renovate-core/src/exec/orchestrator.rs:221`](../../../../../crates/renovate-core/src/exec/orchestrator.rs#L221)<br>[`crates/renovate-core/src/exec/tool_settings.rs:125`](../../../../../crates/renovate-core/src/exec/tool_settings.rs#L125) | partial |
| [`lib/util/exec/utils.spec.ts`](../../lib/util/exec/utils.spec.ts.md) | 21 | 3 | 1 | 17 | [`crates/renovate-core/src/exec/raw.rs:129`](../../../../../crates/renovate-core/src/exec/raw.rs#L129)<br>[`crates/renovate-core/src/util.rs:12244`](../../../../../crates/renovate-core/src/util.rs#L12244) | partial |

