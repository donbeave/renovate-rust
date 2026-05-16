# Codex Goal: Renovate Rust

## Objective

Build `renovate-rust` into a production-quality Rust implementation of the
Renovate CLI: a `renovate` binary that can be used as a drop-in replacement for
common `renovatebot/renovate` CLI workflows.

Follow [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md) as the
implementation playbook. It defines the repository layout, reference checkout,
compatibility rules, Rust design standards, parity tracking files, iteration
workflow, commit rules, and verification policy.

## Definition Of Done

The goal is complete only when the Rust CLI preserves Renovate-compatible
observable behavior for common CLI usage, including:

- CLI command names, flags, aliases, help behavior, and exit codes.
- Environment variable names and parsing.
- Config file discovery, precedence, parsing, migration, and validation.
- Repository scanning and package manager detection.
- Dependency extraction for supported managers.
- Datasource lookup and versioning/range update decisions.
- Manifest and lockfile update planning where in scope.
- Dry-run behavior, logging levels, JSON/machine-readable output, and human
  output.
- Parity documentation in `docs/parity/renovate-source-map.md`,
  `docs/parity/renovate-test-map.md`, and per-spec detail files.
- Intentional divergences recorded in
  `docs/parity/compatibility-decisions.md`.

Do not consider the goal complete just because one coherent slice is committed.
One slice is progress, not completion.

## Operating Rules

- Work autonomously from local evidence. Do not ask what to implement next.
- Treat `../renovate` as the read-only behavioral reference.
- Preserve Renovate compatibility first and Rust design quality second.
- Choose the highest-value next compatibility gap each iteration.
- Keep each implementation slice coherent, reviewable, and committed.
- After every commit, push all committed local changes to the matching remote
  branch.
- Do not run verification commands unless the operator explicitly asks for them
  or names a specific command.
- Never claim checks passed unless they were run in the current turn.
- If blocked by credentials, network, missing services, or scope, document the
  blocker and continue with another local/offline slice.
- Do not stop when a slice is complete. Continue toward the full drop-in
  replacement objective until the goal is achieved or the operator stops the
  goal.

## Progress Loop

Repeat until the Definition Of Done is satisfied:

1. Inspect current repo state, recent commits, and parity docs.
2. Inspect the Renovate reference source/tests/docs for the next missing
   compatibility slice.
3. Update parity tracking before or during implementation.
4. Implement the smallest coherent piece that materially advances the drop-in
   replacement objective.
5. Add or update tests when appropriate, with `// Ported:` comments for tests
   that map to Renovate `.spec.ts` cases.
6. Commit the coherent slice with the required Codex co-author trailer.
7. Push all committed local changes to the matching remote branch.
8. Continue with the next highest-value slice.

