# `lib/modules/manager/copier/extract.spec.ts`

[← `manager/copier`](../../../../_by-module/manager/copier.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | extracts repository and version from .copier-answers.yml | ported | [`crates/renovate-core/src/extractors/copier.rs:105`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L105) |
| 25 | extracts repository and version from .copier-answers.yml with ssh url | ported | [`crates/renovate-core/src/extractors/copier.rs:146`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L146) |
| 44 | extracts repository and version from .copier-answers.yml with ssh url and non-bare repo | ported | [`crates/renovate-core/src/extractors/copier.rs:172`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L172) |
| 63 | extracts repository and version from .copier-answers.yml with ssh url and a username different from git | ported | [`crates/renovate-core/src/extractors/copier.rs:186`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L186) |
| 84 | _(it.each / template — verify manually)_ | ? | — |
| 119 | returns null for invalid .copier-answers.yml | ported | [`crates/renovate-core/src/extractors/copier.rs:140`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L140) |
| 128 | returns null for invalid _src_path | ported | [`crates/renovate-core/src/extractors/copier.rs:195`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L195) |
| 137 | returns null for missing _commit field | ported | [`crates/renovate-core/src/extractors/copier.rs:133`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L133) |
| 145 | returns null for missing _src_path field | ported | [`crates/renovate-core/src/extractors/copier.rs:159`](../../../../../../../crates/renovate-core/src/extractors/copier.rs#L159) |

