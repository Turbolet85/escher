import re, sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
B = {
    1: "red — exit 1 at 3617bc017: no MERGE_HEAD, the state step 2 makes; must-pass green with the merge staged in a scratch clone (run dir controls.md)",
    2: "green — exit 1, no output at 3617bc017; known positive red on the staged merge's unresolved events.rs in a scratch clone (controls.md)",
    3: "green — exit 0 at 3617bc017; must-pass green with the merge staged, known positive red with an AGENTS.md minted in a scratch clone (controls.md)",
    4: "red — exit 0, last line 1 at 3617bc017: taffy's git entry, the move step 6 takes; must-pass green (exit 1, last line 0) on the staged merge in a scratch clone (controls.md)",
    5: "green — last line 13 at 3617bc017; must-pass green on the staged merge, known positive red (last line 12) on one line edited in a scratch clone (controls.md)",
    6: "red — no output at 3617bc017: no lock move yet; must-pass green on the staged merge in a scratch clone, the literal read there (controls.md)",
    7: "green — exit 0 at 3617bc017, 7 s",
    8: "green — exit 0 at 3617bc017, 66 s",
    9: "green — exit 0 at 3617bc017, 560 s",
    10: "green — exit 0 at 3617bc017, 62 s",
    11: "green — exit 0 at 3617bc017, 47 s",
    12: "green — exit 0 at 3617bc017, advisories ok, licenses ok",
    13: "green — exit 0 at 3617bc017, 383 s, 3 of 3 floors, dioxus-native-dom 4308/4727 = 91.14 % (the trial merge read 4308/4863 = 88.59 %)",
    14: "green — exit 0 at 3617bc017, 993 s, stand-repeat: 3 of 3 rounds green",
    15: "green — exit 1, last line 0 at 3617bc017; the subject unchanged by this chunk, its post-merge reading the take's",
    16: "green — exit 0, last line 14 at 3617bc017 (the file written at P4); known positive red (exit 2) where the file is absent, a scratch clone (controls.md)",
}
blocks = s.split("[[gate]]")
out = [blocks[0]]
for i, b in enumerate(blocks[1:], start=1):
    if i in B and "baseline =" not in b:
        lines = b.rstrip("\n").split("\n")
        # insert after the note line
        idx = max(j for j, l in enumerate(lines) if l.startswith("note = "))
        lines.insert(idx + 1, "baseline = '" + B[i] + "'")
        tail = b[len(b.rstrip("\n")):]
        b = "\n".join(lines) + tail
    out.append(b)
open(p, "w", encoding="utf-8").write("[[gate]]".join(out))
print(s.count("[[gate]]"), "entries;", "[[gate]]".join(out).count("baseline ="), "baselines")
