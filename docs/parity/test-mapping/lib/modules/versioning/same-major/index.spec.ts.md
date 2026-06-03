# `lib/modules/versioning/same-major/index.spec.ts`

[← `versioning/same-major`](../../../../_by-module/versioning/same-major.md) · [all modules](../../../../README.md)

**8/10 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | should return true | ported | `crates/renovate-core/src/versioning/same_major.rs:47` |
| 9 | should return false | ported | `crates/renovate-core/src/versioning/same_major.rs:53` |
| 18 | should return true when version has same major | ported | `crates/renovate-core/src/versioning/same_major.rs:62` |
| 23 | should return false when version has different major | ported | `crates/renovate-core/src/versioning/same_major.rs:69` |
| 27 | should return false when version is out of range | ported | `crates/renovate-core/src/versioning/same_major.rs:75` |
| 33 | should return false when version is invalid | ported | `crates/renovate-core/src/versioning/same_major.rs:83` |
| 39 | should return max satisfying version in range | ported | `crates/renovate-core/src/versioning/same_major.rs:89` |
| 50 | should return min satisfying version in range | ported | `crates/renovate-core/src/versioning/same_major.rs:98` |
| 61 | should return true | ported | `crates/renovate-core/src/versioning/same_major.rs:47` |
| 65 | should return false | ported | `crates/renovate-core/src/versioning/same_major.rs:53` |

