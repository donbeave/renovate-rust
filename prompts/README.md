# Prompts

This directory intentionally contains two reusable prompts:

1. [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md) — implementation
   parity: compare upstream Renovate with `renovate-rust`, decide what remains
   to implement, and keep building a compatible Rust replacement.
2. [claude-loop-test-parity.md](claude-loop-test-parity.md) — test parity:
   compare upstream Renovate `.spec.ts` coverage with Rust tests, port the
   runtime behavior that matters, and mark TypeScript-only tests as
   `not-applicable` with reasons.

The prompt filenames keep the existing `claude-loop-*` names for continuity, but
both prompts are agent-neutral. Use either prompt with Jack/Claude or Codex.

## Running until truly done

Both prompts share one machine-checkable **terminal state** (defined in
`claude-loop-renovate-rust.md` → Definition Of Done). The loop may stop only when
all of these hold at once:

1. Source map: zero `not-started`/`partial`/`stub` in-scope rows (all `full`).
2. Test map: zero `pending` rows; `not-applicable` only for genuine TS/test
   mechanics and within the NA budget (well under ~25% of total).
3. Source ↔ test cross-check: no spec is `Done` while its source is unfinished.
4. Differential harness green (upstream Renovate vs `renovate-rust`, empty diff
   or documented divergence).
5. `cargo build` / `fmt --check` / `clippy -D warnings` / `nextest run` pass.

Until then, none of "one slice committed", "clean worktree", "turn limit",
"feels done", or "100% of actionable tests" is completion. The `/goal` form is
what runs to this terminal state; `/loop` does one iteration per tick while a
session stays open. Scope is a **full drop-in replacement** — datasources,
version decisions, lockfile/artifact updates, and platform branch/PR operations
are in scope, not just dependency extraction.

> Note on the current baseline: the test map historically reported ~100% by
> marking ~74% of tests `not-applicable` under an extraction-only scope. Under
> full-drop-in scope those are mostly mis-scoped. The test-parity prompt's
> Phase 0.5 re-audit corrects this before the map can be considered closed.

## Command Notes

Claude Code:

- `/goal` is the right fit when you want the agent to keep working until a
  verifiable end state is reached.
- `/loop` is the right fit for timed recurring work while a Claude Code session
  stays open.
- Current Claude Code docs describe custom slash prompts as Markdown files and
  support passing text after the command as arguments. They also document
  `/loop` for repeated scheduled prompts.

Codex:

- Use Codex goal mode when available in the interactive UI.
- For non-interactive runs, pass the prompt as the initial instruction, for
  example with `codex exec`.
- Local reference checked during this review: `codex-cli 0.133.0`, whose CLI
  exposes `codex [PROMPT]` and `codex exec [PROMPT]`.

If your local UI names the goal command `/go` rather than `/goal`, use the same
prompt body. The important part is the prompt content and completion condition,
not the slash-command spelling.

## Expected Local Layout

Start agents from the parent workspace when possible:

```text
~/Projects/renovate-rust-experiement/
  renovate/       # upstream Renovate reference checkout, read-only
  renovate-rust/  # Rust implementation
```

If you start from the parent workspace, reference prompt files as
`@renovate-rust/prompts/...` in Claude Code. If you start from inside
`renovate-rust/`, reference them as `@prompts/...`.

## Prompt 1: Implementation Parity

Use this when the agent should compare original Renovate to Renovate in Rust,
identify what must still be implemented, plan the implementation, and then
implement production-quality Rust slices until the Rust CLI is a compatible
replacement for common self-hosted Renovate workflows.

### Claude Goal

From `~/Projects/renovate-rust-experiement`:

```text
/goal Follow @renovate-rust/prompts/claude-loop-renovate-rust.md as the active implementation parity goal. Compare the original Renovate checkout in ./renovate with the Rust implementation in ./renovate-rust, keep the prompt's Definition Of Done as the completion condition, and continue implementing the next highest-value compatibility gaps until the Rust `renovate` binary is a production-quality drop-in replacement for common self-hosted Renovate CLI workflows. Keep parity docs current, commit each coherent slice, push committed changes, and do not treat one slice, partial parity progress, a clean worktree, or a turn limit as completion. Do not run verification commands every iteration unless the operator asks, but run the quality gates and the differential harness as part of the terminal-state completion check.
```

From inside `renovate-rust/`:

```text
/goal Follow @prompts/claude-loop-renovate-rust.md as the active implementation parity goal. Compare the original Renovate checkout in ../renovate with this Rust implementation, keep the prompt's Definition Of Done as the completion condition, and continue implementing the next highest-value compatibility gaps until the Rust `renovate` binary is a production-quality drop-in replacement for common self-hosted Renovate CLI workflows. Keep parity docs current, commit each coherent slice, push committed changes, and do not treat one slice, partial parity progress, a clean worktree, or a turn limit as completion. Do not run verification commands every iteration unless the operator asks, but run the quality gates and the differential harness as part of the terminal-state completion check.
```

### Claude Timed Loop

