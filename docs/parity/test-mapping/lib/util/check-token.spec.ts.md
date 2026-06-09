# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | [`crates/renovate-core/src/util.rs:8022`](../../../../../crates/renovate-core/src/util.rs#L8022) |
| 33 | returns early if github token is found | ported | [`crates/renovate-core/src/util.rs:8029`](../../../../../crates/renovate-core/src/util.rs#L8029) |
| 45 | returns early if token warnings are disabled | ported | [`crates/renovate-core/src/util.rs:8043`](../../../../../crates/renovate-core/src/util.rs#L8043) |
| 60 | does not warn if there is dependencies with github sourceurl | ported | [`crates/renovate-core/src/util.rs:8057`](../../../../../crates/renovate-core/src/util.rs#L8057) |
| 68 | logs warning for github-tags datasource | ported | [`crates/renovate-core/src/util.rs:8071`](../../../../../crates/renovate-core/src/util.rs#L8071) |
| 85 | logs warning for github-releases datasource | ported | [`crates/renovate-core/src/util.rs:8088`](../../../../../crates/renovate-core/src/util.rs#L8088) |
| 102 | logs warning once | ported | [`crates/renovate-core/src/util.rs:8105`](../../../../../crates/renovate-core/src/util.rs#L8105) |
| 132 | returns true when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7899`](../../../../../crates/renovate-core/src/util.rs#L7899) |
| 136 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7900`](../../../../../crates/renovate-core/src/util.rs#L7900) |
| 140 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7901`](../../../../../crates/renovate-core/src/util.rs#L7901) |
| 144 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7902`](../../../../../crates/renovate-core/src/util.rs#L7902) |
| 150 | returns true when string is a github server to server token | ported | [`crates/renovate-core/src/util.rs:7911`](../../../../../crates/renovate-core/src/util.rs#L7911) |
| 155 | returns true when string is a 2026-style github installation access token | ported | [`crates/renovate-core/src/util.rs:7912`](../../../../../crates/renovate-core/src/util.rs#L7912) |
| 161 | returns false when string is a github personal access token token | ported | [`crates/renovate-core/src/util.rs:7913`](../../../../../crates/renovate-core/src/util.rs#L7913) |
| 165 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7901`](../../../../../crates/renovate-core/src/util.rs#L7901) |
| 169 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7902`](../../../../../crates/renovate-core/src/util.rs#L7902) |
| 175 | returns true when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7925`](../../../../../crates/renovate-core/src/util.rs#L7925) |
| 181 | returns false when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7926`](../../../../../crates/renovate-core/src/util.rs#L7926) |
| 185 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7900`](../../../../../crates/renovate-core/src/util.rs#L7900) |
| 189 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7902`](../../../../../crates/renovate-core/src/util.rs#L7902) |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | [`crates/renovate-core/src/util.rs:7939`](../../../../../crates/renovate-core/src/util.rs#L7939) |
| 201 | returns undefined when no token is defined | ported | [`crates/renovate-core/src/util.rs:7940`](../../../../../crates/renovate-core/src/util.rs#L7940) |
| 205 | remove x-access-token token prefix | ported | [`crates/renovate-core/src/util.rs:7941`](../../../../../crates/renovate-core/src/util.rs#L7941) |
| 216 | returns undefined when both token are undefined | ported | [`crates/renovate-core/src/util.rs:7953`](../../../../../crates/renovate-core/src/util.rs#L7953) |
| 224 | returns gittagstoken when both token are pat | ported | [`crates/renovate-core/src/util.rs:7954`](../../../../../crates/renovate-core/src/util.rs#L7954) |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | [`crates/renovate-core/src/util.rs:7955`](../../../../../crates/renovate-core/src/util.rs#L7955) |
| 240 | returns gittagstoken when both token are set but not pat | ported | [`crates/renovate-core/src/util.rs:7956`](../../../../../crates/renovate-core/src/util.rs#L7956) |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7957`](../../../../../crates/renovate-core/src/util.rs#L7957) |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7958`](../../../../../crates/renovate-core/src/util.rs#L7958) |
| 264 | take personal access token over fine grained token | ported | [`crates/renovate-core/src/util.rs:7959`](../../../../../crates/renovate-core/src/util.rs#L7959) |
| 272 | take fine grained token over server to server token | ported | [`crates/renovate-core/src/util.rs:7960`](../../../../../crates/renovate-core/src/util.rs#L7960) |
| 280 | take git-tags fine grained token | ported | [`crates/renovate-core/src/util.rs:8005`](../../../../../crates/renovate-core/src/util.rs#L8005) |
| 288 | take git-tags unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8010`](../../../../../crates/renovate-core/src/util.rs#L8010) |
| 296 | take github unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8015`](../../../../../crates/renovate-core/src/util.rs#L8015) |

