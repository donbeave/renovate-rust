# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns url with token | ported | [`crates/renovate-core/src/util.rs:8720`](../../../../../../crates/renovate-core/src/util.rs#L8720) |
| 31 | returns url with username and password | ported | [`crates/renovate-core/src/util.rs:8751`](../../../../../../crates/renovate-core/src/util.rs#L8751) |
| 53 | prefers token over username and password | ported | [`crates/renovate-core/src/util.rs:8790`](../../../../../../crates/renovate-core/src/util.rs#L8790) |
| 73 | returns url with token for different protocols | ported | [`crates/renovate-core/src/util.rs:8814`](../../../../../../crates/renovate-core/src/util.rs#L8814) |
| 91 | returns correct url if token already contains github app username | ported | [`crates/renovate-core/src/util.rs:8833`](../../../../../../crates/renovate-core/src/util.rs#L8833) |
| 112 | returns url with token and already existing git_config_count from parameter | ported | [`crates/renovate-core/src/util.rs:8856`](../../../../../../crates/renovate-core/src/util.rs#L8856) |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | [`crates/renovate-core/src/util.rs:8887`](../../../../../../crates/renovate-core/src/util.rs#L8887) |
| 157 | returns url with token and already existing git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8903`](../../../../../../crates/renovate-core/src/util.rs#L8903) |
| 176 | returns url with token and passthrough existing variables | ported | [`crates/renovate-core/src/util.rs:8919`](../../../../../../crates/renovate-core/src/util.rs#L8919) |
| 199 | return url with token with invalid git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8935`](../../../../../../crates/renovate-core/src/util.rs#L8935) |
| 218 | returns url with token containing username for gitlab token | ported | [`crates/renovate-core/src/util.rs:8951`](../../../../../../crates/renovate-core/src/util.rs#L8951) |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | [`crates/renovate-core/src/util.rs:8970`](../../../../../../crates/renovate-core/src/util.rs#L8970) |
| 259 | returns original environment variables when no token is set | ported | [`crates/renovate-core/src/util.rs:9001`](../../../../../../crates/renovate-core/src/util.rs#L9001) |
| 274 | returns url with token for http hosts | ported | [`crates/renovate-core/src/util.rs:9013`](../../../../../../crates/renovate-core/src/util.rs#L9013) |
| 292 | returns url with token for orgs | ported | [`crates/renovate-core/src/util.rs:9032`](../../../../../../crates/renovate-core/src/util.rs#L9032) |
| 310 | returns url with token for orgs and projects | ported | [`crates/renovate-core/src/util.rs:9051`](../../../../../../crates/renovate-core/src/util.rs#L9051) |
| 330 | returns url with token for orgs and projects and ports | ported | [`crates/renovate-core/src/util.rs:9070`](../../../../../../crates/renovate-core/src/util.rs#L9070) |
| 354 | returns url with token for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9089`](../../../../../../crates/renovate-core/src/util.rs#L9089) |
| 381 | returns empty object if no environment variables exist | ported | [`crates/renovate-core/src/util.rs:9124`](../../../../../../crates/renovate-core/src/util.rs#L9124) |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | [`crates/renovate-core/src/util.rs:9131`](../../../../../../crates/renovate-core/src/util.rs#L9131) |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | [`crates/renovate-core/src/util.rs:9166`](../../../../../../crates/renovate-core/src/util.rs#L9166) |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | [`crates/renovate-core/src/util.rs:9202`](../../../../../../crates/renovate-core/src/util.rs#L9202) |
| 466 | returns environment variables with username and password | ported | [`crates/renovate-core/src/util.rs:9225`](../../../../../../crates/renovate-core/src/util.rs#L9225) |
| 487 | returns environment variables with url encoded username and password | ported | [`crates/renovate-core/src/util.rs:9247`](../../../../../../crates/renovate-core/src/util.rs#L9247) |
| 508 | returns no environment variables when hosttype is not supported | ported | [`crates/renovate-core/src/util.rs:9270`](../../../../../../crates/renovate-core/src/util.rs#L9270) |
| 517 | returns no environment variables when only username is set | ported | [`crates/renovate-core/src/util.rs:9284`](../../../../../../crates/renovate-core/src/util.rs#L9284) |
| 526 | returns no environment variables when only password is set | ported | [`crates/renovate-core/src/util.rs:9299`](../../../../../../crates/renovate-core/src/util.rs#L9299) |
| 535 | returns environment variables when hosttype is explicitly set | ported | [`crates/renovate-core/src/util.rs:9314`](../../../../../../crates/renovate-core/src/util.rs#L9314) |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | [`crates/renovate-core/src/util.rs:9349`](../../../../../../crates/renovate-core/src/util.rs#L9349) |
| 563 | returns environment variables for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9363`](../../../../../../crates/renovate-core/src/util.rs#L9363) |

