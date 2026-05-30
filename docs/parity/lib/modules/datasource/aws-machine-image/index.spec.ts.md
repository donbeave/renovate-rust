# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/aws-machine-image/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-machine-image/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `modules/datasource/aws-machine-image/index › getSortedAwsMachineImages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| with 3 returned images | 137 | not-applicable | — | — | — |
| with 1 returned image | 147 | not-applicable | — | — | — |
| without returned images | 157 | not-applicable | — | — | — |

### `modules/datasource/aws-machine-image/index › getDigest()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without newValue, without returned images to be null | 169 | not-applicable | — | — | — |
| without newValue, with one matching image to return that image | 179 | not-applicable | — | — | — |
| without newValue, with 3 matching image to return the newest image | 189 | not-applicable | — | — | — |
| with matching newValue, with 3 matching image to return the matching image | 199 | not-applicable | — | — | — |
| with not matching newValue, with 3 matching images to return the matching image | 212 | not-applicable | — | — | — |

### `modules/datasource/aws-machine-image/index › getPkgReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without returned images to be null | 227 | not-applicable | — | — | — |
| with one matching image to return that image | 237 | not-applicable | — | — | — |
| with one deprecated matching image to return that image | 256 | not-applicable | — | — | — |
| with 3 matching image to return the newest image | 275 | not-applicable | — | — | — |

### `modules/datasource/aws-machine-image/index › loadConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads filters without aws config | 298 | not-applicable | — | — | — |
| loads filters with multiple aws configs | 313 | not-applicable | — | — | — |

---

