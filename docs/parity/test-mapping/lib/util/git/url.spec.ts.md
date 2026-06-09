# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | supports ports | ported | [`crates/renovate-core/src/util.rs:8346`](../../../../../../crates/renovate-core/src/util.rs#L8346) |
| 40 | returns https url for git url | ported | [`crates/renovate-core/src/util.rs:8361`](../../../../../../crates/renovate-core/src/util.rs#L8361) |
| 44 | returns https url for https url | ported | [`crates/renovate-core/src/util.rs:8367`](../../../../../../crates/renovate-core/src/util.rs#L8367) |
| 48 | returns http url for http url | ported | [`crates/renovate-core/src/util.rs:8373`](../../../../../../crates/renovate-core/src/util.rs#L8373) |
| 52 | returns http url for ssh url with port | ported | [`crates/renovate-core/src/util.rs:8379`](../../../../../../crates/renovate-core/src/util.rs#L8379) |
| 60 | returns gitlab url with token | ported | [`crates/renovate-core/src/util.rs:8391`](../../../../../../crates/renovate-core/src/util.rs#L8391) |
| 75 | returns github url with token | ported | [`crates/renovate-core/src/util.rs:8400`](../../../../../../crates/renovate-core/src/util.rs#L8400) |
| 90 | returns bitbucket-server url | ported | [`crates/renovate-core/src/util.rs:8409`](../../../../../../crates/renovate-core/src/util.rs#L8409) |
| 100 | removes username/password from url | ported | [`crates/renovate-core/src/util.rs:8430`](../../../../../../crates/renovate-core/src/util.rs#L8430) |
| 106 | replaces username/password with given token | ported | [`crates/renovate-core/src/util.rs:8439`](../../../../../../crates/renovate-core/src/util.rs#L8439) |
| 117 | returns original url if no host rule is found | ported | [`crates/renovate-core/src/util.rs:8468`](../../../../../../crates/renovate-core/src/util.rs#L8468) |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | [`crates/renovate-core/src/util.rs:8478`](../../../../../../crates/renovate-core/src/util.rs#L8478) |
| 132 | does not transform urls that are not parseable as git urls | ported | [`crates/renovate-core/src/util.rs:8497`](../../../../../../crates/renovate-core/src/util.rs#L8497) |
| 141 | returns http url with token | ported | [`crates/renovate-core/src/util.rs:8513`](../../../../../../crates/renovate-core/src/util.rs#L8513) |
| 148 | returns https url with token | ported | [`crates/renovate-core/src/util.rs:8532`](../../../../../../crates/renovate-core/src/util.rs#L8532) |
| 155 | returns https url with token for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8551`](../../../../../../crates/renovate-core/src/util.rs#L8551) |
| 162 | returns https url with encoded token | ported | [`crates/renovate-core/src/util.rs:8570`](../../../../../../crates/renovate-core/src/util.rs#L8570) |
| 169 | returns http url with username and password | ported | [`crates/renovate-core/src/util.rs:8589`](../../../../../../crates/renovate-core/src/util.rs#L8589) |
| 179 | returns https url with username and password | ported | [`crates/renovate-core/src/util.rs:8608`](../../../../../../crates/renovate-core/src/util.rs#L8608) |
| 189 | returns https url with username and password for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8627`](../../../../../../crates/renovate-core/src/util.rs#L8627) |
| 199 | returns https url with encoded username and password | ported | [`crates/renovate-core/src/util.rs:8646`](../../../../../../crates/renovate-core/src/util.rs#L8646) |
| 209 | returns https url with encoded gitlab token | ported | [`crates/renovate-core/src/util.rs:8665`](../../../../../../crates/renovate-core/src/util.rs#L8665) |
| 218 | returns https url for ssh url with encoded github token | ported | [`crates/renovate-core/src/util.rs:8684`](../../../../../../crates/renovate-core/src/util.rs#L8684) |

