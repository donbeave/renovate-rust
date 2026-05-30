#!/usr/bin/env python3
"""Batch NA audit helper #2."""
import re, os

SPECS = {
    "lib/modules/manager/pep621/artifacts.spec.ts.md": "Tests TypeScript artifact update with subprocess/fs/datasource mocking; no Rust equivalent",
    "lib/modules/manager/terragrunt/artifacts.spec.ts.md": "Tests TypeScript manager delegation with module mocking; no Rust equivalent",
    "lib/modules/platform/bitbucket/pr-cache.spec.ts.md": "Tests Bitbucket HTTP PR cache with nock mocking; no Rust equivalent",
    "lib/modules/platform/forgejo/pr-cache.spec.ts.md": "Tests Forgejo HTTP PR cache with nock mocking; no Rust equivalent",
    "lib/modules/platform/gitea/pr-cache.spec.ts.md": "Tests Gitea HTTP PR cache with nock mocking; no Rust equivalent",
    "lib/modules/platform/scm.spec.ts.md": "Tests TypeScript module system platform selection; no Rust equivalent",
    "lib/workers/repository/changelog/index.spec.ts.md": "Tests TypeScript worker changelog fetching with HTTP mocking; no Rust equivalent",
    "lib/workers/repository/init/index.spec.ts.md": "Tests TypeScript worker repo init with heavy vitest mocking; no Rust equivalent",
    "lib/workers/repository/update/branch/commit.spec.ts.md": "Tests TypeScript worker git commit with platform mocking; no Rust equivalent",
}

def process_file(path):
    with open(path) as f:
        content = f.read()
    
    reason = SPECS.get(os.path.relpath(path, "docs/parity"), "TypeScript-specific behavior with no Rust equivalent")
    
    pending_before = content.count("| pending |")
    
    # Replace pending rows
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
