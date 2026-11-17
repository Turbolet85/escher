"""Write P5's baseline key after `new = true` on each entry of the plan's gate fence (entry order 1..n)."""
import sys

path = sys.argv[1]
B = {
    1: "green — exit 1, last line 0 on the untouched tree (the Taffy source alone); controls in p5-controls.md: C1 green on the take, C9 red on a git AnyRender source",
    2: "red — exit 1, last line 0: the untouched lock holds anyrender 0.14.0 (the subject step 3 builds); must-pass C2 green on the take",
    3: "red — exit 1, last line 0: the untouched lock holds zip 2.4.2 (the subject step 3 builds); must-pass C3 green on the take",
    4: "red — exit 0, last line 4: the four zip ignores stand (the subject step 6 removes); must-pass C6 green",
    5: "red — exit 0, 711 B of file names: the hybrid names stand (the subject step 5 renames); must-pass C7 green",
    6: "red — exit 1, last line 0 on the untouched tree (no delta yet); must-pass C4 green on the take (3), C10 red on an added tracing line (4)",
    7: "red — exit 1, last line 0 on the untouched tree (no delta yet); must-pass C5 green on the take (3)",
    9: "green — 6 passed on HEAD (the unclamped reading step 7 re-pins); red-before-green is the step 7 control",
    10: "green — 2 passed on HEAD (the SVG case step 8 adds is not there yet); red-before-green is the step 8 control",
    11: "green — exit 0 in 151 s",
    12: "green — run.end 327 passed · 0 failed · 10 ignored, outcome passed (after boot)",
    13: "green — run.end 873 passed · 0 failed · 15 ignored, outcome passed (after boot)",
    14: "green — stayed up 10 s, logged service.name=seven_guis",
    15: "green — exit 0",
    16: "green — exit 0 in 19 s",
    17: "green — exit 0 (cargo +1.91 build)",
    18: "green — exit 0",
    19: "red — exit 1, advisories FAILED, licenses ok, on RUSTSEC-2026-0545 and RUSTSEC-2026-0548 alone (zip 2.4.2, the subject step 3 cures; inputs#I2); the trial take read advisories ok, licenses ok",
    20: "red — exit 1: the record does not exist yet (step 9 writes it); must-pass C8 green",
}
lines = open(path, encoding="utf-8").read().split("\n")
out, n, in_fence = [], 0, False
for line in lines:
    if line.startswith("```toml"):
        in_fence = True
    elif line.startswith("```"):
        in_fence = False
    if in_fence and line == "[[gate]]":
        n += 1
    out.append(line)
    if in_fence and line == "new = true" and n in B:
        assert "'" not in B[n] and '"' not in B[n], n
        out.append(f"baseline = '{B.pop(n)}'")
assert not B, f"unwritten: {sorted(B)}"
open(path, "w", encoding="utf-8").write("\n".join(out))
print(f"baselines written over {n} entries")
