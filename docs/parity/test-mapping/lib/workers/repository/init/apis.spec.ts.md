# `lib/workers/repository/init/apis.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**12/12 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 24 | runs | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:197`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L197) |
| 34 | throws for disabled | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:180`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L180) |
| 49 | throws for forked | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:206`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L206) |
| 66 | does not throw for includeforks=true | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:219`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L219) |
| 79 | does not throw for forkprocessing=enabled | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:234`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L234) |
| 92 | ignores platform.getjsonfile() failures | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:249`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L249) |
| 109 | throws for fork with platform.getjsonfile() failures | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:265`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L265) |
| 124 | uses the onboardingconfigfilename if set | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:276`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L276) |
| 151 | falls back to "renovate.json" if onboardingconfigfilename is not set | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:287`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L287) |
| 172 | falls back to "renovate.json" if onboardingconfigfilename is not valid | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:300`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L300) |
| 191 | checks for re-enablement and continues | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:311`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L311) |
| 211 | checks for re-enablement and skips | ported | [`crates/renovate-core/src/workers/repository/init/apis.rs:327`](../../../../../../../crates/renovate-core/src/workers/repository/init/apis.rs#L327) |

