# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns url with token | ported | [`crates/renovate-core/src/util.rs:8721`](../../../../../../crates/renovate-core/src/util.rs#L8721) |
| 31 | returns url with username and password | ported | [`crates/renovate-core/src/util.rs:8752`](../../../../../../crates/renovate-core/src/util.rs#L8752) |
| 53 | prefers token over username and password | ported | [`crates/renovate-core/src/util.rs:8791`](../../../../../../crates/renovate-core/src/util.rs#L8791) |
| 73 | returns url with token for different protocols | ported | [`crates/renovate-core/src/util.rs:8815`](../../../../../../crates/renovate-core/src/util.rs#L8815) |
| 91 | returns correct url if token already contains github app username | ported | [`crates/renovate-core/src/util.rs:8834`](../../../../../../crates/renovate-core/src/util.rs#L8834) |
| 112 | returns url with token and already existing git_config_count from parameter | ported | [`crates/renovate-core/src/util.rs:8857`](../../../../../../crates/renovate-core/src/util.rs#L8857) |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | [`crates/renovate-core/src/util.rs:8888`](../../../../../../crates/renovate-core/src/util.rs#L8888) |
| 157 | returns url with token and already existing git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8904`](../../../../../../crates/renovate-core/src/util.rs#L8904) |
| 176 | returns url with token and passthrough existing variables | ported | [`crates/renovate-core/src/util.rs:8920`](../../../../../../crates/renovate-core/src/util.rs#L8920) |
| 199 | return url with token with invalid git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8936`](../../../../../../crates/renovate-core/src/util.rs#L8936) |
| 218 | returns url with token containing username for gitlab token | ported | [`crates/renovate-core/src/util.rs:8952`](../../../../../../crates/renovate-core/src/util.rs#L8952) |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | [`crates/renovate-core/src/util.rs:8971`](../../../../../../crates/renovate-core/src/util.rs#L8971) |
| 259 | returns original environment variables when no token is set | ported | [`crates/renovate-core/src/util.rs:9002`](../../../../../../crates/renovate-core/src/util.rs#L9002) |
| 274 | returns url with token for http hosts | ported | [`crates/renovate-core/src/util.rs:9014`](../../../../../../crates/renovate-core/src/util.rs#L9014) |
| 292 | returns url with token for orgs | ported | [`crates/renovate-core/src/util.rs:9033`](../../../../../../crates/renovate-core/src/util.rs#L9033) |
| 310 | returns url with token for orgs and projects | ported | [`crates/renovate-core/src/util.rs:9052`](../../../../../../crates/renovate-core/src/util.rs#L9052) |
| 330 | returns url with token for orgs and projects and ports | ported | [`crates/renovate-core/src/util.rs:9071`](../../../../../../crates/renovate-core/src/util.rs#L9071) |
| 354 | returns url with token for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9090`](../../../../../../crates/renovate-core/src/util.rs#L9090) |
| 381 | returns empty object if no environment variables exist | ported | [`crates/renovate-core/src/util.rs:9125`](../../../../../../crates/renovate-core/src/util.rs#L9125) |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | [`crates/renovate-core/src/util.rs:9132`](../../../../../../crates/renovate-core/src/util.rs#L9132) |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | [`crates/renovate-core/src/util.rs:9167`](../../../../../../crates/renovate-core/src/util.rs#L9167) |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | [`crates/renovate-core/src/util.rs:9203`](../../../../../../crates/renovate-core/src/util.rs#L9203) |
| 466 | returns environment variables with username and password | ported | [`crates/renovate-core/src/util.rs:9226`](../../../../../../crates/renovate-core/src/util.rs#L9226) |
| 487 | returns environment variables with url encoded username and password | ported | [`crates/renovate-core/src/util.rs:9248`](../../../../../../crates/renovate-core/src/util.rs#L9248) |
| 508 | returns no environment variables when hosttype is not supported | ported | [`crates/renovate-core/src/util.rs:9271`](../../../../../../crates/renovate-core/src/util.rs#L9271) |
| 517 | returns no environment variables when only username is set | ported | [`crates/renovate-core/src/util.rs:9285`](../../../../../../crates/renovate-core/src/util.rs#L9285) |
| 526 | returns no environment variables when only password is set | ported | [`crates/renovate-core/src/util.rs:9300`](../../../../../../crates/renovate-core/src/util.rs#L9300) |
| 535 | returns environment variables when hosttype is explicitly set | ported | [`crates/renovate-core/src/util.rs:9315`](../../../../../../crates/renovate-core/src/util.rs#L9315) |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | [`crates/renovate-core/src/util.rs:9350`](../../../../../../crates/renovate-core/src/util.rs#L9350) |
| 563 | returns environment variables for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9364`](../../../../../../crates/renovate-core/src/util.rs#L9364) |

