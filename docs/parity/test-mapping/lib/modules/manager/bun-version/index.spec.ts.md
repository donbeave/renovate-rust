# `lib/modules/manager/bun-version/index.spec.ts`

[← `manager/bun-version`](../../../../_by-module/manager/bun-version.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | returns a result | ported | [`crates/renovate-core/src/extractors/bun_version.rs:42`](../../../../../../../crates/renovate-core/src/extractors/bun_version.rs#L42) |
| 17 | handles empty files | ported | [`crates/renovate-core/src/extractors/bun_version.rs:55`](../../../../../../../crates/renovate-core/src/extractors/bun_version.rs#L55) |
| 22 | handles no newline at the end | ported | [`crates/renovate-core/src/extractors/bun_version.rs:61`](../../../../../../../crates/renovate-core/src/extractors/bun_version.rs#L61) |
| 27 | handles multiple lines | ported | [`crates/renovate-core/src/extractors/bun_version.rs:67`](../../../../../../../crates/renovate-core/src/extractors/bun_version.rs#L67) |
| 32 | handles invalid versions | ported | [`crates/renovate-core/src/extractors/bun_version.rs:73`](../../../../../../../crates/renovate-core/src/extractors/bun_version.rs#L73) |
| 45 | handles ranges | ported | [`crates/renovate-core/src/extractors/bun_version.rs:81`](../../../../../../../crates/renovate-core/src/extractors/bun_version.rs#L81) |

