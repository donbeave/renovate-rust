# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/artifacts.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no updated deps and not lockfile maintenance | 24 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bazel module artifact update pipeline|
| returns null if no MODULE.bazel.lock found | 35 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bazel module artifact update pipeline|
| writes package file and delegates to updateBazelLockfile | 50 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bazel module artifact update pipeline|
| passes isLockFileMaintenance to updateBazelLockfile | 92 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bazel module artifact update pipeline|
| passes bazelisk constraint to updateBazelLockfile | 113 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bazel module artifact update pipeline|
| handles subdirectory MODULE.bazel | 134 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bazel module artifact update pipeline|

---

