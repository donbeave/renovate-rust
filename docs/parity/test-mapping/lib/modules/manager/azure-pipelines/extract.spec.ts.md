# `lib/modules/manager/azure-pipelines/extract.spec.ts`

[← `manager/azure-pipelines`](../../../../_by-module/manager/azure-pipelines.md) · [all modules](../../../../README.md)

**29/29 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 25 | should parse a valid azure-pipelines file | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:356`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L356) |
| 30 | return null on an invalid file | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:741`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L741) |
| 36 | should extract repository information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:398`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L398) |
| 52 | should return null when repository type is not github | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:416`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L416) |
| 65 | should return null when reference is not defined specified | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:432`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L432) |
| 77 | should return null when reference is invalid tag format | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:441`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L441) |
| 90 | should extract azure repository information if project in name | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:457`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L457) |
| 111 | should extract azure repository information if project is not in name but is in the config repository | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:476`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L476) |
| 132 | should return null if repository type is git and project not in name nor in config repository name | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:495`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L495) |
| 150 | should return null if repository type is git and currentrepository is undefined | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:511`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L511) |
| 168 | should return null for git repo type if platform not azure | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:527`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L527) |
| 187 | should extract container information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:382`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L382) |
| 201 | should extract azure-pipelines task information | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:561`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L561) |
| 209 | should return null for invalid task format | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:613`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L613) |
| 215 | returns null for invalid azure pipelines files | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:747`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L747) |
| 221 | extracts dependencies | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:621`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L621) |
| 245 | should return null when there is no dependency found | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:662`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L662) |
| 253 | should extract deployment jobs runonce | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:760`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L760) |
| 277 | should extract deployment jobs on failure | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:780`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L780) |
| 302 | should extract deployment jobs on success | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:801`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L801) |
| 327 | should extract deployment jobs postroute | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:822`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L822) |
| 351 | should extract deployment jobs predeploy | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:839`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L839) |
| 375 | should extract deployment jobs route | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:856`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L856) |
| 399 | should extract deployment jobs rolling | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:873`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L873) |
| 423 | should extract deployment jobs canary | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:890`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L890) |
| 447 | should extract stages | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:682`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L682) |
| 470 | should extract jobs | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:701`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L701) |
| 491 | should extract steps | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:718`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L718) |
| 510 | should return null when task alias used | ported | [`crates/renovate-core/src/extractors/azure_pipelines.rs:733`](../../../../../../../crates/renovate-core/src/extractors/azure_pipelines.rs#L733) |

