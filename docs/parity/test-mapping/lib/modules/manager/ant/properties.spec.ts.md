# `lib/modules/manager/ant/properties.spec.ts`

[← `manager/ant`](../../../../_by-module/manager/ant.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | parses key=value pairs | ported | [`crates/renovate-core/src/extractors/ant.rs:805`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L805) |
| 28 | skips comments and blank lines | ported | [`crates/renovate-core/src/extractors/ant.rs:818`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L818) |
| 39 | supports colon separator | ported | [`crates/renovate-core/src/extractors/ant.rs:825`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L825) |
| 46 | skips malformed lines without separators | ported | [`crates/renovate-core/src/extractors/ant.rs:832`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L832) |
| 57 | implements first-definition-wins | ported | [`crates/renovate-core/src/extractors/ant.rs:845`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L845) |
| 64 | respects pre-existing props (first-definition-wins across sources) | ported | [`crates/renovate-core/src/extractors/ant.rs:852`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L852) |

