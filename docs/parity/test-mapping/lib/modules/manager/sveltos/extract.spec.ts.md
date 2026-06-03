# `lib/modules/manager/sveltos/extract.spec.ts`

[← `manager/sveltos`](../../../../_by-module/manager/sveltos.md) · [all modules](../../../../README.md)

**14/14 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 278 | returns an empty array when parsing fails | ported | [`crates/renovate-core/src/extractors/sveltos.rs:369`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L369) |
| 284 | returns null if extractdefinition returns an empty array | ported | [`crates/renovate-core/src/extractors/sveltos.rs:437`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L437) |
| 298 | returns null for empty | ported | [`crates/renovate-core/src/extractors/sveltos.rs:376`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L376) |
| 302 | returns null for invalid | ported | [`crates/renovate-core/src/extractors/sveltos.rs:448`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L448) |
| 308 | return null for kubernetes manifest | ported | [`crates/renovate-core/src/extractors/sveltos.rs:363`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L363) |
| 313 | return null if deps array would be empty | ported | [`crates/renovate-core/src/extractors/sveltos.rs:383`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L383) |
| 318 | return null if yaml is invalid | ported | [`crates/renovate-core/src/extractors/sveltos.rs:465`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L465) |
| 332 | return result for double quoted projectsveltos.io apiversion reference | ported | [`crates/renovate-core/src/extractors/sveltos.rs:398`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L398) |
| 364 | return result for single quoted projectsveltos.io apiversion reference | ported | [`crates/renovate-core/src/extractors/sveltos.rs:418`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L418) |
| 396 | supports profiles | ported | [`crates/renovate-core/src/extractors/sveltos.rs:477`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L477) |
| 444 | supports clusterprofiles | ported | [`crates/renovate-core/src/extractors/sveltos.rs:287`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L287) |
| 495 | considers registryaliases | ported | [`crates/renovate-core/src/extractors/sveltos.rs:328`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L328) |
| 518 | supports clusterpromotions | ported | [`crates/renovate-core/src/extractors/sveltos.rs:509`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L509) |
| 554 | supports eventtriggers | ported | [`crates/renovate-core/src/extractors/sveltos.rs:581`](../../../../../../../crates/renovate-core/src/extractors/sveltos.rs#L581) |

