# Prompts

Two short, focused prompts. One agent per prompt, running in parallel.

| Prompt | Agent | Owns |
|---|---|---|
| [implementation.md](implementation.md) | implementation agent | `crates/**/src/*.rs`, `docs/parity/modules.md` Impl/Notes columns |
| [test-parity.md](test-parity.md)       | test parity agent    | `crates/**/tests/*.rs` and `mod tests` blocks, `// Ported:` comments |

They do **not** edit the same files. Coverage numbers are computed by
`scripts/parity_coverage.py`, not typed by hand. The legacy 11,658-row
`renovate-test-map.md` is deprecated.

## How priorities are set

`docs/parity/milestones.md` lists the ordered milestones. Both agents always
pick work from the **first incomplete milestone**. The current first one is
`M1: cargo + crates.io + semver end-to-end`.

## Tools

```sh
# Run anywhere in renovate-rust/
python3 scripts/parity_coverage.py            # summary by group, worst modules
python3 scripts/parity_coverage.py ledger     # regenerate docs/parity/modules.md
python3 scripts/parity_coverage.py gaps <mod> # list missing upstream tests for a module
python3 scripts/parity_coverage.py orphans    # list malformed // Ported: comments
python3 scripts/parity_coverage.py json       # machine-readable
```

## Invocations

Run both agents from the workspace root `~/Projects/renovate-rust-experiement`.

### Implementation agent

```text
Follow @renovate-rust/prompts/implementation.md. Work the first incomplete
milestone in @renovate-rust/docs/parity/milestones.md until its Impl
thresholds are met, then move to the next. Commit each coherent slice with
the required Co-authored-by trailer and push.
```

### Test parity agent

```text
Follow @renovate-rust/prompts/test-parity.md. Work the first incomplete
milestone in @renovate-rust/docs/parity/milestones.md until its Coverage
thresholds are met, then move to the next. Skip modules with Impl=none.
Commit each batch with the required Co-authored-by trailer and push.
```

For non-interactive Codex, replace `@renovate-rust/prompts/...` with
`$(cat renovate-rust/prompts/implementation.md)` etc.

## Prompt maintenance

The two prompts are operator-owned. Agents running them must not edit them.
File improvement suggestions in `docs/parity/prompt-improvements.md` — the
operator decides whether to apply.
