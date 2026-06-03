# `lib/modules/manager/xcodegen/extract.spec.ts`

[← `manager/xcodegen`](../../../../_by-module/manager/xcodegen.md) · [all modules](../../../../README.md)

**24/24 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns null for empty content | ported | `crates/renovate-core/src/extractors/xcodegen.rs:529` |
| 11 | returns null for invalid yaml | ported | `crates/renovate-core/src/extractors/xcodegen.rs:563` |
| 22 | returns null for yaml without packages | ported | `crates/renovate-core/src/extractors/xcodegen.rs:523` |
| 36 | returns null for empty packages | ported | `crates/renovate-core/src/extractors/xcodegen.rs:535` |
| 44 | extracts packages from a realistic project.yml | ported | `crates/renovate-core/src/extractors/xcodegen.rs:488` |
| 71 | extracts remote package with url and from | ported | `crates/renovate-core/src/extractors/xcodegen.rs:419` |
| 92 | extracts remote package with github shorthand | ported | `crates/renovate-core/src/extractors/xcodegen.rs:440` |
| 113 | extracts remote package with majorversion | ported | `crates/renovate-core/src/extractors/xcodegen.rs:569` |
| 134 | extracts remote package with minorversion | ported | `crates/renovate-core/src/extractors/xcodegen.rs:580` |
| 155 | extracts remote package with exactversion | ported | `crates/renovate-core/src/extractors/xcodegen.rs:591` |
| 176 | extracts remote package with version | ported | `crates/renovate-core/src/extractors/xcodegen.rs:602` |
| 197 | skips local packages with path | ported | `crates/renovate-core/src/extractors/xcodegen.rs:458` |
| 214 | skips packages with branch reference | ported | `crates/renovate-core/src/extractors/xcodegen.rs:471` |
| 233 | skips packages with revision reference | ported | `crates/renovate-core/src/extractors/xcodegen.rs:542` |
| 252 | skips packages with minversion/maxversion range | ported | `crates/renovate-core/src/extractors/xcodegen.rs:614` |
| 272 | uses gitlab-tags datasource for gitlab urls | ported | `crates/renovate-core/src/extractors/xcodegen.rs:627` |
| 293 | uses github-tags datasource with registryurls for self-hosted ghes | ported | `crates/renovate-core/src/extractors/xcodegen.rs:716` |
| 314 | uses gitlab-tags datasource with registryurls for self-hosted gitlab | ported | `crates/renovate-core/src/extractors/xcodegen.rs:734` |
| 335 | uses git-tags datasource for non-github/gitlab urls | ported | `crates/renovate-core/src/extractors/xcodegen.rs:640` |
| 356 | skips packages without url or github | ported | `crates/renovate-core/src/extractors/xcodegen.rs:554` |
| 373 | skips packages without version specifier | ported | `crates/renovate-core/src/extractors/xcodegen.rs:655` |
| 390 | extracts multiple packages correctly | ported | `crates/renovate-core/src/extractors/xcodegen.rs:667` |
| 427 | handles github url with .git suffix | ported | `crates/renovate-core/src/extractors/xcodegen.rs:692` |
| 448 | handles numeric version values from yaml parsing | ported | `crates/renovate-core/src/extractors/xcodegen.rs:705` |

