# Cascade dispositions — 2026-11-23-circle-drawer-on-a-canvas (wrap 2026-11-24T02-04-06-wrap)

## Step 2 — the sweep
The rows-land dry runs wrote no pattern file: each printed `patterns: none` (architecture «2 set(s) and retire(s) changed prose alone», security-plan, design-system, test-plan's `ops-test-plan.md` and a11y-plan «the call sets and retires no row», layout-templates «1 set(s) … changed prose alone», test-plan's `ops-main-test-plan.md` «1 set(s) … changed prose alone»). No dry run was superseded. Main's own patterns, `cascade-patterns.toml`, sweep each changed claim's old phrasing over the whole masters: `m-threefix` (the cross-OS canvas check's fixtures, «canvas-only shot», `stand_canvas_cross_os`), `m-clearsfocus` (a click on any other element clears focus; the click walk), `m-standboot` (the stand's boots), `m-lintboot` (lint clean at boot), `m-circledom` (Circle Drawer, its canvas id, `.circle` nodes). One call, `cascade.py sweep --patterns-file cascade-patterns.toml`, baseline `a199c213`, listing `sweep-pass-1.1.txt` read whole.

Totals copied: `total (5 patterns) · 57 rows over 19 files` · `per class · new 11/6 · standing 21/4 · leaf 25/13 · curation 0/0 · base 0/0`.

Master rows (32):
- `m-threefix` (6): architecture `dec-vlmnnziw`, test-plan `dec-drtazbk2`, `dec-7qmunnbr`, `dec-6vuh2wac` — no change: «a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64» still holds of every shot in the table, Circle Drawer's included, its text drawn on whole device pixels (the new provisional test-plan row `decision/canvas-text-on-whole-device-pixels` states the condition); `dec-gwo4ddwp`, `dec-ldhhfcng` — amended / added this pass (`edited` rows read: the new text stands, no duplicate of the old fixture list).
- `m-clearsfocus` (5): architecture `dec-svdas4ye` (`decision/click-focuses-button`) — amended this pass, its `except` naming the widget target; the `mixed ×2` rule keeps «a click on any other element still clears focus», true outside the excepted targets; `dec-sgptuaaa`, a11y-plan `dec-3ouwxzjc` — no change, true claims of the click walk sharing the token; `dec-sr3didn3`, security-plan `dec-uz6trulm` — new this pass.
- `m-standboot` (6): architecture `ctr-ymqabqx2` — amended this pass (`edited`); test-plan `ctr-emytbabz`, `ctr-fpq2xcnh`, `ctr-fowaumtb`, `ctr-pxggfjvw` — no change, other boots; `ctr-537tgfbo` — new this pass.
- `m-lintboot` (1): layout-templates `dec-emcqlk3f` — amended this pass.
- `m-circledom` (14): `ctr-ymqabqx2`, `dec-eiwpw2n7`, `dec-tp5vibgi`, `dec-emcqlk3f`, `dec-gwo4ddwp`, `dec-ldhhfcng`, `ctr-537tgfbo`, `dec-bg6u7ulu` — new or amended this pass; layout-templates `ctr-hbpncsrs` (`contract/stand-element-ids/circle-canvas`) — no change: its promise, a stable author id of the canvas, stays true (fit read the proposed set `not_master` type 4); `ctr-h7fu4ira`, `ctr-hae32ptw`, `ctr-hc6v6jkj`, `ctr-hchli3u2`, `ctr-htbykisw` — no change, other Circle Drawer ids, unchanged by the chunk.
Leaf rows (25 over 13 files): each goes to its group's comparer as a swept line it must answer (step 3). Curation 0, base 0.

Not looked for: the click synthesis's own new wording beyond `m-clearsfocus` (a new decision, no old phrasing); the fixture names in leaves beyond «three … fixtures».

