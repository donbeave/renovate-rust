# `lib/modules/datasource/npm/index.spec.ts`

[← `datasource/npm`](../../../../_by-module/datasource/npm.md) · [all modules](../../../../README.md)

**24/24 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | should return null for no versions | ported | [`crates/renovate-core/src/datasources/npm.rs:770`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L770) |
| 55 | should fetch package info from npm | ported | [`crates/renovate-core/src/datasources/npm.rs:789`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L789) |
| 65 | should parse repo url | ported | [`crates/renovate-core/src/datasources/npm.rs:906`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L906) |
| 90 | should parse repo url (string) | ported | [`crates/renovate-core/src/datasources/npm.rs:920`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L920) |
| 111 | should return deprecated | ported | [`crates/renovate-core/src/datasources/npm.rs:990`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L990) |
| 144 | should return attestation | ported | [`crates/renovate-core/src/datasources/npm.rs:1023`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1023) |
| 196 | should handle foobar | ported | [`crates/renovate-core/src/datasources/npm.rs:1060`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1060) |
| 203 | should handle no time | ported | [`crates/renovate-core/src/datasources/npm.rs:806`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L806) |
| 210 | should return null if lookup fails 401 | ported | [`crates/renovate-core/src/datasources/npm.rs:824`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L824) |
| 216 | should return null if lookup fails | ported | [`crates/renovate-core/src/datasources/npm.rs:1146`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1146) |
| 222 | should throw error for unparseable | ported | [`crates/renovate-core/src/datasources/npm.rs:721`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L721) |
| 229 | should throw error for 429 | ported | [`crates/renovate-core/src/datasources/npm.rs:736`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L736) |
| 236 | should throw error for 5xx | ported | [`crates/renovate-core/src/datasources/npm.rs:737`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L737) |
| 243 | should throw error for 408 | ported | [`crates/renovate-core/src/datasources/npm.rs:738`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L738) |
| 250 | should throw error for others | ported | [`crates/renovate-core/src/datasources/npm.rs:739`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L739) |
| 257 | should not send an authorization header if public package | ported | [`crates/renovate-core/src/datasources/npm.rs:1864`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1864) |
| 268 | should send an authorization header if provided | ported | [`crates/renovate-core/src/datasources/npm.rs:1829`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1829) |
| 283 | should use host rules by hostname if provided | ported | [`crates/renovate-core/src/datasources/npm.rs:1932`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1932) |
| 304 | should use host rules by baseurl if provided | ported | [`crates/renovate-core/src/datasources/npm.rs:1936`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1936) |
| 330 | resets npmrc | ported | [`crates/renovate-core/src/datasources/npm.rs:1488`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1488) |
| 337 | should use default registry if missing from npmrc | ported | [`crates/renovate-core/src/datasources/npm.rs:1497`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1497) |
| 348 | should fetch package info from custom registry | ported | [`crates/renovate-core/src/datasources/npm.rs:661`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L661) |
| 363 | should replace any environment variable in npmrc | ported | [`crates/renovate-core/src/datasources/npm.rs:1553`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1553) |
| 380 | should throw error if necessary env var is not present | ported | [`crates/renovate-core/src/datasources/npm.rs:1544`](../../../../../../../crates/renovate-core/src/datasources/npm.rs#L1544) |

