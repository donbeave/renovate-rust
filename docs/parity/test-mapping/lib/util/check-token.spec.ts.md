# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | [`crates/renovate-core/src/util.rs:8118`](../../../../../crates/renovate-core/src/util.rs#L8118) |
| 33 | returns early if github token is found | ported | [`crates/renovate-core/src/util.rs:8125`](../../../../../crates/renovate-core/src/util.rs#L8125) |
| 45 | returns early if token warnings are disabled | ported | [`crates/renovate-core/src/util.rs:8139`](../../../../../crates/renovate-core/src/util.rs#L8139) |
| 60 | does not warn if there is dependencies with github sourceurl | ported | [`crates/renovate-core/src/util.rs:8153`](../../../../../crates/renovate-core/src/util.rs#L8153) |
| 68 | logs warning for github-tags datasource | ported | [`crates/renovate-core/src/util.rs:8167`](../../../../../crates/renovate-core/src/util.rs#L8167) |
| 85 | logs warning for github-releases datasource | ported | [`crates/renovate-core/src/util.rs:8184`](../../../../../crates/renovate-core/src/util.rs#L8184) |
| 102 | logs warning once | ported | [`crates/renovate-core/src/util.rs:8201`](../../../../../crates/renovate-core/src/util.rs#L8201) |
| 132 | returns true when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7995`](../../../../../crates/renovate-core/src/util.rs#L7995) |
| 136 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7996`](../../../../../crates/renovate-core/src/util.rs#L7996) |
| 140 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7997`](../../../../../crates/renovate-core/src/util.rs#L7997) |
| 144 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7998`](../../../../../crates/renovate-core/src/util.rs#L7998) |
| 150 | returns true when string is a github server to server token | ported | [`crates/renovate-core/src/util.rs:8007`](../../../../../crates/renovate-core/src/util.rs#L8007) |
| 155 | returns true when string is a 2026-style github installation access token | ported | [`crates/renovate-core/src/util.rs:8008`](../../../../../crates/renovate-core/src/util.rs#L8008) |
| 161 | returns false when string is a github personal access token token | ported | [`crates/renovate-core/src/util.rs:8009`](../../../../../crates/renovate-core/src/util.rs#L8009) |
| 165 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7997`](../../../../../crates/renovate-core/src/util.rs#L7997) |
| 169 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7998`](../../../../../crates/renovate-core/src/util.rs#L7998) |
| 175 | returns true when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:8021`](../../../../../crates/renovate-core/src/util.rs#L8021) |
| 181 | returns false when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:8022`](../../../../../crates/renovate-core/src/util.rs#L8022) |
| 185 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7996`](../../../../../crates/renovate-core/src/util.rs#L7996) |
| 189 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7998`](../../../../../crates/renovate-core/src/util.rs#L7998) |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | [`crates/renovate-core/src/util.rs:8035`](../../../../../crates/renovate-core/src/util.rs#L8035) |
| 201 | returns undefined when no token is defined | ported | [`crates/renovate-core/src/util.rs:8036`](../../../../../crates/renovate-core/src/util.rs#L8036) |
| 205 | remove x-access-token token prefix | ported | [`crates/renovate-core/src/util.rs:8037`](../../../../../crates/renovate-core/src/util.rs#L8037) |
| 216 | returns undefined when both token are undefined | ported | [`crates/renovate-core/src/util.rs:8049`](../../../../../crates/renovate-core/src/util.rs#L8049) |
| 224 | returns gittagstoken when both token are pat | ported | [`crates/renovate-core/src/util.rs:8050`](../../../../../crates/renovate-core/src/util.rs#L8050) |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | [`crates/renovate-core/src/util.rs:8051`](../../../../../crates/renovate-core/src/util.rs#L8051) |
| 240 | returns gittagstoken when both token are set but not pat | ported | [`crates/renovate-core/src/util.rs:8052`](../../../../../crates/renovate-core/src/util.rs#L8052) |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:8053`](../../../../../crates/renovate-core/src/util.rs#L8053) |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:8054`](../../../../../crates/renovate-core/src/util.rs#L8054) |
| 264 | take personal access token over fine grained token | ported | [`crates/renovate-core/src/util.rs:8055`](../../../../../crates/renovate-core/src/util.rs#L8055) |
| 272 | take fine grained token over server to server token | ported | [`crates/renovate-core/src/util.rs:8056`](../../../../../crates/renovate-core/src/util.rs#L8056) |
| 280 | take git-tags fine grained token | ported | [`crates/renovate-core/src/util.rs:8101`](../../../../../crates/renovate-core/src/util.rs#L8101) |
| 288 | take git-tags unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8106`](../../../../../crates/renovate-core/src/util.rs#L8106) |
| 296 | take github unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8111`](../../../../../crates/renovate-core/src/util.rs#L8111) |

