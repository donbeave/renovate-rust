# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | supports ports | ported | [`crates/renovate-core/src/util.rs:8343`](../../../../../../crates/renovate-core/src/util.rs#L8343) |
| 40 | returns https url for git url | ported | [`crates/renovate-core/src/util.rs:8358`](../../../../../../crates/renovate-core/src/util.rs#L8358) |
| 44 | returns https url for https url | ported | [`crates/renovate-core/src/util.rs:8364`](../../../../../../crates/renovate-core/src/util.rs#L8364) |
| 48 | returns http url for http url | ported | [`crates/renovate-core/src/util.rs:8370`](../../../../../../crates/renovate-core/src/util.rs#L8370) |
| 52 | returns http url for ssh url with port | ported | [`crates/renovate-core/src/util.rs:8376`](../../../../../../crates/renovate-core/src/util.rs#L8376) |
| 60 | returns gitlab url with token | ported | [`crates/renovate-core/src/util.rs:8388`](../../../../../../crates/renovate-core/src/util.rs#L8388) |
| 75 | returns github url with token | ported | [`crates/renovate-core/src/util.rs:8397`](../../../../../../crates/renovate-core/src/util.rs#L8397) |
| 90 | returns bitbucket-server url | ported | [`crates/renovate-core/src/util.rs:8406`](../../../../../../crates/renovate-core/src/util.rs#L8406) |
| 100 | removes username/password from url | ported | [`crates/renovate-core/src/util.rs:8427`](../../../../../../crates/renovate-core/src/util.rs#L8427) |
| 106 | replaces username/password with given token | ported | [`crates/renovate-core/src/util.rs:8436`](../../../../../../crates/renovate-core/src/util.rs#L8436) |
| 117 | returns original url if no host rule is found | ported | [`crates/renovate-core/src/util.rs:8465`](../../../../../../crates/renovate-core/src/util.rs#L8465) |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | [`crates/renovate-core/src/util.rs:8475`](../../../../../../crates/renovate-core/src/util.rs#L8475) |
| 132 | does not transform urls that are not parseable as git urls | ported | [`crates/renovate-core/src/util.rs:8494`](../../../../../../crates/renovate-core/src/util.rs#L8494) |
| 141 | returns http url with token | ported | [`crates/renovate-core/src/util.rs:8510`](../../../../../../crates/renovate-core/src/util.rs#L8510) |
| 148 | returns https url with token | ported | [`crates/renovate-core/src/util.rs:8529`](../../../../../../crates/renovate-core/src/util.rs#L8529) |
| 155 | returns https url with token for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8548`](../../../../../../crates/renovate-core/src/util.rs#L8548) |
| 162 | returns https url with encoded token | ported | [`crates/renovate-core/src/util.rs:8567`](../../../../../../crates/renovate-core/src/util.rs#L8567) |
| 169 | returns http url with username and password | ported | [`crates/renovate-core/src/util.rs:8586`](../../../../../../crates/renovate-core/src/util.rs#L8586) |
| 179 | returns https url with username and password | ported | [`crates/renovate-core/src/util.rs:8605`](../../../../../../crates/renovate-core/src/util.rs#L8605) |
| 189 | returns https url with username and password for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8624`](../../../../../../crates/renovate-core/src/util.rs#L8624) |
| 199 | returns https url with encoded username and password | ported | [`crates/renovate-core/src/util.rs:8643`](../../../../../../crates/renovate-core/src/util.rs#L8643) |
| 209 | returns https url with encoded gitlab token | ported | [`crates/renovate-core/src/util.rs:8662`](../../../../../../crates/renovate-core/src/util.rs#L8662) |
| 218 | returns https url for ssh url with encoded github token | ported | [`crates/renovate-core/src/util.rs:8681`](../../../../../../crates/renovate-core/src/util.rs#L8681) |

