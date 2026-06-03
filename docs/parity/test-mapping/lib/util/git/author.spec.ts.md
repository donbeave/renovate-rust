# `lib/util/git/author.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**8/8 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | returns null if empty email given | ported | [`crates/renovate-core/src/git/author.rs:116`](../../../../../../crates/renovate-core/src/git/author.rs#L116) |
| 12 | catches errors | ported | [`crates/renovate-core/src/git/author.rs:122`](../../../../../../crates/renovate-core/src/git/author.rs#L122) |
| 19 | handles a normal address | ported | [`crates/renovate-core/src/git/author.rs:129`](../../../../../../crates/renovate-core/src/git/author.rs#L129) |
| 23 | parses bot email | ported | [`crates/renovate-core/src/git/author.rs:135`](../../../../../../crates/renovate-core/src/git/author.rs#L135) |
| 30 | parses bot name and email | ported | [`crates/renovate-core/src/git/author.rs:147`](../../../../../../crates/renovate-core/src/git/author.rs#L147) |
| 41 | escapes names | ported | [`crates/renovate-core/src/git/author.rs:159`](../../../../../../crates/renovate-core/src/git/author.rs#L159) |
| 47 | tries again and fails | ported | [`crates/renovate-core/src/git/author.rs:166`](../../../../../../crates/renovate-core/src/git/author.rs#L166) |
| 51 | gives up | ported | [`crates/renovate-core/src/git/author.rs:172`](../../../../../../crates/renovate-core/src/git/author.rs#L172) |

