# `lib/util/git/url.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**23/23 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | supports ports | ported | [`crates/renovate-core/src/util.rs:8441`](../../../../../../crates/renovate-core/src/util.rs#L8441) |
| 40 | returns https url for git url | ported | [`crates/renovate-core/src/util.rs:8456`](../../../../../../crates/renovate-core/src/util.rs#L8456) |
| 44 | returns https url for https url | ported | [`crates/renovate-core/src/util.rs:8462`](../../../../../../crates/renovate-core/src/util.rs#L8462) |
| 48 | returns http url for http url | ported | [`crates/renovate-core/src/util.rs:8468`](../../../../../../crates/renovate-core/src/util.rs#L8468) |
| 52 | returns http url for ssh url with port | ported | [`crates/renovate-core/src/util.rs:8474`](../../../../../../crates/renovate-core/src/util.rs#L8474) |
| 60 | returns gitlab url with token | ported | [`crates/renovate-core/src/util.rs:8486`](../../../../../../crates/renovate-core/src/util.rs#L8486) |
| 75 | returns github url with token | ported | [`crates/renovate-core/src/util.rs:8495`](../../../../../../crates/renovate-core/src/util.rs#L8495) |
| 90 | returns bitbucket-server url | ported | [`crates/renovate-core/src/util.rs:8504`](../../../../../../crates/renovate-core/src/util.rs#L8504) |
| 100 | removes username/password from url | ported | [`crates/renovate-core/src/util.rs:8525`](../../../../../../crates/renovate-core/src/util.rs#L8525) |
| 106 | replaces username/password with given token | ported | [`crates/renovate-core/src/util.rs:8534`](../../../../../../crates/renovate-core/src/util.rs#L8534) |
| 117 | returns original url if no host rule is found | ported | [`crates/renovate-core/src/util.rs:8563`](../../../../../../crates/renovate-core/src/util.rs#L8563) |
| 123 | transforms an ssh git url to https for the purpose of finding hostrules | ported | [`crates/renovate-core/src/util.rs:8573`](../../../../../../crates/renovate-core/src/util.rs#L8573) |
| 132 | does not transform urls that are not parseable as git urls | ported | [`crates/renovate-core/src/util.rs:8592`](../../../../../../crates/renovate-core/src/util.rs#L8592) |
| 141 | returns http url with token | ported | [`crates/renovate-core/src/util.rs:8608`](../../../../../../crates/renovate-core/src/util.rs#L8608) |
| 148 | returns https url with token | ported | [`crates/renovate-core/src/util.rs:8627`](../../../../../../crates/renovate-core/src/util.rs#L8627) |
| 155 | returns https url with token for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8646`](../../../../../../crates/renovate-core/src/util.rs#L8646) |
| 162 | returns https url with encoded token | ported | [`crates/renovate-core/src/util.rs:8665`](../../../../../../crates/renovate-core/src/util.rs#L8665) |
| 169 | returns http url with username and password | ported | [`crates/renovate-core/src/util.rs:8684`](../../../../../../crates/renovate-core/src/util.rs#L8684) |
| 179 | returns https url with username and password | ported | [`crates/renovate-core/src/util.rs:8703`](../../../../../../crates/renovate-core/src/util.rs#L8703) |
| 189 | returns https url with username and password for non-http protocols | ported | [`crates/renovate-core/src/util.rs:8722`](../../../../../../crates/renovate-core/src/util.rs#L8722) |
| 199 | returns https url with encoded username and password | ported | [`crates/renovate-core/src/util.rs:8741`](../../../../../../crates/renovate-core/src/util.rs#L8741) |
| 209 | returns https url with encoded gitlab token | ported | [`crates/renovate-core/src/util.rs:8760`](../../../../../../crates/renovate-core/src/util.rs#L8760) |
| 218 | returns https url for ssh url with encoded github token | ported | [`crates/renovate-core/src/util.rs:8779`](../../../../../../crates/renovate-core/src/util.rs#L8779) |

