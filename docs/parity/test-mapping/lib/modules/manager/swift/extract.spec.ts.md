# `lib/modules/manager/swift/extract.spec.ts`

[← `manager/swift`](../../../../_by-module/manager/swift.md) · [all modules](../../../../README.md)

**21/21 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns null for empty content | ported | [`crates/renovate-core/src/extractors/spm.rs:772`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L772) |
| 11 | returns null for content without dependencies | ported | [`crates/renovate-core/src/extractors/spm.rs:778`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L778) |
| 31 | extracts github dependencies with github-tags datasource | ported | [`crates/renovate-core/src/extractors/spm.rs:793`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L793) |
| 52 | extracts gitlab dependencies with gitlab-tags datasource | ported | [`crates/renovate-core/src/extractors/spm.rs:812`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L812) |
| 73 | extracts self-hosted github dependencies with registryurls | ported | [`crates/renovate-core/src/extractors/spm.rs:830`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L830) |
| 95 | extracts self-hosted gitlab dependencies with registryurls | ported | [`crates/renovate-core/src/extractors/spm.rs:850`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L850) |
| 117 | extracts github dependencies from scp-style ssh url | ported | [`crates/renovate-core/src/extractors/spm.rs:1302`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1302) |
| 138 | extracts gitlab dependencies from scp-style ssh url | ported | [`crates/renovate-core/src/extractors/spm.rs:1321`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1321) |
| 159 | extracts dependencies from ssh:// url | ported | [`crates/renovate-core/src/extractors/spm.rs:1340`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1340) |
| 180 | returns null for unparseable ssh url | ported | [`crates/renovate-core/src/extractors/spm.rs:1359`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1359) |
| 192 | extracts other dependencies with git-tags datasource | ported | [`crates/renovate-core/src/extractors/spm.rs:870`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L870) |
| 213 | extracts exact version dependencies | ported | [`crates/renovate-core/src/extractors/spm.rs:887`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L887) |
| 234 | extracts exact version with label syntax | ported | [`crates/renovate-core/src/extractors/spm.rs:903`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L903) |
| 255 | extracts range version dependencies | ported | [`crates/renovate-core/src/extractors/spm.rs:919`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L919) |
| 276 | extracts dependencies from sample package file | ported | [`crates/renovate-core/src/extractors/spm.rs:936`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L936) |
| 311 | handles malformed urls gracefully | ported | [`crates/renovate-core/src/extractors/spm.rs:971`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L971) |
| 324 | handles dependencies without version | ported | [`crates/renovate-core/src/extractors/spm.rs:985`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L985) |
| 337 | handles dependencies with local package | ported | [`crates/renovate-core/src/extractors/spm.rs:999`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L999) |
| 350 | handles dependencies with name (deprecated args) | ported | [`crates/renovate-core/src/extractors/spm.rs:1011`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1011) |
| 365 | extracts multiple dependencies with different datasources | ported | [`crates/renovate-core/src/extractors/spm.rs:1026`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1026) |
| 383 | extracts multiple dependencies with traits arguments | ported | [`crates/renovate-core/src/extractors/spm.rs:1046`](../../../../../../../crates/renovate-core/src/extractors/spm.rs#L1046) |

