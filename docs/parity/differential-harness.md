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
| `npm-empty` | npm extraction — empty `dependencies` | blocked | green | blocked |
| `npm-skipped` | npm extraction — `file:` and URL installs skipped | blocked | green | blocked |
| `cargo-workspace` | cargo extraction — workspace-inherited dep skipped | blocked | green | blocked |
| `gomod-empty` | gomod extraction — `go` directive only | blocked | green | blocked |
| `gomod-replace` | gomod extraction — local replace skipped | blocked | green | blocked |
| `maven-empty` | maven extraction — empty pom.xml | blocked | green | blocked |
| `dockerfile-scratch` | dockerfile extraction — `scratch` image skipped | blocked | green | blocked |

## Blockers

Upstream Renovate's `--platform=local` does not accept a `repositories` list;
it autodiscovers from the current working directory and errors when repositories
are passed explicitly. `renovate-rust` intentionally diverges here: its local
platform treats the first repository slug as a signal to scan `cwd` as that repo.

This means a live two-sided diff for the local platform is not possible with
identical CLI invocations. Options to restore it:

1. Build a mock platform server (e.g. small Gitea/Forgejo instance in CI) and
   point both implementations at it.
2. Run upstream Renovate without `--platform=local` against a real hosted repo.
3. Accept the divergence and run the Rust side against recorded expected output.

Current approach is #3: the Rust harness tests in
`crates/renovate-cli/tests/parity.rs` run `renovate-rust` against each fixture
and assert the normalized JSON output matches the recorded expectation. These
regression tests pass in CI and guard against output-format or pipeline-behavior
breaks.

## Fixture normalizer

The harness strips volatile fields before comparison:

- `releaseTimestamp` on every dep — registry metadata changes over time.
- Log output (timestamps, ordering) is ignored; only the JSON `--output-format`
  is compared.
