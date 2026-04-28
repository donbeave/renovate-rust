# Prompts

This directory contains reusable prompts for long-running agent work.

## Renovate Rust /loop Prompt

Use [claude-loop-renovate-rust.md](claude-loop-renovate-rust.md) with Claude
Code's native `/loop` command from the repository root.

The prompt file intentionally contains only the prompt body. Keep usage notes,
command examples, and operator documentation in this README.

Claude Code scheduled tasks require Claude Code v2.1.72 or later. Check with:

```sh
claude --version
```

Start Claude Code in this repository, then schedule the prompt every 15 minutes:

```text
/loop 15m Follow @prompts/claude-loop-renovate-rust.md
```

The `@prompts/claude-loop-renovate-rust.md` reference tells Claude Code to read
the prompt file as part of the loop instruction. If file references are not
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

## Expected Local Layout

Run the loop from this checkout:

```text
parent/
  renovate-rust/
```

The prompt expects the upstream Renovate reference repository to be available at
`../renovate` when possible:

```text
parent/
  renovate-rust/
  renovate/
```

If the reference checkout is missing, the prompt instructs the agent to clone
`https://github.com/renovatebot/renovate` into a local reference directory.
