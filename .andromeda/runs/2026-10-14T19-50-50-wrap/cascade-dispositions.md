# Cascade dispositions — 2026-10-14-stand-a11y-assertions

The search: `cascade.py sweep --patterns-file {run_dir}/cascade-patterns.toml` (cascade v1.8), run after the last body of this pass was applied, over the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaves. Baseline `ccfcda48` (the parent of the oldest pre-CI commit, `aab4cda9`).

Patterns — each a retired claim of this pass, by its wording and by the mechanism it asserted:
- `old-accent` — the old accent hex, its rgba form and its 4.39 ratio
- `not-met` — "SC 1.4.3 is not met" on the accent buttons, however worded
- `record-tally` — the old record "pass 9 · fail 5"
- `three-files` — the `a11y` leg "stays / staying its three files"
- `leg-list` — the three-file leg list, as a `--test` list, a parenthetical or "6 + 6 + 4"
- `route-owner` — "Stand a11y assertions" named as an owner
- `route-owned` — "owned by a route entry" / "witness owned by a route" / "owed on the working route" / "owned by a CARRY on the route" (the owner wording without the entry's name)
- `empty-name` — the CRUD list's empty or missing accessible name
- `records-nothing` — "records and gates nothing" / "adds no stand assertion"
- `kb-count` · `contrast-count` — `stand_keyboard` 11 · `stand_contrast` 2
- `run-stand` · `ws-totals` — passed 153 · 928 passed
- `states-unmeas` — the colour-changing / non-boot states as unmeasured
- `boot-only-walk` — the Tab order asserted at boot only
- dropped: `ids-count` (`stand_accessibility_ids` 5) — its control never fired over the pre-pass masters (`control: ids-count never fired …`): no master stated that file's count.

Sections read beyond the rows: architecture §Standard Contracts (CI contracts, Mutation, query and CSSOM, Dioxus DOM bridge), §Established Decisions [Unsupported features], §Existing Scopes; security-plan §Input Validation (the `disabled`, `id`, control-states and falsy-boolean rows); test-plan §1, §3, §4, §9; obs-plan §3; a11y-plan §1, §2, §3, §5, §6, §7, §9 and the keyed contract `§3 → Bootstrap phases` (its lines 3 and 4 hold: 18 control rows measured, the `a11y` job named — no change).

Counts, from the listing: total (15 patterns) · 60 rows over 13 files · per class · new 7/5 · standing 18/4 · leaf 35/7 · curation 0/0 · base 0/0.

## Master rows (new 7 · standing 18)
- design-system.md:24 old-accent new ×3 → amended (this pass's own "until then `#4a6cf7` …, 4.39" history clause — true as history)
- test-plan.md:142 old-accent standing → no change (the harness unit table's tolerance figure "4.48 and 4.39 within 0.005"; that fixture keeps `#4a6cf7` by design — report Counts)
- a11y-plan.md:229 old-accent new ×2 → amended (this pass's "until then … `#4a6cf7` at 4.39 … failing" history clause)
- architecture.md:263 record-tally new ×2 → amended ("pass 9 · fail 5 until then", history)
- test-plan.md:26 record-tally new ×2 → amended (the same history clause)
- obs-plan.md:70 three-files new → amended ("over the three files and the shared module" — the three test files of the census, a true claim sharing the token)
- a11y-plan.md:340 three-files new → amended ("the first three files alone until then", history)
- architecture.md:87 route-owned standing ×2 → no change (range-input and other owners on other route entries — a true claim sharing the token; `route-owner` reads 0 master rows, so no master still names "Stand a11y assertions")
- architecture.md:91 route-owned standing edited → no change (`select`/`textarea` owner, another entry; its line edited only by the citation sweep)
- architecture.md:124 route-owned standing → no change (scroll readings' owner, another entry)
- architecture.md:136 route-owned standing edited → no change (the second position reader's owner, "Stand requirement sweep")
- security-plan.md:119 route-owned standing edited → no change (the `disabled` narrowing, a CARRY on "Stand requirement sweep")
- security-plan.md:123 route-owned standing edited @c7873 → no change ("whether it becomes one is owned by a route entry" — the actionable-key command, not this chunk's; read by offset)
- test-plan.md:14 route-owned standing → no change (scrolling-box reading, another entry)
- test-plan.md:92 route-owned standing → no change (the same)
- a11y-plan.md:102 route-owned standing → no change (the `contain: paint` hit reading, another entry)
- a11y-plan.md:144 route-owned standing edited → no change (`select`/`textarea`, another entry)
- a11y-plan.md:280 empty-name new → amended ("until 2026-10-14-stand-a11y-assertions the list read the accessible name `""`", history)
- test-plan.md:116 kb-count standing edited · contrast-count standing edited · run-stand standing edited → no change (the dated re-count entries of earlier chunks, history; this pass appended its own)
- test-plan.md:330 kb-count · contrast-count · ws-totals standing edited → no change (the same, history)
- a11y-plan.md:280 boot-only-walk standing edited → no change ("with the Tab order unchanged at that chunk" — the 2026-10-06 chunk's reading, history)

## Leaf rows (35 over 7 files) → re-derived at step 3
CLAUDE.md:45 (old-accent ×2, not-met, route-owner ×2, route-owned) · CLAUDE.md:110 (route-owned — another entry's owner; the comparer reads it) · .claude/rules/a11y.md:32, :38, :42 · .claude/docs/a11y-summary.md:15, :21, :22, :24, :26, :35 · .claude/docs/design-summary.md:13 · .claude/docs/services/seven_guis.md:24, :30, :38 · .claude/docs/tests-summary.md:21, :30 · .claude/docs/services/blitz-test-harness.md:34 — each joins step 3's set (`--add` where the table does not already place it).

Curation homes: 0 rows. Judgment bases: 0 rows.

## Leaves
Set: `cascade.py leaves --run-dir {run_dir} --marker 2026-10-14-stand-a11y-assertions --add CLAUDE.md --add .claude/rules/a11y.md --add .claude/docs/a11y-summary.md --add .claude/docs/design-summary.md --add .claude/docs/services/seven_guis.md --add .claude/docs/tests-summary.md --add .claude/docs/services/blitz-test-harness.md` — total · pass · 11 group(s) · 25 leaves · 318341 B · unmapped 1. Mode: pass (this chunk does not close Epoch 6 — "Stand requirement sweep", "Quality gates" and "Cold-agent test" stand markerless after it). One comparer per group, one batch inside one snapshot pair (before `tree 35d95af2…`, after `tree 34a623db…`, index and head unchanged, exactly the 11 `leaf-{group}.md` added). Apply: `leaves --apply` dry run, then the write — total · blocks 37 · applied 37; every receipt count equals its file's blocks.
- architecture-1 · pass · findings 1 · applied 1 (blitz-shell.md:18, omission — the headless halves and the residuals owner) · by hand 0 · declined 0
- architecture-2 · pass · findings 0
- masters (CLAUDE.md) · pass · findings 6 · applied 6 (:45 ×4 — the `inert` owner, SC 1.4.3 met, `ENABLED_TAB_ORDER`, the contrast tables; :38 — `crud-list` named; :79 — the refresh's headless check in the pointer table) · by hand 0 · declined 0
- architecture+layout-templates (services/seven_guis.md) · pass · findings 5 · applied 5 · by hand 0 · declined 0
- architecture+test-plan · pass · findings 1 · applied 1 (blitz-test-harness.md:34) · by hand 0 · declined 0
- architecture+obs-plan · pass · findings 0
- security-plan · pass · findings 1 · applied 1 (rules/security.md:17, the control-states witness owner) · by hand 0 · declined 0
- design-system+layout-templates · pass · findings 3 · applied 3 · by hand 0 · declined 0
- test-plan · pass · findings 8 · applied 8 (tests-summary.md:21 ×6, :29, :30) · by hand 0 · declined 0
- obs-plan · pass · findings 1 · applied 1 (obs-summary.md:49) · by hand 0 · declined 0
- a11y-plan · pass · findings 11 · applied 11 (a11y-summary.md ×7, rules/a11y.md ×4) · by hand 0 · declined 0
- unmapped `.claude/rules/host-linux.md` (host) — the host leaf; no source states it, no comparer.
Each finding's source side was checked at its cited master line as amended this pass. The leaf rows of the step-2 sweep are all covered: CLAUDE.md:45, rules/a11y.md:32/:38/:42, a11y-summary.md:15/:21/:22/:24/:26, design-summary.md:13, seven_guis.md:24/:30/:38, tests-summary.md:21/:30, blitz-test-harness.md:34 rewritten; CLAUDE.md:110 (another entry's owner) and a11y-summary.md:35 (the route's history of the `a11y-ci-gate-wire` discharge naming this entry) read and left — true as written.
