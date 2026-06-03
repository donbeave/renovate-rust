# `lib/modules/versioning/pep440/range.spec.ts`

[← `versioning/pep440`](../../../../_by-module/versioning/pep440.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | _(it.each / template — verify manually)_ | ? | — |
| 24 | returns null without warning if new version is excluded from range | ported | `crates/renovate-core/src/versioning/pep440.rs:1265` |
| 39 | handles v-prefixed version as currentvalue | ported | `crates/renovate-core/src/versioning/pep440.rs:1278` |
| 49 | handles bare version that differs from currentversion without v-prefix | ported | `crates/renovate-core/src/versioning/pep440.rs:1291` |

