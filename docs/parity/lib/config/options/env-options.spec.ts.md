# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/options/env-options.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/options/env-options.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/options/env-options`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps camelCase config names to RENOVATE_SCREAMING_SNAKE_CASE env vars | 6 | not-applicable | — | — | TypeScript env option metadata map; Rust env parsing uses explicit static mappings tested through parser behavior. |
| maps multi-word config names correctly | 14 | not-applicable | — | — | TypeScript env option metadata map; Rust env parsing uses explicit static mappings tested through parser behavior. |
| marks globalOnly options correctly | 22 | not-applicable | — | — | TypeScript option metadata registry; Rust global-only validation is not derived from env option metadata. |
| marks non-globalOnly options correctly | 31 | not-applicable | — | — | TypeScript option metadata registry; Rust global-only validation is not derived from env option metadata. |
| marks inheritConfigSupport options correctly | 40 | not-applicable | — | — | TypeScript option metadata registry; Rust inherit-config support is not derived from env option metadata. |
| excludes options with env: false | 49 | not-applicable | — | — | TypeScript env option metadata map; Rust has no generated env map containing disabled entries. |
| includes the option type | 54 | not-applicable | — | — | TypeScript env option metadata map; Rust env parsing uses typed parser functions rather than runtime option type metadata. |

---

