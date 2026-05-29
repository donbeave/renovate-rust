# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/libyear.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/libyear.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `workers/repository/process/libyear › calculateLibYears`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns early if no packageFiles | 14 | not-applicable | — | — | mocking framework internals — vi.mock on instrumentation/reporting OTel; TypeScript libyear calculation pipeline|
| calculates libYears | 19 | not-applicable | — | — | mocking framework internals — vi.mock on instrumentation/reporting OTel; TypeScript libyear calculation pipeline|
| skips disabled dependencies when calculating libYears | 144 | not-applicable | — | — | mocking framework internals — vi.mock on instrumentation/reporting OTel; TypeScript libyear calculation pipeline|
| de-duplicates if same dep found in different files | 225 | not-applicable | — | — | mocking framework internals — vi.mock on instrumentation/reporting OTel; TypeScript libyear calculation pipeline|
| ignores disabled dependencies | 304 | not-applicable | — | — | mocking framework internals — vi.mock on instrumentation/reporting OTel; TypeScript libyear calculation pipeline|

---

