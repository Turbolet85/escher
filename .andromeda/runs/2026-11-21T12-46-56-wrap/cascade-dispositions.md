# Cascade dispositions — 2026-11-21-engine-debts

## Step 2 — the sweep
The search: `cascade.py sweep --patterns-file cascade-patterns-a11y-plan-1.toml` (`sweep-pass-1.1.txt`), the one pattern a dry run wrote — `a11y1-3`, fixed `2.4.3`, the term the retired `decision/focus-fixup-reads-inert-alone` held. Every other dry run (architecture ×2, security-plan, design-system ×2, test-plan ×2) said `patterns: none: … changed prose alone — no file`: those rows' changes are read by step 3's leaves through each changed row's delta.
Counts, copied: `total (1 pattern) · 13 rows over 5 files` · `per class · new 0/0 · standing 4/2 · leaf 9/3 · curation 0/0 · base 0/0`.

Also read for the amended claims' old wording and mechanism, by hand at P1's pre-pass (`sweep-pre-pass-1.1.txt`, 36 master rows, each dispositioned in the report's `## After implement`, Sweep reach): «reads `is_inert()` alone», «keeps focus and takes keys», «focus stays», «grows for the document's life», «a node created in that same render». Every master row stating one is amended or retired in this pass (TX 2887–2894); `contract/dom-events/inert-focus-fixup`, `decision/inert-focus-fixup` (architecture) and `contract/mounteddata-during-render/set-focus-to` (design-system) stay true as written — no change.

Rows:
- test-plan `decision/a11y-leg-membership` (`.andromeda/test-plan.md:540`) — no change: names SC 2.4.3 as a criterion the leg witnesses, a true claim sharing the token.
- a11y-plan `dec-4jaazltv` (`a11y-plan.md:3`) — no change: the tier's criteria, true.
- a11y-plan `dec-3zaxed7b` (`a11y-plan.md:7`) — no change: the reach's criteria, true.
- a11y-plan `dec-4cglddpk` (`a11y-plan.md:67`) — no change: a check asserting SC 2.1.1 and 2.4.3, true.
- `.claude/rules/a11y.md:19`, `:30`, `:35` — no change: criterion names, true.
- `.claude/rules/a11y.md:32` — re-derived (step 3): «a named SC 2.4.3 gap», the retired limit.
- `.claude/docs/a11y-summary.md:8`, `:14`, `:29` — no change: criterion names, true.
- `.claude/docs/a11y-summary.md:24` — re-derived (step 3): the retired limit.
- `.claude/docs/gotchas.md:27` — re-derived (step 3): the retired limit.

Binds: test-plan §3 ↔ obs-plan §3 and §6, a11y-plan schema ↔ the log schema — no row of either side changed; held.
Curation homes and judgment bases: 0 rows.

## Leaves
The set: `cascade.py leaves` (pass mode — the epoch reads `open`, markerless 2), `total · pass · 13 group(s) · 23 leaves · 691729 B · unmapped 1`; the unmapped `.claude/rules/host-linux.md` is the host leaf, no source's — no comparer. One comparer a group (`--batch leaf`, 13 of 13 pass). The apply: `total · blocks 34 · applied 34` — every finding's source checked against the landed row it cites, each applied by the tool; 0 by hand, 0 declined.
- a11y-plan-1 · pass · 3 findings · applied 3 (`a11y-summary.md` :23, :24, :26)
- a11y-plan-2 · pass · 3 · applied 3 (`rules/a11y.md` :27, :32, :33)
- architecture-1 · pass · 6 · applied 6 (`conventions.md` :6; `services/blitz-dom.md` :15 ×4, :19)
- architecture-2 · pass · 2 · applied 2 (`services/dioxus-native-dom.md` :14, :31)
- architecture-3 · pass · 0
- architecture+security-plan+design-system+test-plan+a11y-plan · pass · 3 · applied 3 (`gotchas.md` :27, :31; `CLAUDE.md` :122)
- architecture+security-plan+test-plan-1 · pass · 2 · applied 2 (`services/escher-driver.md` :6, :36)
- architecture+test-plan · pass · 1 · applied 1 (`commands.md` :63)
- design-system · pass · 0
- security-plan-1 · pass · 3 · applied 3 (`security-summary.md` :13 ×2, :14)
- security-plan-2 · pass · 4 · applied 4 (`rules/security.md` :9, :12 ×2, :17)
- test-plan-1 · pass · 4 · applied 4 (`tests-summary.md` :21 ×2, :37, :52)
- test-plan-2 · pass · 3 · applied 3 (`rules/testing.md` :16, :22, :23)
Beyond the plan, the report named `.claude/rules/a11y.md`'s file list of the `a11y` leg: no leaf lists that leg's files (`git grep` over `.claude/` and `CLAUDE.md` for the new file name and the count words finds none), by the curated rule that a leaf neither lists a leg's files nor states its count — nothing owed. `docs/capabilities.md` was edited by the chunk itself.
