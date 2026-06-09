# `lib/util/exec/index.spec.ts`

[← `util/exec`](../../../_by-module/util/exec.md) · [all modules](../../../README.md)

**26/40 in-scope tests ported** (14 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 910 | _(it.each / template — verify manually)_ | ? | — |
| 943 | supports image prefetch | pending | — |
| 985 | throws when an error is thrown | ported | [`crates/renovate-core/src/exec/orchestrator.rs:276`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L276) |
| 995 | rejects and throws if an error is thrown, even if we specify ignorefailure=true | pending | — |
| 1010 | does not reject and throw if rawexec returns an exit code, and we specify ignorefailure=true | ported | [`crates/renovate-core/src/exec/orchestrator.rs:420`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L420) |
| 1038 | exec takes an array with both `string`s and `commandwithoptions` as an argument | pending | — |
| 1059 | exec takes commandwithoptions as an argument | pending | — |
| 1076 | supports binarysource=install | ported | [`crates/renovate-core/src/exec/orchestrator.rs:214`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L214) |
| 1087 | logs ignored tool constraints for binarysource=global | ported | [`crates/renovate-core/src/exec/orchestrator.rs:456`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L456) |
| 1098 | supports binarysource=install precommands | ported | [`crates/renovate-core/src/exec/orchestrator.rs:291`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L291) |
| 1112 | only calls removedockercontainer in catch block is usedocker is set | ported | [`crates/renovate-core/src/exec/orchestrator.rs:478`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L478) |
| 1127 | wraps error if removedockercontainer throws an error | pending | — |
| 1161 | converts to temporary_error | ported | [`crates/renovate-core/src/exec/orchestrator.rs:492`](../../../../../../crates/renovate-core/src/exec/orchestrator.rs#L492) |
| 1194 | returns default values if no global or repo config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:125`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L125) |
| 1205 | returns default values if empty repo config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:134`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L134) |
| 1216 | returns default values if empty global config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:144`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L144) |
| 1230 | in global config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:153`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L153) |
| 1243 | in repo config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:166`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L166) |
| 1263 | when below global settings, repo settings are used | ported | [`crates/renovate-core/src/exec/tool_settings.rs:184`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L184) |
| 1277 | when repo settings are the same as global settings, they are used | ported | [`crates/renovate-core/src/exec/tool_settings.rs:202`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L202) |
| 1291 | when repo jvmmemory setting is higher than global setting, but lower than global jvmmaxmemory, the repo config is used | ported | [`crates/renovate-core/src/exec/tool_settings.rs:220`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L220) |
| 1303 | when repo jvmmaxmemory setting is lower than global settings, it is applied | ported | [`crates/renovate-core/src/exec/tool_settings.rs:236`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L236) |
| 1315 | when repo jvmmaxmemory setting is lower than global jvmmemory, jvmmemory is set to the same value | ported | [`crates/renovate-core/src/exec/tool_settings.rs:252`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L252) |
| 1328 | when repo jvmmaxmemory setting is lower than repo jvmmemory, jvmmemory is set to the same value | ported | [`crates/renovate-core/src/exec/tool_settings.rs:269`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L269) |
| 1342 | when repo jvmmaxmemory setting is higher than global settings, they are ignored | ported | [`crates/renovate-core/src/exec/tool_settings.rs:287`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L287) |
| 1355 | when repo jvmmaxmemory setting is higher than global settings, a debug log is logged | pending | — |
| 1375 | when global settings are lower than 512m, they are overridden to 512m | ported | [`crates/renovate-core/src/exec/tool_settings.rs:304`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L304) |
| 1388 | when global settings are lower than 512m, a debug log is logged | pending | — |
| 1400 | when repo settings are lower than 512m, they are overridden to 512m | ported | [`crates/renovate-core/src/exec/tool_settings.rs:317`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L317) |
| 1414 | when repo settings are lower than 512m, a debug log is logged | pending | — |
| 1439 | does not return a default value if no global or repo config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:335`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L335) |
| 1449 | does not return default values if empty global config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:343`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L343) |
| 1462 | in global config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:153`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L153) |
| 1474 | in repo config | ported | [`crates/renovate-core/src/exec/tool_settings.rs:166`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L166) |
| 1490 | when below global settings, repo settings are used | ported | [`crates/renovate-core/src/exec/tool_settings.rs:184`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L184) |
| 1502 | when repo settings are the same as global settings, they are used | ported | [`crates/renovate-core/src/exec/tool_settings.rs:202`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L202) |
| 1514 | when repo nodemaxmemory setting is lower than global settings, it is applied | ported | [`crates/renovate-core/src/exec/tool_settings.rs:404`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L404) |
| 1526 | when repo nodemaxmemory setting is higher than global settings, they are ignored | ported | [`crates/renovate-core/src/exec/tool_settings.rs:419`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L419) |
| 1538 | when repo nodemaxmemory setting is higher than global settings, a debug log is logged | pending | — |
| 1558 | takes the values given to it, and returns the jvm arguments | ported | [`crates/renovate-core/src/exec/tool_settings.rs:434`](../../../../../../crates/renovate-core/src/exec/tool_settings.rs#L434) |

