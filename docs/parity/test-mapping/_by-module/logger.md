# Module: `logger`

[← all modules](../README.md)

**Coverage:** 30/80 in-scope tests ported (opt-out=0) across 10 spec files.

| Spec file | it() | ported | pending | opt-out | Rust test file(s) | Status |
|---|--:|--:|--:|--:|---|---|
| [`lib/logger/bunyan.spec.ts`](../lib/logger/bunyan.spec.ts.md) | 2 | 2 | 0 | 0 | [`crates/renovate-cli/src/logging.rs:124`](../../../../crates/renovate-cli/src/logging.rs#L124) | ported |
| [`lib/logger/cmd-serializer.spec.ts`](../lib/logger/cmd-serializer.spec.ts.md) | 2 | 2 | 0 | 0 | [`crates/renovate-core/src/util.rs:5916`](../../../../crates/renovate-core/src/util.rs#L5916) | ported |
| [`lib/logger/config-serializer.spec.ts`](../lib/logger/config-serializer.spec.ts.md) | 3 | 3 | 0 | 0 | [`crates/renovate-core/src/util.rs:5828`](../../../../crates/renovate-core/src/util.rs#L5828) | ported |
| [`lib/logger/err-serializer.spec.ts`](../lib/logger/err-serializer.spec.ts.md) | 5 | 0 | 5 | 0 | — | pending |
| [`lib/logger/index.spec.ts`](../lib/logger/index.spec.ts.md) | 26 | 0 | 26 | 0 | — | pending |
| [`lib/logger/once.spec.ts`](../lib/logger/once.spec.ts.md) | 9 | 3 | 6 | 0 | [`crates/renovate-core/src/util.rs:6766`](../../../../crates/renovate-core/src/util.rs#L6766) | partial |
| [`lib/logger/pretty-stdout.spec.ts`](../lib/logger/pretty-stdout.spec.ts.md) | 15 | 12 | 3 | 0 | [`crates/renovate-core/src/util.rs:10362`](../../../../crates/renovate-core/src/util.rs#L10362) | partial |
| [`lib/logger/remap.spec.ts`](../lib/logger/remap.spec.ts.md) | 6 | 6 | 0 | 0 | [`crates/renovate-core/src/util.rs:8485`](../../../../crates/renovate-core/src/util.rs#L8485) | ported |
| [`lib/logger/renovate-logger.spec.ts`](../lib/logger/renovate-logger.spec.ts.md) | 4 | 0 | 4 | 0 | — | pending |
| [`lib/logger/utils.spec.ts`](../lib/logger/utils.spec.ts.md) | 8 | 2 | 6 | 0 | [`crates/renovate-core/src/util.rs:5945`](../../../../crates/renovate-core/src/util.rs#L5945) | partial |

