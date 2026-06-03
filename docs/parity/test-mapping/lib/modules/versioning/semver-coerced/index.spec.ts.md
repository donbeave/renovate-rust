# `lib/modules/versioning/semver-coerced/index.spec.ts`

[← `versioning/semver-coerced`](../../../../_by-module/versioning/semver-coerced.md) · [all modules](../../../../README.md)

**48/53 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | should return true for strictly equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:236`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L236) |
| 9 | should return true for non-strictly equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:242`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L242) |
| 14 | should return false for non-equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:249`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L249) |
| 18 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:255`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L255) |
| 24 | should return major version number for strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:261`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L261) |
| 28 | should return major version number for non-strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:267`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L267) |
| 32 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:255`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L255) |
| 38 | should return minor version number for strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:279`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L279) |
| 42 | should return minor version number for non-strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:285`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L285) |
| 46 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:255`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L255) |
| 52 | _(it.each / template — verify manually)_ | ? | — |
| 76 | should return false for patch updates | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:324`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L324) |
| 80 | should return false for minor updates | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:330`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L330) |
| 84 | should return true for major updates | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:336`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L336) |
| 88 | should return true for major updates from v0.x | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:342`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L342) |
| 92 | should return true for major updates within v0.x | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:348`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L348) |
| 98 | should return true for strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:354`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L354) |
| 102 | should return true for non-strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:360`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L360) |
| 106 | should return false for non-semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:366`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L366) |
| 112 | should return true for a greater version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:372`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L372) |
| 116 | should return false for lower version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:378`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L378) |
| 120 | should return false if version cannot be coerced | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:384`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L384) |
| 126 | should return true for a lower version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:390`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L390) |
| 130 | should return false for in-range version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:396`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L396) |
| 134 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:255`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L255) |
| 140 | returns true if naked version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:408`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L408) |
| 145 | returns false if equals | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:415`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L415) |
| 150 | returns false when not version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:422`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L422) |
| 156 | _(it.each / template — verify manually)_ | ? | — |
| 179 | should return null for non-digit version strings | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:454`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L454) |
| 183 | should return null for irregular version strings | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:460`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L460) |
| 187 | should support strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:466`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L466) |
| 191 | should treat semver with dash as a valid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:472`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L472) |
| 195 | should treat semver without dash as a valid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:478`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L478) |
| 199 | should treat ranges as valid versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:484`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L484) |
| 205 | should reject github repositories | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:492`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L492) |
| 215 | should return null for non-digit versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:500`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L500) |
| 219 | should support strict semver versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:506`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L506) |
| 223 | should support non-strict versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:512`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L512) |
| 229 | should return true when version is in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:518`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L518) |
| 233 | should return true with non-strict version in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:524`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L524) |
| 237 | should return false when version is not in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:530`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L530) |
| 241 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:255`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L255) |
| 247 | should return max satisfying version in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:542`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L542) |
| 253 | should support coercion | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:551`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L551) |
| 261 | should return min satisfying version in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:560`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L560) |
| 267 | should support coercion | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:551`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L551) |
| 275 | uses newversion | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:578`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L578) |
| 304 | should return zero for equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:586`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L586) |
| 308 | should return -1 for a < b | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:592`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L592) |
| 312 | should return 1 for a > b | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:598`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L598) |
| 316 | should return zero for equal non-strict versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:604`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L604) |
| 320 | works with invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:610`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L610) |

