# Codex Goal: Renovate Rust

## Objective

Build `renovate-rust` into a production-quality Rust implementation of the
Renovate CLI: a `renovate` binary that can be used as a drop-in replacement for
common `renovatebot/renovate` CLI workflows.

Follow [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md) as the
implementation playbook. It defines the repository layout, reference checkout,
compatibility rules, Rust design standards, parity tracking files, iteration
workflow, commit rules, and verification policy.

When this file is used with Codex goal mode, treat it as the active goal file:
prepare the working plan from the Objective and Definition Of Done, execute the
Progress Loop, and keep going until the goal is actually achieved.

The loop prompt is not the goal. It is the operational playbook for each
iteration. Goal preparation, goal status, and completion must come from this
file.

Recommended Codex goal from the repository root:

```text
Use prompts/codex-goal-renovate-rust.md as the active goal file. First prepare the goal exactly from this file: read its Objective, Definition Of Done, Operating Rules, and Progress Loop; inspect repository state, recent commits, and parity docs; then keep executing this file's Progress Loop until the Definition Of Done is actually satisfied. Use prompts/claude-loop-renovate-rust.md only as the implementation playbook for each iteration, not as the completion condition. The required outcome is a production-quality Rust `renovate` binary that works as a Renovate-compatible drop-in replacement for common self-hosted CLI workflows, including compatible CLI flags, environment variables, config discovery and semantics, exit codes, dependency extraction, datasource/versioning decisions, update planning, output modes, and parity tracking. Keep choosing the next highest-value compatibility gap, implementing a coherent slice, updating parity docs, committing it, pushing it to main, and immediately continuing with the next slice. Do not stop after one slice, partial parity progress, a clean worktree, or the loop file's iteration instructions. Do not run verification commands unless the operator explicitly asks.
```

Do not use `claude-loop-renovate-rust.md` by itself as the completion condition
for the full implementation goal. That loop file explains how each iteration
should work; this Codex goal file defines what completion means.

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

Do not consider the goal complete just because `git status --short` is clean.
A clean worktree after a commit is required iteration hygiene, not proof that
the Rust CLI is a drop-in replacement.

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

1. Read this goal file and prepare the current turn's working plan from the
   Objective, Definition Of Done, Operating Rules, and local repo state.
2. Inspect current repo state, recent commits, and parity docs.
3. Inspect the Renovate reference source/tests/docs for the next missing
   compatibility slice.
4. Update parity tracking before or during implementation.
5. Implement the smallest coherent piece that materially advances the drop-in
   replacement objective.
6. Add or update tests when appropriate, with `// Ported:` comments for tests
   that map to Renovate `.spec.ts` cases.
7. Commit the coherent slice with the required Codex co-author trailer.
8. Push all committed local changes to the matching remote branch.
9. Continue with the next highest-value slice until the Definition Of Done is
   fully achieved.
