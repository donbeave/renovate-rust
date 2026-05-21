# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/config-formatter.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/config-formatter.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `createNuGetConfigXml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns xml with registries | 12 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |
| returns xml with authenticated registries | 58 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |
| escapes registry credential names containing special characters | 138 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |
| strips protocol version from feed url | 181 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |
| includes packageSourceMapping when defined | 202 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |
| excludes packageSourceMapping when undefined | 245 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |
| skips duplicate registry URLs | 265 | not-applicable | — | — | tests NuGet XML config generation; Rust NuGet extractor uses own output format |

---

