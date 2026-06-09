# `lib/config/validation.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**128/132 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | _(it.each / template — verify manually)_ | ? | — |
| 26 | returns the deprecationmsg for `dnscache` as a warning | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2876`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2876) |
| 47 | allow enabled field in vulnerabilityalerts | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2891`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2891) |
| 61 | catches global options in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2900`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2900) |
| 86 | catches global options in inherit config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2928`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2928) |
| 107 | only warns for actual globals in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2938`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2938) |
| 124 | does not warn for valid inheritconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2948`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2948) |
| 135 | does not warn for valid platformconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2955`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2955) |
| 147 | warns for invalid platformconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2963`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2963) |
| 156 | catches invalid templates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2970`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2970) |
| 165 | accepts templates referencing runtime-only fields | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2999`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2999) |
| 178 | catches invalid jsonata expressions | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2978`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2978) |
| 192 | catches invalid allowedversions regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3069`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3069) |
| 222 | catches invalid matchcurrentvalue | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3087`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3087) |
| 256 | catches invalid matchnewvalue | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3105`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3105) |
| 290 | validates matchbasebranches | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3123`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3123) |
| 308 | catches invalid matchbasebranches when basebranchpatterns is not defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3137`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3137) |
| 325 | catches invalid matchcurrentversion regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3172`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3172) |
| 360 | catches invalid customdatasources content | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3190`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3190) |
| 397 | validates invalid statuschecknames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3229`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3229) |
| 421 | catches invalid customdatasources record type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3249`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3249) |
| 436 | catches invalid basebranchpatterns regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3262`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3262) |
| 449 | returns nested errors | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4835`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4835) |
| 479 | included managers of the wrong type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3278`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3278) |
| 497 | _(it.each / template — verify manually)_ | ? | — |
| 516 | _(it.each / template — verify manually)_ | ? | — |
| 536 | errors for all types | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4860`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4860) |
| 571 | selectors outside packagerules array trigger errors | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4892`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4892) |
| 601 | ignore packagerule nesting validation for presets | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4919`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4919) |
| 621 | errors for unsafe managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3314`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3314) |
| 640 | validates regex for each managerfilepatterns of format regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3328`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3328) |
| 662 | errors if custommanager has empty managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3347`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3347) |
| 688 | errors if no custommanager customtype | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3361`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3361) |
| 716 | errors if invalid custommanager customtype | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3382`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3382) |
| 745 | errors if empty custommanager matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3404`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3404) |
| 787 | errors if no custommanager managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4938`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4938) |
| 806 | validates regex for each matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3434`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3434) |
| 828 | error if no fileformat in custom jsonata manager | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3454`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3454) |
| 854 | validates jsonata query for each matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3474`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3474) |
| 884 | validates all possible regex manager options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3498`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3498) |
| 903 | passes if custommanager fields are present | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3515`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3515) |
| 935 | errors if extra custommanager fields are present | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3545`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3545) |
| 958 | errors if custommanager fields are missing | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3566`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3566) |
| 980 | errors if custommanager fields are missing: jsonatamanager | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3585`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3585) |
| 1013 | ignore keys | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3610`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3610) |
| 1026 | validates timezone preset | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3618`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3618) |
| 1040 | can contain a valid tool name for containerbase | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3629`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3629) |
| 1055 | can contain a constraint for a non-containerbase tool | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3638`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3638) |
| 1070 | warns if an unsupported constraint is specified | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3647`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3647) |
| 1092 | warns if a constraint is not valid | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3662`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3662) |
| 1113 | errors if constraints is a malformed object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3677`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3677) |
| 1133 | errors if constraints is a malformed array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3692`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3692) |
| 1155 | cannot contain a valid tool name for containerbase | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3706`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3706) |
| 1177 | can contain a constraint for a non-containerbase tool | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3638`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3638) |
| 1192 | cannot contain an additional constraint name with an invalid versioning scheme | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3734`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3734) |
| 1213 | can contain an additional constraint name with a regex versioning scheme | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3751`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3751) |
| 1229 | cannot contain an unsupported constraint | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3762`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3762) |
| 1251 | errors if constraintsversioning is a malformed object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3779`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3779) |
| 1273 | errors if constraintsversioning is a malformed array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3796`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3796) |
| 1294 | validates object with ignored children | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3811`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3811) |
| 1307 | validates valid registryalias objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3819`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3819) |
| 1322 | errors if registryaliases depth is more than 1 | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3835`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3835) |
| 1344 | errors if registryaliases have invalid value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3851`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3851) |
| 1365 | errors if managerfilepatterns has wrong parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3867`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3867) |
| 1408 | errors if manager objects are nested | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3891`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3891) |
| 1428 | warns if hosttype has the wrong parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3902`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3902) |
| 1442 | validates preset values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3910`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3910) |
| 1455 | errors on invalid preset syntax | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3918`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3918) |
| 1472 | skips preset syntax validation for templates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3014`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3014) |
| 1485 | warns if only selectors in packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3929`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3929) |
| 1499 | errors if invalid combinations in packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3940`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3940) |
| 1518 | warns when registryurls is set at the top level of repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3951`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3951) |
| 1533 | warns when defaultregistryurls is set at the top level of repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3968`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3968) |
| 1548 | warns on nested group packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3985`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3985) |
| 1567 | does not error on use of `global:` presets in `globalextends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3999`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3999) |
| 1580 | does not error on use of `global:` presets in global `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4008`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4008) |
| 1593 | errors on use of `global:` presets in inherit `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4016`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4016) |
| 1606 | errors on use of `global:` presets in repo `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4024`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4024) |
| 1620 | warns if customenvvariables are found in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4032`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4032) |
| 1639 | errors if schedule is cron and has no * minutes | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4048`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4048) |
| 1657 | errors if invalid matchhost values in hostrules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4061`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4061) |
| 1699 | errors if forbidden header in hostrules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4087`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4087) |
| 1727 | errors if headers values are not string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4108`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4108) |
| 1754 | errors if allowedheaders is empty | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4127`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4127) |
| 1781 | catches invalid variable name in env config option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4168`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4168) |
| 1809 | catches env config option if configured inside a parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4189`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4189) |
| 1846 | catches when * or ** is combined with others patterns in a regexorglob option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4209`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4209) |
| 1874 | catches when negative number is used for integer type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4256`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4256) |
| 1888 | validates prpriority | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4268`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4268) |
| 1909 | errors if no bumpversion filepattern is provided | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4284`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4284) |
| 1935 | errors if no matchstrings are provided for bumpversion | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4307`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4307) |
| 1959 | allow bumpversion | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4324`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4324) |
| 1985 | returns errors for invalid options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4341`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4341) |
| 2007 | validates hostrules.headers | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4357`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4357) |
| 2027 | errors if hostrules.headers is defined but allowedheaders is not | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4374`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4374) |
| 2051 | validates env | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4392`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4392) |
| 2066 | handles prefixed onboardingconfigfilename | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4403`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4403) |
| 2080 | allows unique onboardingconfigfilename if it is set in configfilenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4414`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4414) |
| 2093 | errors if env object is defined but allowedenv is empty or undefined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4428`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4428) |
| 2112 | validates env against the allowedenv regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4439`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4439) |
| 2127 | validates options with different type but defaultvalue=null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4450`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4450) |
| 2163 | binarysource=docker is deprecated | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4481`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4481) |
| 2180 | binarysource | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4494`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4494) |
| 2198 | binarysource | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4494`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4494) |
| 2215 | basedir | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4518`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4518) |
| 2231 | requireconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4528`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4528) |
| 2248 | dryrun | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4540`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4540) |
| 2265 | repositorycache | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4552`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4552) |
| 2282 | onboardingconfigfilename | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4564`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4564) |
| 2298 | onboardingconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4578`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4578) |
| 2325 | force | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4594`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4594) |
| 2350 | giturl | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4611`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4611) |
| 2368 | validates boolean type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4623`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4623) |
| 2388 | validates integer type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4633`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4633) |
| 2408 | validates array type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4643`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4643) |
| 2439 | validates object type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4658`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4658) |
| 2469 | warns if negative number is used for integer type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4676`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4676) |
| 2486 | warns on invalid customenvvariables objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4688`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4688) |
| 2507 | validates valid customenvvariables objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4701`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4701) |
| 2522 | validates options with different type but defaultvalue=null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4450`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4450) |
| 2542 | fails for missing reportpath if reporttype is "s3" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4732`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4732) |
| 2554 | validates reportpath if reporttype is "s3" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4740`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4740) |
| 2567 | fails for missing reportpath if reporttype is "file" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4751`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4751) |
| 2579 | validates reportpath if reporttype is "file" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4759`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4759) |
| 2592 | warns when registryurls is set at the top level of global config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4770`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4770) |
| 2607 | warns when defaultregistryurls is set at the top level of global config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4781`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4781) |
| 2622 | validates postupgradetasks.installtools tool names | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4792`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4792) |
| 2640 | rejects invalid postupgradetasks.installtools tool names | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4803`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4803) |
| 2664 | catches when * or ** is combined with others patterns in a regexorglob option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4209`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4209) |
| 2705 | errors when using an invalid cache namespace | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3027`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3027) |
| 2728 | allows a valid cache namespace | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3054`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3054) |
| 2744 | allows wildcards | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4149`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4149) |

