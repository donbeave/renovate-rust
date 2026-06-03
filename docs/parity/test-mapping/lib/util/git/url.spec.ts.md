# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | supports ports | ported | `crates/renovate-core/src/util.rs:6825` |
| 40 | returns https url for git url | ported | `crates/renovate-core/src/util.rs:6840` |
| 44 | returns https url for https url | ported | `crates/renovate-core/src/util.rs:6846` |
| 48 | returns http url for http url | ported | `crates/renovate-core/src/util.rs:6852` |
| 52 | returns http url for ssh url with port | ported | `crates/renovate-core/src/util.rs:6858` |
| 60 | returns gitlab url with token | ported | `crates/renovate-core/src/util.rs:6870` |
| 75 | returns github url with token | ported | `crates/renovate-core/src/util.rs:6879` |
| 90 | returns bitbucket-server url | ported | `crates/renovate-core/src/util.rs:6888` |
| 100 | removes username/password from url | ported | `crates/renovate-core/src/util.rs:6909` |
| 106 | replaces username/password with given token | ported | `crates/renovate-core/src/util.rs:6918` |
| 117 | returns original url if no host rule is found | ported | `crates/renovate-core/src/util.rs:6947` |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | `crates/renovate-core/src/util.rs:6957` |
| 132 | does not transform urls that are not parseable as git urls | ported | `crates/renovate-core/src/util.rs:6976` |
| 141 | returns http url with token | ported | `crates/renovate-core/src/util.rs:6992` |
| 148 | returns https url with token | ported | `crates/renovate-core/src/util.rs:7011` |
| 155 | returns https url with token for non-http protocols | ported | `crates/renovate-core/src/util.rs:7030` |
| 162 | returns https url with encoded token | ported | `crates/renovate-core/src/util.rs:7049` |
| 169 | returns http url with username and password | ported | `crates/renovate-core/src/util.rs:7068` |
| 179 | returns https url with username and password | ported | `crates/renovate-core/src/util.rs:7087` |
| 189 | returns https url with username and password for non-http protocols | ported | `crates/renovate-core/src/util.rs:7106` |
| 199 | returns https url with encoded username and password | ported | `crates/renovate-core/src/util.rs:7125` |
| 209 | returns https url with encoded gitlab token | ported | `crates/renovate-core/src/util.rs:7144` |
| 218 | returns https url for ssh url with encoded github token | ported | `crates/renovate-core/src/util.rs:7163` |

