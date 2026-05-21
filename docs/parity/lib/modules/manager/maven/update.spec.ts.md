# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven/update.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update version | 15 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| should do simple replacement | 36 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| should do full replacement | 58 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| should do replacement if version is first | 90 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| should ignore replacement if name does not match | 134 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| should update a cloud native buildpack version | 151 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| should update a cloud native buildpack digest | 173 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |

### `bumpPackageVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bumps pom.xml version | 215 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| bumps pom.xml version keeping SNAPSHOT | 226 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| bumps pom.xml minor version keeping SNAPSHOT | 237 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| bumps pom.xml major version keeping SNAPSHOT | 248 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| bumps pom.xml version keeping qualifier with -SNAPSHOT | 259 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| does not bump version twice | 273 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| does not bump version if version is not a semantic version | 288 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| does not bump version if pom.xml has no version | 299 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| returns content if bumping errors | 305 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| bumps pom.xml version to SNAPSHOT with prerelease | 314 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |
| bumps pom.xml version with prerelease semver level | 325 | not-applicable | — | — | tests Maven POM file version update via regex patching; update logic in Rust not yet implemented |

---

