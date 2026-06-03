# `lib/modules/manager/npm/post-update/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**32/35 ported** (3 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | generates lock files | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:247`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L247) |
| 54 | runs npm install twice | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:256`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L256) |
| 87 | performs lock file updates | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:265`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L265) |
| 107 | performs lock file updates retaining the package.json counterparts | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:274`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L274) |
| 136 | performs npm-shrinkwrap.json updates | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:283`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L283) |
| 163 | performs npm-shrinkwrap.json updates (no package-lock.json) | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:292`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L292) |
| 186 | performs full install | ported | [`crates/renovate-core/src/extractors/npm_post_update.rs:179`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update.rs#L179) |
| 204 | deduplicates dependencies on installation with npm >= 7 | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:301`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L301) |
| 236 | deduplicates package-lock.json dependencies after installation with npm <= 6 | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:310`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L310) |
| 271 | deduplicates npm-shrinkwrap.json dependencies after installation with npm <= 6 | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:319`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L319) |
| 311 | runs twice if remediating | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:328`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L328) |
| 328 | catches errors | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:337`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L337) |
| 344 | finds npm globally | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:343`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L343) |
| 369 | uses docker npm | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:350`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L350) |
| 384 | performs lock file maintenance | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:359`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L359) |
| 402 | works for docker mode | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:368`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L368) |
| 442 | works for install mode | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:377`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L377) |
| 468 | does not install npm if no constraints specified | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:116`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L116) |
| 494 | if nodemaxmemory set on global config | ported | [`crates/renovate-core/src/extractors/npm_post_update/utils.rs:19`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/utils.rs#L19) |
| 539 | if nodemaxmemory set on repo config | ported | [`crates/renovate-core/src/extractors/npm_post_update/utils.rs:28`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/utils.rs#L28) |
| 696 | workspace in sub-folder | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:159`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L159) |
| 728 | workspace in root folder | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:206`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L206) |
| 884 | while performing lockfileupdate (npm-workspaces) | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:386`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L386) |
| 932 | while performing lockfileupdate (npm) | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:400`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L400) |
| 981 | sets --before from minimumreleaseage | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:165`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L165) |
| 1005 | skips --before on unparseable minimumreleaseage | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:175`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L175) |
| 1027 | uses stricter npmrc before date when older than minimumreleaseage | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:414`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L414) |
| 1051 | uses minimumreleaseage date when stricter than npmrc before date | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:424`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L424) |
| 1075 | skips --before when minimumreleaseage is absent even if npmrc has before | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:434`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L434) |
| 1098 | skips --before when .npmrc has min-release-age to avoid npm conflict | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:240`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L240) |
| 1121 | retries without --before on etarget with "with a date before" | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:441`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L441) |
| 1167 | does not retry on non-before etarget errors | ported | [`crates/renovate-core/src/extractors/npm_post_update/npm.rs:450`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/npm.rs#L450) |
| 1212 | _(it.each / template — verify manually)_ | ? | — |
| 1226 | _(it.each / template — verify manually)_ | ? | — |
| 1239 | _(it.each / template — verify manually)_ | ? | — |

