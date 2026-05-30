# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/sbt/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sbt/util.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 0 | **Status:** done

### `sortPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| places build.sbt first | 5 | ported | `sbt.rs` | `sbt_sort_package_files_build_sbt_first` | — |

### `normalizeScalaVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not normalize prior to 2.10 | 20 | ported | `sbt.rs` | `sbt_normalize_scala_version_prior_to_2_10` | — |
| normalizes a Scala 2.10 version number | 25 | ported | `sbt.rs` | `sbt_normalize_scala_2_10` | — |
| normalizes a Scala 2.11 version number | 30 | ported | `sbt.rs` | `sbt_normalize_scala_2_11` | — |
| normalizes a Scala 2.12 version number | 35 | ported | `sbt.rs` | `sbt_normalize_scala_2_12` | — |
| normalizes a Scala 2.13 version number | 40 | ported | `sbt.rs` | `sbt_normalize_scala_2_13` | — |
| normalizes a Scala 3 LTS version number | 45 | ported | `sbt.rs` | `sbt_normalize_scala_3_lts` | — |
| normalizes a Scala 3 current version number | 50 | ported | `sbt.rs` | `sbt_normalize_scala_3_current` | — |

---

