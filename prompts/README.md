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
/goal Follow @renovate-rust/prompts/claude-loop-renovate-rust.md as the active implementation parity goal. Compare the original Renovate checkout in ./renovate with the Rust implementation in ./renovate-rust, keep the prompt's Definition Of Done as the completion condition, and continue implementing the next highest-value compatibility gaps until the Rust `renovate` binary is a production-quality drop-in replacement for common self-hosted Renovate CLI workflows. Keep parity docs current, commit each coherent slice, push committed changes, and do not treat one slice, partial parity progress, a clean worktree, or a turn limit as completion. Do not run verification commands unless the operator explicitly asks.
```

From inside `renovate-rust/`:

```text
/goal Follow @prompts/claude-loop-renovate-rust.md as the active implementation parity goal. Compare the original Renovate checkout in ../renovate with this Rust implementation, keep the prompt's Definition Of Done as the completion condition, and continue implementing the next highest-value compatibility gaps until the Rust `renovate` binary is a production-quality drop-in replacement for common self-hosted Renovate CLI workflows. Keep parity docs current, commit each coherent slice, push committed changes, and do not treat one slice, partial parity progress, a clean worktree, or a turn limit as completion. Do not run verification commands unless the operator explicitly asks.
```

### Claude Timed Loop

Use a timed loop only when you want periodic one-slice progress instead of a
full goal:

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-renovate-rust.md for one implementation parity iteration: compare the next missing Renovate behavior, update parity docs, implement one coherent Rust slice, commit it, push it, and report what changed. Do not run verification commands unless the operator explicitly asks.
```

### Codex Goal Prompt

Use this as the initial Codex prompt, or paste it after your local Codex goal
command:

```text
Follow prompts/claude-loop-renovate-rust.md as the active implementation parity goal. Compare the original Renovate checkout at ../renovate with this Rust implementation, prepare a plan from the prompt's Objective, Definition Of Done, operating rules, and current repository state, then execute its progress loop until the Definition Of Done is actually satisfied. The required outcome is a production-quality Rust `renovate` binary that works as a Renovate-compatible drop-in replacement for common self-hosted CLI workflows, including compatible CLI flags, environment variables, config discovery and semantics, exit codes, dependency extraction, datasource/versioning decisions, update planning, output modes, and parity tracking. Keep choosing the next highest-value compatibility gap, updating parity docs, committing each coherent slice, pushing committed changes, and continuing until completion. Do not stop after one slice, partial parity progress, a clean worktree, or a turn limit. Do not run verification commands unless the operator explicitly asks.
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
/goal Follow @renovate-rust/prompts/claude-loop-test-parity.md until one coherent test parity unit is committed, touched parity detail/root rows are consistent, and `git status --short` is clean. Compare upstream Renovate `.spec.ts` tests with Rust tests, port actionable runtime behavior, and mark TypeScript-only or out-of-scope cases as `not-applicable` with reasons. Do not run verification commands unless the operator explicitly asks; stop after 10 turns if blocked.
```

From inside `renovate-rust/`:

```text
/goal Follow @prompts/claude-loop-test-parity.md until one coherent test parity unit is committed, touched parity detail/root rows are consistent, and `git status --short` is clean. Compare upstream Renovate `.spec.ts` tests in ../renovate with Rust tests, port actionable runtime behavior, and mark TypeScript-only or out-of-scope cases as `not-applicable` with reasons. Do not run verification commands unless the operator explicitly asks; stop after 10 turns if blocked.
```

### Claude Timed Loop

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-test-parity.md for one small test parity unit. Commit completed parity updates and Rust tests, then report what changed. Do not run verification commands unless the operator explicitly asks.
```

### Codex Goal Prompt

Use this as the initial Codex prompt, or paste it after your local Codex goal
command:

```text
Follow prompts/claude-loop-test-parity.md as the active test parity goal. Compare upstream Renovate `.spec.ts` files in ../renovate with the Rust test suite, maintain docs/parity/renovate-test-map.md plus per-spec detail files, and close one coherent parity unit at a time. Every actionable Renovate runtime behavior must either be covered by an equivalent Rust test with a `// Ported:` provenance comment or be explicitly marked `not-applicable` with a concrete reason when it only tests TypeScript, Node/Vitest/Jest mechanics, or out-of-scope hosted infrastructure. Continue until one coherent parity unit is committed, touched parity detail/root rows are consistent, and `git status --short` is clean. Do not run verification commands unless the operator explicitly asks; stop after 10 turns if blocked.
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
