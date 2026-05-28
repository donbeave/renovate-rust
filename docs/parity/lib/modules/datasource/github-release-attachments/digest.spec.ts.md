# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-release-attachments/digest.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-release-attachments/digest.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `modules/datasource/github-release-attachments/digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds SHASUMS.txt file containing digest | 16 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| returns null when not found in digest file asset | 31 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| finds asset by digest | 49 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| returns null when no assets available | 67 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| downloads updated digest file | 86 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| maps digested file name to new version | 98 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| returns null when not found in digest file | 115 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| returns null when digest file not found | 127 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| falls back to digesting file when checksum file is removed | 136 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| digests updated file | 164 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |
| returns null when not found | 178 | not-applicable | — | — | Uses httpMock / GitHubReleaseAttachmentMocker; HTTP mock infrastructure not portable to Rust |

---

