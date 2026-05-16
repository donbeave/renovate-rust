# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/jenkins-plugins/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/jenkins-plugins/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/jenkins-plugins/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for a package miss | 57 | not-applicable | — | — | Renovate's Jenkins plugins `getReleases` source URL and plugin-versions release-list mapping are not implemented in Rust; Rust only reads latest version from update-center metadata. |
| returns package releases for a hit for info and releases | 69 | not-applicable | — | — | Renovate's Jenkins plugins `getReleases` source URL and plugin-versions release-list mapping are not implemented in Rust; Rust only reads latest version from update-center metadata. |
| returns package releases for a hit for info and miss for releases | 104 | not-applicable | — | — | Renovate's Jenkins plugins `getReleases` source URL and plugin-versions release-list mapping are not implemented in Rust; Rust only reads latest version from update-center metadata. |
| returns null empty response | 122 | not-applicable | — | — | Renovate's Jenkins plugins `getReleases` source URL and plugin-versions release-list mapping are not implemented in Rust; Rust only reads latest version from update-center metadata. |
| returns package releases from a custom registry | 131 | not-applicable | — | — | Renovate's Jenkins plugins `getReleases` source URL and plugin-versions release-list mapping are not implemented in Rust; Rust only reads latest version from update-center metadata. |

---

