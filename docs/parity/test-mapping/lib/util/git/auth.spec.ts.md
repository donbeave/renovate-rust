# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns url with token | ported | [`crates/renovate-core/src/util.rs:8726`](../../../../../../crates/renovate-core/src/util.rs#L8726) |
| 31 | returns url with username and password | ported | [`crates/renovate-core/src/util.rs:8757`](../../../../../../crates/renovate-core/src/util.rs#L8757) |
| 53 | prefers token over username and password | ported | [`crates/renovate-core/src/util.rs:8796`](../../../../../../crates/renovate-core/src/util.rs#L8796) |
| 73 | returns url with token for different protocols | ported | [`crates/renovate-core/src/util.rs:8820`](../../../../../../crates/renovate-core/src/util.rs#L8820) |
| 91 | returns correct url if token already contains github app username | ported | [`crates/renovate-core/src/util.rs:8839`](../../../../../../crates/renovate-core/src/util.rs#L8839) |
| 112 | returns url with token and already existing git_config_count from parameter | ported | [`crates/renovate-core/src/util.rs:8862`](../../../../../../crates/renovate-core/src/util.rs#L8862) |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | [`crates/renovate-core/src/util.rs:8893`](../../../../../../crates/renovate-core/src/util.rs#L8893) |
| 157 | returns url with token and already existing git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8909`](../../../../../../crates/renovate-core/src/util.rs#L8909) |
| 176 | returns url with token and passthrough existing variables | ported | [`crates/renovate-core/src/util.rs:8925`](../../../../../../crates/renovate-core/src/util.rs#L8925) |
| 199 | return url with token with invalid git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:8941`](../../../../../../crates/renovate-core/src/util.rs#L8941) |
| 218 | returns url with token containing username for gitlab token | ported | [`crates/renovate-core/src/util.rs:8957`](../../../../../../crates/renovate-core/src/util.rs#L8957) |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | [`crates/renovate-core/src/util.rs:8976`](../../../../../../crates/renovate-core/src/util.rs#L8976) |
| 259 | returns original environment variables when no token is set | ported | [`crates/renovate-core/src/util.rs:9007`](../../../../../../crates/renovate-core/src/util.rs#L9007) |
| 274 | returns url with token for http hosts | ported | [`crates/renovate-core/src/util.rs:9019`](../../../../../../crates/renovate-core/src/util.rs#L9019) |
| 292 | returns url with token for orgs | ported | [`crates/renovate-core/src/util.rs:9038`](../../../../../../crates/renovate-core/src/util.rs#L9038) |
| 310 | returns url with token for orgs and projects | ported | [`crates/renovate-core/src/util.rs:9057`](../../../../../../crates/renovate-core/src/util.rs#L9057) |
| 330 | returns url with token for orgs and projects and ports | ported | [`crates/renovate-core/src/util.rs:9076`](../../../../../../crates/renovate-core/src/util.rs#L9076) |
| 354 | returns url with token for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9095`](../../../../../../crates/renovate-core/src/util.rs#L9095) |
| 381 | returns empty object if no environment variables exist | ported | [`crates/renovate-core/src/util.rs:9130`](../../../../../../crates/renovate-core/src/util.rs#L9130) |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | [`crates/renovate-core/src/util.rs:9137`](../../../../../../crates/renovate-core/src/util.rs#L9137) |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | [`crates/renovate-core/src/util.rs:9172`](../../../../../../crates/renovate-core/src/util.rs#L9172) |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | [`crates/renovate-core/src/util.rs:9208`](../../../../../../crates/renovate-core/src/util.rs#L9208) |
| 466 | returns environment variables with username and password | ported | [`crates/renovate-core/src/util.rs:9231`](../../../../../../crates/renovate-core/src/util.rs#L9231) |
| 487 | returns environment variables with url encoded username and password | ported | [`crates/renovate-core/src/util.rs:9253`](../../../../../../crates/renovate-core/src/util.rs#L9253) |
| 508 | returns no environment variables when hosttype is not supported | ported | [`crates/renovate-core/src/util.rs:9276`](../../../../../../crates/renovate-core/src/util.rs#L9276) |
| 517 | returns no environment variables when only username is set | ported | [`crates/renovate-core/src/util.rs:9290`](../../../../../../crates/renovate-core/src/util.rs#L9290) |
| 526 | returns no environment variables when only password is set | ported | [`crates/renovate-core/src/util.rs:9305`](../../../../../../crates/renovate-core/src/util.rs#L9305) |
| 535 | returns environment variables when hosttype is explicitly set | ported | [`crates/renovate-core/src/util.rs:9320`](../../../../../../crates/renovate-core/src/util.rs#L9320) |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | [`crates/renovate-core/src/util.rs:9355`](../../../../../../crates/renovate-core/src/util.rs#L9355) |
| 563 | returns environment variables for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9369`](../../../../../../crates/renovate-core/src/util.rs#L9369) |

