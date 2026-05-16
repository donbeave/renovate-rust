# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rubygems/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rubygems/index.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/rubygems/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for missing pkg | 24 | not-applicable | — | — | Renovate's RubyGems `getReleases` package-name validation and null contract are not implemented in Rust; Rust uses direct latest-stable lookup. |
| returns null for rubygems.org package miss | 43 | not-applicable | — | — | Renovate's RubyGems `getReleases` null contract is not implemented in Rust; Rust uses direct latest-stable lookup. |
| returns a dep for rubygems.org package hit | 54 | not-applicable | — | — | Renovate's RubyGems full release-list response mapping is not implemented in Rust; Rust only returns latest stable version and timestamp. |
| uses rubygems.org if no registry urls were provided | 85 | not-applicable | — | — | Renovate's RubyGems registry URL selection contract is not implemented in Rust; Rust callers pass the API base directly. |
| uses multiple source urls | 116 | not-applicable | — | — | Renovate's RubyGems multiple source URL fallback contract is not implemented in Rust. |
| falls back to dependencies API | 157 | not-applicable | — | — | Renovate's RubyGems dependencies API fallback is not implemented in Rust. |
| supports /info endpoint | 191 | not-applicable | — | — | Renovate's RubyGems compact `/info` endpoint support is not implemented in Rust. |
| errors when version request fails with server error | 222 | not-applicable | — | — | Renovate's RubyGems server-error contract for version requests is not implemented in Rust. |
| errors when dependencies request fails server error | 238 | not-applicable | — | — | Renovate's RubyGems dependencies API fallback error contract is not implemented in Rust. |
| returns null for GitHub Packages package miss | 258 | not-applicable | — | — | Renovate's RubyGems GitHub Packages support is not implemented in Rust. |
| returns a dep for GitHub Packages package hit | 274 | not-applicable | — | — | Renovate's RubyGems GitHub Packages support is not implemented in Rust. |

---

