# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/interpolator.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/interpolator.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/interpolator › validateInterpolatedValues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if not input | 13 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables interpolation is covered under config-specific tests. |
| does not throw error when keys and values are valid | 19 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables interpolation is covered under config-specific tests. |
| throws when input is not a valid object | 25 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables validation is covered under config-specific tests. |
| throws when keys do not follow specified regex patterns | 31 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables validation is covered under config-specific tests. |
| throws when values are not of type string | 40 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables validation is covered under config-specific tests. |

### `util/interpolator › replaceInterpolatedValuesInObject`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces values and deletes secrets | 48 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables interpolation is covered under config-specific tests. |
| replaces values and keeps secrets | 97 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust secrets/variables interpolation is covered under config-specific tests. |
| does not resolve secrets in onboaringConfig | 115 | not-applicable | — | — | Renovate's TypeScript onboardingConfig interpolation exclusion is not implemented as a standalone Rust utility API. |
| throws error if secrets are used in disallowed options | 155 | not-applicable | — | — | Renovate's TypeScript option-level secrets substitution policy is not implemented as a standalone Rust utility API. |
| throws error if secret key is not present in config | 175 | not-applicable | — | — | Renovate's TypeScript interpolator helper API is not implemented directly; Rust missing-secret behavior is covered under config-specific tests. |

---

