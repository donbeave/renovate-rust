# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 ported** (4 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | `crates/renovate-core/src/util.rs:6655` |
| 33 | returns early if github token is found | ported | `crates/renovate-core/src/util.rs:6662` |
| 45 | returns early if token warnings are disabled | ported | `crates/renovate-core/src/util.rs:6676` |
| 60 | does not warn if there is dependencies with github sourceurl | ported | `crates/renovate-core/src/util.rs:6690` |
| 68 | logs warning for github-tags datasource | ported | `crates/renovate-core/src/util.rs:6704` |
| 85 | logs warning for github-releases datasource | ported | `crates/renovate-core/src/util.rs:6721` |
| 102 | logs warning once | ported | `crates/renovate-core/src/util.rs:6738` |
| 132 | returns true when string is a github personnal access token | ported | `crates/renovate-core/src/util.rs:6532` |
| 136 | returns false when string is a github application token | ported | `crates/renovate-core/src/util.rs:6533` |
| 140 | returns false when string is a github fine grained personal access token | ported | `crates/renovate-core/src/util.rs:6534` |
| 144 | returns false when string is not a token at all | ported | `crates/renovate-core/src/util.rs:6535` |
| 150 | returns true when string is a github server to server token | ported | `crates/renovate-core/src/util.rs:6544` |
| 155 | returns true when string is a 2026-style github installation access token | ported | `crates/renovate-core/src/util.rs:6545` |
| 161 | returns false when string is a github personal access token token | ported | `crates/renovate-core/src/util.rs:6546` |
| 165 | returns false when string is a github fine grained personal access token | ported | `crates/renovate-core/src/util.rs:6534` |
| 169 | returns false when string is not a token at all | ported | `crates/renovate-core/src/util.rs:6535` |
| 175 | returns true when string is a github fine grained personal access token | ported | `crates/renovate-core/src/util.rs:6558` |
| 181 | returns false when string is a github personnal access token | ported | `crates/renovate-core/src/util.rs:6559` |
| 185 | returns false when string is a github application token | ported | `crates/renovate-core/src/util.rs:6533` |
| 189 | returns false when string is not a token at all | ported | `crates/renovate-core/src/util.rs:6535` |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | `crates/renovate-core/src/util.rs:6572` |
| 201 | returns undefined when no token is defined | ported | `crates/renovate-core/src/util.rs:6573` |
| 205 | remove x-access-token token prefix | ported | `crates/renovate-core/src/util.rs:6574` |
| 216 | returns undefined when both token are undefined | ported | `crates/renovate-core/src/util.rs:6586` |
| 224 | returns gittagstoken when both token are pat | ported | `crates/renovate-core/src/util.rs:6587` |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | `crates/renovate-core/src/util.rs:6588` |
| 240 | returns gittagstoken when both token are set but not pat | ported | `crates/renovate-core/src/util.rs:6589` |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | `crates/renovate-core/src/util.rs:6590` |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | `crates/renovate-core/src/util.rs:6591` |
| 264 | take personal access token over fine grained token | ported | `crates/renovate-core/src/util.rs:6592` |
| 272 | take fine grained token over server to server token | ported | `crates/renovate-core/src/util.rs:6593` |
| 280 | take git-tags fine grained token | ported | `crates/renovate-core/src/util.rs:6638` |
| 288 | take git-tags unknown token type when no other token is set | ported | `crates/renovate-core/src/util.rs:6643` |
| 296 | take github unknown token type when no other token is set | ported | `crates/renovate-core/src/util.rs:6648` |

