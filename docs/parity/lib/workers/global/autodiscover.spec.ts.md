# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/global/autodiscover.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/autodiscover.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** done

### `workers/global/autodiscover`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws if local and repositories defined | 28 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| returns local | 34 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| returns if not autodiscovering | 41 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| autodiscovers github but empty | 45 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| autodiscovers github repos | 56 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos | 67 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered dot repos | 81 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos but nothing matches | 95 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos with regex | 109 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos with regex negation | 123 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos with minimatch negation | 141 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| fail if regex pattern is not valid | 155 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos with multiple values | 169 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |
| filters autodiscovered github repos case-insensitive | 192 | not-applicable | — | — | Requires vi.mock(platform/github) + vi.mock(host-rules) mock infrastructure |

---

