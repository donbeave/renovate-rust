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

Recommended command from the repository root:

```text
Follow prompts/codex-goal-renovate-rust.md as the active Codex goal file. Read its Objective, Definition Of Done, Operating Rules, and Progress Loop before making a plan. Use prompts/claude-loop-renovate-rust.md as the implementation playbook. Keep implementing renovate-rust until it provides a production-quality Rust `renovate` binary that is a drop-in replacement for common Renovate CLI workflows. Keep choosing the next highest-value compatibility gap, updating parity docs, committing each coherent slice, and pushing every commit to main until the Definition Of Done in the goal file is satisfied. Do not stop after one slice, after partial parity progress, or merely because `git status --short` is clean. Do not run verification commands unless the operator explicitly asks.
```

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
