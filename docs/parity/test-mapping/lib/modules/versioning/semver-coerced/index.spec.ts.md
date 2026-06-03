# `lib/modules/versioning/semver-coerced/index.spec.ts`

[← `versioning/semver-coerced`](../../../../_by-module/versioning/semver-coerced.md) · [all modules](../../../../README.md)

**48/53 ported** (5 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | should return true for strictly equal versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:236` |
| 9 | should return true for non-strictly equal versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:242` |
| 14 | should return false for non-equal versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:249` |
| 18 | invalid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:255` |
| 24 | should return major version number for strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:261` |
| 28 | should return major version number for non-strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:267` |
| 32 | invalid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:255` |
| 38 | should return minor version number for strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:279` |
| 42 | should return minor version number for non-strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:285` |
| 46 | invalid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:255` |
| 52 | _(it.each / template — verify manually)_ | ? | — |
| 76 | should return false for patch updates | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:324` |
| 80 | should return false for minor updates | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:330` |
| 84 | should return true for major updates | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:336` |
| 88 | should return true for major updates from v0.x | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:342` |
| 92 | should return true for major updates within v0.x | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:348` |
| 98 | should return true for strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:354` |
| 102 | should return true for non-strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:360` |
| 106 | should return false for non-semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:366` |
| 112 | should return true for a greater version in strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:372` |
| 116 | should return false for lower version in strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:378` |
| 120 | should return false if version cannot be coerced | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:384` |
| 126 | should return true for a lower version in strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:390` |
| 130 | should return false for in-range version in strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:396` |
| 134 | invalid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:255` |
| 140 | returns true if naked version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:408` |
| 145 | returns false if equals | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:415` |
| 150 | returns false when not version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:422` |
| 156 | _(it.each / template — verify manually)_ | ? | — |
| 179 | should return null for non-digit version strings | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:454` |
| 183 | should return null for irregular version strings | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:460` |
| 187 | should support strict semver | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:466` |
| 191 | should treat semver with dash as a valid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:472` |
| 195 | should treat semver without dash as a valid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:478` |
| 199 | should treat ranges as valid versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:484` |
| 205 | should reject github repositories | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:492` |
| 215 | should return null for non-digit versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:500` |
| 219 | should support strict semver versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:506` |
| 223 | should support non-strict versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:512` |
| 229 | should return true when version is in range | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:518` |
| 233 | should return true with non-strict version in range | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:524` |
| 237 | should return false when version is not in range | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:530` |
| 241 | invalid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:255` |
| 247 | should return max satisfying version in range | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:542` |
| 253 | should support coercion | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:551` |
| 261 | should return min satisfying version in range | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:560` |
| 267 | should support coercion | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:551` |
| 275 | uses newversion | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:578` |
| 304 | should return zero for equal versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:586` |
| 308 | should return -1 for a < b | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:592` |
| 312 | should return 1 for a > b | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:598` |
| 316 | should return zero for equal non-strict versions | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:604` |
| 320 | works with invalid version | ported | `crates/renovate-core/src/versioning/semver_coerced.rs:610` |

