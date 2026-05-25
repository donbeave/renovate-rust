# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/java-version/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/java-version/common.spec.ts
**Total tests:** 3 | **Ported:** 2 | **Actionable:** 3 | **Status:** partial

### `modules/datasource/java-version/common`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no os and architecture | 10 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `no_os_and_architecture` | jre → image_type=jre, os=None, arch=None |
| system jdk -> (%s, %s, %s) => %o | 20 | not-applicable | — | — | mocks Node.js process.arch/platform; Rust uses compile-time constants not runtime-mockable |
| logs for unsupported os and architecture | 74 | ported | `crates/renovate-core/src/datasources/java_version.rs` | `unsupported_os_and_architecture_returns_none` | unknown arch/OS mapping → None |

---

