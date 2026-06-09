# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | [`crates/renovate-core/src/util.rs:8026`](../../../../../crates/renovate-core/src/util.rs#L8026) |
| 33 | returns early if github token is found | ported | [`crates/renovate-core/src/util.rs:8033`](../../../../../crates/renovate-core/src/util.rs#L8033) |
| 45 | returns early if token warnings are disabled | ported | [`crates/renovate-core/src/util.rs:8047`](../../../../../crates/renovate-core/src/util.rs#L8047) |
| 60 | does not warn if there is dependencies with github sourceurl | ported | [`crates/renovate-core/src/util.rs:8061`](../../../../../crates/renovate-core/src/util.rs#L8061) |
| 68 | logs warning for github-tags datasource | ported | [`crates/renovate-core/src/util.rs:8075`](../../../../../crates/renovate-core/src/util.rs#L8075) |
| 85 | logs warning for github-releases datasource | ported | [`crates/renovate-core/src/util.rs:8092`](../../../../../crates/renovate-core/src/util.rs#L8092) |
| 102 | logs warning once | ported | [`crates/renovate-core/src/util.rs:8109`](../../../../../crates/renovate-core/src/util.rs#L8109) |
| 132 | returns true when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7903`](../../../../../crates/renovate-core/src/util.rs#L7903) |
| 136 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7904`](../../../../../crates/renovate-core/src/util.rs#L7904) |
| 140 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7905`](../../../../../crates/renovate-core/src/util.rs#L7905) |
| 144 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7906`](../../../../../crates/renovate-core/src/util.rs#L7906) |
| 150 | returns true when string is a github server to server token | ported | [`crates/renovate-core/src/util.rs:7915`](../../../../../crates/renovate-core/src/util.rs#L7915) |
| 155 | returns true when string is a 2026-style github installation access token | ported | [`crates/renovate-core/src/util.rs:7916`](../../../../../crates/renovate-core/src/util.rs#L7916) |
| 161 | returns false when string is a github personal access token token | ported | [`crates/renovate-core/src/util.rs:7917`](../../../../../crates/renovate-core/src/util.rs#L7917) |
| 165 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7905`](../../../../../crates/renovate-core/src/util.rs#L7905) |
| 169 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7906`](../../../../../crates/renovate-core/src/util.rs#L7906) |
| 175 | returns true when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7929`](../../../../../crates/renovate-core/src/util.rs#L7929) |
| 181 | returns false when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7930`](../../../../../crates/renovate-core/src/util.rs#L7930) |
| 185 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7904`](../../../../../crates/renovate-core/src/util.rs#L7904) |
| 189 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7906`](../../../../../crates/renovate-core/src/util.rs#L7906) |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | [`crates/renovate-core/src/util.rs:7943`](../../../../../crates/renovate-core/src/util.rs#L7943) |
| 201 | returns undefined when no token is defined | ported | [`crates/renovate-core/src/util.rs:7944`](../../../../../crates/renovate-core/src/util.rs#L7944) |
| 205 | remove x-access-token token prefix | ported | [`crates/renovate-core/src/util.rs:7945`](../../../../../crates/renovate-core/src/util.rs#L7945) |
| 216 | returns undefined when both token are undefined | ported | [`crates/renovate-core/src/util.rs:7957`](../../../../../crates/renovate-core/src/util.rs#L7957) |
| 224 | returns gittagstoken when both token are pat | ported | [`crates/renovate-core/src/util.rs:7958`](../../../../../crates/renovate-core/src/util.rs#L7958) |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | [`crates/renovate-core/src/util.rs:7959`](../../../../../crates/renovate-core/src/util.rs#L7959) |
| 240 | returns gittagstoken when both token are set but not pat | ported | [`crates/renovate-core/src/util.rs:7960`](../../../../../crates/renovate-core/src/util.rs#L7960) |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7961`](../../../../../crates/renovate-core/src/util.rs#L7961) |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7962`](../../../../../crates/renovate-core/src/util.rs#L7962) |
| 264 | take personal access token over fine grained token | ported | [`crates/renovate-core/src/util.rs:7963`](../../../../../crates/renovate-core/src/util.rs#L7963) |
| 272 | take fine grained token over server to server token | ported | [`crates/renovate-core/src/util.rs:7964`](../../../../../crates/renovate-core/src/util.rs#L7964) |
| 280 | take git-tags fine grained token | ported | [`crates/renovate-core/src/util.rs:8009`](../../../../../crates/renovate-core/src/util.rs#L8009) |
| 288 | take git-tags unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8014`](../../../../../crates/renovate-core/src/util.rs#L8014) |
| 296 | take github unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8019`](../../../../../crates/renovate-core/src/util.rs#L8019) |

