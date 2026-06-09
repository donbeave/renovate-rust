# `lib/modules/manager/azure-pipelines/extract.spec.ts`

[← `manager/azure-pipelines`](../../../../_by-module/manager/azure-pipelines.md) · [all modules](../../../../README.md)

**29/29 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 25 | should parse a valid azure-pipelines file | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:394`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L394) |
| 30 | return null on an invalid file | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:776`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L776) |
| 36 | should extract repository information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:436`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L436) |
| 52 | should return null when repository type is not github | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:454`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L454) |
| 65 | should return null when reference is not defined specified | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:470`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L470) |
| 77 | should return null when reference is invalid tag format | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:479`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L479) |
| 90 | should extract azure repository information if project in name | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:495`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L495) |
| 111 | should extract azure repository information if project is not in name but is in the config repository | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:514`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L514) |
| 132 | should return null if repository type is git and project not in name nor in config repository name | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:533`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L533) |
| 150 | should return null if repository type is git and currentrepository is undefined | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:549`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L549) |
| 168 | should return null for git repo type if platform not azure | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:565`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L565) |
| 187 | should extract container information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:420`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L420) |
| 201 | should extract azure-pipelines task information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:637`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L637) |
| 209 | should return null for invalid task format | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:689`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L689) |
| 215 | returns null for invalid azure pipelines files | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:782`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L782) |
| 221 | extracts dependencies | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:599`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L599) |
| 245 | should return null when there is no dependency found | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:697`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L697) |
| 253 | should extract deployment jobs runonce | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:795`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L795) |
| 277 | should extract deployment jobs on failure | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:815`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L815) |
| 302 | should extract deployment jobs on success | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:836`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L836) |
| 327 | should extract deployment jobs postroute | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:857`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L857) |
| 351 | should extract deployment jobs predeploy | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:874`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L874) |
| 375 | should extract deployment jobs route | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:891`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L891) |
| 399 | should extract deployment jobs rolling | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:908`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L908) |
| 423 | should extract deployment jobs canary | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:925`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L925) |
| 447 | should extract stages | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:717`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L717) |
| 470 | should extract jobs | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:736`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L736) |
| 491 | should extract steps | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:753`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L753) |
| 510 | should return null when task alias used | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:768`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L768) |

