# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rubygems/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rubygems/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** done

### `modules/datasource/rubygems/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing pkg | 24 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for rubygems.org package miss | 43 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns a dep for rubygems.org package hit | 54 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses rubygems.org if no registry urls were provided | 85 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses multiple source urls | 116 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| falls back to dependencies API | 157 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports /info endpoint | 191 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| errors when version request fails with server error | 222 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| errors when dependencies request fails server error | 238 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for GitHub Packages package miss | 258 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns a dep for GitHub Packages package hit | 274 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
