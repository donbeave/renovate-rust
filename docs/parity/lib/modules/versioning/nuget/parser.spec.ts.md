# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nuget/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nuget/parser.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/nuget/parser › parseVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid input | 13 | not-applicable | — | — | Renovate's public NuGet version AST parser is not implemented as a Rust API; Rust has only a private parser for comparator/update-summary behavior. |
| parses version | 18 | not-applicable | — | — | Renovate's public NuGet version AST parser is not implemented as a Rust API; Rust has only a private parser for comparator/update-summary behavior. |

### `modules/versioning/nuget/parser › parseFloatingRange`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid input | 32 | not-applicable | — | — | Renovate's NuGet floating-range parser is not implemented in Rust. |
| $input | 39 | not-applicable | — | — | Renovate's NuGet floating-range parser is not implemented in Rust. |

### `modules/versioning/nuget/parser › getFloatingRangeLowerBound`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 78 | not-applicable | — | — | Renovate's NuGet floating-range lower-bound helper is not implemented in Rust. |

### `modules/versioning/nuget/parser › parseExactRange`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid input | 115 | not-applicable | — | — | Renovate's NuGet exact-range AST parser is not implemented in Rust. |
| parses exact range | 123 | not-applicable | — | — | Renovate's NuGet exact-range AST parser is not implemented in Rust. |

### `modules/versioning/nuget/parser › parseBracketRange`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid input | 137 | not-applicable | — | — | Renovate's NuGet bracket-range AST parser is not implemented in Rust. |
| parses range without lower bound | 147 | not-applicable | — | — | Renovate's NuGet bracket-range AST parser is not implemented in Rust. |
| parses range without upper bound | 157 | not-applicable | — | — | Renovate's NuGet bracket-range AST parser is not implemented in Rust. |
| $input | 168 | not-applicable | — | — | Renovate's NuGet bracket-range bounds inclusivity parser is not implemented in Rust. |
| handles whitespaces | 185 | not-applicable | — | — | Renovate's NuGet bracket-range AST parser is not implemented in Rust. |
| handles floating ranges as lower bounds | 195 | not-applicable | — | — | Renovate's NuGet bracket-range AST parser with floating bounds is not implemented in Rust. |

### `modules/versioning/nuget/parser › versionToString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $version | 224 | not-applicable | — | — | Renovate's NuGet version AST stringifier is not implemented in Rust. |

### `modules/versioning/nuget/parser › rangeToString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $version | 242 | not-applicable | — | — | Renovate's NuGet range AST parser/stringifier is not implemented in Rust. |

---

