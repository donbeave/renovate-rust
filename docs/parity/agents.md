# AGENTS.md

This folder is governed by generated parity mapping outputs.

## Never hand-edit parity mapping folders

`docs/parity/source-mapping/` and `docs/parity/test-mapping/` are **entirely generated** outputs from:

- `crates/parity-cli`

and are **always replaced** when regenerated. Do not edit them manually under any circumstances.

Regenerate instead with:

- `cargo run -p parity-cli -- source`
- `cargo run -p parity-cli -- test`
- `cargo run -p parity-cli` (both)

Agents should not attempt to edit files in these directories directly.
