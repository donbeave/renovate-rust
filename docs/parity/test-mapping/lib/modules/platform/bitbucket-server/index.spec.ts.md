# `lib/modules/platform/bitbucket-server/index.spec.ts`

[← `platform/bitbucket-server`](../../../../_by-module/platform/bitbucket-server.md) · [all modules](../../../../README.md)

**3/139 ported** (136 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 240 | should throw if no endpoint | pending | — |
| 245 | should throw if no username/password/token | pending | — |
| 252 | should throw if password and token is set | pending | — |
| 264 | should not throw if username/password | pending | — |
| 275 | should not throw if token | pending | — |
| 285 | should throw if version could not be fetched | pending | — |
| 307 | should not throw if user info fetch fails | pending | — |
| 333 | should skip users api call when gitauthor is configured | pending | — |
| 351 | should skip users api call when no username | pending | — |
| 367 | should fetch user info if token with username | pending | — |
| 389 | should collect username from headers if token with no username | pending | — |
| 411 | should use fallback gitauthor if user info has empty email address | pending | — |
| 442 | should init | pending | — |
| 463 | returns repos | pending | — |
| 482 | works | pending | — |
| 501 | no git url | pending | — |
| 524 | giturl ssh returns ssh url | pending | — |
| 553 | giturl endpoint returns generates endpoint url | pending | — |
| 586 | giturl default returns http from api with injected auth | pending | — |
| 620 | uses ssh url from api if http not in api response | pending | — |
| 644 | uses http url from api with injected auth if http url in api response | pending | — |
| 673 | generates url if api does not contain clone links | pending | — |
| 701 | throws repository_empty if there is no default branch | pending | — |
| 720 | returns false on missing mergeconfig | pending | — |
| 734 | returns false on missing defaultstrategy | pending | — |
| 750 | _(it.each / template — verify manually)_ | ? | — |
| 771 | _(it.each / template — verify manually)_ | ? | — |
| 794 | does not throw | pending | — |
| 801 | does not throw | pending | — |
| 817 | sends the reviewer name as a reviewer | pending | — |
| 834 | throws not-found 1 | pending | — |
| 841 | throws not-found 2 | pending | — |
| 854 | throws not-found 3 | pending | — |
| 871 | does not throws repository-changed after 1 try | pending | — |
| 890 | does not throws repository-changed after 2 tries | pending | — |
| 910 | throws repository-changed after 3 tries | pending | — |
| 928 | deals with invalid reviewers correctly | pending | — |
| 984 | aborts instead of infinite recursion when invalid reviewers cannot be filtered | pending | — |
| 1023 | deals correctly with resolving reviewers | pending | — |
| 1074 | throws | pending | — |
| 1092 | throws when lookup fails | pending | — |
| 1113 | return empty array when no results found | pending | — |
| 1131 | return only active users | pending | — |
| 1156 | only returns exact matches | pending | — |
| 1187 | returns multiple exact matches | pending | — |
| 1223 | does not throw | pending | — |
| 1229 | does not throw | pending | — |
| 1244 | add comment if not found 1 | pending | — |
| 1287 | add comment if not found 2 | pending | — |
| 1330 | add updates comment if necessary 1 | pending | — |
| 1379 | add updates comment if necessary 2 | pending | — |
| 1422 | skips comment 1 | pending | — |
| 1461 | skips comment 2 | pending | — |
| 1501 | does not throw | pending | — |
| 1539 | deletes comment by topic if found | pending | — |
| 1588 | deletes comment by content if found | pending | — |
| 1637 | deletes nothing | pending | — |
| 1678 | has pr | pending | — |
| 1693 | has pr | pending | — |
| 1713 | has no pr | pending | — |
| 1729 | has no existing pr | pending | — |
| 1747 | has pr | pending | — |
| 1767 | has no pr | pending | — |
| 1787 | finds pr from other authors | pending | — |
| 1812 | returns null if no pr found - (includeotherauthors) | pending | — |
| 1833 | posts pr | pending | — |
| 1866 | posts pr default branch | pending | — |
| 1900 | should use platform automerge | pending | — |
| 1939 | platform-native automerge returns early if useplatformautomerge is false | pending | — |
| 1970 | platform-native automerge returns early if bitbucket server <= 8.15.0 is used | pending | — |
| 2004 | platform-native automerge catches errors gracefully | pending | — |
| 2049 | should reattempt automerge | pending | — |
| 2071 | handles unknown error | pending | — |
| 2087 | handles missing prno | pending | — |
| 2102 | returns null for no prno | pending | — |
| 2107 | gets a pr | pending | — |
| 2118 | canrebase | pending | — |
| 2138 | gets a closed pr | pending | — |
| 2158 | puts pr | pending | — |
| 2194 | closes pr | pending | — |
| 2231 | re-opens pr | pending | — |
| 2268 | throws not-found 1 | pending | — |
| 2279 | throws not-found 2 | pending | — |
| 2291 | throws not-found 3 | pending | — |
| 2308 | handles invalid users gracefully by retrying without invalid reviewers | pending | — |
| 2364 | throws repository-changed | pending | — |
| 2381 | throws | pending | — |
| 2400 | posts merge | pending | — |
| 2420 | throws not-found 1 | pending | — |
| 2429 | throws not-found 2 | pending | — |
| 2445 | throws not-found 3 | pending | — |
| 2465 | throws conflicted | pending | — |
| 2485 | unknown error | pending | — |
| 2507 | returns diff files | ported | `crates/renovate-core/src/platform/bitbucket_server.rs:727` |
| 2515 | sanitizes html comments in the body | ported | `crates/renovate-core/src/platform/bitbucket_server.rs:739` |
| 2530 | resizes mend.io merge confidence badges | ported | `crates/renovate-core/src/platform/bitbucket_server.rs:750` |
| 2539 | should be success | pending | — |
| 2554 | should be pending | pending | — |
| 2581 | should be failed | pending | — |
| 2604 | throws repository-changed | pending | — |
| 2614 | should be success | pending | — |
| 2636 | should be pending | pending | — |
| 2658 | should be failure | pending | — |
| 2680 | should be null | pending | — |
| 2708 | should be success 1 | pending | — |
| 2738 | should be success 2 | pending | — |
| 2768 | should be success 3 | pending | — |
| 2798 | should be success 4 | pending | — |
| 2828 | should be success 5 | pending | — |
| 2853 | should be success 6 | pending | — |
| 2876 | returns file content | pending | — |
| 2891 | returns file content in json5 format | pending | — |
| 2911 | returns file content from given repo | pending | — |
| 2926 | returns file content from branch or tag | pending | — |
| 2945 | throws on malformed json | pending | — |
| 2958 | throws on long content | pending | — |
| 2971 | throws on errors | pending | — |
| 2982 | ignores comments and empty lines | pending | — |
| 2992 | parses usernames with escaped spaces | pending | — |
| 3000 | parses groups with escaped spaces | pending | — |
| 3013 | supports reviewer groups with modifiers) | pending | — |
| 3027 | matches paths correctly using glob patterns | pending | — |
| 3044 | respects bottom-to-top rule precedence | pending | — |
| 3054 | supports rules with no owners (ownership ignored) | pending | — |
| 3064 | unescapes multiple escaped spaces correctly | pending | — |
| 3073 | returns input when it is not a group | pending | — |
| 3080 | returns only active users from the matching reviewer group | pending | — |
| 3124 | returns empty array if group is not found | pending | — |
| 3153 | returns empty array if api call fails | pending | — |
| 3167 | returns empty array if all users in group are inactive | pending | — |
| 3200 | prefers repository-level reviewer group over project-level group with same name | pending | — |
| 3247 | uses project-level group when repository-level group is not available | pending | — |
| 3280 | deals with not found groups correctly | pending | — |
| 3306 | handles random without number correctly | pending | — |
| 3353 | handles random with number correctly | pending | — |
| 3402 | handles non-existent modifier correctly | pending | — |
| 3451 | handles paginated responses and finds matching group in next page | pending | — |
| 3559 | giturl endpoint generates url without endpoint path | pending | — |
| 3584 | generates url without endpoint path if api does not contain clone links | pending | — |

