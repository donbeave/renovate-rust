# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | supports ports | ported | [`crates/renovate-core/src/util.rs:8344`](../../../../../../crates/renovate-core/src/util.rs#L8344) |
| 40 | returns https url for git url | ported | [`crates/renovate-core/src/util.rs:8359`](../../../../../../crates/renovate-core/src/util.rs#L8359) |
| 44 | returns https url for https url | ported | [`crates/renovate-core/src/util.rs:8365`](../../../../../../crates/renovate-core/src/util.rs#L8365) |
| 48 | returns http url for http url | ported | [`crates/renovate-core/src/util.rs:8371`](../../../../../../crates/renovate-core/src/util.rs#L8371) |
| 52 | returns http url for ssh url with port | ported | [`crates/renovate-core/src/util.rs:8377`](../../../../../../crates/renovate-core/src/util.rs#L8377) |
| 60 | returns gitlab url with token | ported | [`crates/renovate-core/src/util.rs:8389`](../../../../../../crates/renovate-core/src/util.rs#L8389) |
| 75 | returns github url with token | ported | [`crates/renovate-core/src/util.rs:8398`](../../../../../../crates/renovate-core/src/util.rs#L8398) |
| 90 | returns bitbucket-server url | ported | [`crates/renovate-core/src/util.rs:8407`](../../../../../../crates/renovate-core/src/util.rs#L8407) |
| 100 | removes username/password from url | ported | [`crates/renovate-core/src/util.rs:8428`](../../../../../../crates/renovate-core/src/util.rs#L8428) |
| 106 | replaces username/password with given token | ported | [`crates/renovate-core/src/util.rs:8437`](../../../../../../crates/renovate-core/src/util.rs#L8437) |
| 117 | returns original url if no host rule is found | ported | [`crates/renovate-core/src/util.rs:8466`](../../../../../../crates/renovate-core/src/util.rs#L8466) |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | [`crates/renovate-core/src/util.rs:8476`](../../../../../../crates/renovate-core/src/util.rs#L8476) |
| 132 | does not transform urls that are not parseable as git urls | ported | [`crates/renovate-core/src/util.rs:8495`](../../../../../../crates/renovate-core/src/util.rs#L8495) |
| 141 | returns http url with token | ported | [`crates/renovate-core/src/util.rs:8511`](../../../../../../crates/renovate-core/src/util.rs#L8511) |
| 148 | returns https url with token | ported | [`crates/renovate-core/src/util.rs:8530`](../../../../../../crates/renovate-core/src/util.rs#L8530) |
| 155 | returns https url with token for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8549`](../../../../../../crates/renovate-core/src/util.rs#L8549) |
| 162 | returns https url with encoded token | ported | [`crates/renovate-core/src/util.rs:8568`](../../../../../../crates/renovate-core/src/util.rs#L8568) |
| 169 | returns http url with username and password | ported | [`crates/renovate-core/src/util.rs:8587`](../../../../../../crates/renovate-core/src/util.rs#L8587) |
| 179 | returns https url with username and password | ported | [`crates/renovate-core/src/util.rs:8606`](../../../../../../crates/renovate-core/src/util.rs#L8606) |
| 189 | returns https url with username and password for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8625`](../../../../../../crates/renovate-core/src/util.rs#L8625) |
| 199 | returns https url with encoded username and password | ported | [`crates/renovate-core/src/util.rs:8644`](../../../../../../crates/renovate-core/src/util.rs#L8644) |
| 209 | returns https url with encoded gitlab token | ported | [`crates/renovate-core/src/util.rs:8663`](../../../../../../crates/renovate-core/src/util.rs#L8663) |
| 218 | returns https url for ssh url with encoded github token | ported | [`crates/renovate-core/src/util.rs:8682`](../../../../../../crates/renovate-core/src/util.rs#L8682) |

