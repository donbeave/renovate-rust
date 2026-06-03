# `lib/modules/manager/woodpecker/extract.spec.ts`

[← `manager/woodpecker`](../../../../_by-module/manager/woodpecker.md) · [all modules](../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null for empty | ported | `crates/renovate-core/src/extractors/helmfile.rs:1206` |
| 12 | returns null for non-object yaml | ported | `crates/renovate-core/src/extractors/woodpecker.rs:261` |
| 17 | returns null for malformed yaml | ported | `crates/renovate-core/src/extractors/woodpecker.rs:275` |
| 21 | extracts multiple image lines | ported | `crates/renovate-core/src/extractors/woodpecker.rs:140` |
| 129 | extracts image and replaces registry | ported | `crates/renovate-core/src/extractors/woodpecker.rs:184` |
| 159 | extracts image but no replacement | ported | `crates/renovate-core/src/extractors/woodpecker.rs:205` |
| 189 | extracts image and no double replacement | ported | `crates/renovate-core/src/extractors/woodpecker.rs:226` |
| 220 | extracts the v.1.0.x version | ported | `crates/renovate-core/src/extractors/woodpecker.rs:282` |
| 246 | should parse multiple sources of dependencies together | ported | `crates/renovate-core/src/extractors/woodpecker.rs:296` |
| 286 | return dependency when an plugin-git is cloned | ported | `crates/renovate-core/src/extractors/woodpecker.rs:313` |
| 313 | return null when no dependencies are provided | ported | `crates/renovate-core/src/extractors/woodpecker.rs:268` |

