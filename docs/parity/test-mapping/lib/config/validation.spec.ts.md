# `lib/config/validation.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**126/132 ported** (6 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | _(it.each / template — verify manually)_ | ? | — |
| 26 | returns the deprecationmsg for `dnscache` as a warning | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2875`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2875) |
| 47 | allow enabled field in vulnerabilityalerts | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2890`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2890) |
| 61 | catches global options in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2899`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2899) |
| 86 | catches global options in inherit config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2927`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2927) |
| 107 | only warns for actual globals in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2937`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2937) |
| 124 | does not warn for valid inheritconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2947`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2947) |
| 135 | does not warn for valid platformconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2954`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2954) |
| 147 | warns for invalid platformconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2962`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2962) |
| 156 | catches invalid templates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2969`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2969) |
| 165 | accepts templates referencing runtime-only fields | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2998`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2998) |
| 178 | catches invalid jsonata expressions | ported | [`crates/renovate-core/src/config/migrate_validate.rs:2977`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L2977) |
| 192 | catches invalid allowedversions regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3068`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3068) |
| 222 | catches invalid matchcurrentvalue | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3086`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3086) |
| 256 | catches invalid matchnewvalue | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3104`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3104) |
| 290 | validates matchbasebranches | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3122`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3122) |
| 308 | catches invalid matchbasebranches when basebranchpatterns is not defined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3136`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3136) |
| 325 | catches invalid matchcurrentversion regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3171`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3171) |
| 360 | catches invalid customdatasources content | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3189`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3189) |
| 397 | validates invalid statuschecknames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3228`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3228) |
| 421 | catches invalid customdatasources record type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3248`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3248) |
| 436 | catches invalid basebranchpatterns regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3261`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3261) |
| 449 | returns nested errors | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4793`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4793) |
| 479 | included managers of the wrong type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3277`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3277) |
| 497 | _(it.each / template — verify manually)_ | ? | — |
| 516 | _(it.each / template — verify manually)_ | ? | — |
| 536 | errors for all types | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4818`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4818) |
| 571 | selectors outside packagerules array trigger errors | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4850`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4850) |
| 601 | ignore packagerule nesting validation for presets | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4877`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4877) |
| 621 | errors for unsafe managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3313`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3313) |
| 640 | validates regex for each managerfilepatterns of format regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3327`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3327) |
| 662 | errors if custommanager has empty managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3346`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3346) |
| 688 | errors if no custommanager customtype | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3360`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3360) |
| 716 | errors if invalid custommanager customtype | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3381`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3381) |
| 745 | errors if empty custommanager matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3403`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3403) |
| 787 | errors if no custommanager managerfilepatterns | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4896`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4896) |
| 806 | validates regex for each matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3433`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3433) |
| 828 | error if no fileformat in custom jsonata manager | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3453`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3453) |
| 854 | validates jsonata query for each matchstrings | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3473`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3473) |
| 884 | validates all possible regex manager options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3497`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3497) |
| 903 | passes if custommanager fields are present | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3514`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3514) |
| 935 | errors if extra custommanager fields are present | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3544`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3544) |
| 958 | errors if custommanager fields are missing | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3565`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3565) |
| 980 | errors if custommanager fields are missing: jsonatamanager | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3584`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3584) |
| 1013 | ignore keys | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3609`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3609) |
| 1026 | validates timezone preset | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3617`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3617) |
| 1040 | can contain a valid tool name for containerbase | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3628`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3628) |
| 1055 | can contain a constraint for a non-containerbase tool | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3637`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3637) |
| 1070 | warns if an unsupported constraint is specified | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3646`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3646) |
| 1092 | warns if a constraint is not valid | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3661`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3661) |
| 1113 | errors if constraints is a malformed object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3676`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3676) |
| 1133 | errors if constraints is a malformed array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3691`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3691) |
| 1155 | cannot contain a valid tool name for containerbase | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3705`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3705) |
| 1177 | can contain a constraint for a non-containerbase tool | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3637`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3637) |
| 1192 | cannot contain an additional constraint name with an invalid versioning scheme | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3733`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3733) |
| 1213 | can contain an additional constraint name with a regex versioning scheme | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3750`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3750) |
| 1229 | cannot contain an unsupported constraint | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3761`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3761) |
| 1251 | errors if constraintsversioning is a malformed object | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3778`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3778) |
| 1273 | errors if constraintsversioning is a malformed array | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3795`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3795) |
| 1294 | validates object with ignored children | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3810`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3810) |
| 1307 | validates valid registryalias objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3818`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3818) |
| 1322 | errors if registryaliases depth is more than 1 | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3834`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3834) |
| 1344 | errors if registryaliases have invalid value | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3850`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3850) |
| 1365 | errors if managerfilepatterns has wrong parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3866`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3866) |
| 1408 | errors if manager objects are nested | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3890`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3890) |
| 1428 | warns if hosttype has the wrong parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3901`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3901) |
| 1442 | validates preset values | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3909`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3909) |
| 1455 | errors on invalid preset syntax | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3917`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3917) |
| 1472 | skips preset syntax validation for templates | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3013`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3013) |
| 1485 | warns if only selectors in packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3928`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3928) |
| 1499 | errors if invalid combinations in packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3939`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3939) |
| 1518 | warns when registryurls is set at the top level of repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3950`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3950) |
| 1533 | warns when defaultregistryurls is set at the top level of repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3967`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3967) |
| 1548 | warns on nested group packagerules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3984`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3984) |
| 1567 | does not error on use of `global:` presets in `globalextends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3998`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3998) |
| 1580 | does not error on use of `global:` presets in global `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4007`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4007) |
| 1593 | errors on use of `global:` presets in inherit `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4015`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4015) |
| 1606 | errors on use of `global:` presets in repo `extends` | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4023`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4023) |
| 1620 | warns if customenvvariables are found in repo config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4031`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4031) |
| 1639 | errors if schedule is cron and has no * minutes | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4047`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4047) |
| 1657 | errors if invalid matchhost values in hostrules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4060`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4060) |
| 1699 | errors if forbidden header in hostrules | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4086`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4086) |
| 1727 | errors if headers values are not string | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4107`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4107) |
| 1754 | errors if allowedheaders is empty | pending | — |
| 1781 | catches invalid variable name in env config option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4126`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4126) |
| 1809 | catches env config option if configured inside a parent | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4147`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4147) |
| 1846 | catches when * or ** is combined with others patterns in a regexorglob option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4167`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4167) |
| 1874 | catches when negative number is used for integer type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4214`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4214) |
| 1888 | validates prpriority | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4226`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4226) |
| 1909 | errors if no bumpversion filepattern is provided | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4242`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4242) |
| 1935 | errors if no matchstrings are provided for bumpversion | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4265`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4265) |
| 1959 | allow bumpversion | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4282`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4282) |
| 1985 | returns errors for invalid options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4299`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4299) |
| 2007 | validates hostrules.headers | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4315`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4315) |
| 2027 | errors if hostrules.headers is defined but allowedheaders is not | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4332`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4332) |
| 2051 | validates env | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4350`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4350) |
| 2066 | handles prefixed onboardingconfigfilename | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4361`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4361) |
| 2080 | allows unique onboardingconfigfilename if it is set in configfilenames | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4372`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4372) |
| 2093 | errors if env object is defined but allowedenv is empty or undefined | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4386`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4386) |
| 2112 | validates env against the allowedenv regex | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4397`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4397) |
| 2127 | validates options with different type but defaultvalue=null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4408`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4408) |
| 2163 | binarysource=docker is deprecated | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4439`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4439) |
| 2180 | binarysource | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4452`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4452) |
| 2198 | binarysource | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4452`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4452) |
| 2215 | basedir | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4476`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4476) |
| 2231 | requireconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4486`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4486) |
| 2248 | dryrun | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4498`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4498) |
| 2265 | repositorycache | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4510`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4510) |
| 2282 | onboardingconfigfilename | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4522`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4522) |
| 2298 | onboardingconfig | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4536`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4536) |
| 2325 | force | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4552`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4552) |
| 2350 | giturl | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4569`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4569) |
| 2368 | validates boolean type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4581`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4581) |
| 2388 | validates integer type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4591`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4591) |
| 2408 | validates array type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4601`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4601) |
| 2439 | validates object type options | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4616`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4616) |
| 2469 | warns if negative number is used for integer type | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4634`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4634) |
| 2486 | warns on invalid customenvvariables objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4646`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4646) |
| 2507 | validates valid customenvvariables objects | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4659`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4659) |
| 2522 | validates options with different type but defaultvalue=null | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4408`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4408) |
| 2542 | fails for missing reportpath if reporttype is "s3" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4690`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4690) |
| 2554 | validates reportpath if reporttype is "s3" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4698`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4698) |
| 2567 | fails for missing reportpath if reporttype is "file" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4709`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4709) |
| 2579 | validates reportpath if reporttype is "file" | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4717`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4717) |
| 2592 | warns when registryurls is set at the top level of global config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4728`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4728) |
| 2607 | warns when defaultregistryurls is set at the top level of global config | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4739`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4739) |
| 2622 | validates postupgradetasks.installtools tool names | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4750`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4750) |
| 2640 | rejects invalid postupgradetasks.installtools tool names | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4761`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4761) |
| 2664 | catches when * or ** is combined with others patterns in a regexorglob option | ported | [`crates/renovate-core/src/config/migrate_validate.rs:4167`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L4167) |
| 2705 | errors when using an invalid cache namespace | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3026`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3026) |
| 2728 | allows a valid cache namespace | ported | [`crates/renovate-core/src/config/migrate_validate.rs:3053`](../../../../../crates/renovate-core/src/config/migrate_validate.rs#L3053) |
| 2744 | allows wildcards | pending | — |

