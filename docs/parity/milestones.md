# Renovate-Rust Milestones

This file defines the **ordered milestones** that drive day-to-day agent work.
Each milestone is a small, runnable target — not "100% of Renovate". A
milestone is complete only when its acceptance checks pass, including an
end-to-end run against its fixture repository.

Agents always pick work from the **first incomplete milestone**, never from a
later one. This is the project's prioritization tool.

---

## M0 — Honest baseline (DONE when this file lands)

Goal: the ledger reflects reality and the harness can run on one trivial
fixture.

Acceptance:

- [x] `scripts/parity_coverage.py report` produces a single, deduped,
      module-grouped coverage number.
- [x] `docs/parity/modules.md` is the only ledger the agents maintain.
- [ ] `docs/parity/renovate-test-map.md` and its per-spec detail files carry
      a deprecation banner pointing at `modules.md`.
- [ ] At least one `tests/parity/fixtures/` repo exists and the differential
      harness runs (even if it diffs against a recorded expected output).

---

## M1 — End-to-end "hello world" run

Goal: `renovate --dry-run --platform=local /path/to/fixture` extracts
dependencies for **one manager**, looks up versions from **one datasource**,
applies **one versioning scheme**, and prints a Renovate-compatible report.
No platform calls. No lockfile updates. No PR creation.

Chosen vertical: **cargo + crates.io + semver**.

Acceptance:

- [x] `manager/cargo` Impl=`full`, Coverage ≥ 80%.
- [x] `datasource/crate` Impl=`full`, Coverage ≥ 80%.
- [x] `versioning/semver` Impl=`full`, Coverage ≥ 80% (already 100%).
- [x] `tests/parity/fixtures/cargo-hello/` runs the diff harness and matches
      upstream Renovate output (or the recorded expected output) byte-for-byte
      after the normalizer.
- [x] `cargo build --workspace` and `cargo test -p renovate-core
      -p renovate-cli` pass.

---

## M2 — Second manager + second datasource

Goal: prove the architecture generalizes. Adds **npm + npm registry +
npm-style versioning** as a second end-to-end vertical.

Acceptance:

- [x] `manager/npm` Impl=`full`, Coverage ≥ 80%.
- [x] `datasource/npm` Impl=`full`, Coverage ≥ 80%.
- [x] `versioning/npm` Impl=`full`, Coverage ≥ 80%.
- [x] `tests/parity/fixtures/npm-hello/` matches via the diff harness.

---

## M3 — First platform integration

Goal: write branches and PRs for **one platform**, without lockfile updates.

Chosen vertical: **GitHub REST API, dry-run mode that writes its planned
branch/PR contents to stdout instead of pushing**.

Acceptance:

- [x] `platform/github` Impl=`full` for the read + dry-write surface used by
      a default `renovate` run. Coverage ≥ 60% (the full platform surface is
      huge; 60% covers the live-run path).
- [x] `worker/repository` Impl=`partial` with branch-name generation,
      commit-message rendering, and PR body rendering complete and tested.
- [x] `tests/parity/fixtures/github-cargo/` runs the diff harness for the
      branch + PR plan.

---

## M4 — Lockfile artifacts

Goal: invoke external package managers for lockfile updates on the
cargo + npm verticals.

Acceptance:

- [x] `manager/cargo` artifacts pipeline produces the same updated
      `Cargo.lock` as upstream Renovate on the fixture repos.
- [x] `manager/npm` artifacts pipeline produces the same updated
      `package-lock.json` as upstream Renovate.
- [x] Exec layer (`util/exec/...`) Impl=`full` for the surface used by these
      managers, Coverage ≥ 50%.

---

## M5 — Production-grade self-hosted CLI

Goal: a real user can replace `renovatebot/renovate` with the Rust binary for
common self-hosted workflows across the top managers and platforms.

**Top-10 module list** (determined from upstream complexity, issue volume, and
self-hosted relevance):

| # | Module | Category | Status |
|---|--------|----------|--------|
| 1 | `manager/npm` | manager | full, 81% |
| 2 | `manager/maven` | manager | full, 100% |
| 3 | `datasource/npm` | datasource | full, 81% |
| 4 | `platform/github` | platform | full, 80% |
| 5 | `manager/dockerfile` | manager | full, 100% |
| 6 | `manager/github-actions` | manager | full, 100% |
| 7 | `datasource/maven` | datasource | **partial, 56%** ← active blocker |
| 8 | `versioning/semver` | versioning | full, 100% |
| 9 | `platform/local` | platform | full, 100% |
| 10 | `manager/terraform` | manager | full, 100% |

Acceptance:

- [x] 9 of 10 top modules have Impl=`full` and Coverage ≥ 80%.
- [ ] `datasource/maven` has Impl=`full` and Coverage ≥ 80%.
- [x] All `tests/parity/fixtures/` repos pass the diff harness.
- [ ] `cargo build --workspace --all-features`, `cargo fmt --all --check`,
      `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
      and `cargo nextest run --workspace --all-features` all pass.

---

## What "full parity" means now

Full parity is **the union of all milestones**, not a single number. There is
no global "100% done" gate. The two agents stop when M5 is complete; long-tail
modules beyond M5 are tracked in the ledger but worked on opportunistically.

## How to pick the next thing to do

1. Open this file. Find the first incomplete milestone.
2. Run `python3 scripts/parity_coverage.py report` and find the worst-covered
   module **inside the active milestone's scope**.
3. Implementation agent: work on the worst-covered module whose `Impl` is
   below the milestone's threshold.
4. Test parity agent: run `python3 scripts/parity_coverage.py gaps <module>`
   on a module that is `Impl=partial` or `Impl=full` but below the coverage
   threshold, and port the listed upstream tests.
