# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | [`crates/renovate-core/src/util.rs:6655`](../../../../../crates/renovate-core/src/util.rs#L6655) |
| 33 | returns early if github token is found | ported | [`crates/renovate-core/src/util.rs:6662`](../../../../../crates/renovate-core/src/util.rs#L6662) |
| 45 | returns early if token warnings are disabled | ported | [`crates/renovate-core/src/util.rs:6676`](../../../../../crates/renovate-core/src/util.rs#L6676) |
| 60 | does not warn if there is dependencies with github sourceurl | ported | [`crates/renovate-core/src/util.rs:6690`](../../../../../crates/renovate-core/src/util.rs#L6690) |
| 68 | logs warning for github-tags datasource | ported | [`crates/renovate-core/src/util.rs:6704`](../../../../../crates/renovate-core/src/util.rs#L6704) |
| 85 | logs warning for github-releases datasource | ported | [`crates/renovate-core/src/util.rs:6721`](../../../../../crates/renovate-core/src/util.rs#L6721) |
| 102 | logs warning once | ported | [`crates/renovate-core/src/util.rs:6738`](../../../../../crates/renovate-core/src/util.rs#L6738) |
| 132 | returns true when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:6532`](../../../../../crates/renovate-core/src/util.rs#L6532) |
| 136 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:6533`](../../../../../crates/renovate-core/src/util.rs#L6533) |
| 140 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:6534`](../../../../../crates/renovate-core/src/util.rs#L6534) |
| 144 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:6535`](../../../../../crates/renovate-core/src/util.rs#L6535) |
| 150 | returns true when string is a github server to server token | ported | [`crates/renovate-core/src/util.rs:6544`](../../../../../crates/renovate-core/src/util.rs#L6544) |
| 155 | returns true when string is a 2026-style github installation access token | ported | [`crates/renovate-core/src/util.rs:6545`](../../../../../crates/renovate-core/src/util.rs#L6545) |
| 161 | returns false when string is a github personal access token token | ported | [`crates/renovate-core/src/util.rs:6546`](../../../../../crates/renovate-core/src/util.rs#L6546) |
| 165 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:6534`](../../../../../crates/renovate-core/src/util.rs#L6534) |
| 169 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:6535`](../../../../../crates/renovate-core/src/util.rs#L6535) |
| 175 | returns true when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:6558`](../../../../../crates/renovate-core/src/util.rs#L6558) |
| 181 | returns false when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:6559`](../../../../../crates/renovate-core/src/util.rs#L6559) |
| 185 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:6533`](../../../../../crates/renovate-core/src/util.rs#L6533) |
| 189 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:6535`](../../../../../crates/renovate-core/src/util.rs#L6535) |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | [`crates/renovate-core/src/util.rs:6572`](../../../../../crates/renovate-core/src/util.rs#L6572) |
| 201 | returns undefined when no token is defined | ported | [`crates/renovate-core/src/util.rs:6573`](../../../../../crates/renovate-core/src/util.rs#L6573) |
| 205 | remove x-access-token token prefix | ported | [`crates/renovate-core/src/util.rs:6574`](../../../../../crates/renovate-core/src/util.rs#L6574) |
| 216 | returns undefined when both token are undefined | ported | [`crates/renovate-core/src/util.rs:6586`](../../../../../crates/renovate-core/src/util.rs#L6586) |
| 224 | returns gittagstoken when both token are pat | ported | [`crates/renovate-core/src/util.rs:6587`](../../../../../crates/renovate-core/src/util.rs#L6587) |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | [`crates/renovate-core/src/util.rs:6588`](../../../../../crates/renovate-core/src/util.rs#L6588) |
| 240 | returns gittagstoken when both token are set but not pat | ported | [`crates/renovate-core/src/util.rs:6589`](../../../../../crates/renovate-core/src/util.rs#L6589) |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:6590`](../../../../../crates/renovate-core/src/util.rs#L6590) |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:6591`](../../../../../crates/renovate-core/src/util.rs#L6591) |
| 264 | take personal access token over fine grained token | ported | [`crates/renovate-core/src/util.rs:6592`](../../../../../crates/renovate-core/src/util.rs#L6592) |
| 272 | take fine grained token over server to server token | ported | [`crates/renovate-core/src/util.rs:6593`](../../../../../crates/renovate-core/src/util.rs#L6593) |
| 280 | take git-tags fine grained token | ported | [`crates/renovate-core/src/util.rs:6638`](../../../../../crates/renovate-core/src/util.rs#L6638) |
| 288 | take git-tags unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:6643`](../../../../../crates/renovate-core/src/util.rs#L6643) |
| 296 | take github unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:6648`](../../../../../crates/renovate-core/src/util.rs#L6648) |

