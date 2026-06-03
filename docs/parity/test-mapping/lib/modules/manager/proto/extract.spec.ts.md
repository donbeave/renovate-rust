# `lib/modules/manager/proto/extract.spec.ts`

[← `manager/proto`](../../../../_by-module/manager/proto.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns null for empty content | ported | [`crates/renovate-core/src/extractors/proto.rs:179`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L179) |
| 14 | returns null for invalid toml | ported | [`crates/renovate-core/src/extractors/proto.rs:185`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L185) |
| 18 | returns null when only config sections exist | ported | [`crates/renovate-core/src/extractors/proto.rs:191`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L191) |
| 29 | extracts a single tool version | ported | [`crates/renovate-core/src/extractors/proto.rs:198`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L198) |
| 46 | extracts multiple tool versions | ported | [`crates/renovate-core/src/extractors/proto.rs:209`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L209) |
| 76 | skips non-version sections | ported | [`crates/renovate-core/src/extractors/proto.rs:223`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L223) |
| 105 | handles proto self-versioning | ported | [`crates/renovate-core/src/extractors/proto.rs:233`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L233) |
| 122 | handles moon tool | ported | [`crates/renovate-core/src/extractors/proto.rs:242`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L242) |
| 139 | handles uv tool | ported | [`crates/renovate-core/src/extractors/proto.rs:251`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L251) |
| 156 | marks unknown tools as unsupported-datasource | ported | [`crates/renovate-core/src/extractors/proto.rs:260`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L260) |
| 172 | skips alias values like latest | ported | [`crates/renovate-core/src/extractors/proto.rs:269`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L269) |
| 188 | skips alias value stable | ported | [`crates/renovate-core/src/extractors/proto.rs:279`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L279) |
| 204 | handles partial versions | ported | [`crates/renovate-core/src/extractors/proto.rs:287`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L287) |
| 221 | extracts all supported tools from fixture | ported | [`crates/renovate-core/src/extractors/proto.rs:297`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L297) |
| 278 | extracts all supported built-in tools | ported | [`crates/renovate-core/src/extractors/proto.rs:310`](../../../../../../../crates/renovate-core/src/extractors/proto.rs#L310) |

