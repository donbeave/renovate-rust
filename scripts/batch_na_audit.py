#!/usr/bin/env python3
"""Batch NA audit helper — flips pending rows to not-applicable with reasons."""
import re, sys, glob, os

REASONS = {
    "lib/util/promises.spec.ts.md": "Tests TypeScript-specific p-all/p-map promise wrappers and ExternalHostError aggregation; Rust uses native Future/join_all",
    "lib/util/jsonata.spec.ts.md": "Tests jsonata npm package (JS expression language); no Rust equivalent",
    "lib/util/emoji.spec.ts.md": "Tests emojibase npm package (emoji shortcode library); TS-specific library integration",
    "lib/util/yaml.spec.ts.md": "Tests Zod schema validation on parsed YAML; Rust does not use Zod",
    "lib/util/stats.spec.ts.md": "Tests logger.debug/trace integration of report() methods; Rust get_report() returns data without logging",
    "lib/util/http/got.spec.ts.md": "Tests got npm HTTP client configuration (rejectUnauthorized, option cloning); TS-specific library",
    "lib/util/http/jira.spec.ts.md": "Tests Jira HTTP client wrapper baseUrl behavior; TS-specific HTTP client setup",
}

def process_file(path):
    with open(path) as f:
        content = f.read()
    
    reason = REASONS.get(os.path.relpath(path, "docs/parity"), "TypeScript-specific library or runtime behavior with no Rust equivalent")
    
    # Count before
    pending_before = content.count("| pending |")
    
    # Replace pending rows with not-applicable rows (add reason)
    lines = content.split("\n")
    new_lines = []
    for line in lines:
        if "| pending |" in line and "| — |" in line and "Status" not in line:
            # Replace the last empty reason cell with the reason
            if line.rstrip().endswith("| — |"):
                line = line.rstrip()[:-4] + reason + " |"
            elif "| — |" in line:
                line = line.replace("| — |", "| " + reason + " |", 1)
        new_lines.append(line)
    
    content = "\n".join(new_lines)
    
    # Replace remaining pending status cells
    content = content.replace("| pending |", "| not-applicable |")
    
    pending_after = content.count("| pending |")
    na_after = content.count("| not-applicable |")
    ported = content.count("| ported |")
    total = pending_after + na_after + ported
    
    # Update header
    content = re.sub(
        r"\*\*Total tests:\*\* \d+ \| \*\*Ported:\*\* \d+ \| \*\*Actionable:\*\* \d+ \| \*\*Status:\*\* \w+",
        f"**Total tests:** {total} | **Ported:** {ported} | **Actionable:** {total} | **Status:** {'done' if pending_after == 0 else 'partial'}",
        content,
    )
    
    with open(path, "w") as f:
        f.write(content)
    
    return pending_before, pending_after, na_after, ported, total

def main():
    docs_dir = "docs/parity"
    total_pending = 0
    total_na = 0
    
    for rel_path, reason in REASONS.items():
        path = os.path.join(docs_dir, rel_path)
        if not os.path.exists(path):
            print(f"MISSING: {path}")
            continue
        before, after, na, ported, total = process_file(path)
        moved = before - after
        print(f"{rel_path}: moved {moved} pending → NA (now {after} pending, {na} NA, {ported} ported, {total} total)")
        total_pending += after
        total_na += na
    
    print(f"\nBatch complete.")

if __name__ == "__main__":
    main()
