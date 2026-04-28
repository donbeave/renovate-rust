# renovate-rust

A Rust reimplementation of [`renovatebot/renovate`](https://github.com/renovatebot/renovate)'s
CLI, focused on:

- **Drop-in compatibility** for common CLI workflows: same flag names,
  environment variables, config file discovery, exit codes, and
  machine-readable output wherever behavior is observable.
- **Performance** as a first-class product goal (startup time, repo
  scanning, parsing, datasource lookups, allocation behavior, bounded
  concurrency).
- **Calmer human output**: colored, grouped, and explicit about why an
  update was skipped — but always overridable for CI / `NO_COLOR`.

This is early scaffolding. See `docs/parity/implementation-ledger.md` for
what's done and what's next.

## Status

Slice 0001: workspace + early CLI flags (`-v` / `--version` / `--help`).

## Repository layout

```
.
├── Cargo.toml                  # workspace root + shared lints
├── rust-toolchain.toml         # pinned to stable 1.95.0
├── rustfmt.toml
├── .config/nextest.toml        # nextest profiles (default + ci)
├── crates/
│   ├── renovate-cli/           # `renovate` binary
│   └── renovate-core/          # shared domain types (placeholder)
├── docs/parity/                # parity ledger, test map, decisions
├── prompts/                    # operator-owned automation prompts
└── ...
```

The behavioral reference is the upstream Renovate checkout, expected at
`../renovate` (sibling to this repo). Treat it as read-only.

## Local development

Required toolchain: stable Rust 1.95.0 (managed via `rust-toolchain.toml`)
and [`cargo-nextest`](https://nexte.st/).

Quality gates (run before committing):

```bash
cargo build  --workspace --all-features
cargo fmt    --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

Run the CLI locally:

```bash
cargo run -p renovate-cli -- --version
cargo run -p renovate-cli -- --help
```

## Contributing

See `AGENTS.md`, `BRANCHING.md`, and `COMMITS.md`. Agent-only rules live
in `AGENTS.md`; everything else applies equally to humans and agents.
