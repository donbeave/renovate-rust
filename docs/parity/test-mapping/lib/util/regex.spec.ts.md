# `lib/util/regex.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**5/5 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | uses re2 | ported | [`crates/renovate-core/src/util/regex.rs:201`](../../../../../crates/renovate-core/src/util/regex.rs#L201) |
| 10 | throws unsafe 2 | ported | [`crates/renovate-core/src/util.rs:11054`](../../../../../crates/renovate-core/src/util.rs#L11054) |
| 14 | reuses flags from regex | ported | [`crates/renovate-core/src/util/regex.rs:213`](../../../../../crates/renovate-core/src/util/regex.rs#L213) |
| 18 | caches non-stateful regex | ported | [`crates/renovate-core/src/util/regex.rs:225`](../../../../../crates/renovate-core/src/util/regex.rs#L225) |
| 23 | does not cache stateful regex | ported | [`crates/renovate-core/src/util/regex.rs:247`](../../../../../crates/renovate-core/src/util/regex.rs#L247) |
| 28 | falls back to regexp | opt-out | Node runtime fallback test depends on dynamic RE2 module import behavior that has no direct Rust equivalent; Rust path uses a single local regex engine. |

