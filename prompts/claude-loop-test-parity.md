You are working on the renovate-rust repository.

## Mission

The Rust implementation must have **at minimum the same test coverage as the
original Renovate TypeScript repository**. Every `it()` and `it.each()` test
case in every `.spec.ts` file is a porting requirement: it must either be
ported to an equivalent Rust test or explicitly marked `not-applicable` with a
documented reason.

This is the minimum bar. The Rust test suite may and should go beyond it —
additional edge cases, Rust-specific invariants, performance assertions — but
extras do not substitute for the TypeScript parity requirement.

Your goal in this loop is to drive the Rust test suite toward that minimum bar
by maintaining `docs/parity/renovate-test-map.md` as a precise, per-test audit
trail, and by writing or annotating Rust tests to close the gaps.

Run autonomously. Do not ask questions. Make the best engineering decision you
can from local evidence. Never stop because of a missing credential or external
service. Document the blocker, skip that entry, and continue.

Never modify `prompts/claude-loop-test-parity.md` while executing this loop.

---

## Workspace layout

- Normal Claude Code working directory: `~/Projects/renovate-rust-experiement`
- `renovate/` — upstream Renovate reference clone (read-only)
- `renovate-rust/` — the Rust implementation repository
- If you are already inside `renovate-rust/`, use `.` as the project root and
  `../renovate` as the reference.
- All writes, tests, and commits target `renovate-rust/`. Never edit `renovate/`.

---

## The target file

`docs/parity/renovate-test-map.md`

This file is being rebuilt from the old file-level table format into the
per-test format defined below. Read it at the start of every iteration to know
which entries already exist and what phase they are in.

The file must begin with a summary block that is updated every commit:

```markdown
# Renovate Test Map

**Overall progress:** 340 / 1,247 test cases ported (27%) — updated 2026-04-29

Status key: `ported` · `pending` · `not-applicable`
```

Update the counts by running:
```sh
grep -c "| ported |" docs/parity/renovate-test-map.md
grep -c "| pending \|" docs/parity/renovate-test-map.md
grep -c "| not-applicable |" docs/parity/renovate-test-map.md
```

---

## Per-test format

The file body is a sequence of sections, one per `.spec.ts` file. Every section
follows this template exactly. The table **always has six columns** — Reason is
`—` for `ported` and `pending` rows.

```markdown
## `lib/modules/manager/ansible-galaxy/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible-galaxy/extract.spec.ts
**Total tests:** 14 | **Ported:** 9 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 15 | ported | `ansible_galaxy.rs` | `test_returns_null_for_empty` | — |
| extracts multiple dependencies from requirements.yml | 23 | pending | — | — | — |
| validates TypeScript types | 67 | not-applicable | — | — | TypeScript type-system test; no runtime behavior |
```

### Format rules

- `##` heading = spec file path as inline code, relative to Renovate repo root.
- **Reference** = GitHub blob URL for the spec file (no line anchor on the file
  itself; line anchors appear in the `// Ported:` comment on the Rust side).
- **Total tests** = count of `it(` and `it.each(` call sites in the spec file.
- **Ported** = count of rows with Status `ported`.
- **Status** summary (file level): `ported` · `partial` · `pending` · `not-applicable`.
- `###` heading = full `describe()` path. Nested describes join with ` › `.
  Example: top-level `extractPackageFile()` → `### \`extractPackageFile()\``
  Nested: outer `extractPackageFile()` + inner `git deps` → `### \`extractPackageFile() › git deps\``
- Table columns always present in this order:
  1. **Original test name** — exact string from the `it(` call, or the
     `it.each` description template (e.g. `returns $type for $input`).
  2. **Line** — 1-based line of the `it(` or `it.each(` call.
  3. **Status** — `ported` · `pending` · `not-applicable`.
  4. **Rust file** — filename only, no path. `—` when not ported.
  5. **Rust test name** — the `#[test] fn` name. `—` when not ported.
  6. **Reason** — mandatory for `not-applicable`; `—` for everything else.

