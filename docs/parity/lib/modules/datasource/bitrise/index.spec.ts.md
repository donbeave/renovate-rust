# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/bitrise/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/bitrise/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/bitrise/index › getReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unsupported registryUrl | 9 | not-applicable | — | — | Renovate's Bitrise `getReleases` GitHub contents API traversal and null-on-unsupported-registry contract are not implemented in Rust; Rust uses a steplib release asset index and latest-version summary. |
| support GitHub Enterprise API URL | 19 | not-applicable | — | — | Renovate's Bitrise GitHub Enterprise contents API URL handling and release-list mapping are not implemented in Rust; Rust only parses github.com steplib URLs for release asset indexes. |
| returns version and filters out the asset folder | 63 | not-applicable | — | — | Renovate's Bitrise GitHub contents API traversal, per-version step.yml parsing, and release-list mapping are not implemented in Rust. |
| returns null if there are no releases | 137 | not-applicable | — | — | Renovate's Bitrise GitHub contents API traversal and null-on-empty-release-list contract are not implemented in Rust. |
| returns null if the package has an unexpected format | 159 | not-applicable | — | — | Renovate's Bitrise GitHub contents API traversal and null-on-unexpected-content contract are not implemented in Rust. |
| returns null if the file object has no content | 179 | not-applicable | — | — | Renovate's Bitrise per-version step.yml content validation is not implemented in Rust. |
| returns null if the file object has an unexpected encoding | 206 | not-applicable | — | — | Renovate's Bitrise per-version step.yml encoding validation is not implemented in Rust. |

---

