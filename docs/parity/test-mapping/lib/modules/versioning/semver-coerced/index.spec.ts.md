# `lib/modules/versioning/semver-coerced/index.spec.ts`

[← `versioning/semver-coerced`](../../../../_by-module/versioning/semver-coerced.md) · [all modules](../../../../README.md)

**48/53 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | should return true for strictly equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:235`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L235) |
| 9 | should return true for non-strictly equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:241`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L241) |
| 14 | should return false for non-equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:248`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L248) |
| 18 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:254`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L254) |
| 24 | should return major version number for strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:260`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L260) |
| 28 | should return major version number for non-strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:266`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L266) |
| 32 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:254`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L254) |
| 38 | should return minor version number for strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:278`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L278) |
| 42 | should return minor version number for non-strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:284`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L284) |
| 46 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:254`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L254) |
| 52 | _(it.each / template — verify manually)_ | ? | — |
| 76 | should return false for patch updates | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:323`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L323) |
| 80 | should return false for minor updates | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:329`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L329) |
| 84 | should return true for major updates | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:335`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L335) |
| 88 | should return true for major updates from v0.x | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:341`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L341) |
| 92 | should return true for major updates within v0.x | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:347`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L347) |
| 98 | should return true for strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:353`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L353) |
| 102 | should return true for non-strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:359`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L359) |
| 106 | should return false for non-semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:365`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L365) |
| 112 | should return true for a greater version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:371`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L371) |
| 116 | should return false for lower version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:377`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L377) |
| 120 | should return false if version cannot be coerced | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:383`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L383) |
| 126 | should return true for a lower version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:389`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L389) |
| 130 | should return false for in-range version in strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:395`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L395) |
| 134 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:254`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L254) |
| 140 | returns true if naked version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:407`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L407) |
| 145 | returns false if equals | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:414`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L414) |
| 150 | returns false when not version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:421`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L421) |
| 156 | _(it.each / template — verify manually)_ | ? | — |
| 179 | should return null for non-digit version strings | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:453`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L453) |
| 183 | should return null for irregular version strings | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:459`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L459) |
| 187 | should support strict semver | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:465`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L465) |
| 191 | should treat semver with dash as a valid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:471`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L471) |
| 195 | should treat semver without dash as a valid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:477`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L477) |
| 199 | should treat ranges as valid versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:483`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L483) |
| 205 | should reject github repositories | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:491`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L491) |
| 215 | should return null for non-digit versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:499`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L499) |
| 219 | should support strict semver versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:505`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L505) |
| 223 | should support non-strict versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:511`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L511) |
| 229 | should return true when version is in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:517`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L517) |
| 233 | should return true with non-strict version in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:523`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L523) |
| 237 | should return false when version is not in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:529`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L529) |
| 241 | invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:254`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L254) |
| 247 | should return max satisfying version in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:541`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L541) |
| 253 | should support coercion | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:550`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L550) |
| 261 | should return min satisfying version in range | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:559`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L559) |
| 267 | should support coercion | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:550`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L550) |
| 275 | uses newversion | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:577`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L577) |
| 304 | should return zero for equal versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:585`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L585) |
| 308 | should return -1 for a < b | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:591`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L591) |
| 312 | should return 1 for a > b | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:597`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L597) |
| 316 | should return zero for equal non-strict versions | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:603`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L603) |
| 320 | works with invalid version | ported | [`crates/renovate-core/src/versioning/semver_coerced.rs:609`](../../../../../../../crates/renovate-core/src/versioning/semver_coerced.rs#L609) |

