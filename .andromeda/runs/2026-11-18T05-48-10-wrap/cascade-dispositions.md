# Cascade dispositions — 2026-11-18-capability-statement

## Sweep
Search: `cascade.py sweep --patterns-file cascade-patterns-architecture-1.toml` — the one pattern the landed dry runs wrote (arch1-1, fixed `docs/capabilities.md`, the term decision/capability-statement r1 held and r2 lacks; control architecture.md:1565, r1). The other three landings (security-plan tx 2039, test-plan tx 2040, a11y-plan tx 2041) changed prose alone and wrote no file: step 3's leaves read each changed row's delta. No main pattern file (no ops-main operation this wrap).
Counts (copied): `total (1 pattern) · 11 rows over 10 files` · `per class · new 0/0 · standing 0/0 · leaf 11/10 · curation 0/0 · base 0/0`.
Rows (part sweep-pass-1.1.txt, read whole):
- 11 leaf rows — CLAUDE.md:92, CLAUDE.md:100, .claude/rules/a11y.md:19, .claude/rules/observability.md:24, .claude/rules/testing.md:21, .claude/docs/a11y-summary.md:8, .claude/docs/commands.md:31, .claude/docs/obs-summary.md:49, .claude/docs/services/escher-driver.md:35, .claude/docs/services/escher-telemetry.md:31, .claude/docs/tests-summary.md:29: re-derived (step 3) — the path itself stays true (the file stands); the row's wording dropped it only by the sheet's no-path rule. The claims beside it (CLAUDE.md:92 · 100 «its five counts», tests-summary:29) are the comparers' to read against the changed rows' deltas.
- standing 0 · curation 0 · base 0: no master, curation home or judgment base states the path.

## Leaves
Set: `cascade.py leaves` — pass mode (`epoch · «Epoch 6 — Polish & ship» · markerless 1 · gated 0 · open`), `total · pass · 13 group(s) · 24 leaves · 715924 B · unmapped 1`; `unmapped .claude/rules/host-linux.md (host)` — a host leaf, no master source: no comparer, no change. Batch `leaf`, 13 comparers, `check`: 13 of 13 pass. Apply: `total · blocks 10 · applied 10` — no receipt refused (the RULED re-spawn route of inputs#I5 not reached).
Per group (mode pass each):
- a11y-plan-1: 1 finding · applied 1 (a11y-summary.md:8, stale) · by hand 0 · declined 0
- a11y-plan-2: 1 · applied 1 (rules/a11y.md:19, stale) · by hand 1 (rules/a11y.md:42, below) · declined 0
- added (obs-summary.md, rules/observability.md): 0 findings
- architecture-1 · architecture-2 · architecture-3: 0 findings each
- architecture+security-plan+test-plan-1: 1 · applied 1 (services/escher-driver.md:35, omission) · by hand 1 (escher-driver.md:61, below) · declined 0
- architecture+security-plan+test-plan+a11y-plan: 2 · applied 2 (CLAUDE.md:92, :100, omission) · by hand 0 · declined 0
- architecture+test-plan: 1 · applied 1 (commands.md:31, omission) · by hand 3 (commands.md:61, :62, :63, below) · declined 0
- security-plan-1: 1 · applied 1 (security-summary.md:42, omission) · declined 0
- security-plan-2: 1 · applied 1 (rules/security.md:26, omission) · declined 0
- test-plan-1: 1 · applied 1 (tests-summary.md:29, omission) · by hand 3 (tests-summary.md:21 @c28827, @c31059, @c27923, below) · declined 0
- test-plan-2: 1 · applied 1 (rules/testing.md:21, omission) · declined 0

By hand — the false «not in the `a11y` leg» clauses (`ci-leg.sh:41`'s `a11y)` line names `select_popup`, `select_popup_painted`, `stand_act_popup` and `key_press_after_container_lost_its_box`). No comparer flagged them. Removed on the operator's word (inputs#I6), nothing written in their place beyond what keeps the sentence true, no file list written; each leaf drafted whole and landed by `rec.py harness land` (`--dry-run` first):
- the three the report named (Insufficient fixes), on the Q1 answer: `.claude/rules/a11y.md:42` («none is among this leg's files, though `stand_keyboard`, one of them,» → «and `stand_keyboard`, one of this leg's files,»), tx 2051; `.claude/docs/tests-summary.md:21` @c28827 (`stand_act_popup`, «, not among the `a11y` leg's files» dropped) and @c31059 (`key_press_after_container_lost_its_box`, «and none of the `a11y` leg's files» and «or the `a11y` leg» dropped), tx 2052;
- two more found by the check grep, on the Q2 answer: `.claude/docs/commands.md:61` and `.claude/docs/services/escher-driver.md:61` (`stand_act_popup`, «, not among the `a11y` leg's named tests» dropped);
- three more of the exact class, under Q2's standing word: `.claude/docs/commands.md:62` (`select_popup` · `select_popup_painted`, «not by `run stand` or the `a11y` leg» → «not by `run stand`»), `.claude/docs/commands.md:63` (`key_press_after_container_lost_its_box`, «, the `stand-repeat` leg or the `a11y` leg» → «or the `stand-repeat` leg»), `.claude/docs/tests-summary.md:21` @c27923 (`select_popup`, as :62); tx 2053 (commands.md), 2054 (escher-driver.md), 2055 (tests-summary.md).
The class search (a python regex over `git ls-files .claude CLAUDE.md` for «not among/in … leg», «neither the `a11y`», «not by … the `a11y` leg» and kin) read 7 hits: the 5 above, and 2 true and left — `a11y-summary.md:25` (`dioxus_runtime_probe`, on no a11y line) and `tests-summary.md:21` @c1883 (a redraw test the a11y line does not name).
