# P5 review — 2026-11-20-driver-debts · «Driver debts» (v040-05)

The plan's render is `.andromeda/cache/docs/2026-11-20-driver-debts/plan.md` (rev 3); open it in an editor. No plan
stands in a chunk's folder. Writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-driver-debts` is a diff of
the chunk's records.

## Goal
Three driver debts are fixed, each by the founder's own word (`inputs#I2`: F1 A · F2 A · F3 A):
- **`press --ctrl`**: held as the host's action key, ⌘ on macOS and Ctrl elsewhere. A non-adjacent `multiple` set
  becomes reachable: `--ctrl` on the arrows, then `space`.
- **`graph --part N`**: paged as `snapshot` is, at its 10,000-byte budget, with the state list in part 1 only. The
  help says a later part's empty state list means "read part 1". A part reads
  `{"nodes","states","part","parts"}` (`inputs#I1`).
- **The reached record reads the stop**: it resolves ids before the dispatch and writes them after it. Every listener
  counts when nothing stopped the event, only the innermost when something did, and the target alone for a
  non-bubbling event. So coverage can no longer read a handler "exercised" that a stop kept the event from.

## Top acceptance criteria
- The `fx-rows` multiple select: choose row 1, `press --ctrl arrow-down` twice, `press space` → rows 1 and 3 chosen,
  row 2 not, in both layout modes. Without `ctrl` → row 3 alone.
- Cells' graph reads in parts over the session, MCP, the command line and a kept check. Each part is within 10,000
  bytes, and so within the answer bound. Part 1 alone carries the states, the parts together equal the whole graph,
  and a small screen still answers whole.
- Cells' editor stops its Enter: a kept check of click-then-Enter leaves `cells-grid` `keydown` unexercised. The
  witness runs in process (`handler_coverage`) and through `escher-session coverage cells`; at HEAD it reads
  exercised.
- No part number or count reaches a span or log, and the masks hold. `capabilities.md` drops the two old limits and
  states the narrower coverage one.

## Touchpoints
22 files, none new:
- escher-driver: schema, command, execute, json, describe, coverage's doc, and their tests;
- dioxus-native-dom: reached.rs and the bridge in `dioxus_document.rs`;
- four stand checks (`stand_graph`, `stand_act_select`, `stand_handler_coverage`, `stand_mcp_tools`) and seven_guis'
  `cli_commands`;
- `docs/capabilities.md`, and its pin only where a count moves.

## Test commands
11 entries:
- unit: the two library crates, and the capability pin;
- blitz-tests: this chunk's targets, and their companions;
- seven_guis' process checks;
- `fast`, `doc` and `stand-repeat`;
- the operator's hygiene, push and CI read. The diff is OS-sensitive: `--ctrl` resolves through the
  `cfg(target_os = "macos")` `ACTION_MODIFIER`, witnessed on `matrix_test`'s macOS and windows legs.

Baselines on the untouched tree, all green:
- the units: 247 passed;
- this chunk's blitz-tests targets: 38 passed;
- their companions: 50 passed;
- seven_guis: 42 passed;
- the capability pin: OK;
- `doc`: no warning (a cached build);
- `stand-repeat`: 3 of 3 rounds green, in 909 s.

Five one-shot `gate.py control` runs at implement, one per new check.

Mechanical checks pass on rev 3:
- the fence parses;
- planlint reports 0 hits;
- research's lists parse (22 items);
- the snapshot citations hold (`I1`, `I2`).

## How P4 read its inputs
- The rows of architecture, security-plan, test-plan and obs-plan were read whole.
- The rows of a11y-plan, design-system and layout-templates, the standing lists other than tests', and the seven
  history files were read by search for the chunk's terms. The rows total 245,091 B, under the whole-read bound; the
  search kept the window's room for the plan.
- No row the plan cites stands outside the rows files.

## Concretization preview — v040-05
**Now:** «Each of the three debts reaches the founder as a fork; the graph is read in parts as the founder ruled,
every part within the answer bound; each debt fixed is proven on the stand through the driver's carriers, and each one
kept stands as a residual with his reason.»

**Proposed:** «The three debts reached the founder as one forks file
(`.andromeda/runs/2026-11-20T18-56-36-phase/founder-forks.md`), and he fixed all three (`inputs#I2`: F1 A · F2 A ·
F3 A). Proven on the stand in both layout modes:
- `press --ctrl` builds a non-adjacent `multiple` set through `stand_act_select`
  `a_press_with_ctrl_builds_a_set_of_rows_that_are_not_adjacent`, in process and over MCP (`stand_mcp_tools`);
- `graph --part N` reads Cells in parts, each part within `SNAPSHOT_TEXT_BUDGET` and so within the 1,048,576-byte
  answer bound, the parts together the whole graph. Shown by `stand_graph`
  `cells_reads_in_parts_each_within_the_budget_and_together_the_whole_graph` in process, over MCP
  (`stand_mcp_tools`), through the binary (`cli_commands`) and in a kept check (`stand_replay`);
- a stopped dispatch no longer reads an ancestor's handler exercised, by `stand_handler_coverage`
  `an_enter_the_cell_editor_stops_leaves_the_grids_keydown_unexercised` in process and `cli_commands`
  `an_enter_the_cell_editor_stops_leaves_the_grids_keydown_unexercised_through_the_coverage_role` through the coverage
  role.
No debt was kept, so no residual stands for one.»

This makes the existing outcome concrete; it weakens nothing. The "each one kept" arm is vacuous: he kept none.

## Expected amendments (wrap)
- The rows naming the old limits move: press without Ctrl, an unpaged graph, a reached record written before the
  dispatch, and a bubbling dispatch counting every ancestor. They are listed by handle in the plan's Implementation
  notes, across architecture, security-plan, test-plan and obs-plan.
- `--ctrl` on `press` and `--part` on `graph` are boundary widenings (`rul-izni57qv`), answered by the founder's own
  word, relayed by the overseer (`inputs#I2`).
- The residuals `res-hfvjj77l`, `res-7cc5bi4k` and `res-4dr3wt62` (absorbed → v040-05) close with the chunk's
  evidence, through the cap's notes.

## Beyond the plan
none

Apply? (yes / review / cancel)
