# `lib/workers/repository/process/lookup/filter-checks.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/12 in-scope tests ported** (12 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 74 | returns latest release if internalchecksfilter=none | pending | — |
| 87 | uses datasource-level interception mechanism | pending | — |
| 121 | returns non-pending latest release if internalchecksfilter=flexible and none pass checks | pending | — |
| 135 | returns pending latest release if internalchecksfilter=strict and none pass checks | pending | — |
| 149 | returns non-latest release if internalchecksfilter=strict and some pass checks | pending | — |
| 163 | returns non-latest release if internalchecksfilter=flexible and some pass checks | pending | — |
| 177 | picks up minimumreleaseage settings from packagerules | pending | — |
| 194 | picks up minimumreleaseage settings from updatetype | pending | — |
| 218 | does not return the latest release, if minimumreleaseagebehaviour=timestamp-required | pending | — |
| 252 | returns the latest release, if minimumreleaseagebehaviour=timestamp-optional | pending | — |
| 286 | returns latest release, if minimumreleaseagebehaviour=timestamp-required but minimumreleaseage=0 days | pending | — |
| 321 | picks up minimumconfidence settings from updatetype | pending | — |

