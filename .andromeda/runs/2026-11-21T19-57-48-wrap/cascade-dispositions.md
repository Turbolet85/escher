# Cascade dispositions — 2026-11-21-single-select-sized-as-a-list-box

## Step 2 — the sweep
Skipped: no file holds a pattern. Every `rows-land` dry run printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file` (architecture ×2, security-plan, a11y-plan, design-system ×2, obs-plan, test-plan). Step 3's leaves read each changed row's delta.

Old wording across the masters: the P1 pre-pass sweep (`sweep-pre-pass-1.1.txt`, pattern `q-size`) found the retired «a single select's `size` is unread / not read» clause standing in 3 master rows. They are architecture `decision/select-size-hint` and security-plan `decision/multiple-select-size-rows` · `decision/single-select-selectedness`, each set in this pass (tx 2950, 2951). The same pattern's leaf rows (`.claude/rules/security.md:17`, `.claude/docs/security-summary.md:13`, `.claude/docs/services/blitz-dom.md:15`) were added to step 3 by `--add` and re-derived there. Curation homes: `q-size` 0 · bases 0.

Lateral binds: no harness command, log field or violation-schema row moved — none owed.

## Leaves
`cascade.py leaves` · pass · epoch «Epoch 2 — Debts of 0.1.0–0.3.0» open (markerless 1) · baseline efe014e9 · 16 groups · 25 leaves · `--add` `.claude/rules/a11y.md`, `.claude/docs/tests-summary.md`, `.claude/rules/security.md`, `.claude/docs/security-summary.md`, `.claude/docs/services/blitz-dom.md` (the report's named leaves) · unmapped 1: `.claude/rules/host-linux.md` (host) — no master source, no step-3 edit.

Comparers: 16 of 16 pass (`fanout.py check`). Apply: `total · blocks 83 · applied 83` — by hand 0 · declined 0. Every omission was read against its source row before the write, and two replacements were read whole (`a11y.md:33` #4, `a11y-summary.md:23` #3): each keeps the statement it extends.
- a11y-plan-1 (pass): 11 found · 11 applied
- a11y-plan-2 (pass): 10 · 10
- architecture-1 (pass): 16 · 16
- architecture-2 (pass): 3 · 3
- architecture-3 (pass): 2 · 2
- architecture+obs-plan (pass): 0
- the six-master group (pass): 4 · 4 (CLAUDE.md)
- architecture+security-plan+test-plan-1 (pass): 6 · 6
- architecture+test-plan (pass): 2 · 2
- design-system (pass): 3 · 3
- obs-plan-1 (pass): 2 · 2
- obs-plan-2 (pass): 1 · 1
- security-plan-1 (pass): 9 · 9
- security-plan-2 (pass): 8 · 8
- test-plan-1 (pass): 3 · 3
- test-plan-2 (pass): 3 · 3

Not a step-3 edit (no master states it): the per-file test counts in `.claude/rules/a11y.md:35` («18 tests», «25 tests», «10 tests», now 24 · 28 · 17 by the report) and `.claude/docs/tests-summary.md`. They are code-retold counts, left for the whole-mode leaves pass at Epoch 2's close and named in the handoff.
