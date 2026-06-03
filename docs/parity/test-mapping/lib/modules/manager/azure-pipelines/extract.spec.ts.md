# `lib/modules/manager/azure-pipelines/extract.spec.ts`

[← `manager/azure-pipelines`](../../../../_by-module/manager/azure-pipelines.md) · [all modules](../../../../README.md)

**28/29 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 25 | should parse a valid azure-pipelines file | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:236`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L236) |
| 30 | return null on an invalid file | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:580`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L580) |
| 36 | should extract repository information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:278`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L278) |
| 52 | should return null when repository type is not github | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:296`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L296) |
| 65 | should return null when reference is not defined specified | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:312`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L312) |
| 77 | should return null when reference is invalid tag format | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:321`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L321) |
| 90 | should extract azure repository information if project in name | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:337`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L337) |
| 111 | should extract azure repository information if project is not in name but is in the config repository | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:356`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L356) |
| 132 | should return null if repository type is git and project not in name nor in config repository name | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:375`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L375) |
| 150 | should return null if repository type is git and currentrepository is undefined | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:391`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L391) |
| 168 | should return null for git repo type if platform not azure | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:407`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L407) |
| 187 | should extract container information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:262`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L262) |
| 201 | should extract azure-pipelines task information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:441`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L441) |
| 209 | should return null for invalid task format | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:493`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L493) |
| 215 | returns null for invalid azure pipelines files | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:586`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L586) |
| 221 | extracts dependencies | pending | — |
| 245 | should return null when there is no dependency found | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:501`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L501) |
| 253 | should extract deployment jobs runonce | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:599`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L599) |
| 277 | should extract deployment jobs on failure | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:619`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L619) |
| 302 | should extract deployment jobs on success | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:640`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L640) |
| 327 | should extract deployment jobs postroute | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:661`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L661) |
| 351 | should extract deployment jobs predeploy | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:678`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L678) |
| 375 | should extract deployment jobs route | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:695`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L695) |
| 399 | should extract deployment jobs rolling | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:712`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L712) |
| 423 | should extract deployment jobs canary | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:729`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L729) |
| 447 | should extract stages | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:521`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L521) |
| 470 | should extract jobs | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:540`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L540) |
| 491 | should extract steps | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:557`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L557) |
| 510 | should return null when task alias used | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:572`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L572) |

