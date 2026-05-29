# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/exec/containerbase.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/exec/containerbase.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** not-applicable

### `util/exec/containerbase › isDynamicInstall()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if binarySource is not install | 22 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| returns false if not containerbase | 26 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| returns false if any unsupported tools | 31 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| returns true if supported tools | 42 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|

### `util/exec/containerbase › getToolConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config for a known tool | 51 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| returns undefined for an unknown tool | 60 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|

### `util/exec/containerbase › resolveConstraint()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns from config | 67 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| returns highest stable | 73 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| returns highest unstable | 87 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| respects latest | 96 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| supports rust docker tags | 113 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| throws for unknown tools | 127 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| throws no releases | 133 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| falls back to latest version if no compatible release | 142 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| falls back to latest version if invalid constraint | 151 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| supports python ranges "$version" => "$expected" | 160 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| removes pep440 == | 184 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| supports flutter ranges "$version" => "$expected" | 193 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|
| supports dart ranges "$version" => "$expected" | 223 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|

### `util/exec/containerbase › generateInstallCommands()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns install commands | 269 | not-applicable | — | — | mocking framework internals — mockDeep on datasource; TypeScript containerbase tool management|

---

