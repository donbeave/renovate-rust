# `lib/modules/manager/homebrew/extract.spec.ts`

[← `manager/homebrew`](../../../../_by-module/manager/homebrew.md) · [all modules](../../../../README.md)

**17/17 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | skips sourceforge dependency 1 | ported | [`crates/renovate-core/src/extractors/homebrew.rs:709`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L709) |
| 32 | skips sourceforge dependency 2 | ported | [`crates/renovate-core/src/extractors/homebrew.rs:749`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L749) |
| 54 | skips github dependency with wrong format | ported | [`crates/renovate-core/src/extractors/homebrew.rs:763`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L763) |
| 77 | extracts "releases" github dependency | ported | [`crates/renovate-core/src/extractors/homebrew.rs:680`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L680) |
| 99 | extracts "archive" github dependency | ported | [`crates/renovate-core/src/extractors/homebrew.rs:641`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L641) |
| 121 | handles old "archive" github url format | ported | [`crates/renovate-core/src/extractors/homebrew.rs:662`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L662) |
| 152 | handles no space before class header | ported | [`crates/renovate-core/src/extractors/homebrew.rs:779`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L779) |
| 183 | returns null for invalid class header 1 | ported | [`crates/renovate-core/src/extractors/homebrew.rs:741`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L741) |
| 198 | returns null for invalid class header 2 | ported | [`crates/renovate-core/src/extractors/homebrew.rs:727`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L727) |
| 213 | skips if there is no url field | ported | [`crates/renovate-core/src/extractors/homebrew.rs:720`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L720) |
| 235 | skips if invalid url protocol | ported | [`crates/renovate-core/src/extractors/homebrew.rs:800`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L800) |
| 257 | skips if invalid url | ported | [`crates/renovate-core/src/extractors/homebrew.rs:809`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L809) |
| 279 | skips if there is no sha256 field | ported | [`crates/renovate-core/src/extractors/homebrew.rs:820`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L820) |
| 301 | skips if sha256 field is invalid | ported | [`crates/renovate-core/src/extractors/homebrew.rs:698`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L698) |
| 323 | extracts npm scoped package dependency | ported | [`crates/renovate-core/src/extractors/homebrew.rs:831`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L831) |
| 354 | extracts npm unscoped package dependency | ported | [`crates/renovate-core/src/extractors/homebrew.rs:851`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L851) |
| 385 | skips npm package from custom registry | ported | [`crates/renovate-core/src/extractors/homebrew.rs:870`](../../../../../../../crates/renovate-core/src/extractors/homebrew.rs#L870) |

