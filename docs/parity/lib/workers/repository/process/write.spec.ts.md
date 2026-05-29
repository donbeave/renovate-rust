# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/write.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/write.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** not-applicable

### `workers/repository/process/write › writeUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops after automerge | 48 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| increments branch counter | 106 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| return no-work if branch fingerprint is not different | 147 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| updates branch fingerprint when new commit is made | 176 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| caches same fingerprint when no commit is made and branch cache existed | 219 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| caches same fingerprint when no commit is made | 264 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| creates new branchCache when cache is not enabled | 306 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|

### `workers/repository/process/write › canSkipBranchUpdateCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if no cache | 357 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| returns false when fingerprints are not same | 368 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| returns true | 378 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|

### `workers/repository/process/write › syncBranchState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates minimal branch state when cache is not populated | 390 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| when base branch name is different updates it and invalidates related cache | 405 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| when base branch sha is different updates it and invalidates related values | 438 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| when branch sha is different updates it and invalidates related values | 473 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| when branch sha is different updates it and sets commitTimestamp | 509 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| no change if all parameters are same | 548 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|

---

