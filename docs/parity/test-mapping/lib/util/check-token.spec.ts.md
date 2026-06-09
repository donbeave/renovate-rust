# `lib/util/check-token.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**30/34 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | does nothing if data is empty | ported | [`crates/renovate-core/src/util.rs:8020`](../../../../../crates/renovate-core/src/util.rs#L8020) |
| 33 | returns early if github token is found | ported | [`crates/renovate-core/src/util.rs:8027`](../../../../../crates/renovate-core/src/util.rs#L8027) |
| 45 | returns early if token warnings are disabled | ported | [`crates/renovate-core/src/util.rs:8041`](../../../../../crates/renovate-core/src/util.rs#L8041) |
| 60 | does not warn if there is dependencies with github sourceurl | ported | [`crates/renovate-core/src/util.rs:8055`](../../../../../crates/renovate-core/src/util.rs#L8055) |
| 68 | logs warning for github-tags datasource | ported | [`crates/renovate-core/src/util.rs:8069`](../../../../../crates/renovate-core/src/util.rs#L8069) |
| 85 | logs warning for github-releases datasource | ported | [`crates/renovate-core/src/util.rs:8086`](../../../../../crates/renovate-core/src/util.rs#L8086) |
| 102 | logs warning once | ported | [`crates/renovate-core/src/util.rs:8103`](../../../../../crates/renovate-core/src/util.rs#L8103) |
| 132 | returns true when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7897`](../../../../../crates/renovate-core/src/util.rs#L7897) |
| 136 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7898`](../../../../../crates/renovate-core/src/util.rs#L7898) |
| 140 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7899`](../../../../../crates/renovate-core/src/util.rs#L7899) |
| 144 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7900`](../../../../../crates/renovate-core/src/util.rs#L7900) |
| 150 | returns true when string is a github server to server token | ported | [`crates/renovate-core/src/util.rs:7909`](../../../../../crates/renovate-core/src/util.rs#L7909) |
| 155 | returns true when string is a 2026-style github installation access token | ported | [`crates/renovate-core/src/util.rs:7910`](../../../../../crates/renovate-core/src/util.rs#L7910) |
| 161 | returns false when string is a github personal access token token | ported | [`crates/renovate-core/src/util.rs:7911`](../../../../../crates/renovate-core/src/util.rs#L7911) |
| 165 | returns false when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7899`](../../../../../crates/renovate-core/src/util.rs#L7899) |
| 169 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7900`](../../../../../crates/renovate-core/src/util.rs#L7900) |
| 175 | returns true when string is a github fine grained personal access token | ported | [`crates/renovate-core/src/util.rs:7923`](../../../../../crates/renovate-core/src/util.rs#L7923) |
| 181 | returns false when string is a github personnal access token | ported | [`crates/renovate-core/src/util.rs:7924`](../../../../../crates/renovate-core/src/util.rs#L7924) |
| 185 | returns false when string is a github application token | ported | [`crates/renovate-core/src/util.rs:7898`](../../../../../crates/renovate-core/src/util.rs#L7898) |
| 189 | returns false when string is not a token at all | ported | [`crates/renovate-core/src/util.rs:7900`](../../../../../crates/renovate-core/src/util.rs#L7900) |
| 195 | returns the token string when hostrule match search with a valid personal access token | ported | [`crates/renovate-core/src/util.rs:7937`](../../../../../crates/renovate-core/src/util.rs#L7937) |
| 201 | returns undefined when no token is defined | ported | [`crates/renovate-core/src/util.rs:7938`](../../../../../crates/renovate-core/src/util.rs#L7938) |
| 205 | remove x-access-token token prefix | ported | [`crates/renovate-core/src/util.rs:7939`](../../../../../crates/renovate-core/src/util.rs#L7939) |
| 216 | returns undefined when both token are undefined | ported | [`crates/renovate-core/src/util.rs:7951`](../../../../../crates/renovate-core/src/util.rs#L7951) |
| 224 | returns gittagstoken when both token are pat | ported | [`crates/renovate-core/src/util.rs:7952`](../../../../../crates/renovate-core/src/util.rs#L7952) |
| 232 | returns githubtoken is pat and gittagsgithubtoken is not a pat | ported | [`crates/renovate-core/src/util.rs:7953`](../../../../../crates/renovate-core/src/util.rs#L7953) |
| 240 | returns gittagstoken when both token are set but not pat | ported | [`crates/renovate-core/src/util.rs:7954`](../../../../../crates/renovate-core/src/util.rs#L7954) |
| 248 | returns gittagstoken when gittagstoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7955`](../../../../../crates/renovate-core/src/util.rs#L7955) |
| 256 | returns githubtoken when githubtoken not pat and gittagsgithubtoken is not set | ported | [`crates/renovate-core/src/util.rs:7956`](../../../../../crates/renovate-core/src/util.rs#L7956) |
| 264 | take personal access token over fine grained token | ported | [`crates/renovate-core/src/util.rs:7957`](../../../../../crates/renovate-core/src/util.rs#L7957) |
| 272 | take fine grained token over server to server token | ported | [`crates/renovate-core/src/util.rs:7958`](../../../../../crates/renovate-core/src/util.rs#L7958) |
| 280 | take git-tags fine grained token | ported | [`crates/renovate-core/src/util.rs:8003`](../../../../../crates/renovate-core/src/util.rs#L8003) |
| 288 | take git-tags unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8008`](../../../../../crates/renovate-core/src/util.rs#L8008) |
| 296 | take github unknown token type when no other token is set | ported | [`crates/renovate-core/src/util.rs:8013`](../../../../../crates/renovate-core/src/util.rs#L8013) |

