# AGENTS.md

This repository uses `main` as its primary branch. This file is the canonical
home for rules and restrictions that apply only to AI agents. Rules that apply
equally to human contributors and agents live in topic-specific files linked
under **Shared conventions** below.

Agents may commit requested work directly to `main`. After every agent-created
commit, push all committed local changes to the matching remote branch. Do not
create feature branches or pull requests unless the operator explicitly asks for
them.

## Pull Request Merging (agent-only)

**Agents must never merge a pull request without explicit per-PR confirmation
from the human operator.**

- Open the PR, share the URL, and stop.
- Prior "just do it", "do everything autonomously", or similar authorization
  applies only to the active workstream. It does not carry forward to later PRs.
- Do not use branch-protection bypass or admin merge unless the operator
  explicitly authorizes it for the specific PR.
- If merge authorization is ambiguous, ask before merging.

### Verify PR metadata before merging

When the operator confirms a PR can be merged, verify that the PR title and
description still match the actual diff before invoking the merge.

- Read the current PR metadata.
- Read the actual diff being merged.
- Update stale title, summary, or test-plan text before merging.
- Surface meaningful metadata corrections briefly in the final reply.

If overriding a squash or merge commit title, preserve the GitHub PR reference
in the title, for example: `feat(cli): add config discovery (#42)`.

## Commit Attribution (agent-only)

Every commit created by an AI agent in this repository must include exactly one
`Co-authored-by` trailer identifying the agent tool that made the commit.

Trailers by agent:

- Claude Code:

  ```text
  Co-authored-by: Claude <noreply@anthropic.com>
  ```

- OpenAI Codex:

  ```text
  Co-authored-by: Codex <codex@openai.com>
  ```

- Sourcegraph Amp:

  ```text
  Co-authored-by: Amp <amp@ampcode.com>
  ```

- Opencode:

  ```text
  Co-authored-by: opencode-agent[bot] <opencode-agent[bot]@users.noreply.github.com>
  ```

Do not stack multiple agent trailers on one commit. If you are uncertain which
agent is creating the commit, ask.

## Renovate Reference Repository (agent-only)

Treat the upstream Renovate repository as the behavioral reference for this
project.

- Prefer the local checkout at `../renovate`.
- Do not clone Renovate; the sibling reference checkout is expected to already
  exist.
- Treat the reference checkout as read-only. Do not edit files, install
  dependencies, generate artifacts, run mutating commands, or commit there.
- Do not commit downloaded registries, credentials, tokens, or generated cache
  directories.
- Use Renovate docs, source, and tests to drive Rust parity work.

## Autonomous Implementation (agent-only)

For long-running implementation work, especially work driven by the native
Claude Code `/loop` prompt, make local engineering decisions without waiting for
the operator.

- Prefer Renovate compatibility first and Rust idioms second when they conflict.
- Keep each iteration small, buildable, tested, and documented.
- If blocked by network, credentials, or external service access, document the
  blocker and continue with another local slice.
- Do not edit `prompts/implementation.md` or `prompts/test-parity.md` while
  executing them. They are operator-owned configuration. Record improvement
  suggestions in `docs/parity/prompt-improvements.md` instead.
- Never rewrite unrelated user changes.

## Refactoring Philosophy (agent-only)

**Large-scale refactoring is always acceptable.** Do not take the easiest path
out of fear of fundamental change. The goal is a better technical solution, not
the smallest possible diff.

- Refactor internal implementation freely: module structure, data types,
  pipeline architecture, abstraction layers, naming, error handling, async
  design — anything internal may change if it improves correctness, clarity,
  or performance.
- Preserve external compatibility: CLI commands, flags, environment variables,
  config file format, config semantics, exit codes, and machine-readable output
  must remain consistent with the original Renovate CLI where it makes sense to
  do so. Renovate compatibility is the external contract; internal design is
  fully under our control.
- Do not accumulate technical debt by avoiding refactors. If a prior design
  decision turns out to be wrong, fix it completely rather than working around
  it with hacks or compatibility shims.
- When a refactor touches many files, commit it as a single atomic change with
  a clear description of the motivation.

## Parity Tracking (agent-only)

Parity is tracked by two generated surfaces, both owned by the Rust `parity-cli`
tool and **never hand-edited**:

- `docs/parity/source-mapping/` — a split tree (root `README.md` → one page per
  group: managers, datasources, …) mapping every upstream `lib/**/*.ts`
  implementation file → Rust file(s) + status, from `@parity` tags in the Rust
  source. Regenerate with `cargo run -p parity-cli -- source`.
- `docs/parity/test-mapping/` — a split, linked tree (root `README.md` → one
  page per module → one page per spec, with each `it()`'s status and Rust
  destination) mapping every upstream `it()`/`test()` → ported / pending /
  deleted, from `// Ported:` comments. Regenerate with
  `cargo run -p parity-cli -- test`.

The ordered milestones in `docs/parity/milestones.md` decide what to work on
next. The implementation agent owns `@parity` tags; the test parity agent owns
`// Ported:` comments. See `prompts/`.

- The Rust implementation must eventually reach behavioral parity with the
  in-scope upstream surface: managers, datasources, versioning, platforms,
  artifact/lockfile updates, git operations, branch/PR generation, release
  notes, dependency dashboard, onboarding, and config discovery. Out of scope
  is hosted-only infrastructure (Mend SaaS, GitHub App, marketplace plugin,
  hosted dashboards, webhook ingestors, billing).
