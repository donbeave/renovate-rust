# `lib/modules/manager/mise/backends.spec.ts`

[← `manager/mise`](../../../../_by-module/manager/mise.md) · [all modules](../../../../README.md)

**29/37 ported** (8 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | should create a tooling config | ported | [`crates/renovate-core/src/extractors/mise.rs:2669`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2669) |
| 27 | should trim the leading v from version | ported | [`crates/renovate-core/src/extractors/mise.rs:2679`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2679) |
| 40 | should create a tooling config for crate | ported | [`crates/renovate-core/src/extractors/mise.rs:2686`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2686) |
| 47 | should create a tooling config for git tag | ported | [`crates/renovate-core/src/extractors/mise.rs:2695`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2695) |
| 57 | should provide skipreason for git branch | ported | [`crates/renovate-core/src/extractors/mise.rs:2704`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2704) |
| 70 | should create a tooling config for git rev | ported | [`crates/renovate-core/src/extractors/mise.rs:2712`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2712) |
| 80 | should provide skipreason for invalid version | ported | [`crates/renovate-core/src/extractors/mise.rs:2720`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2720) |
| 91 | should create a tooling config | ported | [`crates/renovate-core/src/extractors/mise.rs:2669`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2669) |
| 100 | should create a tooling config | ported | [`crates/renovate-core/src/extractors/mise.rs:2669`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2669) |
| 109 | should create a tooling config with empty options | ported | [`crates/renovate-core/src/extractors/mise.rs:2744`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2744) |
| 119 | should not set extractversion if the version has leading v | ported | [`crates/renovate-core/src/extractors/mise.rs:2754`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2754) |
| 127 | should set extractversion with custom version_prefix | ported | [`crates/renovate-core/src/extractors/mise.rs:2762`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2762) |
| 140 | should set extractversion with version_prefix even if version has leading v | ported | [`crates/renovate-core/src/extractors/mise.rs:2772`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2772) |
| 153 | should handle empty version_prefix with version not having v | ported | [`crates/renovate-core/src/extractors/mise.rs:2782`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2782) |
| 163 | should handle empty version_prefix with version having v | ported | [`crates/renovate-core/src/extractors/mise.rs:2789`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2789) |
| 173 | should escape special regex characters in version_prefix | ported | [`crates/renovate-core/src/extractors/mise.rs:2796`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2796) |
| 186 | should escape brackets and parentheses in version_prefix | ported | [`crates/renovate-core/src/extractors/mise.rs:2806`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2806) |
| 201 | should create a tooling config | ported | [`crates/renovate-core/src/extractors/mise.rs:2669`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2669) |
| 210 | should create a tooling config | ported | [`crates/renovate-core/src/extractors/mise.rs:2669`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2669) |
| 219 | should create a tooling config for pypi package | ported | [`crates/renovate-core/src/extractors/mise.rs:2832`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2832) |
| 226 | should create a tooling config for github shorthand | ported | [`crates/renovate-core/src/extractors/mise.rs:2840`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2840) |
| 233 | should create a tooling config for github url | ported | [`crates/renovate-core/src/extractors/mise.rs:2848`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2848) |
| 242 | should create a tooling config for git url | ported | [`crates/renovate-core/src/extractors/mise.rs:2856`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2856) |
| 251 | provides skipreason for zip file url | ported | [`crates/renovate-core/src/extractors/mise.rs:2864`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2864) |
| 262 | should create a tooling config for github shorthand | ported | [`crates/renovate-core/src/extractors/mise.rs:2840`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2840) |
| 269 | should create a tooling config for github url | ported | [`crates/renovate-core/src/extractors/mise.rs:2848`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2848) |
| 278 | provides skipreason for other url | ported | [`crates/renovate-core/src/extractors/mise.rs:2887`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2887) |
| 289 | should create a tooling config with empty options | ported | [`crates/renovate-core/src/extractors/mise.rs:2744`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2744) |
| 298 | should set extractversion if the version does not have leading v | ported | [`crates/renovate-core/src/extractors/mise.rs:2904`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2904) |
| 307 | should not set extractversion if the version has leading v | ported | [`crates/renovate-core/src/extractors/mise.rs:2754`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2754) |
| 315 | should ignore options unless tag_regex is provided | ported | [`crates/renovate-core/src/extractors/mise.rs:2918`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2918) |
| 326 | should set extractversion if tag_regex is provided | ported | [`crates/renovate-core/src/extractors/mise.rs:2925`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2925) |
| 339 | should set extractversion without v? when tag_regex is provided and version starts with v | ported | [`crates/renovate-core/src/extractors/mise.rs:2936`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2936) |
| 352 | should trim the leading ^ from tag_regex | ported | [`crates/renovate-core/src/extractors/mise.rs:2950`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2950) |
| 365 | should only trim the leading ^ from tag_regex when version starts with v | ported | [`crates/renovate-core/src/extractors/mise.rs:2964`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2964) |
| 378 | should trim the leading ^v from tag_regex | ported | [`crates/renovate-core/src/extractors/mise.rs:2978`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2978) |
| 391 | should trim the leading ^v? from tag_regex | ported | [`crates/renovate-core/src/extractors/mise.rs:2992`](../../../../../../../crates/renovate-core/src/extractors/mise.rs#L2992) |

