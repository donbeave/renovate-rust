# `lib/modules/manager/gomod/line-parser.spec.ts`

[← `manager/gomod`](../../../../_by-module/manager/gomod.md) · [all modules](../../../../README.md)

**32/32 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 4 | should return null for invalid input | ported | `crates/renovate-core/src/extractors/gomod.rs:1557` |
| 8 | should parse go version | ported | `crates/renovate-core/src/extractors/gomod.rs:1563` |
| 21 | should skip invalid go version | ported | `crates/renovate-core/src/extractors/gomod.rs:1576` |
| 35 | should parse toolchain version | ported | `crates/renovate-core/src/extractors/gomod.rs:1585` |
| 48 | should skip invalid toolchain version | ported | `crates/renovate-core/src/extractors/gomod.rs:1597` |
| 61 | should parse require definition | ported | `crates/renovate-core/src/extractors/gomod.rs:1606` |
| 73 | should parse require definition with pseudo-version | ported | `crates/renovate-core/src/extractors/gomod.rs:1618` |
| 87 | should parse require definition with placeholder pseudo-version | ported | `crates/renovate-core/src/extractors/gomod.rs:1633` |
| 102 | should parse require multi-line | ported | `crates/renovate-core/src/extractors/gomod.rs:1644` |
| 117 | should parse require definition with quotes | ported | `crates/renovate-core/src/extractors/gomod.rs:1654` |
| 129 | should parse go modules without paths - 1 | ported | `crates/renovate-core/src/extractors/gomod.rs:1663` |
| 140 | should parse go modules without paths - 2 | ported | `crates/renovate-core/src/extractors/gomod.rs:1673` |
| 151 | should parse require multi-line definition with quotes | ported | `crates/renovate-core/src/extractors/gomod.rs:1682` |
| 166 | should parse require definition with indirect dependency | ported | `crates/renovate-core/src/extractors/gomod.rs:1691` |
| 179 | should parse require multi-line definition with indirect dependency | ported | `crates/renovate-core/src/extractors/gomod.rs:1701` |
| 195 | should parse replace definition | ported | `crates/renovate-core/src/extractors/gomod.rs:1710` |
| 206 | should parse replace multi-line definition | ported | `crates/renovate-core/src/extractors/gomod.rs:1720` |
| 220 | should parse replace definition with quotes | ported | `crates/renovate-core/src/extractors/gomod.rs:1730` |
| 231 | should parse replace multi-line definition with quotes | ported | `crates/renovate-core/src/extractors/gomod.rs:1739` |
| 245 | should parse replace definition with version | ported | `crates/renovate-core/src/extractors/gomod.rs:1748` |
| 257 | should parse replace definition with pseudo-version | ported | `crates/renovate-core/src/extractors/gomod.rs:1758` |
| 272 | should parse replace definition with placeholder pseudo-version | ported | `crates/renovate-core/src/extractors/gomod.rs:1770` |
| 288 | should parse replace indirect definition | ported | `crates/renovate-core/src/extractors/gomod.rs:1780` |
| 301 | should parse replace multi-line definition with version | ported | `crates/renovate-core/src/extractors/gomod.rs:1789` |
| 316 | should parse replace definition pointing to relative local path | ported | `crates/renovate-core/src/extractors/gomod.rs:1799` |
| 327 | should parse replace definition pointing to absolute local path | ported | `crates/renovate-core/src/extractors/gomod.rs:1808` |
| 338 | should parse tool definition | ported | `crates/renovate-core/src/extractors/gomod.rs:1817` |
| 349 | should parse tool multi-line | ported | `crates/renovate-core/src/extractors/gomod.rs:1831` |
| 363 | should parse tool definition with quotes | ported | `crates/renovate-core/src/extractors/gomod.rs:1840` |
| 374 | should parse go tool without paths - 1 | ported | `crates/renovate-core/src/extractors/gomod.rs:1849` |
| 385 | should parse go tool without paths - 2 | ported | `crates/renovate-core/src/extractors/gomod.rs:1857` |
| 396 | should parse tool multi-line definition with quotes | ported | `crates/renovate-core/src/extractors/gomod.rs:1865` |

