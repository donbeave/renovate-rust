# Prompts

This directory contains reusable prompts for long-running agent work.

## Renovate Rust /loop Prompt

Use [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md) with Claude
Code's native `/loop` command from the parent workdir that contains both
checkouts.

The prompt file intentionally contains only the prompt body. Keep usage notes,
command examples, and operator documentation in this README.

Claude Code scheduled tasks require Claude Code v2.1.72 or later. Check with:

```sh
claude --version
```

Start Claude Code in `~/Projects/renovate-rust-experiement`, then schedule the prompt every
15 minutes:

```text
/loop 15m Follow @renovate-rust/prompts/claude-loop-renovate-rust.md
```

The `@renovate-rust/prompts/claude-loop-renovate-rust.md` reference tells
Claude Code to read the prompt file as part of the loop instruction. If you
start Claude Code from inside `~/Projects/renovate-rust-experiement/renovate-rust`, use
`@prompts/claude-loop-renovate-rust.md` instead. If file references are not
available in your Claude Code build, open the prompt file and paste its prompt
body after `/loop 15m`.

The loop prompt is operator-owned configuration. Agents running the loop must
not edit [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md). If an
agent finds an improvement to the prompt, it should record the suggestion in
project docs instead.

The loop is session-scoped. Keep the Claude Code session open, or resume it with
Claude Code's resume/continue flow before the scheduled task expires. If you
edit the prompt while a loop is running, cancel and recreate the loop for the
most predictable behavior.

---

## Test Parity /loop Prompt

Use [claude-loop-test-parity.md](claude-loop-test-parity.md) to rebuild and
maintain `docs/parity/renovate-test-map.md` with **per-test-case granularity**.

This prompt replaces the old file-level table format with a three-phase
workflow:

1. **Inventory** — parse each `.spec.ts` and write every `describe`/`it()` call
   into the tracking file with `pending` status.
2. **Mapping** — grep the Rust codebase to find existing coverage and link each
   ported test.
3. **Porting** — write any test that has no Rust equivalent yet, then mark it
   `ported`.

Start Claude Code in `~/Projects/renovate-rust-experiement`, then run:

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
