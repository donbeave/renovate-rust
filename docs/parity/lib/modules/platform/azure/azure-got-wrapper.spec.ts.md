# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/azure-got-wrapper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/azure-got-wrapper.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** not-applicable

### `gitApi`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw an error if no config found | 15 | not-applicable | — | — | All tests use vi.resetModules()/vi.importActual to reload the Azure DevOps SDK client; tests exercise Azure SDK PAT/bearer/password auth handler construction, not pure logic |
| should set personal access token and endpoint | 21 | not-applicable | — | — | All tests use vi.resetModules()/vi.importActual to reload the Azure DevOps SDK client; tests exercise Azure SDK PAT/bearer/password auth handler construction, not pure logic |
| should set bearer token and endpoint | 42 | not-applicable | — | — | All tests use vi.resetModules()/vi.importActual to reload the Azure DevOps SDK client; tests exercise Azure SDK PAT/bearer/password auth handler construction, not pure logic |
| should set password and endpoint | 63 | not-applicable | — | — | All tests use vi.resetModules()/vi.importActual to reload the Azure DevOps SDK client; tests exercise Azure SDK PAT/bearer/password auth handler construction, not pure logic |

---

