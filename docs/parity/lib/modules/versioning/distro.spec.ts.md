# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/versioning/distro.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/distro.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/distro`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isCodename("$version") === $expected | 12 | not-applicable | — | — | Renovate's TypeScript distro-info versioning helper and distro release data are not implemented as a Rust versioning API. |
| getVersionByCodename("$version") === $expected | 27 | not-applicable | — | — | Renovate's TypeScript distro-info versioning helper and distro release data are not implemented as a Rust versioning API. |
| getCodenameByVersion("$version") === $expected | 44 | not-applicable | — | — | Renovate's TypeScript distro-info versioning helper and distro release data are not implemented as a Rust versioning API. |
| exists("$version") === $expected | 61 | not-applicable | — | — | Renovate's TypeScript distro-info versioning helper and distro release data are not implemented as a Rust versioning API. |
| isEolLts("$version") === $expected | 80 | not-applicable | — | — | Renovate's TypeScript distro-info versioning helper and distro release data are not implemented as a Rust versioning API. |
| isReleased("$version") === $expected | 98 | not-applicable | — | — | Renovate's TypeScript distro-info versioning helper and distro release data are not implemented as a Rust versioning API. |
| retrieves schedule of the previous previous release | 115 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| retrieves schedule of the previous release | 122 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| retrieves schedule of the most recent release | 129 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| sends a float as an argument | 136 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| sends an out of bound argument | 143 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| sends another out of bound argument | 147 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| retrieves focal release schedule | 151 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| retrieves non-existent release schedule | 158 | not-applicable | — | — | Renovate's TypeScript distro release schedule lookup is not implemented as a Rust versioning API. |
| works with debian | 162 | not-applicable | — | — | Renovate's TypeScript Debian/Ubuntu distro-info release data helpers are not implemented as a Rust versioning API. |

---

