# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/artifacts.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 34 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| skips if no lock file in config | 38 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|

### `updateArtifacts() › when using .lockb lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if cannot read lock file | 44 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| returns null if lock content unchanged | 51 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| returns updated lock content | 61 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| updates lock file when workspace package is updated | 82 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| supports lockFileMaintenance | 116 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| supports lockFileMaintenance (without updated deps) | 138 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| handles temporary error | 158 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| handles full error | 176 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|

### `updateArtifacts() › when using .lock lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if cannot read lock file | 196 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| returns null if lock content unchanged | 203 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| returns updated lock content | 213 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| supports lockFileMaintenance | 234 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| supports lockFileMaintenance (without updated deps) | 256 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| handles temporary error | 276 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|
| handles full error | 294 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|

### `bun command execution`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| check install options with configs | 315 | not-applicable | — | — | mocking framework internals — vi.mock on exec/fs; TypeScript Bun artifact update pipeline|

---

