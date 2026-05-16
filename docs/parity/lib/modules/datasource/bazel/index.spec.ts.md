# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bazel/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bazel/index.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/bazel/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for error | 26 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list, external-host-error, and local-file registry contracts are not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| returns null for 404 | 33 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list, null-on-404, and local-file registry contracts are not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| returns null for empty result | 38 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list, empty-result, and local-file registry contracts are not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| returns null for empty 200 OK | 43 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list, empty-body, and local-file registry contracts are not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| throws for 5xx | 51 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list, external-host-error, and local-file registry contracts are not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| metadata without yanked versions | 58 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list response mapping is not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| metadata with yanked versions | 77 | not-applicable | — | — | Renovate's Bazel datasource `getReleases` release-list response mapping is not implemented in Rust; Rust only exposes a Bazel Central Registry latest-version summary. |
| should handle local file correctly | 106 | not-applicable | — | — | Renovate's Bazel local-file registry lookup is not implemented in Rust. |
| should return null for invalid file path | 135 | not-applicable | — | — | Renovate's Bazel local-file registry lookup is not implemented in Rust. |
| should return null for empty file content | 146 | not-applicable | — | — | Renovate's Bazel local-file registry lookup is not implemented in Rust. |

---

