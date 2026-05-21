# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gomod/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gomod/update.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces existing value | 12 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces golang version update | 28 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces go toolchain | 44 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces two values in one file | 60 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| returns same | 90 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| bumps major v0 > v1 | 104 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces major updates > 1 | 123 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| bumps major with single package name component | 142 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| bumps major with multiple package name components | 161 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces major gopkg.in updates | 182 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| skip replacing incompatible major updates | 202 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| returns null if mismatch | 223 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| returns null if error | 237 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces multiline | 247 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces quoted multiline | 263 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| replaces major multiline | 280 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| bumps major multiline | 299 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| bumps major v0 > v1 multiline | 318 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| update multiline digest | 337 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| skips already-updated multiline digest | 356 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| updates pseudo-version with digest updateType | 373 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles multiline mismatch | 403 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles +incompatible tag | 418 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles +incompatible tag without duplicating it | 437 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles replace line with minor version update | 458 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles replace line with major version update | 474 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles replace line with major version update that bumps both sides of the replace | 494 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles replace line with digest | 525 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles no pinned version to latest available version | 546 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| handles multiline replace update | 565 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| should return null for replacement | 589 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| should perform indirect upgrades when top-level | 598 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |
| should perform indirect upgrades when in require blocks | 614 | not-applicable | — | — | tests go mod tidy/update commands via Node.js exec; external tool invocation out of scope |

---

