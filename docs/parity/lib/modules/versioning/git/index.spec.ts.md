# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/git/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/git/index.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `modules/versioning/git/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isValid("$input") === $expected | 4 | ported | `git.rs` | `is_valid_matches_renovate_git_spec` | — |
| isCompatible("$version") === $expected | 20 | ported | `git.rs` | `is_compatible_matches_renovate_git_spec` | — |
| isGreaterThan("$a", "$b") === $expected | 32 | ported | `git.rs` | `is_greater_than_matches_renovate_git_spec` | — |

---

## Utility specs (`lib/util/`)

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
<!-- util/string-match.spec.ts converted to per-test format above -->
<!-- util/package-rules/index.spec.ts converted to per-test format above -->
<!-- util/package-rules/managers.spec.ts converted to per-test format above -->
<!-- util/package-rules/dep-names.spec.ts converted to per-test format above -->
<!-- util/package-rules/current-value.spec.ts converted to per-test format above -->
<!-- util/package-rules/current-age.spec.ts converted to per-test format above -->
<!-- util/package-rules/current-version.spec.ts converted to per-test format above -->
<!-- util/package-rules/files.spec.ts converted to per-test format above -->
<!-- util/package-rules/new-value.spec.ts converted to per-test format above -->
<!-- util/package-rules/package-names.spec.ts converted to per-test format above -->
<!-- util/package-rules/repositories.spec.ts converted to per-test format above -->
<!-- util/package-rules/jsonata.spec.ts converted to per-test format above -->
---

## Full Upstream Scan Backfill

The sections below were generated from a full `../renovate/**/*.spec.ts` scan on 2026-05-12 so every currently unrepresented upstream spec has explicit rows. They should be converted from `pending` to `ported` or `not-applicable` during normal parity work.

---

