# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/github-actions/integration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/github-actions/integration.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 17 | **Status:** not-applicable

### `modules/manager/github-actions/integration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| proposes major update when using tagged major, if a major is available | 33 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| switches major-only version to major.minor if no major is available | 87 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes major and minor updates for tagged major.minor | 138 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes minor update for full semver | 203 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes updates for SHA-pinned action with major-only comment | 252 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes updates for SHA-pinned action with major.minor comment | 312 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes updates for SHA-pinned action with full semver comment | 386 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes minor and major updates for floating minor tag | 458 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes no update for major, when only newer patch/minor releases exist | 522 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes minor+major+digest updates for SHA-pinned with floating major comment | 557 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes no update for SHA-pinned when only patch version available and digest unchanged | 617 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| preserves floating major tag when newer patch/minor versions exist with full semver | 652 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| preserves floating major tag when only floating minor tags exist | 702 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| migrates floating major tag to major.minor when only floating minor tags exist | 733 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes minor update for floating minor tag without returning less-specific floating major | 780 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| handles multiple deps in one workflow | 828 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|
| proposes minor and major updates for semver tag | 904 | not-applicable | — | — | mocking framework internals — vi.mock on http/platform/fs; TypeScript GitHub Actions integration|

---
