# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/maven/compare.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/maven/compare.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/maven/compare`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $x == $y | 15 | not-applicable | — | — | Renovate's full Maven `compare.ts` fixture table is not implemented as a Rust parity API; Rust currently exposes a narrower Maven comparator and update-summary helper. |
| $x < $y | 106 | not-applicable | — | — | Renovate's full Maven `compare.ts` fixture table is not implemented as a Rust parity API; Rust currently exposes a narrower Maven comparator and update-summary helper. |
| $qualifier | 203 | not-applicable | — | — | Renovate's Maven MNG-7644 fixture table is not implemented as a Rust parity API; Rust currently exposes a narrower Maven comparator and update-summary helper. |
| isSubversion("$majorVersion", "$minorVersion") === $expected | 226 | not-applicable | — | — | Renovate's Maven `isSubversion` helper is not implemented as a Rust API. |
| should tokenize | 454 | not-applicable | — | — | Renovate's Maven tokenizer object-shape helper is not implemented as a Rust API. |
| $x == $y | 463 | not-applicable | — | — | Renovate's non-standard Maven `compare.ts` fixture table is not implemented as a Rust parity API; Rust currently exposes a narrower Maven comparator and update-summary helper. |
| $x < $y | 478 | not-applicable | — | — | Renovate's non-standard Maven `compare.ts` fixture table is not implemented as a Rust parity API; Rust currently exposes a narrower Maven comparator and update-summary helper. |
| filters out incorrect range: $input | 490 | not-applicable | — | — | Renovate's Maven range parser is not implemented as a Rust API. |
| parseRange("$input") | 521 | not-applicable | — | — | Renovate's Maven range parser and stringifier are not implemented as a Rust API. |
| autoExtendMavenRange("$range", "$version") === $expected | 560 | not-applicable | — | — | Renovate's Maven range auto-extension helper is not implemented as a Rust API. |

---

