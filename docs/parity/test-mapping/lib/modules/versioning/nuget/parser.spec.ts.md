# `lib/modules/versioning/nuget/parser.spec.ts`

[← `versioning/nuget`](../../../../_by-module/versioning/nuget.md) · [all modules](../../../../README.md)

**10/15 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | returns null for invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1156`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1156) |
| 18 | parses version | ported | [`crates/renovate-core/src/versioning/nuget.rs:1163`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1163) |
| 32 | rejects invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1181`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1181) |
| 39 | _(it.each / template — verify manually)_ | ? | — |
| 78 | _(it.each / template — verify manually)_ | ? | — |
| 115 | rejects invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1181`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1181) |
| 123 | parses exact range | ported | [`crates/renovate-core/src/versioning/nuget.rs:1456`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1456) |
| 137 | rejects invalid input | ported | [`crates/renovate-core/src/versioning/nuget.rs:1181`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1181) |
| 147 | parses range without lower bound | ported | [`crates/renovate-core/src/versioning/nuget.rs:1488`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1488) |
| 157 | parses range without upper bound | ported | [`crates/renovate-core/src/versioning/nuget.rs:1509`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1509) |
| 168 | _(it.each / template — verify manually)_ | ? | — |
| 185 | handles whitespaces | ported | [`crates/renovate-core/src/versioning/nuget.rs:1569`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1569) |
| 195 | handles floating ranges as lower bounds | ported | [`crates/renovate-core/src/versioning/nuget.rs:1597`](../../../../../../../crates/renovate-core/src/versioning/nuget.rs#L1597) |
| 224 | _(it.each / template — verify manually)_ | ? | — |
| 242 | _(it.each / template — verify manually)_ | ? | — |

