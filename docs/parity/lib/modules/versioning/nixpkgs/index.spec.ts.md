# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nixpkgs/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nixpkgs/index.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/nixpkgs/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 6 | not-applicable | — | — | Renovate's Nixpkgs channel versioning scheme is not implemented as a Rust versioning API; Rust Nix support is extractor oriented. |
| isStable("$version") === $expected | 32 | not-applicable | — | — | Renovate's Nixpkgs stability classifier is not implemented as a Rust versioning API; Rust Nix support is extractor oriented. |
| equals($a, $b) === $expected | 50 | not-applicable | — | — | Renovate's Nixpkgs comparator is not implemented as a Rust versioning API; Rust Nix support is extractor oriented. |
| $versions -> sortVersions -> $expected | 62 | not-applicable | — | — | Renovate's Nixpkgs sorting comparator is not implemented as a Rust versioning API; Rust Nix support is extractor oriented. |
| equals($a, $b) === $expected | 73 | not-applicable | — | — | Renovate's Nixpkgs compatibility helper is not implemented as a Rust versioning API; Rust Nix support is extractor oriented. |

---

