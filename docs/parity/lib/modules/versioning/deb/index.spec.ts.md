# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/deb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/deb/index.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/deb/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $expected | 4 | not-applicable | — | — | Renovate's `deb` version validation API is not implemented in Rust; Rust only tags some extracted Docker dependencies with Debian versioning metadata. |
| equals("$a", "$b") === $expected | 60 | not-applicable | — | — | Renovate's `deb` version equality comparator is not implemented in Rust. |
| isGreaterThan("$a", "$b") === $expected | 84 | not-applicable | — | — | Renovate's `deb` version ordering comparator is not implemented in Rust. |
| isSingleVersion("$version") === $expected | 128 | not-applicable | — | — | Renovate's `deb` single-version classifier is not implemented in Rust. |
| getMajor("$version") === $expected | 136 | not-applicable | — | — | Renovate's `deb` major component parser is not implemented in Rust. |
| getMinor("$version") === $expected | 149 | not-applicable | — | — | Renovate's `deb` minor component parser is not implemented in Rust. |
| getPatch("$version") === $expected | 162 | not-applicable | — | — | Renovate's `deb` patch component parser is not implemented in Rust. |

---

