# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns url with token | ported | [`crates/renovate-core/src/util.rs:8722`](../../../../../../crates/renovate-core/src/util.rs#L8722) |
| 31 | returns url with username and password | ported | [`crates/renovate-core/src/util.rs:8753`](../../../../../../crates/renovate-core/src/util.rs#L8753) |
| 53 | prefers token over username and password | ported | [`crates/renovate-core/src/util.rs:8792`](../../../../../../crates/renovate-core/src/util.rs#L8792) |
| 73 | returns url with token for different protocols | ported | [`crates/renovate-core/src/util.rs:8816`](../../../../../../crates/renovate-core/src/util.rs#L8816) |
| 91 | returns correct url if token already contains github app username | ported | [`crates/renovate-core/src/util.rs:8835`](../../../../../../crates/renovate-core/src/util.rs#L8835) |
| 112 | returns url with token and already existing git_config_count from parameter | ported | [`crates/renovate-core/src/util.rs:8858`](../../../../../../crates/renovate-core/src/util.rs#L8858) |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | [`crates/renovate-core/src/util.rs:8889`](../../../../../../crates/renovate-core/src/util.rs#L8889) |
| 157 | returns url with token and already existing git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8905`](../../../../../../crates/renovate-core/src/util.rs#L8905) |
| 176 | returns url with token and passthrough existing variables | ported | [`crates/renovate-core/src/util.rs:8921`](../../../../../../crates/renovate-core/src/util.rs#L8921) |
| 199 | return url with token with invalid git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8937`](../../../../../../crates/renovate-core/src/util.rs#L8937) |
| 218 | returns url with token containing username for gitlab token | ported | [`crates/renovate-core/src/util.rs:8953`](../../../../../../crates/renovate-core/src/util.rs#L8953) |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | [`crates/renovate-core/src/util.rs:8972`](../../../../../../crates/renovate-core/src/util.rs#L8972) |
| 259 | returns original environment variables when no token is set | ported | [`crates/renovate-core/src/util.rs:9003`](../../../../../../crates/renovate-core/src/util.rs#L9003) |
| 274 | returns url with token for http hosts | ported | [`crates/renovate-core/src/util.rs:9015`](../../../../../../crates/renovate-core/src/util.rs#L9015) |
| 292 | returns url with token for orgs | ported | [`crates/renovate-core/src/util.rs:9034`](../../../../../../crates/renovate-core/src/util.rs#L9034) |
| 310 | returns url with token for orgs and projects | ported | [`crates/renovate-core/src/util.rs:9053`](../../../../../../crates/renovate-core/src/util.rs#L9053) |
| 330 | returns url with token for orgs and projects and ports | ported | [`crates/renovate-core/src/util.rs:9072`](../../../../../../crates/renovate-core/src/util.rs#L9072) |
| 354 | returns url with token for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9091`](../../../../../../crates/renovate-core/src/util.rs#L9091) |
| 381 | returns empty object if no environment variables exist | ported | [`crates/renovate-core/src/util.rs:9126`](../../../../../../crates/renovate-core/src/util.rs#L9126) |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | [`crates/renovate-core/src/util.rs:9133`](../../../../../../crates/renovate-core/src/util.rs#L9133) |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | [`crates/renovate-core/src/util.rs:9168`](../../../../../../crates/renovate-core/src/util.rs#L9168) |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | [`crates/renovate-core/src/util.rs:9204`](../../../../../../crates/renovate-core/src/util.rs#L9204) |
| 466 | returns environment variables with username and password | ported | [`crates/renovate-core/src/util.rs:9227`](../../../../../../crates/renovate-core/src/util.rs#L9227) |
| 487 | returns environment variables with url encoded username and password | ported | [`crates/renovate-core/src/util.rs:9249`](../../../../../../crates/renovate-core/src/util.rs#L9249) |
| 508 | returns no environment variables when hosttype is not supported | ported | [`crates/renovate-core/src/util.rs:9272`](../../../../../../crates/renovate-core/src/util.rs#L9272) |
| 517 | returns no environment variables when only username is set | ported | [`crates/renovate-core/src/util.rs:9286`](../../../../../../crates/renovate-core/src/util.rs#L9286) |
| 526 | returns no environment variables when only password is set | ported | [`crates/renovate-core/src/util.rs:9301`](../../../../../../crates/renovate-core/src/util.rs#L9301) |
| 535 | returns environment variables when hosttype is explicitly set | ported | [`crates/renovate-core/src/util.rs:9316`](../../../../../../crates/renovate-core/src/util.rs#L9316) |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | [`crates/renovate-core/src/util.rs:9351`](../../../../../../crates/renovate-core/src/util.rs#L9351) |
| 563 | returns environment variables for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9365`](../../../../../../crates/renovate-core/src/util.rs#L9365) |

