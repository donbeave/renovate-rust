# `lib/modules/manager/gradle/extract/consistent-versions-plugin.spec.ts`

[← `manager/gradle`](../../../../../_by-module/manager/gradle.md) · [all modules](../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | works for sub folders | ported | `crates/renovate-core/src/extractors/gradle.rs:2732` |
| 24 | detects lock file header introduced with gradle-consistent-versions version 2.20.0 | ported | `crates/renovate-core/src/extractors/gradle.rs:2751` |
| 36 | detects lock file header introduced with gradle-consistent-versions version 2.23.0 | ported | `crates/renovate-core/src/extractors/gradle.rs:2762` |
| 48 | correct position for crlf and lf | ported | `crates/renovate-core/src/extractors/gradle.rs:2773` |
| 60 | test bogus input lines | ported | `crates/renovate-core/src/extractors/gradle.rs:2792` |
| 97 | supports multiple levels of glob | ported | `crates/renovate-core/src/extractors/gradle.rs:3407` |

