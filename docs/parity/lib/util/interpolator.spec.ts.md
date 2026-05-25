# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/interpolator.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/interpolator.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** pending

### `util/interpolator › validateInterpolatedValues`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if not input | 13 | pending | — | — | — |
| does not throw error when keys and values are valid | 19 | pending | — | — | — |
| throws when input is not a valid object | 25 | pending | — | — | — |
| throws when keys do not follow specified regex patterns | 31 | pending | — | — | — |
| throws when values are not of type string | 40 | pending | — | — | — |

### `util/interpolator › replaceInterpolatedValuesInObject`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces values and deletes secrets | 48 | pending | — | — | — |
| replaces values and keeps secrets | 97 | pending | — | — | — |
| does not resolve secrets in onboaringConfig | 115 | pending | — | — | — |
| throws error if secrets are used in disallowed options | 155 | pending | — | — | — |
| throws error if secret key is not present in config | 175 | pending | — | — | — |

---