### Handling `it.each`

An `it.each([...])` call is one row in the table — do not expand individual
cases into separate rows. Use the description template as the test name. In
Rust, implement it as `#[rstest]` / `#[case]` or as a loop over a fixed
input array. Mark the row `ported` once *all* cases from the original `it.each`
are covered by the Rust equivalent.

---

## Phases

### Phase 0 — Backfill existing Rust tests

Many Rust tests were written before the `// Ported:` comment convention existed.
Audit and annotate them each iteration as a background task, not a hard blocker.

**Audit commands:**

```sh
# Total #[test] occurrences in the codebase
grep -rn "#\[test\]" crates/ --include="*.rs" | wc -l

# Tests that already carry a // Ported: comment
grep -rn "// Ported:" crates/ --include="*.rs" | wc -l

# Individual #[test] entries whose preceding line is NOT // Ported:
grep -rn "#\[test\]" crates/ --include="*.rs" -B1 \
  | grep -v "// Ported:" | grep -v "^--$" | grep "#\[test\]"
```

For each unattributed test found:
1. Read the test body.
2. Identify the corresponding `it()` call in the TypeScript spec (look for
   matching fixture data, assertion values, or function names).
3. If a confident match is found: add the `// Ported:` comment to the Rust test
   and update or create the corresponding row in `renovate-test-map.md`.
4. **If no confident match is found after inspecting the most likely spec file,
   classify the test as Rust-specific and move on.** Do not spend more than a
   few minutes on any single unmatchable test. Leave it uncommented; it does not
   belong in the test map.

Process a batch of Rust files per iteration, not all at once. Phase 0 runs
alongside other phases — it does not block Phases 1–3.

---

### Phase 1 — Inventory a spec file

When a `.spec.ts` file has no section in the new per-test format yet:

1. Read the spec file from the Renovate reference checkout.
2. Count `it(` and `it.each(` call sites with:
   ```sh
   grep -cE "^\s+it(\.|\.each)?\(" path/to/extract.spec.ts
   ```
3. Extract describe and it call names with line numbers:
   ```sh
   grep -nE "^\s+(describe|it|it\.each|it\.skip|xit)\(" path/to/extract.spec.ts
   ```
4. Build the section: create one row per `it` / `it.each` call, all with
   Status `pending`. Track nesting depth manually using the describe block
   structure. Note `it.skip` and `xit` rows as `not-applicable` with Reason
   `intentionally skipped in source`.

---

### Phase 2 — Map existing Rust coverage

When a section exists and rows are still `pending`:

1. First, grep for tests that already carry a provenance comment:
   ```sh
   grep -rn "// Ported:" crates/ --include="*.rs"
   ```
   These are definitively ported — link them immediately.

2. For remaining `pending` rows, search by function name keywords:
   ```sh
   grep -rn "fn test_" crates/ --include="*.rs" | grep -i "keyword"
   ```

3. Also search assertion values, fixture filenames, or error strings that the
   original test uses:
   ```sh
   grep -rn "fixture_string_or_value" crates/ --include="*.rs"
   ```

4. Use judgment — a Rust test does not need the same name as the TypeScript
   original. Check that the *behavior under test* is the same.

5. When you confirm a match that lacks the `// Ported:` comment, add the
   comment to the Rust test as part of this iteration.

---

### Phase 3 — Port missing tests

When a section has `pending` rows and Phase 2 mapping is complete for that file:

1. Read `prompts/claude-loop-renovate-rust.md` before writing any Rust code.
   It is the canonical reference for Rust standards, crate conventions, scope
   boundaries, and quality gates for this project.

2. Read the original TypeScript test and its fixtures.

3. Find fixtures in `../renovate/lib/modules/manager/<manager>/__fixtures__/`.
   Copy needed fixtures into the corresponding Rust fixtures directory.

4. **Implement the actual behavior being tested — not a stub that hardcodes
   the expected return value.** The implementation must correctly handle the
   inputs described by the test, not just satisfy the single assertion.

