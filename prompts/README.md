# Prompts

This directory contains reusable prompts for long-running agent work.

## Choosing `/goal` or `/loop`

Use `/goal` for finite work with a verifiable end state. This is the best fit
for most substantial implementation or parity work because Claude Code keeps
starting another turn until the goal evaluator says the condition is met.

Use `/loop` for recurring checks on a timer, such as polling CI, watching a PR,
or doing periodic maintenance while a session stays open. A fixed interval like
`/loop 15m ...` runs until you stop it or it expires. Omitting the interval lets
Claude self-pace between iterations.

Current Claude Code requirements:

- `/loop` scheduled tasks require Claude Code v2.1.72 or later.
- `/goal` requires Claude Code v2.1.139 or later.

Check with:

```sh
claude --version
```

Good Claude Code goal for focused parity work:

```text
/goal Continue following @renovate-rust/prompts/claude-loop-test-parity.md until one coherent parity unit is committed, touched parity detail/root rows are consistent, and `git status --short` is clean. Do not run verification commands unless the operator explicitly asks; stop after 10 turns if blocked.
```

Good Claude Code goal for the full Renovate Rust implementation:

```text
/goal Use @renovate-rust/prompts/claude-loop-renovate-rust.md as the implementation playbook and keep working until renovate-rust provides a production-quality Rust `renovate` binary that is a drop-in replacement for common Renovate CLI workflows. Preserve Renovate-compatible CLI flags, environment variables, config discovery/semantics, exit codes, dependency extraction, update planning, output modes, and parity tracking. Each turn should choose the next highest-value compatibility gap, implement a coherent slice, update parity docs, commit it, push it to main, then continue. Do not treat a committed slice or a clean worktree as goal completion; the goal is complete only when the drop-in replacement definition is satisfied. Do not run verification commands unless the operator explicitly asks; if blocked, document the blocker, commit and push any coherent progress, then continue with another local/offline slice.
```

Use a timed loop only when repetition is the point:

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-test-parity.md for one small parity unit. Do not run verification commands unless the operator explicitly asks. Commit the completed unit, then report what changed.
```

For Codex goal mode, use [codex-goal-renovate-rust.md](codex-goal-renovate-rust.md)
as the objective file for implementation work. The file is written in Codex goal
format: it states the objective, definition of done, operating rules, and
repeatable progress loop. The agent should first read that file, prepare its
working plan from the objective and definition of done, then keep executing
coherent implementation slices until the full drop-in replacement goal is
actually satisfied. A committed slice is only progress, not completion. Example
objective:

```text
Follow prompts/codex-goal-renovate-rust.md. Read it as the Codex goal file, prepare the working plan from its Objective and Definition Of Done, then implement renovate-rust until it provides a production-quality Rust `renovate` binary that is a drop-in replacement for common Renovate CLI workflows. Keep choosing the next highest-value compatibility gap, updating parity docs, committing each coherent slice, and pushing every commit to main until the Definition Of Done in that file is satisfied. Do not stop after one slice or merely because `git status --short` is clean. Do not run verification commands unless the operator explicitly asks.
```

For parity-only work, use the same condition text without the Claude Code slash
command wrapper:

```text
Continue following prompts/claude-loop-test-parity.md until one coherent parity unit is committed, touched parity detail/root rows are consistent, and git status --short is clean. Do not run verification commands unless the operator explicitly asks; stop after 10 turns if blocked.
```

Notes for reliable operation:

- Start Claude Code from `~/Projects/renovate-rust-experiement` when using the
  `@renovate-rust/...` references below. If already inside `renovate-rust/`,
  use `@prompts/...` instead.
- `/goal` conditions should name the real proof of completion. For bounded
  parity work, that can be a committed unit and a clean `git status`. For the
  full Renovate Rust implementation, the proof is the drop-in replacement
  definition of done, not a single committed slice.
- Include a turn or time bound in long goals so the agent stops cleanly if the
  work is blocked.
- `/loop` tasks are session-scoped. They fire only while Claude Code is running
  and idle, are restored on `--resume` or `--continue` only while unexpired, and
  recurring tasks expire after seven days.
- The prompt files here intentionally contain prompt bodies only. Keep command
  examples and operator guidance in this README.

## Renovate Rust Prompt

Use [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md) with Claude
Code's `/goal` or `/loop` command from the parent workdir that contains both
checkouts.

The prompt file intentionally contains only the prompt body. Keep usage notes,
command examples, and operator documentation in this README.

Start Claude Code in `~/Projects/renovate-rust-experiement`, then run the
long-running implementation goal:

```text
/goal Use @renovate-rust/prompts/claude-loop-renovate-rust.md as the implementation playbook and keep working until renovate-rust provides a production-quality Rust `renovate` binary that is a drop-in replacement for common Renovate CLI workflows. Preserve Renovate-compatible CLI flags, environment variables, config discovery/semantics, exit codes, dependency extraction, update planning, output modes, and parity tracking. Each turn should choose the next highest-value compatibility gap, implement a coherent slice, update parity docs, commit it, push it to main, then continue. Do not treat a committed slice or a clean worktree as goal completion; the goal is complete only when the drop-in replacement definition is satisfied. Do not run verification commands unless the operator explicitly asks; if blocked, document the blocker, commit and push any coherent progress, then continue with another local/offline slice.
```

For periodic maintenance instead, schedule the prompt every 15 minutes:

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-renovate-rust.md
```

