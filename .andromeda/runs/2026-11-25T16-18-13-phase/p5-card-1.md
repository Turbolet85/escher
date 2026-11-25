# P5 review — 2026-11-25-circle-drawer-virtual-nodes: «Circle Drawer's circles as virtual nodes»

**The plan:** `doc:2026-11-25-circle-drawer-virtual-nodes/plan` rev 4, rendered to open in an editor at
`.andromeda/cache/docs/2026-11-25-circle-drawer-virtual-nodes/plan.md`. No plan stands in the chunk's folder. Writing
in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff
--root . 2026-11-25-circle-drawer-virtual-nodes` is a diff of the chunk's records.

The plan is on the answers F1 A · F2 B · F3 A · F4 A · F5 B · F6 A · F7 A · F8 (role `Image`) (inputs#I2; F5 B on the
founder's own F13 A, inputs#I3) and F9 B (inputs#I4).

## Goal
Circle Drawer declares each circle as a virtual node `circle-canvas///c{id}`:
- role `Image`, named `Circle {id}, {d} px`;
- its region the bounding square cut to the 700 × 400 content box;
- its key from an id the app gives each circle, carried as `data-circles`' fourth field.

The circles then read in the tree, snapshot, diff, graph, `actions`, `layout` and exploration.

The `canvas` full answer tags every line ` n={k}` or ` n=-`. A command is grouped under the region holding its centre
whose own centre is nearest it, ties going to the first declared. On the fixture this rule groups 12 of 12 and 100 of
100 circles under their own node; the last-declared rule gets 69 of 100. The index is positional, a stated limit.

Coverage lists `click` under each virtual node's id. blitz-dom re-exports the role type behind `custom-widget`, an
escher addition to its public surface. There is no manifest or lock move.

## Top acceptance criteria
- **v040-10 (claimed):** summary and full answers, equal across two reads and on linux, windows and macOS (the
  committed table re-measured). A click's diff only adds lines, each ` n={its index}`. A resize names its circle by the
  shared ` n=`. The grouping reads 12/12 and 100/100. The lint is clean on the stand. The dump size is stated, and the
  crossing holds (numbers only).
- The circles as nodes in the tree and snapshot. Key stability: a removed circle reads `stale`, an unknown id
  `not-found`.
- Graph, actions, layout and exploration read the id, never the name. Coverage lists a node's click (CARRY 6).
- The limits are witnessed on the stand (each in `stand_circle_nodes`):
  - a click on a node adds a circle (F1 A);
  - no circle covers another (F4 A, CARRY 3);
  - the label region is the content box (F3 A, CARRY 2);
  - the lint holds no virtual node (F6 A, CARRY 7);
  - the positional index.
- **CARRY 5:** the final HEAD's CI windows job log holds `unused import: `snapshot`` 0 times.
- Controls C1–C6 each enter with their red reading. The probe holds every manifest and the lock unchanged.

## Touchpoints
- **New:** `tests/blitz-tests/tests/stand_circle_nodes.rs`.
- **Modified (13):**
  - `packages/blitz-dom/src/lib.rs`
  - `examples/seven_guis/src/tasks/circle_drawer.rs`
  - `packages/escher-driver/src/{canvas,json/canvas,json_tests,canvas_tests,coverage}.rs`
  - `tests/blitz-tests/tests/virtual_nodes_in_every_read.rs`
  - `tests/blitz-tests/tests/{session_common,common}/mod.rs`
  - `tests/blitz-tests/tests/stand_circle_drawer_canvas.rs`
  - `tests/blitz-tests/tests/stand_canvas_commands.rs`
  - `docs/capabilities.md`
