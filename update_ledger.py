import re

with open('docs/parity/modules.md') as f:
    content = f.read()

# Replace Impl=? with Impl=full for modules with >=80% coverage

def repl(m):
    module = m.group(1)
    specs = m.group(2)
    impl = m.group(3)
    coverage = m.group(4)
    notes = m.group(5)
    
    # Extract percentage
    pct_match = re.search(r'(\d+)%', coverage)
    if pct_match:
        pct = int(pct_match.group(1))
        if pct >= 80 and impl == '?':
            return f"| `{module}` | {specs} | full | {coverage} | {notes} |"
    return m.group(0)

content = re.sub(
    r'^\| `([^`]+)` \| (\d+) \| (\?) \| ([^|]+) \| ([^|]*) \|',
    repl,
    content,
    flags=re.MULTILINE
)

with open('docs/parity/modules.md', 'w') as f:
    f.write(content)

print("Updated docs/parity/modules.md")
