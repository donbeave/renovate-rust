# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns url with token | ported | [`crates/renovate-core/src/util.rs:8723`](../../../../../../crates/renovate-core/src/util.rs#L8723) |
| 31 | returns url with username and password | ported | [`crates/renovate-core/src/util.rs:8754`](../../../../../../crates/renovate-core/src/util.rs#L8754) |
| 53 | prefers token over username and password | ported | [`crates/renovate-core/src/util.rs:8793`](../../../../../../crates/renovate-core/src/util.rs#L8793) |
| 73 | returns url with token for different protocols | ported | [`crates/renovate-core/src/util.rs:8817`](../../../../../../crates/renovate-core/src/util.rs#L8817) |
| 91 | returns correct url if token already contains github app username | ported | [`crates/renovate-core/src/util.rs:8836`](../../../../../../crates/renovate-core/src/util.rs#L8836) |
| 112 | returns url with token and already existing git_config_count from parameter | ported | [`crates/renovate-core/src/util.rs:8859`](../../../../../../crates/renovate-core/src/util.rs#L8859) |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | [`crates/renovate-core/src/util.rs:8890`](../../../../../../crates/renovate-core/src/util.rs#L8890) |
| 157 | returns url with token and already existing git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8906`](../../../../../../crates/renovate-core/src/util.rs#L8906) |
| 176 | returns url with token and passthrough existing variables | ported | [`crates/renovate-core/src/util.rs:8922`](../../../../../../crates/renovate-core/src/util.rs#L8922) |
| 199 | return url with token with invalid git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8938`](../../../../../../crates/renovate-core/src/util.rs#L8938) |
| 218 | returns url with token containing username for gitlab token | ported | [`crates/renovate-core/src/util.rs:8954`](../../../../../../crates/renovate-core/src/util.rs#L8954) |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | [`crates/renovate-core/src/util.rs:8973`](../../../../../../crates/renovate-core/src/util.rs#L8973) |
| 259 | returns original environment variables when no token is set | ported | [`crates/renovate-core/src/util.rs:9004`](../../../../../../crates/renovate-core/src/util.rs#L9004) |
| 274 | returns url with token for http hosts | ported | [`crates/renovate-core/src/util.rs:9016`](../../../../../../crates/renovate-core/src/util.rs#L9016) |
| 292 | returns url with token for orgs | ported | [`crates/renovate-core/src/util.rs:9035`](../../../../../../crates/renovate-core/src/util.rs#L9035) |
| 310 | returns url with token for orgs and projects | ported | [`crates/renovate-core/src/util.rs:9054`](../../../../../../crates/renovate-core/src/util.rs#L9054) |
| 330 | returns url with token for orgs and projects and ports | ported | [`crates/renovate-core/src/util.rs:9073`](../../../../../../crates/renovate-core/src/util.rs#L9073) |
| 354 | returns url with token for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9092`](../../../../../../crates/renovate-core/src/util.rs#L9092) |
| 381 | returns empty object if no environment variables exist | ported | [`crates/renovate-core/src/util.rs:9127`](../../../../../../crates/renovate-core/src/util.rs#L9127) |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | [`crates/renovate-core/src/util.rs:9134`](../../../../../../crates/renovate-core/src/util.rs#L9134) |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | [`crates/renovate-core/src/util.rs:9169`](../../../../../../crates/renovate-core/src/util.rs#L9169) |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | [`crates/renovate-core/src/util.rs:9205`](../../../../../../crates/renovate-core/src/util.rs#L9205) |
| 466 | returns environment variables with username and password | ported | [`crates/renovate-core/src/util.rs:9228`](../../../../../../crates/renovate-core/src/util.rs#L9228) |
| 487 | returns environment variables with url encoded username and password | ported | [`crates/renovate-core/src/util.rs:9250`](../../../../../../crates/renovate-core/src/util.rs#L9250) |
| 508 | returns no environment variables when hosttype is not supported | ported | [`crates/renovate-core/src/util.rs:9273`](../../../../../../crates/renovate-core/src/util.rs#L9273) |
| 517 | returns no environment variables when only username is set | ported | [`crates/renovate-core/src/util.rs:9287`](../../../../../../crates/renovate-core/src/util.rs#L9287) |
| 526 | returns no environment variables when only password is set | ported | [`crates/renovate-core/src/util.rs:9302`](../../../../../../crates/renovate-core/src/util.rs#L9302) |
| 535 | returns environment variables when hosttype is explicitly set | ported | [`crates/renovate-core/src/util.rs:9317`](../../../../../../crates/renovate-core/src/util.rs#L9317) |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | [`crates/renovate-core/src/util.rs:9352`](../../../../../../crates/renovate-core/src/util.rs#L9352) |
| 563 | returns environment variables for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9366`](../../../../../../crates/renovate-core/src/util.rs#L9366) |

