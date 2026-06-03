# `lib/modules/manager/argocd/extract.spec.ts`

[← `manager/argocd`](../../../../_by-module/manager/argocd.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns null for empty | ported | [`crates/renovate-core/src/extractors/argocd.rs:417`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L417) |
| 15 | returns null for invalid | ported | [`crates/renovate-core/src/extractors/argocd.rs:462`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L462) |
| 21 | return null for kubernetes manifest | ported | [`crates/renovate-core/src/extractors/argocd.rs:391`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L391) |
| 26 | return null if deps array would be empty | ported | [`crates/renovate-core/src/extractors/argocd.rs:398`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L398) |
| 34 | return result for double quoted argoproj.io apiversion reference | ported | [`crates/renovate-core/src/extractors/argocd.rs:424`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L424) |
| 61 | return result for single quoted argoproj.io apiversion reference | ported | [`crates/renovate-core/src/extractors/argocd.rs:443`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L443) |
| 88 | full test | ported | [`crates/renovate-core/src/extractors/argocd.rs:470`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L470) |
| 203 | supports applicationsets | ported | [`crates/renovate-core/src/extractors/argocd.rs:589`](../../../../../../../crates/renovate-core/src/extractors/argocd.rs#L589) |

