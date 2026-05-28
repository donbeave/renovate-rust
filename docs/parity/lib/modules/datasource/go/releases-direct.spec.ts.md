# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/go/releases-direct.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/go/releases-direct.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** done

### `modules/datasource/go/releases-direct › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for null getDatasource result | 26 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| throws for getDatasource error | 34 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| processes real data | 43 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support forgejo | 69 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support gitlab | 130 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support gitea | 148 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support git | 209 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support self hosted gitlab private repositories | 228 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support bitbucket tags | 247 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support ghe | 269 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| works for known servers | 298 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| support gitlab subgroups | 327 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| works for nested modules on github | 347 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| falls back to unprefixed tags | 383 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |
| works for nested modules on github v2+ major upgrades | 409 | not-applicable | — | — | Requires vi.mock on BaseGoDatasource and underlying tag/release datasources |

---
