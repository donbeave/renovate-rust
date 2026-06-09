# `lib/workers/repository/onboarding/branch/config.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**1/9 in-scope tests ported** (8 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 32 | returns the json stringified onboarding config | pending | — |
| 48 | handles finding a preset in the same group level | ported | [`crates/renovate-core/src/workers/repository/onboarding/branch/config.rs:98`](../../../../../../../../crates/renovate-core/src/workers/repository/onboarding/branch/config.rs#L98) |
| 58 | handles finding an organization dot platform preset | pending | — |
| 71 | handles finding a preset in the same group | pending | — |
| 87 | handles finding a preset in a parent group | pending | — |
| 103 | handles falling back to finding a organization preset | pending | — |
| 119 | handles not finding any preset | pending | — |
| 128 | ignores an unknown error | pending | — |
| 137 | ignores unsupported platform | pending | — |

