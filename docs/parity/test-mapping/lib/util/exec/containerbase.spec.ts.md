# `lib/util/exec/containerbase.spec.ts`

[← `util/exec`](../../../_by-module/util/exec.md) · [all modules](../../../README.md)

**9/20 ported** (11 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | returns false if binarysource is not install | ported | [`crates/renovate-core/src/exec/containerbase.rs:320`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L320) |
| 26 | returns false if not containerbase | ported | [`crates/renovate-core/src/exec/containerbase.rs:331`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L331) |
| 31 | returns false if any unsupported tools | ported | [`crates/renovate-core/src/exec/containerbase.rs:314`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L314) |
| 42 | returns true if supported tools | ported | [`crates/renovate-core/src/exec/containerbase.rs:305`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L305) |
| 51 | returns config for a known tool | ported | [`crates/renovate-core/src/exec/containerbase.rs:291`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L291) |
| 60 | returns undefined for an unknown tool | ported | [`crates/renovate-core/src/exec/containerbase.rs:299`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L299) |
| 67 | returns from config | ported | [`crates/renovate-core/src/exec/containerbase.rs:360`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L360) |
| 73 | returns highest stable | pending | — |
| 87 | returns highest unstable | pending | — |
| 96 | respects latest | pending | — |
| 113 | supports rust docker tags | pending | — |
| 127 | throws for unknown tools | pending | — |
| 133 | throws no releases | pending | — |
| 142 | falls back to latest version if no compatible release | pending | — |
| 151 | falls back to latest version if invalid constraint | pending | — |
| 160 | _(it.each / template — verify manually)_ | ? | — |
| 184 | removes pep440 == | ported | [`crates/renovate-core/src/exec/containerbase.rs:393`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L393) |
| 193 | _(it.each / template — verify manually)_ | ? | — |
| 223 | _(it.each / template — verify manually)_ | ? | — |
| 269 | returns install commands | ported | [`crates/renovate-core/src/exec/containerbase.rs:404`](../../../../../../crates/renovate-core/src/exec/containerbase.rs#L404) |

