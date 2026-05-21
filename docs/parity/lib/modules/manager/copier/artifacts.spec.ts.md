# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/copier/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/copier/artifacts.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if newVersion is not provided | 62 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses newValue for vcs-ref when both newValue and newVersion are provided | 87 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| reports an error if no upgrade is specified | 116 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| reports an error updated deps is undefined | 137 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| invokes copier update with the correct options by default | 159 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| propagates Git environment from hostRules | 179 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| invokes copier update with nested destination and answer file | 229 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports dynamic install with constraints python=$pythonConstraint copier=$copierConstraint | 249 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| includes --trust when allowScripts is true and ignoreScripts is false | 297 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not include --trust when ignoreScripts is true | 320 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles exec errors | 338 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not report changes if answers-file was not changed | 357 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated artifacts if repo status has changes | 380 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| warns about, but adds conflicts | 443 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

