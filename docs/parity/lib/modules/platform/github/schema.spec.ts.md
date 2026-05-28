# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/schema.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be parse directory response | 5 | pending | — | — | — |
| should parse response for single file | 87 | pending | — | — | — |
| should skip vulnerability alerts with unsupported ecosystems | 111 | pending | — | — | — |
| should log vulnerability alerts with parse errors | 152 | not-applicable | — | — | Asserts expect(logger.logger.debug).toHaveBeenCalledWith — logger spy |
| should filter vulnerability alerts with missing security_vulnerability | 181 | not-applicable | — | — | Asserts expect(logger.logger.debug).not.toHaveBeenCalledWith — logger spy |
| should parse severity and cvss_severities fields | 206 | pending | — | — | — |

---

