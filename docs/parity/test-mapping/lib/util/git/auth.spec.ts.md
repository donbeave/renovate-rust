# `lib/util/git/auth.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**30/30 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | returns url with token | ported | `crates/renovate-core/src/util.rs:7202` |
| 31 | returns url with username and password | ported | `crates/renovate-core/src/util.rs:7233` |
| 53 | prefers token over username and password | ported | `crates/renovate-core/src/util.rs:7272` |
| 73 | returns url with token for different protocols | ported | `crates/renovate-core/src/util.rs:7296` |
| 91 | returns correct url if token already contains github app username | ported | `crates/renovate-core/src/util.rs:7315` |
| 112 | returns url with token and already existing git_config_count from parameter | ported | `crates/renovate-core/src/util.rs:7338` |
| 134 | returns url with token and already existing git_config_count from parameter over environment | ported | `crates/renovate-core/src/util.rs:7369` |
| 157 | returns url with token and already existing git_config_count from environment | ported | `crates/renovate-core/src/util.rs:7385` |
| 176 | returns url with token and passthrough existing variables | ported | `crates/renovate-core/src/util.rs:7401` |
| 199 | return url with token with invalid git_config_count from environment | ported | `crates/renovate-core/src/util.rs:7417` |
| 218 | returns url with token containing username for gitlab token | ported | `crates/renovate-core/src/util.rs:7433` |
| 239 | returns url with token containing username for gitlab token without hosttype | ported | `crates/renovate-core/src/util.rs:7452` |
| 259 | returns original environment variables when no token is set | ported | `crates/renovate-core/src/util.rs:7483` |
| 274 | returns url with token for http hosts | ported | `crates/renovate-core/src/util.rs:7495` |
| 292 | returns url with token for orgs | ported | `crates/renovate-core/src/util.rs:7514` |
| 310 | returns url with token for orgs and projects | ported | `crates/renovate-core/src/util.rs:7533` |
| 330 | returns url with token for orgs and projects and ports | ported | `crates/renovate-core/src/util.rs:7552` |
| 354 | returns url with token for bitbucket-server | ported | `crates/renovate-core/src/util.rs:7571` |
| 381 | returns empty object if no environment variables exist | ported | `crates/renovate-core/src/util.rs:7606` |
| 385 | returns environment variables with token if hostrule for api.github.com exists | ported | `crates/renovate-core/src/util.rs:7613` |
| 402 | returns environment variables with token if hostrule for multiple hostsrules | ported | `crates/renovate-core/src/util.rs:7648` |
| 446 | returns environment variables with token if hostrule is for gitlab | ported | `crates/renovate-core/src/util.rs:7684` |
| 466 | returns environment variables with username and password | ported | `crates/renovate-core/src/util.rs:7707` |
| 487 | returns environment variables with url encoded username and password | ported | `crates/renovate-core/src/util.rs:7729` |
| 508 | returns no environment variables when hosttype is not supported | ported | `crates/renovate-core/src/util.rs:7752` |
| 517 | returns no environment variables when only username is set | ported | `crates/renovate-core/src/util.rs:7766` |
| 526 | returns no environment variables when only password is set | ported | `crates/renovate-core/src/util.rs:7781` |
| 535 | returns environment variables when hosttype is explicitly set | ported | `crates/renovate-core/src/util.rs:7796` |
| 554 | returns empty environment variables when matchhost contains invalid protocol | ported | `crates/renovate-core/src/util.rs:7831` |
| 563 | returns environment variables for bitbucket-server | ported | `crates/renovate-core/src/util.rs:7845` |

