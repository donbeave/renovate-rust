# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-release-attachments/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-release-attachments/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `modules/datasource/github-release-attachments/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases | 23 | not-applicable | — | — | Uses vi.mock(host-rules) / vi.spyOn(githubGraphql) mocks; not portable |
| requires currentDigest | 99 | not-applicable | — | — | Uses vi.mock(host-rules) / vi.spyOn(githubGraphql) mocks; not portable |
| defaults to currentDigest when currentVersion is missing | 107 | not-applicable | — | — | Uses vi.mock(host-rules) / vi.spyOn(githubGraphql) mocks; not portable |
| returns updated digest in new release | 119 | not-applicable | — | — | Uses vi.mock(host-rules) / vi.spyOn(githubGraphql) mocks; not portable |
| ignores failures verifying currentDigest | 141 | not-applicable | — | — | Uses vi.mock(host-rules) / vi.spyOn(githubGraphql) mocks; not portable |

---

