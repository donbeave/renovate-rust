# CLAUDE.md

Claude Code should read this file at session start, then follow the shared
repository conventions linked from [AGENTS.md](AGENTS.md).

## Project Goal

Build `renovate-rust` into a high-quality Rust implementation of
`renovatebot/renovate`, with a compatible CLI, compatible behavior where users
depend on it, and clearer human-facing output where compatibility allows.

## Required Context

Before implementation work, inspect:

- [AGENTS.md](AGENTS.md)
- [BRANCHING.md](BRANCHING.md)
- [COMMITS.md](COMMITS.md)
- [prompts/README.md](prompts/README.md)
- [prompts/claude-loop-renovate-rust.md](prompts/claude-loop-renovate-rust.md)
- the local Renovate reference checkout at `../renovate`, when present

## Working Rules

- Use stable Rust and idiomatic crate layout.
- Use `clap` derive APIs for CLI parsing.
- Prefer `cargo nextest run` over `cargo test` for unit and integration tests.
- Keep `cargo fmt`, `cargo clippy`, build, and tests clean before committing
  Rust-affecting changes.
- Commit only coherent slices and stage only files related to the current task.
- Do not push or merge without explicit operator instruction.

## Native /loop Work

When the operator asks for repeated autonomous progress, use the prompt in
[prompts/claude-loop-renovate-rust.md](prompts/claude-loop-renovate-rust.md).
The usage command is documented in [prompts/README.md](prompts/README.md). Each
loop should choose one parity slice, add or update tests, implement it, run the
applicable checks, document the result, and commit the slice when it is clean.
Do not edit the loop prompt file while executing the loop.
