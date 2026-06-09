# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | [`crates/renovate-core/src/util.rs:8023`](../../../../../crates/renovate-core/src/util.rs#L8023) |
| 33 | returns early if github token is found | ported | [`crates/renovate-core/src/util.rs:8030`](../../../../../crates/renovate-core/src/util.rs#L8030) |
| 45 | returns early if token warnings are disabled | ported | [`crates/renovate-core/src/util.rs:8044`](../../../../../crates/renovate-core/src/util.rs#L8044) |
| 60 | does not warn if there is dependencies with github sourceurl | ported | [`crates/renovate-core/src/util.rs:8058`](../../../../../crates/renovate-core/src/util.rs#L8058) |
| 68 | logs warning for github-tags datasource | ported | [`crates/renovate-core/src/util.rs:8072`](../../../../../crates/renovate-core/src/util.rs#L8072) |
| 85 | logs warning for github-releases datasource | ported | [`crates/renovate-core/src/util.rs:8089`](../../../../../crates/renovate-core/src/util.rs#L8089) |
| 102 | logs warning once | ported | [`crates/renovate-core/src/util.rs:8106`](../../../../../crates/renovate-core/src/util.rs#L8106) |
| 132 | returns true when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7900`](../../../../../crates/renovate-core/src/util.rs#L7900) |
| 136 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7901`](../../../../../crates/renovate-core/src/util.rs#L7901) |
| 140 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7902`](../../../../../crates/renovate-core/src/util.rs#L7902) |
| 144 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7903`](../../../../../crates/renovate-core/src/util.rs#L7903) |
| 150 | returns true when string is a github server to server token | ported | [`crates/renovate-core/src/util.rs:7912`](../../../../../crates/renovate-core/src/util.rs#L7912) |
| 155 | returns true when string is a 2026-style github installation access token | ported | [`crates/renovate-core/src/util.rs:7913`](../../../../../crates/renovate-core/src/util.rs#L7913) |
| 161 | returns false when string is a github personal access token token | ported | [`crates/renovate-core/src/util.rs:7914`](../../../../../crates/renovate-core/src/util.rs#L7914) |
| 165 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7902`](../../../../../crates/renovate-core/src/util.rs#L7902) |
| 169 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7903`](../../../../../crates/renovate-core/src/util.rs#L7903) |
| 175 | returns true when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7926`](../../../../../crates/renovate-core/src/util.rs#L7926) |
| 181 | returns false when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7927`](../../../../../crates/renovate-core/src/util.rs#L7927) |
| 185 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7901`](../../../../../crates/renovate-core/src/util.rs#L7901) |
| 189 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7903`](../../../../../crates/renovate-core/src/util.rs#L7903) |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | [`crates/renovate-core/src/util.rs:7940`](../../../../../crates/renovate-core/src/util.rs#L7940) |
| 201 | returns undefined when no token is defined | ported | [`crates/renovate-core/src/util.rs:7941`](../../../../../crates/renovate-core/src/util.rs#L7941) |
| 205 | remove x-access-token token prefix | ported | [`crates/renovate-core/src/util.rs:7942`](../../../../../crates/renovate-core/src/util.rs#L7942) |
| 216 | returns undefined when both token are undefined | ported | [`crates/renovate-core/src/util.rs:7954`](../../../../../crates/renovate-core/src/util.rs#L7954) |
| 224 | returns gittagstoken when both token are pat | ported | [`crates/renovate-core/src/util.rs:7955`](../../../../../crates/renovate-core/src/util.rs#L7955) |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | [`crates/renovate-core/src/util.rs:7956`](../../../../../crates/renovate-core/src/util.rs#L7956) |
| 240 | returns gittagstoken when both token are set but not pat | ported | [`crates/renovate-core/src/util.rs:7957`](../../../../../crates/renovate-core/src/util.rs#L7957) |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7958`](../../../../../crates/renovate-core/src/util.rs#L7958) |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7959`](../../../../../crates/renovate-core/src/util.rs#L7959) |
| 264 | take personal access token over fine grained token | ported | [`crates/renovate-core/src/util.rs:7960`](../../../../../crates/renovate-core/src/util.rs#L7960) |
| 272 | take fine grained token over server to server token | ported | [`crates/renovate-core/src/util.rs:7961`](../../../../../crates/renovate-core/src/util.rs#L7961) |
| 280 | take git-tags fine grained token | ported | [`crates/renovate-core/src/util.rs:8006`](../../../../../crates/renovate-core/src/util.rs#L8006) |
| 288 | take git-tags unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8011`](../../../../../crates/renovate-core/src/util.rs#L8011) |
| 296 | take github unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8016`](../../../../../crates/renovate-core/src/util.rs#L8016) |

