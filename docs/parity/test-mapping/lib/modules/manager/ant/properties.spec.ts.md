# `lib/modules/manager/ant/properties.spec.ts`

[← `manager/ant`](../../../../_by-module/manager/ant.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | parses key=value pairs | ported | `crates/renovate-core/src/extractors/ant.rs:805` |
| 28 | skips comments and blank lines | ported | `crates/renovate-core/src/extractors/ant.rs:818` |
| 39 | supports colon separator | ported | `crates/renovate-core/src/extractors/ant.rs:825` |
| 46 | skips malformed lines without separators | ported | `crates/renovate-core/src/extractors/ant.rs:832` |
| 57 | implements first-definition-wins | ported | `crates/renovate-core/src/extractors/ant.rs:845` |
| 64 | respects pre-existing props (first-definition-wins across sources) | ported | `crates/renovate-core/src/extractors/ant.rs:852` |

