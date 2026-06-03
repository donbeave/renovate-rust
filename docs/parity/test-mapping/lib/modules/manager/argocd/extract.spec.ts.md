# `lib/modules/manager/argocd/extract.spec.ts`

[← `manager/argocd`](../../../../_by-module/manager/argocd.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns null for empty | ported | `crates/renovate-core/src/extractors/argocd.rs:417` |
| 15 | returns null for invalid | ported | `crates/renovate-core/src/extractors/argocd.rs:462` |
| 21 | return null for kubernetes manifest | ported | `crates/renovate-core/src/extractors/argocd.rs:391` |
| 26 | return null if deps array would be empty | ported | `crates/renovate-core/src/extractors/argocd.rs:398` |
| 34 | return result for double quoted argoproj.io apiversion reference | ported | `crates/renovate-core/src/extractors/argocd.rs:424` |
| 61 | return result for single quoted argoproj.io apiversion reference | ported | `crates/renovate-core/src/extractors/argocd.rs:443` |
| 88 | full test | ported | `crates/renovate-core/src/extractors/argocd.rs:470` |
| 203 | supports applicationsets | ported | `crates/renovate-core/src/extractors/argocd.rs:589` |

