# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/schema.spec.ts
**Total tests:** 6 | **Ported:** 4 | **Actionable:** 6 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be parse directory response | 5 | ported | `platform/github.rs` | `github_content_response_directory` | — |
| should parse response for single file | 87 | ported | `platform/github.rs` | `github_content_response_single_file` | — |
| should skip vulnerability alerts with unsupported ecosystems | 111 | ported | `platform/github.rs` | `github_vulnerability_alerts_filter_unsupported_ecosystem` | — |
| should log vulnerability alerts with parse errors | 152 | not-applicable | — | — | Asserts expect(logger.logger.debug).toHaveBeenCalledWith — logger spy |
| should filter vulnerability alerts with missing security_vulnerability | 181 | not-applicable | — | — | Asserts expect(logger.logger.debug).not.toHaveBeenCalledWith — logger spy |
| should parse severity and cvss_severities fields | 206 | ported | `platform/github.rs` | `github_vulnerability_alerts_parse_severity_fields` | — |

---

