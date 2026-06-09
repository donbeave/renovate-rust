# `lib/config/validation.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**126/132 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | _(it.each / template — verify manually)_ | ? | — |
| 26 | returns the deprecationmsg for `dnscache` as a warning | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2877`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2877) |
| 47 | allow enabled field in vulnerabilityalerts | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2892`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2892) |
| 61 | catches global options in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2901`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2901) |
| 86 | catches global options in inherit config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2929`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2929) |
| 107 | only warns for actual globals in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2939`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2939) |
| 124 | does not warn for valid inheritconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2949`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2949) |
| 135 | does not warn for valid platformconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2956`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2956) |
| 147 | warns for invalid platformconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2964`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2964) |
| 156 | catches invalid templates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2971`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2971) |
| 165 | accepts templates referencing runtime-only fields | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3000`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3000) |
| 178 | catches invalid jsonata expressions | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2979`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2979) |
| 192 | catches invalid allowedversions regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3070`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3070) |
| 222 | catches invalid matchcurrentvalue | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3088`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3088) |
| 256 | catches invalid matchnewvalue | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3106`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3106) |
| 290 | validates matchbasebranches | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3124`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3124) |
| 308 | catches invalid matchbasebranches when basebranchpatterns is not defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3138`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3138) |
| 325 | catches invalid matchcurrentversion regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3173`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3173) |
| 360 | catches invalid customdatasources content | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3191`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3191) |
| 397 | validates invalid statuschecknames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3230`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3230) |
| 421 | catches invalid customdatasources record type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3250`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3250) |
| 436 | catches invalid basebranchpatterns regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3263`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3263) |
| 449 | returns nested errors | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4795`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4795) |
| 479 | included managers of the wrong type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3279`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3279) |
| 497 | _(it.each / template — verify manually)_ | ? | — |
| 516 | _(it.each / template — verify manually)_ | ? | — |
| 536 | errors for all types | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4820`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4820) |
| 571 | selectors outside packagerules array trigger errors | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4852`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4852) |
| 601 | ignore packagerule nesting validation for presets | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4879`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4879) |
| 621 | errors for unsafe managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3315`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3315) |
| 640 | validates regex for each managerfilepatterns of format regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3329`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3329) |
| 662 | errors if custommanager has empty managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3348`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3348) |
| 688 | errors if no custommanager customtype | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3362`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3362) |
| 716 | errors if invalid custommanager customtype | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3383`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3383) |
| 745 | errors if empty custommanager matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3405`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3405) |
| 787 | errors if no custommanager managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4898`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4898) |
| 806 | validates regex for each matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3435`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3435) |
| 828 | error if no fileformat in custom jsonata manager | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3455`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3455) |
| 854 | validates jsonata query for each matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3475`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3475) |
| 884 | validates all possible regex manager options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3499`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3499) |
| 903 | passes if custommanager fields are present | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3516`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3516) |
| 935 | errors if extra custommanager fields are present | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3546`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3546) |
| 958 | errors if custommanager fields are missing | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3567`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3567) |
| 980 | errors if custommanager fields are missing: jsonatamanager | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3586`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3586) |
| 1013 | ignore keys | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3611`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3611) |
| 1026 | validates timezone preset | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3619`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3619) |
| 1040 | can contain a valid tool name for containerbase | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3630`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3630) |
| 1055 | can contain a constraint for a non-containerbase tool | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3639`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3639) |
| 1070 | warns if an unsupported constraint is specified | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3648`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3648) |
| 1092 | warns if a constraint is not valid | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3663`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3663) |
| 1113 | errors if constraints is a malformed object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3678`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3678) |
| 1133 | errors if constraints is a malformed array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3693`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3693) |
| 1155 | cannot contain a valid tool name for containerbase | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3707`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3707) |
| 1177 | can contain a constraint for a non-containerbase tool | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3639`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3639) |
| 1192 | cannot contain an additional constraint name with an invalid versioning scheme | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3735`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3735) |
| 1213 | can contain an additional constraint name with a regex versioning scheme | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3752`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3752) |
| 1229 | cannot contain an unsupported constraint | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3763`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3763) |
| 1251 | errors if constraintsversioning is a malformed object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3780`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3780) |
| 1273 | errors if constraintsversioning is a malformed array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3797`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3797) |
| 1294 | validates object with ignored children | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3812`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3812) |
| 1307 | validates valid registryalias objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3820`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3820) |
| 1322 | errors if registryaliases depth is more than 1 | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3836`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3836) |
| 1344 | errors if registryaliases have invalid value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3852`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3852) |
| 1365 | errors if managerfilepatterns has wrong parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3868`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3868) |
| 1408 | errors if manager objects are nested | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3892`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3892) |
| 1428 | warns if hosttype has the wrong parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3903`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3903) |
| 1442 | validates preset values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3911`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3911) |
| 1455 | errors on invalid preset syntax | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3919`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3919) |
| 1472 | skips preset syntax validation for templates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3015`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3015) |
| 1485 | warns if only selectors in packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3930`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3930) |
| 1499 | errors if invalid combinations in packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3941`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3941) |
| 1518 | warns when registryurls is set at the top level of repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3952`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3952) |
| 1533 | warns when defaultregistryurls is set at the top level of repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3969`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3969) |
| 1548 | warns on nested group packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3986`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3986) |
| 1567 | does not error on use of `global:` presets in `globalextends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4000`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4000) |
| 1580 | does not error on use of `global:` presets in global `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4009`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4009) |
| 1593 | errors on use of `global:` presets in inherit `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4017`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4017) |
| 1606 | errors on use of `global:` presets in repo `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4025`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4025) |
| 1620 | warns if customenvvariables are found in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4033`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4033) |
| 1639 | errors if schedule is cron and has no * minutes | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4049`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4049) |
| 1657 | errors if invalid matchhost values in hostrules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4062`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4062) |
| 1699 | errors if forbidden header in hostrules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4088`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4088) |
| 1727 | errors if headers values are not string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4109`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4109) |
| 1754 | errors if allowedheaders is empty | pending | — |
| 1781 | catches invalid variable name in env config option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4128`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4128) |
| 1809 | catches env config option if configured inside a parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4149`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4149) |
| 1846 | catches when * or ** is combined with others patterns in a regexorglob option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4169`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4169) |
| 1874 | catches when negative number is used for integer type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4216`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4216) |
| 1888 | validates prpriority | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4228`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4228) |
| 1909 | errors if no bumpversion filepattern is provided | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4244`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4244) |
| 1935 | errors if no matchstrings are provided for bumpversion | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4267`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4267) |
| 1959 | allow bumpversion | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4284`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4284) |
| 1985 | returns errors for invalid options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4301`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4301) |
| 2007 | validates hostrules.headers | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4317`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4317) |
| 2027 | errors if hostrules.headers is defined but allowedheaders is not | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4334`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4334) |
| 2051 | validates env | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4352`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4352) |
| 2066 | handles prefixed onboardingconfigfilename | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4363`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4363) |
| 2080 | allows unique onboardingconfigfilename if it is set in configfilenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4374`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4374) |
| 2093 | errors if env object is defined but allowedenv is empty or undefined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4388`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4388) |
| 2112 | validates env against the allowedenv regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4399`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4399) |
| 2127 | validates options with different type but defaultvalue=null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4410`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4410) |
| 2163 | binarysource=docker is deprecated | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4441`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4441) |
| 2180 | binarysource | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4454`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4454) |
| 2198 | binarysource | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4454`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4454) |
| 2215 | basedir | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4478`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4478) |
| 2231 | requireconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4488`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4488) |
| 2248 | dryrun | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4500`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4500) |
| 2265 | repositorycache | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4512`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4512) |
| 2282 | onboardingconfigfilename | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4524`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4524) |
| 2298 | onboardingconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4538`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4538) |
| 2325 | force | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4554`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4554) |
| 2350 | giturl | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4571`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4571) |
| 2368 | validates boolean type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4583`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4583) |
| 2388 | validates integer type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4593`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4593) |
| 2408 | validates array type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4603`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4603) |
| 2439 | validates object type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4618`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4618) |
| 2469 | warns if negative number is used for integer type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4636`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4636) |
| 2486 | warns on invalid customenvvariables objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4648`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4648) |
| 2507 | validates valid customenvvariables objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4661`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4661) |
| 2522 | validates options with different type but defaultvalue=null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4410`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4410) |
| 2542 | fails for missing reportpath if reporttype is "s3" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4692`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4692) |
| 2554 | validates reportpath if reporttype is "s3" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4700`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4700) |
| 2567 | fails for missing reportpath if reporttype is "file" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4711`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4711) |
| 2579 | validates reportpath if reporttype is "file" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4719`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4719) |
| 2592 | warns when registryurls is set at the top level of global config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4730`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4730) |
| 2607 | warns when defaultregistryurls is set at the top level of global config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4741`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4741) |
| 2622 | validates postupgradetasks.installtools tool names | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4752`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4752) |
| 2640 | rejects invalid postupgradetasks.installtools tool names | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4763`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4763) |
| 2664 | catches when * or ** is combined with others patterns in a regexorglob option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4169`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4169) |
| 2705 | errors when using an invalid cache namespace | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3028`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3028) |
| 2728 | allows a valid cache namespace | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3055`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3055) |
| 2744 | allows wildcards | pending | — |

