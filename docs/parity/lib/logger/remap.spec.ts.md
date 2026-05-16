# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/logger/remap.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/logger/remap.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `logger/remap`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no remaps are set | 15 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| performs global remaps | 24 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| performs repository-level remaps | 33 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| prioritizes repository-level remaps over global remaps | 44 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| supports regex patterns | 55 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |
| does not match against invalid regex patterns | 64 | not-applicable | — | — | Bunyan JavaScript log-level remapping hook; Rust tracing logger does not expose mutable per-message remap configuration. |

---

