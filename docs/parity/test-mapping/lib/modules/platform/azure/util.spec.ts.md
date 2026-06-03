# `lib/modules/platform/azure/util.spec.ts`

[← `platform/azure`](../../../../_by-module/platform/azure.md) · [all modules](../../../../README.md)

**26/29 in-scope tests ported** (3 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | should return undefined if null context passed | ported | [`crates/renovate-core/src/platform/azure_utils.rs:196`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L196) |
| 21 | should combine valid genre and name with slash | ported | [`crates/renovate-core/src/platform/azure_utils.rs:204`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L204) |
| 29 | should combine valid empty genre and name without a slash | ported | [`crates/renovate-core/src/platform/azure_utils.rs:212`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L212) |
| 39 | should return undefined if null context passed | ported | [`crates/renovate-core/src/platform/azure_utils.rs:196`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L196) |
| 44 | should parse valid genre and name with slash | ported | [`crates/renovate-core/src/platform/azure_utils.rs:226`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L226) |
| 54 | should parse valid genre and name with multiple slashes | ported | [`crates/renovate-core/src/platform/azure_utils.rs:237`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L237) |
| 64 | should parse valid empty genre and name without a slash | ported | [`crates/renovate-core/src/platform/azure_utils.rs:251`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L251) |
| 74 | should be renamed | ported | [`crates/renovate-core/src/platform/azure_utils.rs:259`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L259) |
| 79 | should log error and return undefined | ported | [`crates/renovate-core/src/platform/azure_utils.rs:266`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L266) |
| 84 | should return the input | ported | [`crates/renovate-core/src/platform/azure_utils.rs:273`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L273) |
| 91 | should be formated (closed) | ported | [`crates/renovate-core/src/platform/azure_utils.rs:177`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L177) |
| 96 | should be formated (closed v2) | ported | [`crates/renovate-core/src/platform/azure_utils.rs:183`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L183) |
| 101 | should be formated (not closed) | ported | [`crates/renovate-core/src/platform/azure_utils.rs:189`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L189) |
| 108 | converts readable stream to string | pending | — |
| 113 | handles error | pending | — |
| 122 | should configure basic auth | ported | [`crates/renovate-core/src/platform/azure_utils.rs:413`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L413) |
| 130 | should configure personal access token | ported | [`crates/renovate-core/src/platform/azure_utils.rs:424`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L424) |
| 137 | should configure bearer token | ported | [`crates/renovate-core/src/platform/azure_utils.rs:437`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L437) |
| 144 | should be the same | ported | [`crates/renovate-core/src/platform/azure_utils.rs:280`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L280) |
| 149 | should be truncated | ported | [`crates/renovate-core/src/platform/azure_utils.rs:286`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L286) |
| 160 | should return the object with same strings | ported | [`crates/renovate-core/src/platform/azure_utils.rs:293`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L293) |
| 165 | should return the object with project and repo | ported | [`crates/renovate-core/src/platform/azure_utils.rs:301`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L301) |
| 170 | should return an error | ported | [`crates/renovate-core/src/platform/azure_utils.rs:309`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L309) |
| 180 | returns null when repos array is empty | ported | [`crates/renovate-core/src/platform/azure_utils.rs:316`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L316) |
| 186 | returns null when repo is not found | ported | [`crates/renovate-core/src/platform/azure_utils.rs:322`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L322) |
| 192 | finds repo | ported | [`crates/renovate-core/src/platform/azure_utils.rs:333`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L333) |
| 205 | supports shorthand names | ported | [`crates/renovate-core/src/platform/azure_utils.rs:362`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L362) |
| 214 | is case-independent | ported | [`crates/renovate-core/src/platform/azure_utils.rs:381`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L381) |
| 224 | throws when repo name is invalid | ported | [`crates/renovate-core/src/platform/azure_utils.rs:447`](../../../../../../../crates/renovate-core/src/platform/azure_utils.rs#L447) |

