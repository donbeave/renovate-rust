# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/copier/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/copier/artifacts.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if newVersion is not provided | 62 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| uses newValue for vcs-ref when both newValue and newVersion are provided | 87 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| reports an error if no upgrade is specified | 116 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| reports an error updated deps is undefined | 137 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| invokes copier update with the correct options by default | 159 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| propagates Git environment from hostRules | 179 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| invokes copier update with nested destination and answer file | 229 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| supports dynamic install with constraints python=$pythonConstraint copier=$copierConstraint | 249 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| includes --trust when allowScripts is true and ignoreScripts is false | 297 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| does not include --trust when ignoreScripts is true | 320 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| handles exec errors | 338 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| does not report changes if answers-file was not changed | 357 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns updated artifacts if repo status has changes | 380 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| warns about, but adds conflicts | 443 | not-applicable | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests copier artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

---

