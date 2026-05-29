# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/releases.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/releases.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/releases › getReleaseNotes()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should contain only stable | 41 | not-applicable | — | — | mocking framework internals — vi.mock on github/datasource; TypeScript changelog release lookup|
| should contain currentVersion unstable | 57 | not-applicable | — | — | mocking framework internals — vi.mock on github/datasource; TypeScript changelog release lookup|
| should contain newVersion unstable | 74 | not-applicable | — | — | mocking framework internals — vi.mock on github/datasource; TypeScript changelog release lookup|
| should contain both currentVersion newVersion unstable | 91 | not-applicable | — | — | mocking framework internals — vi.mock on github/datasource; TypeScript changelog release lookup|
| should valueToVersion | 110 | not-applicable | — | — | mocking framework internals — vi.mock on github/datasource; TypeScript changelog release lookup|
| should return any previous version if current version is non-existent | 126 | not-applicable | — | — | mocking framework internals — vi.mock on github/datasource; TypeScript changelog release lookup|

---

