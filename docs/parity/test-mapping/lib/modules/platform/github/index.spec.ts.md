# `lib/modules/platform/github/index.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**158/206 ported** (48 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 64 | should throw if no token | ported | [`crates/renovate-core/src/platform/github.rs:5962`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5962) |
| 70 | should throw if endpoint is invalid url | ported | [`crates/renovate-core/src/platform/github.rs:5989`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5989) |
| 79 | should throw if using fine-grained token with ghe <3.10 | ported | [`crates/renovate-core/src/platform/github.rs:6109`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6109) |
| 94 | should throw if using fine-grained token with ghe unknown version | ported | [`crates/renovate-core/src/platform/github.rs:6131`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6131) |
| 106 | should support fine-grained token with ghe >=3.10 | ported | [`crates/renovate-core/src/platform/github.rs:6151`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6151) |
| 128 | should throw if user failure | ported | [`crates/renovate-core/src/platform/github.rs:2938`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2938) |
| 133 | should support default endpoint no email access | ported | [`crates/renovate-core/src/platform/github.rs:2565`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2565) |
| 145 | should support default endpoint no email result | ported | [`crates/renovate-core/src/platform/github.rs:2566`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2566) |
| 157 | should support gitauthor and username | ported | [`crates/renovate-core/src/platform/github.rs:2567`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2567) |
| 170 | if on github.com, a warning is shown | ported | [`crates/renovate-core/src/platform/github.rs:2603`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2603) |
| 195 | if on github enterprise, a warning is not shown | ported | [`crates/renovate-core/src/platform/github.rs:2622`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2622) |
| 217 | no warning is shown | ported | [`crates/renovate-core/src/platform/github.rs:2568`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2568) |
| 240 | if on github enterprise, a warning is not shown | ported | [`crates/renovate-core/src/platform/github.rs:2622`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2622) |
| 267 | if on github.com, a warning is shown | ported | [`crates/renovate-core/src/platform/github.rs:2603`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2603) |
| 287 | if on github enterprise, a warning is not shown | ported | [`crates/renovate-core/src/platform/github.rs:2622`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2622) |
| 306 | if on github.com, a warning is shown | ported | [`crates/renovate-core/src/platform/github.rs:2603`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2603) |
| 326 | if on github enterprise, a warning is not shown | ported | [`crates/renovate-core/src/platform/github.rs:2622`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2622) |
| 345 | should support default endpoint with email | ported | [`crates/renovate-core/src/platform/github.rs:2564`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2564) |
| 361 | should use public email from user profile when available | ported | [`crates/renovate-core/src/platform/github.rs:5996`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5996) |
| 375 | should fall back to user/emails when there is no public email | ported | [`crates/renovate-core/src/platform/github.rs:6015`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6015) |
| 394 | should fall back gracefully when user/emails returns an error (no user:email scope) | ported | [`crates/renovate-core/src/platform/github.rs:6040`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6040) |
| 413 | should autodetect email/user on default endpoint with github app | pending | — |
| 503 | should throw error when cant request app information on default endpoint with github app | pending | — |
| 510 | should autodetect email/user on custom endpoint with github app | pending | — |
| 537 | should autodetect email/user on ghe cloud endpoint with github app | pending | — |
| 563 | should support custom endpoint | ported | [`crates/renovate-core/src/platform/github.rs:2602`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2602) |
| 587 | should support custom endpoint without version | ported | [`crates/renovate-core/src/platform/github.rs:2621`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2621) |
| 613 | should return an array of repos | ported | [`crates/renovate-core/src/platform/github.rs:2722`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2722) |
| 636 | should filters repositories by topics | ported | [`crates/renovate-core/src/platform/github.rs:3113`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3113) |
| 663 | should return an array of repos when using github app endpoint | ported | [`crates/renovate-core/src/platform/github.rs:5144`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5144) |
| 690 | should return an array of repos when using github app installation token | ported | [`crates/renovate-core/src/platform/github.rs:3521`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3521) |
| 801 | should squash | ported | [`crates/renovate-core/src/platform/github.rs:6469`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6469) |
| 809 | no token | ported | [`crates/renovate-core/src/platform/github.rs:5963`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5963) |
| 817 | app token | ported | [`crates/renovate-core/src/platform/github.rs:6245`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6245) |
| 826 | should fork when using forktoken | pending | — |
| 844 | should throw if fork needed but forkcreation=false | pending | — |
| 859 | throws if the repo is a fork | pending | — |
| 873 | throws when cannot fork due to username error | pending | — |
| 888 | throws when listing forks with 404 | pending | — |
| 901 | throws when listing forks with 500 | pending | — |
| 914 | throws when error creating fork | pending | — |
| 932 | should update fork when using forktoken and forkorg | pending | — |
| 944 | detects fork default branch mismatch | pending | — |
| 960 | should merge | ported | [`crates/renovate-core/src/platform/github.rs:6494`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6494) |
| 989 | should rebase | ported | [`crates/renovate-core/src/platform/github.rs:6519`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6519) |
| 1016 | should not guess at merge | ported | [`crates/renovate-core/src/platform/github.rs:6544`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6544) |
| 1036 | should throw error if archived | ported | [`crates/renovate-core/src/platform/github.rs:3036`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3036) |
| 1060 | throws not-found | ported | [`crates/renovate-core/src/platform/github.rs:2704`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2704) |
| 1067 | throws unexpected graphql errors | ported | [`crates/renovate-core/src/platform/github.rs:5270`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5270) |
| 1084 | throws graphql rate limit error | pending | — |
| 1101 | should throw error if renamed | ported | [`crates/renovate-core/src/platform/github.rs:5284`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5284) |
| 1124 | should not be case sensitive | ported | [`crates/renovate-core/src/platform/github.rs:5895`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5895) |
| 1151 | should detect repoforcerebase | ported | [`crates/renovate-core/src/platform/github.rs:4742`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4742) |
| 1185 | should handle 404 | ported | [`crates/renovate-core/src/platform/github.rs:3138`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3138) |
| 1198 | should handle 403 | ported | [`crates/renovate-core/src/platform/github.rs:3158`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3158) |
| 1211 | should throw 401 | ported | [`crates/renovate-core/src/platform/github.rs:2587`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2587) |
| 1225 | should return empty object when parentrepo is set | ported | [`crates/renovate-core/src/platform/github.rs:5060`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5060) |
| 1245 | should ignore non_fast_forward ruleset for determining rebase | ported | [`crates/renovate-core/src/platform/github.rs:4840`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4840) |
| 1269 | should detect strict required status checks ruleset | ported | [`crates/renovate-core/src/platform/github.rs:2841`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2841) |
| 1288 | should continue if no expected rulesets have been found | ported | [`crates/renovate-core/src/platform/github.rs:4893`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4893) |
| 1309 | should abort and throws on internal error | ported | [`crates/renovate-core/src/platform/github.rs:4920`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4920) |
| 1320 | should fallback to legacy branch protection when rulesets not found | ported | [`crates/renovate-core/src/platform/github.rs:4940`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4940) |
| 1337 | should return false when no force rebase rules found | ported | [`crates/renovate-core/src/platform/github.rs:2987`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2987) |
| 1360 | should return cached result on subsequent calls | ported | [`crates/renovate-core/src/platform/github.rs:4998`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4998) |
| 1385 | should return cached false result on subsequent calls | ported | [`crates/renovate-core/src/platform/github.rs:5030`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5030) |
| 1459 | fetches single page | ported | [`crates/renovate-core/src/platform/github.rs:4036`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4036) |
| 1470 | fetches multiple pages | ported | [`crates/renovate-core/src/platform/github.rs:5845`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5845) |
| 1489 | synchronizes cache | pending | — |
| 1542 | compacts body from response | pending | — |
| 1587 | filters prs by renovate username when no forktoken or ignoreprauthor | pending | — |
| 1602 | fetches all prs when forktoken is set | pending | — |
| 1629 | fetches all prs when ignoreprauthor is set | pending | — |
| 1648 | stops sync early when non-renovate prs dominate | pending | — |
| 1694 | advances watermark from unfiltered page so next sync is cheaper | pending | — |
| 1751 | derives cutoff from cached items when lastmodified is missing | pending | — |
| 1817 | stops at max sync pages | pending | — |
| 1861 | stops at custom max sync pages | pending | — |
| 1906 | reconciles mixed pages with both renovate and non-renovate prs | pending | — |
| 1957 | continues past timestamp tie at page boundary | pending | — |
| 2007 | should return null if no pr exists | ported | [`crates/renovate-core/src/platform/github.rs:5785`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5785) |
| 2021 | should cache and return the pr object | ported | [`crates/renovate-core/src/platform/github.rs:3868`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3868) |
| 2056 | should reopen autoclosed pr | pending | — |
| 2091 | force pushes when local sha differs from pr sha | pending | — |
| 2135 | aborts reopening if branch recreation fails | pending | — |
| 2164 | aborts reopening if pr reopening fails | pending | — |
| 2188 | should pass through success | ported | [`crates/renovate-core/src/platform/github.rs:2839`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2839) |
| 2204 | should not consider internal statuses as success | ported | [`crates/renovate-core/src/platform/github.rs:2840`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2840) |
| 2226 | should pass through failed | ported | [`crates/renovate-core/src/platform/github.rs:2955`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2955) |
| 2242 | defaults to pending | ported | [`crates/renovate-core/src/platform/github.rs:2986`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2986) |
| 2257 | should fail if a check run has failed | ported | [`crates/renovate-core/src/platform/github.rs:3233`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3233) |
| 2289 | should succeed if no status and all passed check runs | ported | [`crates/renovate-core/src/platform/github.rs:3264`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3264) |
| 2327 | should fail if a check run is pending | ported | [`crates/renovate-core/src/platform/github.rs:3293`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3293) |
| 2360 | returns state if found | ported | [`crates/renovate-core/src/platform/github.rs:4674`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4674) |
| 2389 | returns null | ported | [`crates/renovate-core/src/platform/github.rs:2702`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2702) |
| 2415 | returns yellow if state not present in context object | ported | [`crates/renovate-core/src/platform/github.rs:4720`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4720) |
| 2434 | returns if already set | ported | [`crates/renovate-core/src/platform/github.rs:5073`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5073) |
| 2459 | sets branch status | ported | [`crates/renovate-core/src/platform/github.rs:5101`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5101) |
| 2505 | returns null if issues disabled | ported | [`crates/renovate-core/src/platform/github.rs:4260`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4260) |
| 2513 | returns issue | ported | [`crates/renovate-core/src/platform/github.rs:4275`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4275) |
| 2533 | returns null if issue not found | ported | [`crates/renovate-core/src/platform/github.rs:4303`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4303) |
| 2542 | logs debug message if issue deleted | ported | [`crates/renovate-core/src/platform/github.rs:6987`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6987) |
| 2557 | returns null if no issue | ported | [`crates/renovate-core/src/platform/github.rs:4318`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4318) |
| 2594 | finds issue | ported | [`crates/renovate-core/src/platform/github.rs:4333`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4333) |
| 2647 | creates issue | ported | [`crates/renovate-core/src/platform/github.rs:4359`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4359) |
| 2697 | creates issue if not ensuring only once | ported | [`crates/renovate-core/src/platform/github.rs:6788`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6788) |
| 2741 | does not create issue if ensuring only once | ported | [`crates/renovate-core/src/platform/github.rs:6789`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6789) |
| 2783 | creates issue with labels | ported | [`crates/renovate-core/src/platform/github.rs:4383`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4383) |
| 2819 | closes others if ensuring only once | ported | [`crates/renovate-core/src/platform/github.rs:6824`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6824) |
| 2872 | updates issue | ported | [`crates/renovate-core/src/platform/github.rs:4414`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4414) |
| 2931 | updates issue with labels | ported | [`crates/renovate-core/src/platform/github.rs:4439`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4439) |
| 2991 | skips update if unchanged | ported | [`crates/renovate-core/src/platform/github.rs:2827`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2827) |
| 3035 | deletes if duplicate | ported | [`crates/renovate-core/src/platform/github.rs:6871`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6871) |
| 3079 | creates issue if reopen flag false and issue is not open | ported | [`crates/renovate-core/src/platform/github.rs:6906`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6906) |
| 3132 | does not create issue if reopen flag false and issue is already open | ported | [`crates/renovate-core/src/platform/github.rs:6950`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6950) |
| 3179 | closes issue | ported | [`crates/renovate-core/src/platform/github.rs:4464`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4464) |
| 3223 | swallows 410 gone when the issue was deleted on the platform | ported | [`crates/renovate-core/src/platform/github.rs:4496`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4496) |
| 3254 | swallows 404 not found when the issue was deleted on the platform | ported | [`crates/renovate-core/src/platform/github.rs:4481`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4481) |
| 3285 | rethrows non-deletion errors | ported | [`crates/renovate-core/src/platform/github.rs:4511`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4511) |
| 3318 | should delete the label | ported | [`crates/renovate-core/src/platform/github.rs:5475`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5475) |
| 3328 | should add the given assignees to the issue | ported | [`crates/renovate-core/src/platform/github.rs:5494`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5494) |
| 3344 | should retry on 404 and succeed | ported | [`crates/renovate-core/src/platform/github.rs:5519`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5519) |
| 3364 | should throw after 3 consecutive 404 responses | ported | [`crates/renovate-core/src/platform/github.rs:5545`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5545) |
| 3374 | should throw immediately on non-404 errors | ported | [`crates/renovate-core/src/platform/github.rs:7301`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7301) |
| 3386 | should add the given reviewers to the pr | ported | [`crates/renovate-core/src/platform/github.rs:5567`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5567) |
| 3398 | add comment if not found | ported | [`crates/renovate-core/src/platform/github.rs:4530`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4530) |
| 3417 | adds comment if found in closed pr list | ported | [`crates/renovate-core/src/platform/github.rs:6612`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6612) |
| 3445 | add updates comment if necessary | ported | [`crates/renovate-core/src/platform/github.rs:4551`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4551) |
| 3464 | skips comment | ported | [`crates/renovate-core/src/platform/github.rs:6695`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6695) |
| 3481 | handles comment with no description | ported | [`crates/renovate-core/src/platform/github.rs:6766`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6766) |
| 3500 | deletes comment by topic if found | ported | [`crates/renovate-core/src/platform/github.rs:4568`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4568) |
| 3519 | deletes comment by content if found | ported | [`crates/renovate-core/src/platform/github.rs:6671`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6671) |
| 3540 | finds pr by branch name | ported | [`crates/renovate-core/src/platform/github.rs:4035`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4035) |
| 3582 | finds pr with non-open state | ported | [`crates/renovate-core/src/platform/github.rs:4064`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4064) |
| 3611 | skips pr with non-matching state | ported | [`crates/renovate-core/src/platform/github.rs:4095`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4095) |
| 3637 | skips prs from forks | ported | [`crates/renovate-core/src/platform/github.rs:4125`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4125) |
| 3662 | skips pr with non-matching title | ported | [`crates/renovate-core/src/platform/github.rs:4172`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4172) |
| 3687 | caches pr list | ported | [`crates/renovate-core/src/platform/github.rs:4201`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4201) |
| 3722 | finds pr from other authors | ported | [`crates/renovate-core/src/platform/github.rs:4230`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4230) |
| 3752 | returns null if no pr found - (includeotherauthors) | ported | [`crates/renovate-core/src/platform/github.rs:5784`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5784) |
| 3769 | should create and return a pr object | ported | [`crates/renovate-core/src/platform/github.rs:2749`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2749) |
| 3791 | should use defaultbranch | ported | [`crates/renovate-core/src/platform/github.rs:2750`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2750) |
| 3809 | should create a draftpr if set in the settings | ported | [`crates/renovate-core/src/platform/github.rs:5926`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5926) |
| 3849 | should allow maintainer edits if explicitly enabled via options | ported | [`crates/renovate-core/src/platform/github.rs:6269`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6269) |
| 3873 | should allow maintainer edits if not explicitly set | ported | [`crates/renovate-core/src/platform/github.rs:6306`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6306) |
| 3894 | should disallow maintainer edits if explicitly disabled | ported | [`crates/renovate-core/src/platform/github.rs:6343`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6343) |
| 4009 | should skip automerge if disabled in repo settings | ported | [`crates/renovate-core/src/platform/github.rs:7274`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7274) |
| 4022 | should skip automerge if ghe <3.3.0 | pending | — |
| 4057 | should perform automerge if ghe >=3.3.0 | pending | — |
| 4103 | should set automatic merge | ported | [`crates/renovate-core/src/platform/github.rs:7031`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7031) |
| 4118 | should handle graphql errors | ported | [`crates/renovate-core/src/platform/github.rs:3037`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3037) |
| 4131 | should handle rest api errors | ported | [`crates/renovate-core/src/platform/github.rs:2784`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2784) |
| 4144 | should pass commit message as commitheadline and commitbody for squash merge | pending | — |
| 4175 | should pass commit message as commitheadline and commitbody for merge commit | pending | — |
| 4209 | should pass multi-line commit message body for squash merge | pending | — |
| 4242 | should not pass commit message headline/body for rebase merge | pending | — |
| 4287 | should set the milestone on the pr | ported | [`crates/renovate-core/src/platform/github.rs:6380`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6380) |
| 4319 | should log a warning but not throw on error | ported | [`crates/renovate-core/src/platform/github.rs:6423`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6423) |
| 4381 | should return null if no prno is passed | ported | [`crates/renovate-core/src/platform/github.rs:3858`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3858) |
| 4386 | should return pr | ported | [`crates/renovate-core/src/platform/github.rs:3867`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3867) |
| 4429 | should return closed pr | ported | [`crates/renovate-core/src/platform/github.rs:3893`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3893) |
| 4454 | should return merged pr | ported | [`crates/renovate-core/src/platform/github.rs:3917`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3917) |
| 4480 | should return null if no pr is returned from github | ported | [`crates/renovate-core/src/platform/github.rs:3943`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3943) |
| 4495 | should return a pr object - 0 | ported | [`crates/renovate-core/src/platform/github.rs:3958`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3958) |
| 4521 | should return a pr object - 1 | ported | [`crates/renovate-core/src/platform/github.rs:3984`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3984) |
| 4557 | should return a pr object - 2 | ported | [`crates/renovate-core/src/platform/github.rs:4010`](../../../../../../../crates/renovate-core/src/platform/github.rs#L4010) |
| 4591 | should update the pr | ported | [`crates/renovate-core/src/platform/github.rs:2809`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2809) |
| 4605 | should update and close the pr | ported | [`crates/renovate-core/src/platform/github.rs:3018`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3018) |
| 4620 | should update target branch | ported | [`crates/renovate-core/src/platform/github.rs:2810`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2810) |
| 4636 | should add and remove labels | ported | [`crates/renovate-core/src/platform/github.rs:6571`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6571) |
| 4676 | warns if adding labels failed | ported | [`crates/renovate-core/src/platform/github.rs:6590`](../../../../../../../crates/renovate-core/src/platform/github.rs#L6590) |
| 4780 | should set automatic merge | ported | [`crates/renovate-core/src/platform/github.rs:7031`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7031) |
| 4798 | handles unknown error | ported | [`crates/renovate-core/src/platform/github.rs:7054`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7054) |
| 4820 | should merge the pr | ported | [`crates/renovate-core/src/platform/github.rs:7030`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7030) |
| 4852 | should handle merge error | ported | [`crates/renovate-core/src/platform/github.rs:7053`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7053) |
| 4873 | should handle merge block | ported | [`crates/renovate-core/src/platform/github.rs:7073`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7073) |
| 4895 | should handle approvers required | ported | [`crates/renovate-core/src/platform/github.rs:7093`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7093) |
| 4917 | should warn if automergestrategy is not supported | ported | [`crates/renovate-core/src/platform/github.rs:7132`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7132) |
| 4936 | should use configured automergestrategy | ported | [`crates/renovate-core/src/platform/github.rs:7113`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7113) |
| 4963 | returns updated pr body | ported | [`crates/renovate-core/src/platform/github.rs:3591`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3591) |
| 4969 | returns not-updated pr body for ghe | ported | [`crates/renovate-core/src/platform/github.rs:3600`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3600) |
| 4996 | should try squash first | ported | [`crates/renovate-core/src/platform/github.rs:7151`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7151) |
| 5015 | should try merge after squash | ported | [`crates/renovate-core/src/platform/github.rs:7174`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7174) |
| 5036 | should try rebase after merge | ported | [`crates/renovate-core/src/platform/github.rs:7208`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7208) |
| 5061 | should give up | ported | [`crates/renovate-core/src/platform/github.rs:7253`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7253) |
| 5090 | avoids fetching if repo has vulnerability alerts disabled | ported | [`crates/renovate-core/src/platform/github.rs:5293`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5293) |
| 5100 | returns empty if error | ported | [`crates/renovate-core/src/platform/github.rs:5359`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5359) |
| 5113 | returns array if found | ported | [`crates/renovate-core/src/platform/github.rs:5203`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5203) |
| 5163 | returns empty if disabled | ported | [`crates/renovate-core/src/platform/github.rs:5301`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5301) |
| 5177 | handles network error | ported | [`crates/renovate-core/src/platform/github.rs:5309`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5309) |
| 5191 | calls logger.debug with only items that include securityvulnerability | ported | [`crates/renovate-core/src/platform/github.rs:5177`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5177) |
| 5247 | returns normalized names for pip ecosystem | ported | [`crates/renovate-core/src/platform/github.rs:5322`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5322) |
| 5283 | handles pagination correctly | ported | [`crates/renovate-core/src/platform/github.rs:5343`](../../../../../../../crates/renovate-core/src/platform/github.rs#L5343) |
| 5382 | returns null | ported | [`crates/renovate-core/src/platform/github.rs:2702`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2702) |
| 5393 | returns file content | ported | [`crates/renovate-core/src/platform/github.rs:2639`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2639) |
| 5405 | returns file content in json5 format | ported | [`crates/renovate-core/src/platform/github.rs:3064`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3064) |
| 5422 | returns file content from given repo | ported | [`crates/renovate-core/src/platform/github.rs:3089`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3089) |
| 5434 | returns file content from branch or tag | ported | [`crates/renovate-core/src/platform/github.rs:3790`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3790) |
| 5446 | throws on malformed json | ported | [`crates/renovate-core/src/platform/github.rs:3211`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3211) |
| 5456 | throws on errors | ported | [`crates/renovate-core/src/platform/github.rs:3222`](../../../../../../../crates/renovate-core/src/platform/github.rs#L3222) |
| 5482 | returns null if pre-commit phase has failed | ported | [`crates/renovate-core/src/platform/github.rs:2703`](../../../../../../../crates/renovate-core/src/platform/github.rs#L2703) |
| 5502 | returns null on rest error | ported | [`crates/renovate-core/src/platform/github.rs:7321`](../../../../../../../crates/renovate-core/src/platform/github.rs#L7321) |
| 5517 | commits and returns sha string | pending | — |
| 5546 | performs rebase | pending | — |
| 5575 | continues if rebase fails due to 422 | pending | — |
| 5606 | aborts if rebase fails due to non-422 | pending | — |
| 5635 | aborts if commit sha doesn't exist | pending | — |

