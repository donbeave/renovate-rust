# `lib/workers/repository/init/apis.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**1/12 in-scope tests ported** (11 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 24 | runs | pending | — |
| 34 | throws for disabled | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:181`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L181) |
| 49 | throws for forked | pending | — |
| 66 | does not throw for includeforks=true | pending | — |
| 79 | does not throw for forkprocessing=enabled | pending | — |
| 92 | ignores platform.getjsonfile() failures | pending | — |
| 109 | throws for fork with platform.getjsonfile() failures | pending | — |
| 124 | uses the onboardingconfigfilename if set | pending | — |
| 151 | falls back to "renovate.json" if onboardingconfigfilename is not set | pending | — |
| 172 | falls back to "renovate.json" if onboardingconfigfilename is not valid | pending | — |
| 191 | checks for re-enablement and continues | pending | — |
| 211 | checks for re-enablement and skips | pending | — |