Use a timed loop only when you want periodic one-slice progress instead of a
full goal:

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-renovate-rust.md for one implementation parity iteration: compare the next missing Renovate behavior, update parity docs, implement one coherent Rust slice, commit it, push it, and report what changed. Do not run verification commands every iteration unless the operator asks, but run the quality gates and the differential harness as part of the terminal-state completion check.
```

### Codex Goal Prompt

Use this as the initial Codex prompt, or paste it after your local Codex goal
command:

```text
Follow prompts/claude-loop-renovate-rust.md as the active implementation parity goal. Compare the original Renovate checkout at ../renovate with this Rust implementation, prepare a plan from the prompt's Objective, Definition Of Done, operating rules, and current repository state, then execute its progress loop until the Definition Of Done is actually satisfied. The required outcome is a production-quality Rust `renovate` binary that works as a Renovate-compatible drop-in replacement for common self-hosted CLI workflows, including compatible CLI flags, environment variables, config discovery and semantics, exit codes, dependency extraction, datasource/versioning decisions, update planning, output modes, and parity tracking. Keep choosing the next highest-value compatibility gap, updating parity docs, committing each coherent slice, pushing committed changes, and continuing until completion. Do not stop after one slice, partial parity progress, a clean worktree, or a turn limit. Do not run verification commands every iteration unless the operator asks, but run the quality gates and the differential harness as part of the terminal-state completion check.
```

For non-interactive Codex from inside `renovate-rust/`:

```sh
codex exec "$(cat prompts/claude-loop-renovate-rust.md)"
```

## Prompt 2: Test Parity

Use this when the agent should compare upstream Renovate tests to Rust tests and
ensure the Rust suite covers every Renovate runtime behavior that makes sense in
Rust. Tests that only verify TypeScript, Node, Vitest/Jest, or hosted-only
infrastructure should be marked `not-applicable` with a reason instead of being
ported.

### Claude Goal

From `~/Projects/renovate-rust-experiement`:

```text
/goal Follow @renovate-rust/prompts/claude-loop-test-parity.md as the active test parity goal. Compare upstream Renovate `.spec.ts` tests in ./renovate with Rust tests. First re-audit mis-scoped `not-applicable` rows (Phase 0.5) until the NA budget is met, then port actionable runtime behavior so every detail file reaches zero `pending` rows. Keep the source↔test cross-check consistent. Continue until the shared terminal state in claude-loop-renovate-rust.md holds for the test side; do not treat one unit, a clean worktree, a turn limit, or a high percentage as completion. Run the quality gates and the differential harness as part of the completion check.
```

From inside `renovate-rust/`:

```text
/goal Follow @prompts/claude-loop-test-parity.md as the active test parity goal. Compare upstream Renovate `.spec.ts` tests in ../renovate with Rust tests. First re-audit mis-scoped `not-applicable` rows (Phase 0.5) until the NA budget is met, then port actionable runtime behavior so every detail file reaches zero `pending` rows. Keep the source↔test cross-check consistent. Continue until the shared terminal state in claude-loop-renovate-rust.md holds for the test side; do not treat one unit, a clean worktree, a turn limit, or a high percentage as completion. Run the quality gates and the differential harness as part of the completion check.
```

### Claude Timed Loop

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-test-parity.md for one small test parity unit. Commit completed parity updates and Rust tests, then report what changed. Do not run verification commands every iteration unless the operator asks, but run the quality gates and the differential harness as part of the terminal-state completion check.
```

### Codex Goal Prompt

Use this as the initial Codex prompt, or paste it after your local Codex goal
command:

```text
Follow prompts/claude-loop-test-parity.md as the active test parity goal. Compare upstream Renovate `.spec.ts` files in ../renovate with the Rust test suite, maintain docs/parity/renovate-test-map.md plus per-spec detail files. First run the Phase 0.5 re-audit: reclassify mis-scoped `not-applicable` rows (platform/datasource/artifact/git/PR/exec are in scope under full-drop-in) to `pending` until the NA budget holds. Then port actionable behavior, each ported Rust test carrying a `// Ported:` provenance comment, until every detail file has zero `pending` rows. Keep `not-applicable` only for genuine TypeScript/Node/Vitest/Jest mechanics or hosted-only infrastructure, each with a concrete reason. Keep the source↔test cross-check consistent. Continue until the shared terminal state in claude-loop-renovate-rust.md holds for the test side; do not treat one unit, a clean worktree, a turn limit, or a high percentage as completion. Run the quality gates and the differential harness as part of the completion check.
```

For non-interactive Codex from inside `renovate-rust/`:

```sh
codex exec "$(cat prompts/claude-loop-test-parity.md)"
```

## Maintenance Rules

- Keep this directory to the two prompt bodies above plus this README.
- Prompt bodies are operator-owned configuration. Agents running a prompt should
  not edit that prompt unless the operator explicitly asks for prompt changes.
- Record future prompt improvement suggestions in
  `docs/parity/prompt-improvements.md`.
- If a timed `/loop` is running and you edit a prompt file, cancel and recreate
  the loop so the active instruction is unambiguous.
