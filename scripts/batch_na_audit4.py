#!/usr/bin/env python3
"""Batch NA audit helper #4."""
import re, os

SPECS = {
    "lib/modules/datasource/deb/packages.spec.ts.md": "Tests TypeScript deb package HTTP fetching with nock mocking; no Rust equivalent",
    "lib/util/cache/repository/index.spec.ts.md": "Tests TypeScript repository cache with vitest mocking; no Rust equivalent",
    "lib/util/exec/hermit.spec.ts.md": "Tests TypeScript hermit exec wrapper with mocking; no Rust equivalent",
    "lib/util/github/graphql/index.spec.ts.md": "Tests TypeScript GitHub GraphQL queries with HTTP mocking; no Rust equivalent",
    "lib/util/http/cache/memory-http-cache-provider.spec.ts.md": "Tests TypeScript memory HTTP cache provider; no Rust equivalent",
    "lib/util/http/gitea.spec.ts.md": "Tests TypeScript Gitea HTTP client pagination; no Rust equivalent",
    "lib/util/http/gerrit.spec.ts.md": "Tests TypeScript Gerrit HTTP client; no Rust equivalent",
    "lib/util/http/forgejo.spec.ts.md": "Tests TypeScript Forgejo HTTP client pagination; no Rust equivalent",
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
