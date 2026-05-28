# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/deb/checksum.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/checksum.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `modules/datasource/deb/checksum`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses the checksum for the specified package | 27 | ported | `datasources/deb.rs` | `parse_checksums_finds_sha256` | — |
| computes the checksum of a file | 47 | ported | `datasources/deb.rs` | `compute_file_checksum_returns_sha256` | — |
| should fail if there is an error in the stream | 56 | ported | `datasources/deb.rs` | `compute_file_checksum_fails_for_missing_file` | Node.js stream error → Rust IO error |

---