- The per-module Coverage target is **≥ 80% `ported / in-scope`** (the module
  table in `test-mapping/README.md`, where in-scope = upstream tests minus
  opt-outs).
- **Opt-out registry — `docs/parity/opt-out.toml`.** This is the single place
  recording items that will **never** be ported because they exercise behavior
  with no Rust analogue — TypeScript/Node.js runtime specifics, type-shape
  assertions, framework plumbing. List the upstream source file or test there,
  with a `reason`. `parity-cli` then reports it as `opt-out` (not `pending`),
  excludes it from the coverage denominator, and agents stop picking it. This
  replaces the old "no not-applicable mechanism" rule: do not leave a genuinely
  un-portable item `pending` — opt it out with a reason instead. Only exclude
  for a real no-Rust-analogue reason, never to dodge work.
- Additional Rust tests beyond the upstream baseline are welcome but they do
  not raise Coverage (they have no `// Ported:` comment).

## Ported Test Attribution (agent-only)

Every Rust test that was ported from a Renovate TypeScript spec file **must**
carry a provenance comment on the line immediately above the test attribute.

### The one canonical example — copy this form exactly

Upstream TypeScript test
(`renovate/lib/modules/manager/ansible/extract.spec.ts`):

```typescript
// line 16:
    it('extracts multiple image lines from docker_service', () => {
      const res = extractPackageFile(Fixtures.get('main2.yaml'), '', {});
      expect(res?.deps).toMatchSnapshot();
      expect(res?.deps).toHaveLength(4);
    });
```

Matching Rust test
(`renovate-rust/crates/renovate-core/src/extractors/ansible.rs`):

```rust
// Ported: "extracts multiple image lines from docker_service" — lib/modules/manager/ansible/extract.spec.ts line 16
#[test]
fn extracts_docker_service_images() {
    let content = r#"---
- name: run containers
  docker_service:
    definition:
      services:
        gitlab:
          image: sameersbn/gitlab:11.5.1
        db:
          image: sameersbn/postgresql:10
        redis:
          image: sameersbn/redis:4.0.9-1
        nginx:
          image: nginx:1.15.7
"#;
    let deps = extract(content);
    assert_eq!(deps.len(), 4);
}
```

Pattern: **`// Ported: "<exact it() text>" — <spec path> line <N>`** on the
**single line immediately above** `#[test]` / `#[tokio::test]` / `#[rstest]`.
Use the em dash `—` (U+2014), not a hyphen. The quoted text is verbatim from
`it(...)` — same case, same punctuation (inner quotes included). The spec path
is the **canonical full repo-relative path**, e.g.
`lib/modules/manager/cargo/extract.spec.ts` — not the manager-relative
shorthand, not a bare filename. The line number is the 1-based line of the `it(`
call in the upstream spec. `parity-cli` matches on `(spec file, description)`,
so a wrong path or typo'd description shows up as `deleted`, not `ported`.

### Variants of the same pattern

`#[rstest]` parameterized test — one comment above the attribute, regardless of
how many `#[case]` rows follow:

```rust
// Ported: "parses $type dependency" — lib/modules/manager/cargo/extract.spec.ts line 142
#[rstest]
#[case("[dependencies]", DepType::Normal)]
#[case("[dev-dependencies]", DepType::Dev)]
#[case("[build-dependencies]", DepType::Build)]
fn parses_dep_type(#[case] header: &str, #[case] expected: DepType) {
    // ...
}
```

Async test:

```rust
// Ported: "returns latest release for valid package" — lib/modules/datasource/crate/index.spec.ts line 87
#[tokio::test]
async fn returns_latest_release() {
    // ...
}
```

### Rules — no exceptions

1. **One `// Ported:` per upstream `it()`.** If three Rust tests cover the
   same upstream test, only the first carries `// Ported:`; the other two are
   useful Rust-specific additions and stay un-attributed.
2. **No `// Ported:` on a test that doesn't actually exercise the upstream
   behavior.** Hard-coding the expected return value to make the test pass is
   a defect, not a port.
3. **No attributes between the comment and the test attribute.** If
   `#[cfg(feature = "x")]` is needed, put `// Ported:` above the `#[cfg(...)]`.
4. **Em dash, not hyphen,** before the spec reference.
5. **Verify your work** before committing:
   ```sh
   cargo run -p parity-cli -- test    # refresh test-mapping/; your test moves pending → ported
   cargo run -p parity-cli -- check   # flags any deleted: wrong path or stale/typo'd description
   ```
   A `// Ported:` whose `(spec file, description)` does not match a live upstream
   `it()` shows up as `deleted`, and `check` exits non-zero. Find the specific
   pending tests for a module with `cargo run -p parity-cli -- gaps <module>`.

## Shared Conventions

Rules in the files below apply to everyone working in the repo, human and
agent:

- [BRANCHING.md](BRANCHING.md) - direct-to-main workflow and branch exceptions.
- [COMMITS.md](COMMITS.md) - Conventional Commits, verification commands, and
  commit trailers.

## Generated Parity Artifacts (agent-only)

- `docs/parity/source-mapping/` and `docs/parity/test-mapping/` are generated by `crates/parity-cli` (`cargo run -p parity-cli -- source` and `cargo run -p parity-cli -- test`) and must never be hand-edited.
- `cargo run -p parity-cli` regenerates both trees.
