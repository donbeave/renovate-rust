import re

with open('docs/parity/modules.md') as f:
    lines = f.readlines()

full_modules = []
for line in lines:
    m = re.match(r'^\| `([^`]+)` \| \d+ \| \? \| (\d+/\d+) \((\d+)%\) \|', line)
    if m:
        module, ratio, pct = m.group(1), m.group(2), int(m.group(3))
        ported, total = map(int, ratio.split('/'))
        if pct >= 80:
            full_modules.append(module)

print(f'Found {len(full_modules)} modules at >=80%:')
for m in full_modules:
    print(m)
