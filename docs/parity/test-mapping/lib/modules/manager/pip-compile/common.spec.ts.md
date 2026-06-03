# `lib/modules/manager/pip-compile/common.spec.ts`

[← `manager/pip-compile`](../../../../_by-module/manager/pip-compile.md) · [all modules](../../../../README.md)

**7/27 in-scope tests ported** (20 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | _(it.each / template — verify manually)_ | ? | — |
| 48 | _(it.each / template — verify manually)_ | ? | — |
| 80 | _(it.each / template — verify manually)_ | ? | — |
| 92 | _(it.each / template — verify manually)_ | ? | — |
| 104 | _(it.each / template — verify manually)_ | ? | — |
| 116 | _(it.each / template — verify manually)_ | ? | — |
| 130 | throws when no source files passed as arguments | pending | — |
| 139 | throws on malformed header | pending | — |
| 143 | throws on mutually exclusive options | pending | — |
| 154 | returned sourcefiles returns all source files | pending | — |
| 172 | _(it.each / template — verify manually)_ | ? | — |
| 184 | _(it.each / template — verify manually)_ | ? | — |
| 196 | detects custom command | pending | — |
| 205 | _(it.each / template — verify manually)_ | ? | — |
| 220 | extracts python version from valid header | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:307`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L307) |
| 229 | returns undefined if version cannot be extracted | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:314`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L314) |
| 235 | handles both registryurls and additionalregistryurls | pending | — |
| 262 | handles multiple additionalregistryurls | pending | — |
| 291 | handles hosts with only a username | pending | — |
| 309 | handles hosts with only a password | pending | — |
| 327 | handles invalid urls | pending | — |
| 342 | handles multiple package files | pending | — |
| 373 | matches pip_setup setup.py | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:267`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L267) |
| 377 | matches setup-cfg setup.cfg | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:273`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L273) |
| 381 | matches pep621 pyproject.toml | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:279`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L279) |
| 385 | matches pip_requirements any .in file | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:285`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L285) |
| 390 | matches pip_requirements any .txt file | ported | [`crates/renovate-core/src/extractors/pip_compile.rs:292`](../../../../../../../crates/renovate-core/src/extractors/pip_compile.rs#L292) |

