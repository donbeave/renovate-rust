# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/custom/regex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/custom/regex/index.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has default config | 13 | pending | — | — | — |
| has displayName | 19 | pending | — | — | — |
| extracts multiple dependencies | 23 | pending | — | — | — |
| returns null if no dependencies found | 50 | pending | — | — | — |
| returns null if invalid template | 64 | pending | — | — | — |
| extracts extractVersion | 81 | pending | — | — | — |
| extracts registryUrl | 103 | pending | — | — | — |
| extracts and applies a registryUrlTemplate | 141 | pending | — | — | — |
| extracts and does not apply a registryUrlTemplate if the result is an invalid url | 162 | not-applicable | — | — | Asserts expect(logger.warn).toHaveBeenCalledWith — logger spy infrastructure |
| extracts multiple dependencies with multiple matchStrings | 195 | pending | — | — | — |
| extracts dependency with autoReplaceStringTemplate | 221 | pending | — | — | — |
| extracts indentation: maintains indentation value if whitespace or empty | 241 | pending | — | — | — |
| extracts indentation: discards non-whitespace content | 270 | pending | — | — | — |
| extracts with combination strategy | 299 | pending | — | — | — |
| extracts with combination strategy and non standard capture groups | 319 | pending | — | — | — |
| extracts with combination strategy and multiple matches | 343 | pending | — | — | — |
| extracts with combination strategy and registry url | 363 | pending | — | — | — |
| extracts with combination strategy: sets replaceString when current version group present | 384 | pending | — | — | — |
| extracts with combination strategy: sets replaceString when current digest group present | 413 | pending | — | — | — |
| extracts with combination strategy and templates | 442 | pending | — | — | — |
| extracts with combination strategy and empty file | 463 | pending | — | — | — |
| extracts with recursive strategy and single match | 479 | pending | — | — | — |
| extracts with recursive strategy and multiple matches | 498 | pending | — | — | — |
| extracts with recursive strategy and multiple layers | 517 | pending | — | — | — |
| extracts with recursive strategy and fail because of not sufficient regexes | 537 | pending | — | — | — |
| extracts with recursive strategy and fail because there is no match | 552 | pending | — | — | — |
| extracts with recursive strategy and merged groups | 567 | pending | — | — | — |
| extracts with recursive strategy and without depName | 588 | pending | — | — | — |
| dotnet | 620 | pending | — | — | — |
| uses package file as dep name | 685 | pending | — | — | — |
| uses package file dir as dep name | 705 | pending | — | — | — |

---

