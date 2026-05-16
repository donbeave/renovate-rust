# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/github-releases/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/github-releases/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/github-releases/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns releases | 20 | not-applicable | — | — | Renovate's GitHub Releases `getReleases` full release-list, changelog URL, and digest contract are not implemented in Rust; Rust only exposes a latest-stable-release update summary. |
| should be independent of the current digest | 116 | not-applicable | — | — | Renovate's GitHub Releases digest lookup is not implemented in Rust; Rust only exposes a latest-stable-release update summary. |
| should be independent of the current value | 128 | not-applicable | — | — | Renovate's GitHub Releases digest lookup is not implemented in Rust; Rust only exposes a latest-stable-release update summary. |
| returns updated digest in new release | 136 | not-applicable | — | — | Renovate's GitHub Releases digest lookup is not implemented in Rust; Rust only exposes a latest-stable-release update summary. |
| returns null if the new value/tag does not exist | 149 | not-applicable | — | — | Renovate's GitHub Releases digest lookup is not implemented in Rust; Rust only exposes a latest-stable-release update summary. |

---

