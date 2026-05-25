# Differential Parity Harness

Objective proof that `renovate-rust` is a drop-in replacement: run upstream
Renovate and `renovate-rust` over the same fixture repositories with equivalent
invocations and assert the observable output is identical. This is terminal-state
gate #4 in `prompts/claude-loop-renovate-rust.md`.

## How it works

1. Each fixture repo lives under `tests/parity/fixtures/<repo-name>/` and holds:
   - the input repository contents (manifests, lockfiles, config);
   - recorded HTTP cassettes for every datasource/platform call the run makes;
   - the expected normalized output.
2. The runner executes both implementations in a dry-run, machine-readable mode,
   replaying the same recorded HTTP responses to each, then normalizes and diffs.
3. Parity for a fixture = empty diff, or every remaining difference is recorded
   as an intentional divergence in `compatibility-decisions.md`.

## Rules

- Offline and deterministic. No live network. A missing recording is a pending
  recording task, never a reason to skip a fixture.
- Compare the observable contract only: detected managers, extracted
  dependencies, datasource / current / target versions, update types, branch and
  PR/commit names, skip reasons, exit codes. Normalize away timestamps,
  durations, absolute paths, and log ordering.
- Add a fixture repo for each manager and each major pipeline stage as it is
  implemented (extraction → datasource → update decision → manifest/lock edit →
  branch/PR content).

## Status

`green` = empty diff or documented divergence. `red` = unexplained diff.
`pending` = fixture/recording not built yet.

| Fixture repo | Behavior covered | Upstream run | Rust run | Diff status |
|---|---|---|---|---|
| _none yet_ | — | — | — | pending |

## Blockers

Record here when upstream Renovate cannot be executed (no Node, no install). In
that case, build Rust behavior plus recorded expected-output fixtures and run the
Rust side against the recorded expectation; restore the live two-sided diff once
Node is available.
