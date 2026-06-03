# `lib/modules/platform/github/scm.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | platformcommit = disabled => delegate to git | ported | [`crates/renovate-core/src/platform/scm.rs:161`](../../../../../../../crates/renovate-core/src/platform/scm.rs#L161) |
| 39 | platformcommit = enabled => delegate to github | ported | [`crates/renovate-core/src/platform/scm.rs:171`](../../../../../../../crates/renovate-core/src/platform/scm.rs#L171) |
| 52 | platformcommit = auto => delegate to git | ported | [`crates/renovate-core/src/platform/scm.rs:178`](../../../../../../../crates/renovate-core/src/platform/scm.rs#L178) |
| 65 | platformcommit = auto and is a github app => delegate to github | ported | [`crates/renovate-core/src/platform/scm.rs:184`](../../../../../../../crates/renovate-core/src/platform/scm.rs#L184) |

