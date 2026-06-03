# `lib/modules/manager/pip_requirements/artifacts.spec.ts`

[← `manager/pip_requirements`](../../../../_by-module/manager/pip_requirements.md) · [all modules](../../../../README.md)

**6/8 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 51 | returns null if no updateddeps were provided | ported | [`crates/renovate-core/src/extractors/pip_artifact_runner.rs:217`](../../../../../../../crates/renovate-core/src/extractors/pip_artifact_runner.rs#L217) |
| 62 | returns null if no hashes | ported | [`crates/renovate-core/src/extractors/pip_artifact_runner.rs:235`](../../../../../../../crates/renovate-core/src/extractors/pip_artifact_runner.rs#L235) |
| 74 | returns null if unchanged | ported | [`crates/renovate-core/src/extractors/pip_artifact_runner.rs:259`](../../../../../../../crates/renovate-core/src/extractors/pip_artifact_runner.rs#L259) |
| 98 | returns updated file | ported | [`crates/renovate-core/src/extractors/pip_artifact_runner.rs:302`](../../../../../../../crates/renovate-core/src/extractors/pip_artifact_runner.rs#L302) |
| 130 | ignores falsy depnames | ported | [`crates/renovate-core/src/extractors/pip_artifact_runner.rs:357`](../../../../../../../crates/renovate-core/src/extractors/pip_artifact_runner.rs#L357) |
| 162 | catches and returns errors | ported | [`crates/renovate-core/src/extractors/pip_artifact_runner.rs:412`](../../../../../../../crates/renovate-core/src/extractors/pip_artifact_runner.rs#L412) |
| 191 | supports docker mode | pending | — |
| 245 | supports install mode | pending | — |

