"""Print every lost item of the run's cascade compares: its entity, fact and why line."""
import glob
import sys

run_dir = sys.argv[1]
for path in sorted(glob.glob(f"{run_dir}/cascade-compare-*.md")):
    doc = path.rsplit("cascade-compare-", 1)[1][:-3]
    lines = open(path, encoding="utf-8").read().splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("@compare_item") and "part=lost" in line:
            fields = dict(tok.split("=", 1) for tok in line.split()[1:] if "=" in tok)
            why = ""
            j = i + 1
            while j < len(lines) and not lines[j].startswith("@compare_item"):
                if lines[j].startswith("why:") and not why:
                    why = lines[j][5:]
                j += 1
            print(f"{doc} · {fields.get('entity')} · {fields.get('how')} · ({fields.get('label')} {fields.get('fact')}) · {why}")
            i = j
            continue
        i += 1
