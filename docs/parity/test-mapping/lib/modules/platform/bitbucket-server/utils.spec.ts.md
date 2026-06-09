# `lib/modules/platform/bitbucket-server/utils.spec.ts`

[← `platform/bitbucket-server`](../../../../_by-module/platform/bitbucket-server.md) · [all modules](../../../../README.md)

**3/17 in-scope tests ported** (14 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 95 | getinvalidreviewers | ported | [`crates/renovate-core/src/util.rs:11389`](../../../../../../../crates/renovate-core/src/util.rs#L11389) |
| 128 | works giturl:undefined generate endpoint | pending | — |
| 147 | works giturl:undefined use endpoint with injected auth | pending | — |
| 166 | works giturl:undefined use ssh | pending | — |
| 180 | works giturl:default | pending | — |
| 197 | giturl:default invalid http url throws config_git_url_unavailable | pending | — |
| 211 | giturl:default no http url returns generated url | pending | — |
| 230 | giturl:ssh no ssh url throws config_git_url_unavailable | pending | — |
| 244 | works giturl:ssh | pending | — |
| 256 | works giturl:endpoint | pending | — |
| 273 | works giturl:endpoint no basic auth | pending | — |
| 295 | works giturl:endpoint | pending | — |
| 307 | giturl:default no http url returns generated url | pending | — |
| 321 | actually respects the giturl setting | pending | — |
| 333 | throws on invalid endpoint url | pending | — |
| 347 | should not configure bearer token | ported | [`crates/renovate-core/src/util.rs:11405`](../../../../../../../crates/renovate-core/src/util.rs#L11405) |
| 352 | should configure bearer token | ported | [`crates/renovate-core/src/util.rs:11412`](../../../../../../../crates/renovate-core/src/util.rs#L11412) |

