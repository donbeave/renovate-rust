# `lib/modules/versioning/nuget/parser.spec.ts`

[← `versioning/nuget`](../../../../_by-module/versioning/nuget.md) · [all modules](../../../../README.md)

**10/15 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns null for invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1152`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1152) |
| 18 | parses version | ported | [`crates/renovate-core/src/versioning/nuget.rs:1159`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1159) |
| 32 | rejects invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1177`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1177) |
| 39 | _(it.each / template — verify manually)_ | ? | — |
| 78 | _(it.each / template — verify manually)_ | ? | — |
| 115 | rejects invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1177`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1177) |
| 123 | parses exact range | ported | [`crates/renovate-core/src/versioning/nuget.rs:1462`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1462) |
| 137 | rejects invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1177`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1177) |
| 147 | parses range without lower bound | ported | [`crates/renovate-core/src/versioning/nuget.rs:1494`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1494) |
| 157 | parses range without upper bound | ported | [`crates/renovate-core/src/versioning/nuget.rs:1515`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1515) |
| 168 | _(it.each / template — verify manually)_ | ? | — |
| 185 | handles whitespaces | ported | [`crates/renovate-core/src/versioning/nuget.rs:1575`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1575) |
| 195 | handles floating ranges as lower bounds | ported | [`crates/renovate-core/src/versioning/nuget.rs:1603`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1603) |
| 224 | _(it.each / template — verify manually)_ | ? | — |
| 242 | _(it.each / template — verify manually)_ | ? | — |

