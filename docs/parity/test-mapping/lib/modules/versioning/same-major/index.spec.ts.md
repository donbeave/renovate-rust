# `lib/modules/versioning/same-major/index.spec.ts`

[← `versioning/same-major`](../../../../_by-module/versioning/same-major.md) · [all modules](../../../../README.md)

**8/10 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | should return true | ported | [`crates/renovate-core/src/versioning/same_major.rs:47`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L47) |
| 9 | should return false | ported | [`crates/renovate-core/src/versioning/same_major.rs:53`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L53) |
| 18 | should return true when version has same major | ported | [`crates/renovate-core/src/versioning/same_major.rs:62`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L62) |
| 23 | should return false when version has different major | ported | [`crates/renovate-core/src/versioning/same_major.rs:69`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L69) |
| 27 | should return false when version is out of range | ported | [`crates/renovate-core/src/versioning/same_major.rs:75`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L75) |
| 33 | should return false when version is invalid | ported | [`crates/renovate-core/src/versioning/same_major.rs:83`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L83) |
| 39 | should return max satisfying version in range | ported | [`crates/renovate-core/src/versioning/same_major.rs:89`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L89) |
| 50 | should return min satisfying version in range | ported | [`crates/renovate-core/src/versioning/same_major.rs:98`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L98) |
| 61 | should return true | ported | [`crates/renovate-core/src/versioning/same_major.rs:47`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L47) |
| 65 | should return false | ported | [`crates/renovate-core/src/versioning/same_major.rs:53`](../../../../../../../crates/renovate-core/src/versioning/same_major.rs#L53) |

