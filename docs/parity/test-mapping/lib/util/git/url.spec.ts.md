# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | supports ports | ported | [`crates/renovate-core/src/util.rs:8345`](../../../../../../crates/renovate-core/src/util.rs#L8345) |
| 40 | returns https url for git url | ported | [`crates/renovate-core/src/util.rs:8360`](../../../../../../crates/renovate-core/src/util.rs#L8360) |
| 44 | returns https url for https url | ported | [`crates/renovate-core/src/util.rs:8366`](../../../../../../crates/renovate-core/src/util.rs#L8366) |
| 48 | returns http url for http url | ported | [`crates/renovate-core/src/util.rs:8372`](../../../../../../crates/renovate-core/src/util.rs#L8372) |
| 52 | returns http url for ssh url with port | ported | [`crates/renovate-core/src/util.rs:8378`](../../../../../../crates/renovate-core/src/util.rs#L8378) |
| 60 | returns gitlab url with token | ported | [`crates/renovate-core/src/util.rs:8390`](../../../../../../crates/renovate-core/src/util.rs#L8390) |
| 75 | returns github url with token | ported | [`crates/renovate-core/src/util.rs:8399`](../../../../../../crates/renovate-core/src/util.rs#L8399) |
| 90 | returns bitbucket-server url | ported | [`crates/renovate-core/src/util.rs:8408`](../../../../../../crates/renovate-core/src/util.rs#L8408) |
| 100 | removes username/password from url | ported | [`crates/renovate-core/src/util.rs:8429`](../../../../../../crates/renovate-core/src/util.rs#L8429) |
| 106 | replaces username/password with given token | ported | [`crates/renovate-core/src/util.rs:8438`](../../../../../../crates/renovate-core/src/util.rs#L8438) |
| 117 | returns original url if no host rule is found | ported | [`crates/renovate-core/src/util.rs:8467`](../../../../../../crates/renovate-core/src/util.rs#L8467) |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | [`crates/renovate-core/src/util.rs:8477`](../../../../../../crates/renovate-core/src/util.rs#L8477) |
| 132 | does not transform urls that are not parseable as git urls | ported | [`crates/renovate-core/src/util.rs:8496`](../../../../../../crates/renovate-core/src/util.rs#L8496) |
| 141 | returns http url with token | ported | [`crates/renovate-core/src/util.rs:8512`](../../../../../../crates/renovate-core/src/util.rs#L8512) |
| 148 | returns https url with token | ported | [`crates/renovate-core/src/util.rs:8531`](../../../../../../crates/renovate-core/src/util.rs#L8531) |
| 155 | returns https url with token for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8550`](../../../../../../crates/renovate-core/src/util.rs#L8550) |
| 162 | returns https url with encoded token | ported | [`crates/renovate-core/src/util.rs:8569`](../../../../../../crates/renovate-core/src/util.rs#L8569) |
| 169 | returns http url with username and password | ported | [`crates/renovate-core/src/util.rs:8588`](../../../../../../crates/renovate-core/src/util.rs#L8588) |
| 179 | returns https url with username and password | ported | [`crates/renovate-core/src/util.rs:8607`](../../../../../../crates/renovate-core/src/util.rs#L8607) |
| 189 | returns https url with username and password for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8626`](../../../../../../crates/renovate-core/src/util.rs#L8626) |
| 199 | returns https url with encoded username and password | ported | [`crates/renovate-core/src/util.rs:8645`](../../../../../../crates/renovate-core/src/util.rs#L8645) |
| 209 | returns https url with encoded gitlab token | ported | [`crates/renovate-core/src/util.rs:8664`](../../../../../../crates/renovate-core/src/util.rs#L8664) |
| 218 | returns https url for ssh url with encoded github token | ported | [`crates/renovate-core/src/util.rs:8683`](../../../../../../crates/renovate-core/src/util.rs#L8683) |