- **Research:** the lists follow the plan (+1 −20 from the take-up's rev 1, the files of the options not taken),
  rev 3.

## Test commands (16 entries)
- lock probe
- `build`
- unit tests: escher-driver, seven_guis, dioxus-native-dom
- `stand_circle_nodes`
- the three virtual-node targets
- seventeen stand companions
- `a11y`, `stand-repeat`, `fast`, `doc`
- the operator entries: hygiene, push, `ci.py` read, the windows job-log read

**Baselines:**
- green: probe (controlled in a scratch clone, red on a manifest line), seven_guis lib, virtual-node targets,
  companions (166.77 s), `stand-repeat` (1059.61 s);
- red: `stand_circle_nodes` (the target does not exist yet).

**Rows read by search:** the union totals 321,996 B, past 250,000. Each of the 18 cited handles was checked present in
its rows file.

## Found at P5
- **The local windows check (`testing.md` 2026-10-19's recipe) cannot build on this host.** aws-lc-sys's build script
  needs `x86_64-w64-mingw32-gcc`, which is absent, and cargo exits 101 before blitz-tests compiles. It is replaced by an
  operator read of the CI windows job's log, controlled on two recorded runs: CI#45619756069 reads 1 (red), and
  CI#45580739068 reads 0 (green). Record: run dir `control-probes.md`.
- **CARRY 5's «read 3 times» counted three distinct warnings.** `snapshot` is one of them. The other two are on the
  tree, outside this scope:
  - `tests/blitz-tests/tests/stand_act_error_text.rs:27` (`std::time::Duration`);
  - `examples/seven_guis/tests/host_binary.rs:10` (`Command`, `Stdio`).
  
  Not folded: folding them is your word, and costs the row read's re-run.

## Concretization preview — v040-10
- **Current:** «On the stand, the verb answers Circle Drawer's draw commands at a summary and a full level, the same
  answer on two runs and on every CI host, a diff naming exactly what one action changed, the commands grouped under the
  virtual node that covers them; a canvas lint names a label overflowing its region on a planted fixture and reads clean
  on the stand; a dump's size on the stand is stated; the crossing the founder ruled holds on every carrier.»
- **Proposed concrete form:** On the stand, in both layout modes:
  - `canvas` answers Circle Drawer's commands at a summary and a full level, two reads byte-equal, equal to
    `stand_canvas_commands`' committed table on linux, windows and macOS (the CI read of the final HEAD).
  - A click's line diff adds only the new circle's lines, each ending ` n={its index}`.
  - A resize's diff holds only lines carrying ` n=1`, naming its circle.
  - Every full-answer line ends ` n={k}` or ` n=-`, `k` being the 1-based declaration index of the taken node whose
    region holds the command's centre and lies nearest it, ties to the first declared. 12 of 12 and 100 of 100 fixture
    circles read their own index (`stand_circle_nodes`).
  - `stand_canvas_lint` names a planted label past its region and reads no finding on the stand at boot, after a click
    and at 12 and 100 circles.
  - The full dump's size at 1, 12 and 100 circles is stated in `docs/capabilities.md`.
  - Every carrier's canvas answer holds the tag's number or `-` and no virtual id, name, glyph id or text
    (`stand_canvas_carriers`, `stand_masks_on_new_answers`).
  
  Nothing is weakened: «grouped under the virtual node that covers them» is read as the rule above, and «a diff naming
  exactly what one action changed» as the two diffs above.
- **v040-11:** not claimed. Its keyboard reach, its click clause and its `covered` / `off-screen` / `disabled` stand
  readings are «Canvas keyboard path»'s (inputs#I2). A dated note is written at the claim step.

## Beyond the plan
1. A `CARRY:` onto «Canvas keyboard path»: v040-11's click clause («a click on one acts on its circle and its diff
   names it») with F1's options and their costs as measured in `evidence/founder-forks.md`:
   - B1: a declared `selected` field, the founder's;
   - B2: the state carried in the name, not taken (an agent reads a state from a state field).
   
   F4 B (a sibling virtual node read as covering) rides it (inputs#I2).
2. CARRY 4 (`cry-kou37dij`) moves to «Cold-agent live run» with its corrected premise: Circle Drawer is served by no
   `escher-session` role, so the binary reading needs it served first, the founder's F7 B/C (inputs#I2).
3. The two out-of-scope windows warnings above — `stand_act_error_text.rs:27` and `seven_guis/tests/host_binary.rs:10`
   — pinned by the wrap as a carry-forward on «Canvas keyboard path», the earliest markerless entry.
4. `testing.md` 2026-10-19's local windows check unbuildable on this host (aws-lc-sys, no mingw C compiler). It goes to
   the wrap's curation as a correction of that rule's recipe.

Apply? (yes / review / cancel)