5. Place the test in the existing Rust extractor file for the manager, or
   create a new test module if none exists.

6. Every ported test must carry a provenance comment on the line immediately
   above `#[test]`:
   ```rust
   // Ported: "<original it() description>" — <manager>/<spec-file>.spec.ts line <N>
   #[test]
   fn test_returns_null_for_invalid_yaml() {
   ```
   - Quoted string = exact text from the `it(` call.
   - Path = relative to `lib/modules/manager/`.
   - Line = 1-based line of the `it(` call.
   - No exceptions.

7. After the test passes, update the row: Status → `ported`, fill in Rust file
   and Rust test name.

---

## When to mark `not-applicable`

Use `not-applicable` (never `pending`) for tests that should never be ported.
Always fill in the Reason column. Examples:

| Situation | Reason text |
|---|---|
| Verifies TypeScript type shapes, generics, or type guards | `TypeScript type-system test; no runtime behavior` |
| Tests Jest/Vitest mock infrastructure or fixture loading helpers | `mocking framework internals` |
| Tests TypeScript module import/export resolution | `TypeScript module system` |
| Feature is out of scope for the Rust CLI | `out of scope: hosted only` / `out of scope: GitHub App` |
| `it.skip` / `xit` in the original source | `intentionally skipped in source` |

**When in doubt, port the test.** Rust can ignore the TypeScript mechanics and
test the underlying runtime behavior directly.

There is no `skipped` status. If a test is in scope but the implementation does
not exist yet, it stays `pending` until it is ported or `not-applicable` is
justified.

---

## Iteration order

Each iteration makes multiple commits. Spread work across phases rather than
spending the whole budget on one phase:

1. **Phase 0 batch** — process a batch of unattributed Rust tests (aim for
   one Rust file's worth). Update the test map for any matches found.

2. **Phase 3 work** — pick one spec file section that has finished Phase 2 and
   still has `pending` rows. Write the missing Rust tests, run quality gates,
   commit.

3. **Phase 2 work** — pick one spec file section in Phase 1 format and run the
   mapping pass. Update status of any matched rows, commit.

4. **Phase 1 work** — convert one spec file from the old table format (or add a
   new section from scratch). Commit.

Within each step, prefer spec files where the Rust extractor already exists —
they are most likely to have matchable or portable tests.

If a spec file has no corresponding Rust file yet, still create the Phase 1
inventory section. Use `—` for Rust file and Rust test name at the file-level
metadata. The section documents the gap explicitly.

---

## Quality gates before every commit

```sh
cargo build --workspace --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

Fix all failures before committing. If a failure predates your changes, note it
in the commit message and skip only that test, not the gate.

---

## Commit rules

- Follow `AGENTS.md`, `CLAUDE.md`, and `COMMITS.md`.
- Every commit must include:
  ```
  Co-authored-by: Claude <noreply@anthropic.com>
  ```
- Multiple commits per session are expected and correct — commit after each
  coherent unit of work (one Phase 0 batch, one spec file mapped, one spec file
  ported).
- Always commit `renovate-test-map.md` together with any Rust files changed in
  the same unit of work. Update the summary block counts on every commit.
- Example commit messages:
  - `docs(parity): inventory ansible-galaxy extract spec (14 tests)`
  - `test(ansible-galaxy): port 5 extract tests from renovate spec`
  - `docs(parity): map existing Rust coverage for cargo extract spec`
  - `test(parity): backfill Ported comments in pre-commit extractor`

---

## Start now

1. Run the Phase 0 audit to get a baseline count of unattributed tests.
2. Read `docs/parity/renovate-test-map.md` to understand current state.
3. Update the summary block if the counts are stale.
4. Work through phases in the Iteration order above until the loop budget runs out.

At every commit:
- Quality gates must pass.
- Every Rust test you touched or wrote has a `// Ported:` comment if it maps
  to a TypeScript spec test.
- `renovate-test-map.md` summary block reflects accurate current counts.
