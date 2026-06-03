# `lib/util/git/author.spec.ts`

[← `util/git`](../../../_by-module/util/git.md) · [all modules](../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null if empty email given | ported | `crates/renovate-core/src/git/author.rs:116` |
| 12 | catches errors | ported | `crates/renovate-core/src/git/author.rs:122` |
| 19 | handles a normal address | ported | `crates/renovate-core/src/git/author.rs:129` |
| 23 | parses bot email | ported | `crates/renovate-core/src/git/author.rs:135` |
| 30 | parses bot name and email | ported | `crates/renovate-core/src/git/author.rs:147` |
| 41 | escapes names | ported | `crates/renovate-core/src/git/author.rs:159` |
| 47 | tries again and fails | ported | `crates/renovate-core/src/git/author.rs:166` |
| 51 | gives up | ported | `crates/renovate-core/src/git/author.rs:172` |

