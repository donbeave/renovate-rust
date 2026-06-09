# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | supports ports | ported | [`crates/renovate-core/src/util.rs:8349`](../../../../../../crates/renovate-core/src/util.rs#L8349) |
| 40 | returns https url for git url | ported | [`crates/renovate-core/src/util.rs:8364`](../../../../../../crates/renovate-core/src/util.rs#L8364) |
| 44 | returns https url for https url | ported | [`crates/renovate-core/src/util.rs:8370`](../../../../../../crates/renovate-core/src/util.rs#L8370) |
| 48 | returns http url for http url | ported | [`crates/renovate-core/src/util.rs:8376`](../../../../../../crates/renovate-core/src/util.rs#L8376) |
| 52 | returns http url for ssh url with port | ported | [`crates/renovate-core/src/util.rs:8382`](../../../../../../crates/renovate-core/src/util.rs#L8382) |
| 60 | returns gitlab url with token | ported | [`crates/renovate-core/src/util.rs:8394`](../../../../../../crates/renovate-core/src/util.rs#L8394) |
| 75 | returns github url with token | ported | [`crates/renovate-core/src/util.rs:8403`](../../../../../../crates/renovate-core/src/util.rs#L8403) |
| 90 | returns bitbucket-server url | ported | [`crates/renovate-core/src/util.rs:8412`](../../../../../../crates/renovate-core/src/util.rs#L8412) |
| 100 | removes username/password from url | ported | [`crates/renovate-core/src/util.rs:8433`](../../../../../../crates/renovate-core/src/util.rs#L8433) |
| 106 | replaces username/password with given token | ported | [`crates/renovate-core/src/util.rs:8442`](../../../../../../crates/renovate-core/src/util.rs#L8442) |
| 117 | returns original url if no host rule is found | ported | [`crates/renovate-core/src/util.rs:8471`](../../../../../../crates/renovate-core/src/util.rs#L8471) |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | [`crates/renovate-core/src/util.rs:8481`](../../../../../../crates/renovate-core/src/util.rs#L8481) |
| 132 | does not transform urls that are not parseable as git urls | ported | [`crates/renovate-core/src/util.rs:8500`](../../../../../../crates/renovate-core/src/util.rs#L8500) |
| 141 | returns http url with token | ported | [`crates/renovate-core/src/util.rs:8516`](../../../../../../crates/renovate-core/src/util.rs#L8516) |
| 148 | returns https url with token | ported | [`crates/renovate-core/src/util.rs:8535`](../../../../../../crates/renovate-core/src/util.rs#L8535) |
| 155 | returns https url with token for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8554`](../../../../../../crates/renovate-core/src/util.rs#L8554) |
| 162 | returns https url with encoded token | ported | [`crates/renovate-core/src/util.rs:8573`](../../../../../../crates/renovate-core/src/util.rs#L8573) |
| 169 | returns http url with username and password | ported | [`crates/renovate-core/src/util.rs:8592`](../../../../../../crates/renovate-core/src/util.rs#L8592) |
| 179 | returns https url with username and password | ported | [`crates/renovate-core/src/util.rs:8611`](../../../../../../crates/renovate-core/src/util.rs#L8611) |
| 189 | returns https url with username and password for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8630`](../../../../../../crates/renovate-core/src/util.rs#L8630) |
| 199 | returns https url with encoded username and password | ported | [`crates/renovate-core/src/util.rs:8649`](../../../../../../crates/renovate-core/src/util.rs#L8649) |
| 209 | returns https url with encoded gitlab token | ported | [`crates/renovate-core/src/util.rs:8668`](../../../../../../crates/renovate-core/src/util.rs#L8668) |
| 218 | returns https url for ssh url with encoded github token | ported | [`crates/renovate-core/src/util.rs:8687`](../../../../../../crates/renovate-core/src/util.rs#L8687) |

