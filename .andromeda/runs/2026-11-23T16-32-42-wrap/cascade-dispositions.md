# Cascade dispositions — 2026-11-23-canvas-shot-across-hosts (wrap 2026-11-23T16-32-42-wrap)

## Step 2 — the sweep
- The `rows-land` dry runs wrote no pattern file: every one printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file` (ops-architecture.md 2, ops-test-plan.md 4, ops-main-architecture.2.md 1, ops-main-test-plan.2.md 3, ops-main-test-plan.md 2). No `cascade-patterns-{doc}-{k}.toml` exists to take or supersede.
- Main's own patterns, `cascade-patterns.toml`: `m-bytesdiffer` (the amended claim's phrasings — «PNG bytes differ», «differ between linux», «differ by OS», «byte-identical on one host») and `m-noshotxos` (the no-shot cross-OS rule's phrasings — «compared byte for byte across OSes», «holds no screenshot step», «no shot in cross-OS»), each over the seven masters, the key files, the leaves, the three curation homes and the two bases. Both controls fired (architecture.md:1401, architecture.md:1634).
- First run (`sweep-pass-1.1.txt`, before the last landing — partial by construction) found two test-plan rows whose `why` stated the amended claim flat — `decision/checks-hold-no-screenshot-step` (dec-cnfe6knx) and `decision/no-screenshot-in-recorded-check-scripts` (dec-bdbzxscn); both scoped by main's `ops-main-test-plan.md` (TX 3410).
- Final run, after the last landing: `sweep-pass-2.1.txt` — `total (2 patterns) · 22 rows over 8 files` · `per class · new 0/0 · standing 8/2 · leaf 14/6 · curation 0/0 · base 0/0`.

Master rows (8):
- test-plan:26 dec-acdz5ghz (m-bytesdiffer, m-noshotxos) — amended (TX 3407): its `why` scoped to a shot holding document text; «byte-identical on one host only» is now said of such a shot — true.
- architecture:1634 dec-vlmnnziw (m-noshotxos ×2) — amended (TX 3406 `except`, TX 3408 `why`).
- test-plan:67 dec-drtazbk2 · :113 dec-7qmunnbr · :592 dec-6vuh2wac (m-noshotxos ×2 each) — amended (TX 3407 `except`, TX 3409 `why`).
- test-plan:445 dec-cnfe6knx · :456 dec-bdbzxscn (m-noshotxos) — amended (TX 3410 `why`); the rule «committed checks hold no screenshot step» stands true.
- No other master holds the old wording: security-plan's dec-lbxbb3dy and dec-jsatogzw (the P1 pre-pass's `shotos` rows) state the committed checks' content, no reason — no change.

Leaf rows (14 over 6 files) — each goes to its group's comparer as a line it must answer (`cascade.py leaves` reads this sweep): CLAUDE.md:122 (×2 each pattern) · .claude/rules/testing.md:36 · .claude/docs/commands.md:61 (both) · .claude/docs/services/escher-driver.md:61 (both) · .claude/docs/services/seven_guis.md:11, :38 (both) · .claude/docs/tests-summary.md:21, :26 (both).

Curation 0 · base 0 — no curation home or judgment base states the old wording by these patterns.

Lateral binds: test-plan §3 ↔ obs-plan §3 / §6 and the a11y violation schema — no harness command, log field or violation schema moved; none owed.

## Leaves
Set: `cascade.py leaves` (pass mode; epoch «Epoch 3 — The canvas painted» open, markerless 1) — `total · pass · 8 group(s) · 18 leaves · 507861 B · unmapped 1`; sources architecture and test-plan (`leaf-delta-architecture.diff`, `leaf-delta-test-plan.diff`); the sweep's 6 leaf files (`sweep-pass-2.1.txt`) all inside groups. Apply: `total · blocks 10 · applied 10`.
- architecture-1 · pass · 5 leaves · no swept line · 0 findings · applied 0 · by hand 0 · declined 0.
- architecture-2 · pass · 2 leaves · no swept line · 0 findings · applied 0 · by hand 0 · declined 0.
- architecture-3 · pass · 3 leaves · swept seven_guis.md:11 (m-bytesdiffer), :38 (m-bytesdiffer, m-noshotxos) · 2 findings (stale ×2) · applied 2 · by hand 0 · declined 0.
- architecture+test-plan-1 · pass · 3 leaves · swept commands.md:61 (both patterns) · 2 findings (stale, omission — `stand_canvas_cross_os` beside `stand_canvas_shot` in the command line's listing) · applied 2 · by hand 0 · declined 0.
- architecture+test-plan-2 · pass · 1 leaf · swept escher-driver.md:61 (both) · 1 finding (stale) · applied 1 · by hand 0 · declined 0.
- architecture+test-plan-3 · pass · 1 leaf · swept CLAUDE.md:122 (both) · 1 finding (stale) · applied 1 · by hand 0 · declined 0.
- test-plan-1 · pass · 1 leaf · swept tests-summary.md:21, :26 (both) · 2 findings (stale ×2) · applied 2 · by hand 0 · declined 0.
- test-plan-2 · pass · 2 leaves · swept testing.md:36 (m-bytesdiffer) · 2 findings (stale, omission — the canvas-only exception beside the cross-OS readings line) · applied 2 · by hand 0 · declined 0.
Every swept line was answered by a finding; no comparer wrote a `@leaf_stands` record; none was refused or re-spawned; no leaf line was written by hand.
- unmapped `.claude/rules/host-linux.md` (host) — the host leaf, no comparer's and no step-3 edit.
- Not reached by this pass, carried: `.claude/rules/verification-harness.md`'s Test-data bootstrap line «`run stand` runs the 24 `stand_*` readers of the shared module» (route CARRY `cry-p6543j6z` on «Circle Drawer on a canvas», Epoch 3's whole-mode leaf pass) — the test-plan-2 comparer read the leaf in pass mode, no swept line named it, and it wrote no finding on it.
