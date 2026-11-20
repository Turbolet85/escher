# P5 review — 2026-11-20-select-list-and-selection-debts · «Select list and selection debts»

**Plan:** `doc:2026-11-20-select-list-and-selection-debts/plan`, rev 2, 351 lines; open its render at
`.andromeda/cache/docs/2026-11-20-select-list-and-selection-debts/plan.md`.
- No plan stands in a chunk's folder. Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-select-list-and-selection-debts`
  diffs the chunk's records.

## Goal
Four of the five debts are fixed, each as the founder ruled (inputs#I2):
- **F1 A** — the lint reads an open list's highlighted row as painted. A new read-only harness reader,
  `Harness::highlight_contrast`, does it (your choice, inputs#I3). A check pins the list's highlight constants to the
  default sheet's `multiple` chosen-row pair.
- **F2 A** — one row height, the tallest row's line height, headers included; no row is shorter than today.
- **F3 A** — while a single select's list is open, its tree names the highlighted option as its active descendant.
  The snapshot gains `highlighted`, the 15th node key, a closed bool written only where true; the diff gains its
  13th reading. A highlight move by key or pointer marks the changed set.
- **F5 A** — Cells' current cell carries `aria-selected="true"`.
- **F4 C** — not built here. `res-ysz67imk` stays; the wrap places its own route entry.

## Top acceptance criteria
- `matrix#v040-04` — all five forks reached the founder. The four fixes are proven headless in both layout modes, each
  by a named check. F4 stands as the residual with his reason.
- The lint reads 12.56 on a highlighted row and gives no finding there; plain rows below 4.5 still read findings.
- A 32px option in a 13px select is painted whole inside its row.
- The tree's active descendant is set while the list is open, tree focus stays on the select, and `stand_keyboard`'s
  list walks stay green.
- A `press` in an open list returns `differs: [highlighted]`. `NODE_FIELDS` has 15 keys and `Reading::ALL` 13. The
  committed Counter checks replay byte-equal.
- Cells' current cell alone reads `selected`, and an arrow moves the reading.
- CI green on the pushed HEAD, the cross-OS Cells digests held on windows and macOS. `fast`, `doc`, `a11y` and
  `stand-repeat` green.

## Touchpoints (20, no new file)
- **Harness:** `blitz-test-harness/src/contrast.rs`.
- **Driver:** `escher-driver/src/{lint,schema,schema_tests,json/snapshot,json_tests}.rs`.
- **Engine:** `blitz-dom/src/{document,accessibility,events/pointer}.rs`.
- **Snapshot:** `dioxus-native-dom/src/{snapshot,snapshot_text,snapshot_diff}.rs`.
- **Stand:** `seven_guis/src/tasks/cells.rs`.
- **Tests:** six blitz-tests targets (`select_popup`, `select_popup_painted`, `stand_lint`, `stand_act_popup`,
  `stand_cells_served`, `stand_cross_os_readings`).
- **Docs:** `docs/capabilities.md`.

## Test commands (12 entries; baselines on the untouched tree, all green)
| # | Entry | Baseline |
|---|---|---|
| 1 | four crates' `--lib` | 448 passed |
| 2 | the six targets the steps write | 65 passed |
| 3 | thirteen companion targets | 118 passed |
| 4 | five seven_guis process checks | 42 passed |
| 5 | the capability-statement pin | OK |
| 6 | `a11y` leg | green |
| 7 | `fast` | seen green at the last wrap |
| 8 | `doc` leg | green |
| 9 | `stand-repeat` | 3 of 3 rounds green, 903 s |
| 10–12 | the operator's hygiene, push and CI read | `--wait 2700`; the last run took 2,373 s |

Five one-shot `gate.py control` runs at implement: the painted read removed, the row height from the select, no
active descendant, a move marking nothing, and Cells stating nothing.

## Checks
- **Mechanical:** sections, paths, the fence (12 entries), placeholders, step references, snapshot citations
  (I1–I3) and research's lists (20 items) all pass. planlint reports 0 hits. Size: 351 lines.
- **Validation-1:** aligned with the scope as amended at P3 (the rulings folded, F4 out, one premise corrected: a
  `multiple` select's chosen row is already read right by the lint).
- **CI:** at take-up the run on the last wrap's sha read `in progress`; it now reads green (CI#44758244908, 17/17).
- **Row-read coverage:** no master picked 0 rows. Of the history files, a11y and design were read whole; the other
  five were searched by the chunk's terms. That narrower read is recorded in the plan step's evolve record.
- **Expected amendments (wrap):** listed in the plan's notes across a11y-plan, design-system, architecture,
  security-plan, test-plan and layout-templates.
  - security-plan's select-list crossing gains a fourth closed state. It falls under pattern `rul-izni57qv`
    (Boundary widening) as a closed bool inside a ratified carrier, answered by the founder's own F3 A as relayed by
    the overseer (inputs#I2).

## Concretization preview — `matrix#v040-04` (claimed after the word)
- **Current acceptance:** «Each of the five debts reaches the founder as a fork; each one fixed is proven headless on
  the stand or a fixture in both layout modes, and each one kept stands as a residual with his reason.»
- **Proposed:** «The five debts reached the founder as five forks, answered F1 A · F2 A · F3 A · F4 C · F5 A
  (inputs#I2). Each fix is proven headless in both layout modes by its named check, green:
  - F1 — `stand_lint`'s highlighted-row test and `select_popup_painted`'s pair-pin and `highlight_contrast` tests;
  - F2 — `select_popup`'s row-height test and `select_popup_painted`'s unclipped-glyph test;
  - F3 — `select_popup`'s active-descendant and marks tests, and `stand_act_popup`'s press-diff test;
  - F5 — `stand_cells_served`'s selected-cell test.

  F4 stands kept as `res-ysz67imk`, with his reason «Отдельной записью маршрута» (its own route entry). Implement's
  report enumerates the five dispositions by construction.»
- **Method:** integration (unchanged). Nothing is weakened: the outcome is the same, now named per fork.

## Beyond the plan
| # | Proposal | Owner |
|---|---|---|
| 1 | A new route entry for a single select's `size` rendered as a list box (HTML's rendering, `size` × 1.2em, keys moving the choice, no popup). Proposed title «Single select sized as a list box», placed in Epoch 2 «Debts of 0.1.0–0.3.0» after «Engine debts», on the founder's F4 C (inputs#I2). | the wrap's route-resolve |
| 2 | The residual `res-ysz67imk` re-targeted from v040-04 to that new entry, with the founder's reason. | the wrap |

Apply? (yes / review / cancel)
