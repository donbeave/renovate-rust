# `lib/modules/platform/github/schema.spec.ts`

[← `platform/github`](../../../../_by-module/platform/github.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | should be parse directory response | ported | `crates/renovate-core/src/platform/github.rs:5145` |
| 88 | should parse response for single file | ported | `crates/renovate-core/src/platform/github.rs:5157` |
| 112 | should skip vulnerability alerts with unsupported ecosystems | ported | `crates/renovate-core/src/platform/github.rs:5178` |
| 153 | should log vulnerability alerts with parse errors | ported | `crates/renovate-core/src/platform/github.rs:5243` |
| 182 | should filter vulnerability alerts with missing security_vulnerability | ported | `crates/renovate-core/src/platform/github.rs:5255` |
| 207 | should parse severity and cvss_severities fields | ported | `crates/renovate-core/src/platform/github.rs:5204` |

