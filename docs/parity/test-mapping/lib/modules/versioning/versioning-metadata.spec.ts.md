# `lib/modules/versioning/versioning-metadata.spec.ts`

[← `versioning/_common`](../../../_by-module/versioning/_common.md) · [all modules](../../../README.md)

**0/0 in-scope tests ported** (0 pending, 2 opt-out) · status: opt-out

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | readme with no h1 or h2 markdown headers | opt-out | tests behavior when a versioning's README has no h1/h2 (for metadata extraction/docs); pure docstring/markdown structure test with no core versioning resolution logic. No equivalent metadata extraction for 'versioning packages' in Rust (static per-versioning modules); opt as non-business, TS-specific doc tooling. |
| 40 | contains mandatory fields | opt-out | asserts presence of mandatory fields in versioning metadata (from its package.json/README or similar); same doc/reflection nature as sibling, no Rust analogue or core impact (covered by the fact that all versionings have their id/name etc. in the static registry). |

