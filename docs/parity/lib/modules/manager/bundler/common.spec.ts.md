# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bundler/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bundler/common.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `getBundlerConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses existing constraint | 31 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| extracts from lockfile | 41 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| returns null | 49 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|

### `getRubyConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses existing constraint | 59 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| extracts from gemfile | 71 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| extracts from .ruby-version | 81 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| extracts from .tool-versions | 92 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| extracts from lockfile | 105 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| returns null | 120 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|

### `getLockFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns packageFileName.lock | 132 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|
| returns Gemfile.lock | 138 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec; TypeScript Bundler common utilities|

---