## Leaves
Mode: WHOLE (`--all`) — this chunk closes «Epoch 3 — The canvas painted» (`epoch · … · markerless 0 · gated 0 · closes`), and the CARRY `cry-p6543j6z` names this pass. Set: `total · whole · 16 group(s) · 25 leaves · 776680 B · unmapped 1` — `unmapped .claude/rules/host-linux.md (host)`: the host's own leaf, no source states it, no comparer. Comparers: one batch `leaf`, 16 of 16 pass (`fanout.py check`). Apply: `total · blocks 25 · applied 24 · held 1`. Every swept line answered (25 swept rows over 13 files):
- a11y-plan-1 (`a11y-summary.md`) — swept :25 answered by finding #1; **declined**: its replace-with keeps «Circle Drawer 1» beside the added text, and the chunk moved `stand_non_lean_tasks` `MEASURED` and `CLICKABLE` for Circle Drawer 1 → 0, so applying it re-pins a stale count; the count is no master's (type 5), so no comparer can source it — carried (below).
- a11y-plan-2 (`rules/a11y.md`) — swept :28 answered by `@leaf_stands` (the actionable-key check still reads Circle Drawer empty), read true.
- architecture-1 — no swept line; 3 findings applied (`blitz-dom.md:15` ×2 the click-focus exception and the click synthesis; `blitz-paint.md:30` the `kurbo_css/css_box.rs` tree path, its file read present).
- architecture-2 (`dioxus-native-dom.md`) — swept :16 ×2 answered by `@leaf_stands` ×2, read true; 0 findings.
- architecture+layout-templates (`seven_guis.md`) — swept :38 `m-threefix` and :38 `m-lintboot` answered by findings #1 #2; swept :11, :38 `m-standboot`, :38 `m-circledom` by `@leaf_stands` ×3, read true; findings #3 (:19 the Circle Drawer boot) and #4 (:32 circles painted) applied; 4 applied.
- architecture+obs-plan (`escher-telemetry.md`) — no swept line, no finding.
- architecture+security-plan+test-plan-1 (`escher-driver.md`) — swept :61 by finding #1 (applied); swept :62 by `@leaf_stands`, read true.
- architecture+test-plan — swept `commands.md:61` `m-threefix` by finding #1 (applied), `m-standboot` by `@leaf_stands`, read true; finding #2 `blitz-test-harness.md:16` applied.
- design-system+layout-templates (`design-summary.md`) — swept :27 by findings #1 #2 (applied); #3 :13 the canvas colours applied.
- masters (`CLAUDE.md`, `gotchas.md`) — swept `CLAUDE.md:122` by finding #1 (applied).
- obs-plan-1 (`obs-summary.md`) — no swept line; finding #1 :49 the chaining panic hook applied (source `dec-66qxmezv`).
- obs-plan-2 (`rules/observability.md`) — no swept line, no finding.
- security-plan-1 (`security-summary.md`) — swept :13 by finding #1 (applied).
- security-plan-2 (`rules/security.md`) — swept :17 by findings #1 #2 (applied).
- test-plan-1 (`tests-summary.md`) — swept :26 and :14 by findings #1 #2 (applied); swept :21 ×2 by `@leaf_stands` ×2, read true as to the sources (no row states the per-task readings).
- test-plan-2 (`rules/testing.md`, `rules/verification-harness.md`) — swept `testing.md:36` by #1 #2, `:30` by #3, `verification-harness.md:36` by #4 (all applied); swept `testing.md:18` by `@leaf_stands`, read true.
By hand: none. Re-spawned: none (no comparer refused, no `@leaf_stands` read false).

Stale lines no comparer reaches — named and carried, not fixed by hand (the wrap directive, point 6):
- `.claude/docs/a11y-summary.md:25` — «Circle Drawer 1» twice (the `MEASURED` and `CLICKABLE` readings), measured 0 and 0 since this chunk (the report's Counts / qualifiers moved).
- `.claude/docs/tests-summary.md:21` — «Circle Drawer 1» (the `stand_non_lean_tasks` 7-row table), measured 0.
- `.claude/docs/services/seven_guis.md:38` — «Circle Drawer 1» twice, the same readings.
- `.claude/rules/verification-harness.md:21` — «`run stand` runs the 24 `stand_*` readers of the shared module»: 31 `stand_*` files declare `mod fixtures;` (`git grep -l 'mod fixtures;' -- 'tests/blitz-tests/tests/stand_*.rs'`, this wrap). The whole-mode comparer of `test-plan-2` read the leaf and wrote no finding on the line: no master row states the count. CARRY `cry-p6543j6z`'s pass ran and did not reach it; re-carried.
