# `lib/modules/manager/crossplane/extract.spec.ts`

[← `manager/crossplane`](../../../../_by-module/manager/crossplane.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns null for empty | ported | [`crates/renovate-core/src/extractors/crossplane.rs:192`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L192) |
| 16 | strips invalid templates | ported | [`crates/renovate-core/src/extractors/crossplane.rs:198`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L198) |
| 20 | return null for kubernetes manifest | ported | [`crates/renovate-core/src/extractors/crossplane.rs:151`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L151) |
| 25 | return invalid-value if deps are not valid images and ignore if missing | ported | [`crates/renovate-core/src/extractors/crossplane.rs:205`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L205) |
| 37 | return result for double quoted pkg.crossplane.io apiversion reference | ported | [`crates/renovate-core/src/extractors/crossplane.rs:279`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L279) |
| 58 | return result for single quoted pkg.crossplane.io apiversion reference | ported | [`crates/renovate-core/src/extractors/crossplane.rs:292`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L292) |
| 79 | return no results for invalid resource | ported | [`crates/renovate-core/src/extractors/crossplane.rs:178`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L178) |
| 94 | full test | ported | [`crates/renovate-core/src/extractors/crossplane.rs:132`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L132) |
| 137 | should work even if there are other resources in the file | ported | [`crates/renovate-core/src/extractors/crossplane.rs:158`](../../../../../../../crates/renovate-core/src/extractors/crossplane.rs#L158) |

