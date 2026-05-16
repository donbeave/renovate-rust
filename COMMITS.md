# Commits

This file covers commit message format, agent attribution trailers, and
pre-commit verification.

## Commit Messages

All commits in this repository should follow
[Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).

Subject format:

```text
<type>[optional scope][!]: <description>
```

Allowed types:

| Type       | Use for                                     |
| ---------- | ------------------------------------------- |
| `feat`     | New user-visible feature                    |
| `fix`      | Bug fix                                     |
| `docs`     | Documentation-only change                   |
| `style`    | Formatting, whitespace; no logic change     |
| `refactor` | Internal restructuring; no behavior change  |
| `perf`     | Performance improvement                     |
| `test`     | Adding or updating tests                    |
| `build`    | Build system, tooling, dependencies         |
| `ci`       | CI configuration                            |
| `chore`    | Routine maintenance                         |
| `revert`   | Reverts a prior commit                      |

Scope is optional but encouraged when it clarifies the change area, for example
`feat(cli): add Renovate-compatible dry-run flag`.

Breaking changes use `!` after the type or scope, such as `feat!:` or
`feat(config)!:`, and include a `BREAKING CHANGE:` footer in the body.

PR titles should also follow Conventional Commits because squash-merge commonly
uses the PR title as the commit subject.

## Sign-off and Attribution

If the repository or remote checks require DCO sign-off, create commits with
`-s` and ensure the `Signed-off-by` trailer matches the commit author:

```sh
git commit -s -m "feat(scope): description"
```

Agent-specific `Co-authored-by` trailer requirements live in
[AGENTS.md](AGENTS.md). A commit can have both a `Signed-off-by` trailer and
one agent `Co-authored-by` trailer.

## Pre-commit Verification

Do not run Cargo verification commands automatically before or after every
commit. Run Cargo checks only when the operator explicitly asks for them, or
when a task instruction names a specific Cargo command.

For documentation-only and parity-tracking changes, inspect the diff and run:

```sh
git diff --check
```

When the operator requests Rust verification, use the strongest applicable
checks for the change:

```sh
cargo build --workspace --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

Also run doctests only when requested and public documentation examples exist or
changed:

```sh
cargo test --doc --workspace --all-features
```

If the operator requested Cargo checks and formatting fails, run `cargo fmt
--all`, then re-run the requested verification command. If `cargo nextest` is
missing during a requested check, document the blocker instead of installing
tools unless the operator asked you to install them.