The `@renovate-rust/prompts/claude-loop-renovate-rust.md` reference tells
Claude Code to read the prompt file as part of the loop instruction. If you
start Claude Code from inside `~/Projects/renovate-rust-experiement/renovate-rust`, use
`@prompts/claude-loop-renovate-rust.md` instead. If file references are not
available in your Claude Code build, open the prompt file and paste its prompt
body after `/goal` or `/loop 15m`.

The prompt is operator-owned configuration. Agents running it must
not edit [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md). If an
agent finds an improvement to the prompt, it should record the suggestion in
project docs instead.

If you edit a prompt file while a fixed `/loop` is running, cancel and recreate
the loop for the most predictable behavior. A `/goal` reads the file through the
conversation context for its active run, so restart the goal after prompt edits.

---

## Test Parity Prompt

Use [claude-loop-test-parity.md](claude-loop-test-parity.md) to rebuild and
maintain the split test parity tracker.

`docs/parity/renovate-test-map.md` is now a compact root index with one row per
upstream `.spec.ts` file. The root row has only two statuses:

- `Done`
- `Not done`

Per-test-case details live in one Markdown file per original Renovate spec path,
for example:

```text
docs/parity/lib/modules/manager/ansible-galaxy/extract.spec.ts.md
```

The detail file tracks `ported`, `pending`, and `not-applicable` rows, Rust test
links, counts, and reasons. Only update the root row to `Done` once the linked
detail file has no remaining `pending` rows.

The prompt uses a three-phase workflow:

1. **Inventory** — parse each `.spec.ts` and write every `describe`/`it()` call
   into the matching detail file with `pending` status, then add or update the
   root index row.
2. **Mapping** — grep the Rust codebase to find existing coverage and link each
   ported test in the detail file.
3. **Porting** — write any test that has no Rust equivalent yet, then mark it
   `ported` in the detail file and update the root index row if the spec is now
   complete.

Start Claude Code in `~/Projects/renovate-rust-experiement`, then run a bounded
goal for one parity unit:

```text
/goal Continue following @renovate-rust/prompts/claude-loop-test-parity.md until one coherent parity unit is committed, touched parity detail/root rows are consistent, and `git status --short` is clean. Do not run verification commands unless the operator explicitly asks; stop after 10 turns if blocked.
```

For periodic maintenance instead, run:

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-test-parity.md
```

Or from inside `renovate-rust/`:

```text
/loop 15m Follow @prompts/claude-loop-test-parity.md
```

The loop prompt is operator-owned configuration. Agents must not edit
[claude-loop-test-parity.md](claude-loop-test-parity.md). Record suggested
improvements in project docs instead.

---

## Expected Local Layout

Run the loop from this parent directory:

```text
~/Projects/renovate-rust-experiement/
  renovate/
  renovate-rust/
```

The prompt expects the upstream Renovate reference repository to be available as
a sibling checkout. This checkout is required, should already exist, and must be
treated as read-only by agents:

```text
~/Projects/renovate-rust-experiement/
  renovate/
  renovate-rust/
```
