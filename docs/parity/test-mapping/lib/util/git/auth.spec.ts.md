# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns url with token | ported | [`crates/renovate-core/src/util.rs:8818`](../../../../../../crates/renovate-core/src/util.rs#L8818) |
| 31 | returns url with username and password | ported | [`crates/renovate-core/src/util.rs:8849`](../../../../../../crates/renovate-core/src/util.rs#L8849) |
| 53 | prefers token over username and password | ported | [`crates/renovate-core/src/util.rs:8888`](../../../../../../crates/renovate-core/src/util.rs#L8888) |
| 73 | returns url with token for different protocols | ported | [`crates/renovate-core/src/util.rs:8912`](../../../../../../crates/renovate-core/src/util.rs#L8912) |
| 91 | returns correct url if token already contains github app username | ported | [`crates/renovate-core/src/util.rs:8931`](../../../../../../crates/renovate-core/src/util.rs#L8931) |
| 112 | returns url with token and already existing git_config_count from parameter | ported | [`crates/renovate-core/src/util.rs:8954`](../../../../../../crates/renovate-core/src/util.rs#L8954) |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | [`crates/renovate-core/src/util.rs:8985`](../../../../../../crates/renovate-core/src/util.rs#L8985) |
| 157 | returns url with token and already existing git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:9001`](../../../../../../crates/renovate-core/src/util.rs#L9001) |
| 176 | returns url with token and passthrough existing variables | ported | [`crates/renovate-core/src/util.rs:9017`](../../../../../../crates/renovate-core/src/util.rs#L9017) |
| 199 | return url with token with invalid git_config_count from environment | ported | [`crates/renovate-core/src/util.rs:9033`](../../../../../../crates/renovate-core/src/util.rs#L9033) |
| 218 | returns url with token containing username for gitlab token | ported | [`crates/renovate-core/src/util.rs:9049`](../../../../../../crates/renovate-core/src/util.rs#L9049) |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | [`crates/renovate-core/src/util.rs:9068`](../../../../../../crates/renovate-core/src/util.rs#L9068) |
| 259 | returns original environment variables when no token is set | ported | [`crates/renovate-core/src/util.rs:9099`](../../../../../../crates/renovate-core/src/util.rs#L9099) |
| 274 | returns url with token for http hosts | ported | [`crates/renovate-core/src/util.rs:9111`](../../../../../../crates/renovate-core/src/util.rs#L9111) |
| 292 | returns url with token for orgs | ported | [`crates/renovate-core/src/util.rs:9130`](../../../../../../crates/renovate-core/src/util.rs#L9130) |
| 310 | returns url with token for orgs and projects | ported | [`crates/renovate-core/src/util.rs:9149`](../../../../../../crates/renovate-core/src/util.rs#L9149) |
| 330 | returns url with token for orgs and projects and ports | ported | [`crates/renovate-core/src/util.rs:9168`](../../../../../../crates/renovate-core/src/util.rs#L9168) |
| 354 | returns url with token for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9187`](../../../../../../crates/renovate-core/src/util.rs#L9187) |
| 381 | returns empty object if no environment variables exist | ported | [`crates/renovate-core/src/util.rs:9222`](../../../../../../crates/renovate-core/src/util.rs#L9222) |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | [`crates/renovate-core/src/util.rs:9229`](../../../../../../crates/renovate-core/src/util.rs#L9229) |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | [`crates/renovate-core/src/util.rs:9264`](../../../../../../crates/renovate-core/src/util.rs#L9264) |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | [`crates/renovate-core/src/util.rs:9300`](../../../../../../crates/renovate-core/src/util.rs#L9300) |
| 466 | returns environment variables with username and password | ported | [`crates/renovate-core/src/util.rs:9323`](../../../../../../crates/renovate-core/src/util.rs#L9323) |
| 487 | returns environment variables with url encoded username and password | ported | [`crates/renovate-core/src/util.rs:9345`](../../../../../../crates/renovate-core/src/util.rs#L9345) |
| 508 | returns no environment variables when hosttype is not supported | ported | [`crates/renovate-core/src/util.rs:9368`](../../../../../../crates/renovate-core/src/util.rs#L9368) |
| 517 | returns no environment variables when only username is set | ported | [`crates/renovate-core/src/util.rs:9382`](../../../../../../crates/renovate-core/src/util.rs#L9382) |
| 526 | returns no environment variables when only password is set | ported | [`crates/renovate-core/src/util.rs:9397`](../../../../../../crates/renovate-core/src/util.rs#L9397) |
| 535 | returns environment variables when hosttype is explicitly set | ported | [`crates/renovate-core/src/util.rs:9412`](../../../../../../crates/renovate-core/src/util.rs#L9412) |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | [`crates/renovate-core/src/util.rs:9447`](../../../../../../crates/renovate-core/src/util.rs#L9447) |
| 563 | returns environment variables for bitbucket-server | ported | [`crates/renovate-core/src/util.rs:9461`](../../../../../../crates/renovate-core/src/util.rs#L9461) |

