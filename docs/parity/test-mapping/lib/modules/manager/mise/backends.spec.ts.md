# `lib/modules/manager/mise/backends.spec.ts`

[← `manager/mise`](../../../../_by-module/manager/mise.md) · [all modules](../../../../README.md)

**29/37 ported** (8 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | should create a tooling config | ported | `crates/renovate-core/src/extractors/mise.rs:2669` |
| 27 | should trim the leading v from version | ported | `crates/renovate-core/src/extractors/mise.rs:2679` |
| 40 | should create a tooling config for crate | ported | `crates/renovate-core/src/extractors/mise.rs:2686` |
| 47 | should create a tooling config for git tag | ported | `crates/renovate-core/src/extractors/mise.rs:2695` |
| 57 | should provide skipreason for git branch | ported | `crates/renovate-core/src/extractors/mise.rs:2704` |
| 70 | should create a tooling config for git rev | ported | `crates/renovate-core/src/extractors/mise.rs:2712` |
| 80 | should provide skipreason for invalid version | ported | `crates/renovate-core/src/extractors/mise.rs:2720` |
| 91 | should create a tooling config | ported | `crates/renovate-core/src/extractors/mise.rs:2669` |
| 100 | should create a tooling config | ported | `crates/renovate-core/src/extractors/mise.rs:2669` |
| 109 | should create a tooling config with empty options | ported | `crates/renovate-core/src/extractors/mise.rs:2744` |
| 119 | should not set extractversion if the version has leading v | ported | `crates/renovate-core/src/extractors/mise.rs:2754` |
| 127 | should set extractversion with custom version_prefix | ported | `crates/renovate-core/src/extractors/mise.rs:2762` |
| 140 | should set extractversion with version_prefix even if version has leading v | ported | `crates/renovate-core/src/extractors/mise.rs:2772` |
| 153 | should handle empty version_prefix with version not having v | ported | `crates/renovate-core/src/extractors/mise.rs:2782` |
| 163 | should handle empty version_prefix with version having v | ported | `crates/renovate-core/src/extractors/mise.rs:2789` |
| 173 | should escape special regex characters in version_prefix | ported | `crates/renovate-core/src/extractors/mise.rs:2796` |
| 186 | should escape brackets and parentheses in version_prefix | ported | `crates/renovate-core/src/extractors/mise.rs:2806` |
| 201 | should create a tooling config | ported | `crates/renovate-core/src/extractors/mise.rs:2669` |
| 210 | should create a tooling config | ported | `crates/renovate-core/src/extractors/mise.rs:2669` |
| 219 | should create a tooling config for pypi package | ported | `crates/renovate-core/src/extractors/mise.rs:2832` |
| 226 | should create a tooling config for github shorthand | ported | `crates/renovate-core/src/extractors/mise.rs:2840` |
| 233 | should create a tooling config for github url | ported | `crates/renovate-core/src/extractors/mise.rs:2848` |
| 242 | should create a tooling config for git url | ported | `crates/renovate-core/src/extractors/mise.rs:2856` |
| 251 | provides skipreason for zip file url | ported | `crates/renovate-core/src/extractors/mise.rs:2864` |
| 262 | should create a tooling config for github shorthand | ported | `crates/renovate-core/src/extractors/mise.rs:2840` |
| 269 | should create a tooling config for github url | ported | `crates/renovate-core/src/extractors/mise.rs:2848` |
| 278 | provides skipreason for other url | ported | `crates/renovate-core/src/extractors/mise.rs:2887` |
| 289 | should create a tooling config with empty options | ported | `crates/renovate-core/src/extractors/mise.rs:2744` |
| 298 | should set extractversion if the version does not have leading v | ported | `crates/renovate-core/src/extractors/mise.rs:2904` |
| 307 | should not set extractversion if the version has leading v | ported | `crates/renovate-core/src/extractors/mise.rs:2754` |
| 315 | should ignore options unless tag_regex is provided | ported | `crates/renovate-core/src/extractors/mise.rs:2918` |
| 326 | should set extractversion if tag_regex is provided | ported | `crates/renovate-core/src/extractors/mise.rs:2925` |
| 339 | should set extractversion without v? when tag_regex is provided and version starts with v | ported | `crates/renovate-core/src/extractors/mise.rs:2936` |
| 352 | should trim the leading ^ from tag_regex | ported | `crates/renovate-core/src/extractors/mise.rs:2950` |
| 365 | should only trim the leading ^ from tag_regex when version starts with v | ported | `crates/renovate-core/src/extractors/mise.rs:2964` |
| 378 | should trim the leading ^v from tag_regex | ported | `crates/renovate-core/src/extractors/mise.rs:2978` |
| 391 | should trim the leading ^v? from tag_regex | ported | `crates/renovate-core/src/extractors/mise.rs:2992` |

