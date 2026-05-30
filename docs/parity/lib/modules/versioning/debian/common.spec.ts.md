# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/debian/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/debian/common.spec.ts
**Total tests:** 5 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `modules/versioning/debian/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no rolling release data | 15 | not-applicable | — | — | mocking framework internals — dataFiles.set() mock + logger.debug spy|
| isDatedCodeName("$input") === $expected | 31 | ported | `deb.rs` | `debian_is_dated_codename` | — |
| getDatedContainerImageCodename("$input") === $expected | 48 | ported | `deb.rs` | `debian_get_dated_container_image_codename` | — |
| getDatedContainerImageVersion("$input") === $expected | 69 | ported | `deb.rs` | `debian_get_dated_container_image_version` | — |
| getDatedContainerImageSuffix("$input") === $expected | 87 | ported | `deb.rs` | `debian_get_dated_container_image_suffix` | — |

---

