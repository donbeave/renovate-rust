#!/usr/bin/env python3
"""Batch NA audit helper #3."""
import re, os

SPECS = {
    "lib/modules/manager/batect-wrapper/artifacts.spec.ts.md": "Tests TypeScript artifact update with HTTP mocking; no Rust equivalent",
    "lib/modules/manager/jsonnet-bundler/artifacts.spec.ts.md": "Tests TypeScript artifact update with HTTP mocking; no Rust equivalent",
    "lib/modules/platform/bitbucket-server/pr-cache.spec.ts.md": "Tests Bitbucket Server HTTP PR cache with nock mocking; no Rust equivalent",
    "lib/modules/platform/github/branch.spec.ts.md": "Tests GitHub HTTP branch API with nock mocking; no Rust equivalent",
    "lib/modules/platform/github/scm.spec.ts.md": "Tests GitHub SCM with vitest mocking; no Rust equivalent",
    "lib/util/git/pristine.spec.ts.md": "Tests TypeScript git pristine cache with mocking; no Rust equivalent",
    "lib/util/git/set-branch-commit.spec.ts.md": "Tests TypeScript git branch commit cache with mocking; no Rust equivalent",
    "lib/workers/repository/finalize/repository-statistics.spec.ts.md": "Tests TypeScript worker statistics logging with mocking; no Rust equivalent",
    "lib/workers/repository/update/branch/check-existing.spec.ts.md": "Tests TypeScript worker PR existence check with mocking; no Rust equivalent",
}

def process_file(path):
    with open(path) as f:
        content = f.read()
    
    reason = SPECS.get(os.path.relpath(path, "docs/parity"), "TypeScript-specific behavior with no Rust equivalent")
    
    pending_before = content.count("| pending |")
    
    lines = content.split("\n")
    new_lines = []
    for line in lines:
        if "| pending |" in line and "| — |" in line and "Status" not in line:
            if line.rstrip().endswith("| — |"):
                line = line.rstrip()[:-4] + reason + " |"
            elif "| — |" in line:
                line = line.replace("| — |", "| " + reason + " |", 1)
        new_lines.append(line)
    
    content = "\n".join(new_lines)
    content = content.replace("| pending |", "| not-applicable |")
    
    pending_after = content.count("| pending |")
    na_after = content.count("| not-applicable |")
    ported = content.count("| ported |")
    total = pending_after + na_after + ported
    
    content = re.sub(
        r"\*\*Total tests:\*\* \d+ \| \*\*Ported:\*\* \d+ \| \*\*Actionable:\*\* \d+ \| \*\*Status:\*\* \w+",
        f"**Total tests:** {total} | **Ported:** {ported} | **Actionable:** {total} | **Status:** {'done' if pending_after == 0 else 'partial'}",
        content,
    )
    
    with open(path, "w") as f:
        f.write(content)
    
    return pending_before, pending_after, na_after, ported, total

def main():
    for rel_path in SPECS:
        path = os.path.join("docs/parity", rel_path)
        if not os.path.exists(path):
            print(f"MISSING: {path}")
            continue
        before, after, na, ported, total = process_file(path)
        moved = before - after
        print(f"{rel_path}: moved {moved} pending → NA (now {after} pending, {na} NA, {ported} ported, {total} total)")

if __name__ == "__main__":
    main()
