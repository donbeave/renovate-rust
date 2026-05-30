# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/copier/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/copier/artifacts.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** pending-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if newVersion is not provided | 62 | pending | — | — | — |
| uses newValue for vcs-ref when both newValue and newVersion are provided | 87 | pending | — | — | — |
| reports an error if no upgrade is specified | 116 | pending | — | — | — |
| reports an error updated deps is undefined | 137 | pending | — | — | — |
| invokes copier update with the correct options by default | 159 | pending | — | — | — |
| propagates Git environment from hostRules | 179 | pending | — | — | — |
| invokes copier update with nested destination and answer file | 229 | pending | — | — | — |
| supports dynamic install with constraints python=$pythonConstraint copier=$copierConstraint | 249 | pending | — | — | — |
| includes --trust when allowScripts is true and ignoreScripts is false | 297 | pending | — | — | — |
| does not include --trust when ignoreScripts is true | 320 | pending | — | — | — |
| handles exec errors | 338 | pending | — | — | — |
| does not report changes if answers-file was not changed | 357 | pending | — | — | — |
| returns updated artifacts if repo status has changes | 380 | pending | — | — | — |
| warns about, but adds conflicts | 443 | pending | — | — | — |

---

