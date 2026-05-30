# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/deno/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/deno/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `modules/versioning/deno/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$version") === $isValid | 4 | ported | crates/renovate-core/src/versioning/deno.rs | is_valid_matches_renovate_deno_spec | — |
| getSatisfyingVersion("$versions","$range") === $maxSatisfying | 31 | ported | crates/renovate-core/src/versioning/deno.rs | get_satisfying_version_matches_renovate_deno_spec | — |
| isSingleVersion("$version") === $isSingle | 47 | ported | crates/renovate-core/src/versioning/deno.rs | is_single_version_matches_renovate_deno_spec | — |
| subset("$a", "$b") === $expected | 58 | ported | crates/renovate-core/src/versioning/deno.rs | subset_matches_renovate_deno_spec | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 72 | ported | crates/renovate-core/src/versioning/deno.rs | get_new_value_matches_renovate_deno_spec | — |

---

