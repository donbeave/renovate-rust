# `lib/modules/manager/gradle/utils.spec.ts`

[← `manager/gradle`](../../../../_by-module/manager/gradle.md) · [all modules](../../../../README.md)

**11/12 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | extracts the actual version | ported | [`crates/renovate-core/src/extractors/gradle.rs:2152`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2152) |
| 41 | returns null for invalid inputs | ported | [`crates/renovate-core/src/extractors/gradle.rs:2173`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2173) |
| 57 | _(it.each / template — verify manually)_ | ? | — |
| 85 | _(it.each / template — verify manually)_ | ? | — |
| 105 | filetype checks | ported | [`crates/renovate-core/src/extractors/gradle.rs:2275`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2275) |
| 120 | reorderfiles | ported | [`crates/renovate-core/src/extractors/gradle.rs:2292`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2292) |
| 250 | getvars | ported | [`crates/renovate-core/src/extractors/gradle.rs:2443`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2443) |
| 276 | empty registry | ported | [`crates/renovate-core/src/extractors/gradle.rs:2537`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2537) |
| 285 | updates the registry | ported | [`crates/renovate-core/src/extractors/gradle.rs:2557`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2557) |
| 306 | no default catalog file | ported | [`crates/renovate-core/src/extractors/gradle.rs:2607`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2607) |
| 317 | adds variables with default "libs" prefix | ported | [`crates/renovate-core/src/extractors/gradle.rs:2620`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2620) |
| 357 | adds variables with custom libraries extension name | ported | [`crates/renovate-core/src/extractors/gradle.rs:2665`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2665) |

