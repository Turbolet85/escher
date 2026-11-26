# P5 review — 2026-11-26-canvas-keyboard-path «Canvas keyboard path»

The plan, rev 5, is rendered to `.andromeda/cache/docs/2026-11-26-canvas-keyboard-path/plan.md` for reading in an editor.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-26-canvas-keyboard-path` is a
  diff of the chunk's records.

## Goal
Circle Drawer's canvas becomes one Tab stop (F1 A): the arrows move the selected circle in declared order, clamped, and
Enter opens its diameter dialog (F2 A). The engine, the tree and the snapshot agree on `circle-canvas` at every step.

A click inside a circle selects it and adds none (the founder's B1). `VirtualNodeDeclaration.selected` is read as the
snapshot's `selected`, and a click focuses the canvas (side question B).

On the stand a circle reads:
- `covered` when a later circle's region holds its centre (F3 B);
- `disabled` while the dialog adjusts another circle (F4 B);
- `stale` when removed.

The canvas keeps 700 × 400 with the dialog open (the app defect you named), and the two windows warnings are fixed.

## Measured at P4 (probe worktree `escher-probe-ckp`, left in place)
- **F5 B: no.** With the canvas held at 400 px, `scroll circle-close` moves the page 39 px. A circle at the canvas's
  top reads in view, and its `click` is accepted (`evidence/probe-offscreen.md`). `off-screen` is not built.
- **The click's focus** lands inside its own settled step, in both layout modes. No `advance` is added
  (`evidence/probe-focus.md`).

## Top acceptance criteria
- One Tab stop: Circle Drawer's walk reads `CIRCLE_DRAWER_TAB_ORDER` forward and back, the three focus readings
  agreeing on `circle-canvas`, and no circle is ever the tree's focus.
- The arrows select in declared order, clamped; Enter opens the dialog on the selected circle; with the dialog open a
  press changes nothing.
- `click` on a circle's node selects it, adds none, and its diff changes that node with `differs` holding `selected`.
- A click focuses the canvas in its own step. Then `press enter` opens the dialog on that circle. The focus events are
  `blur`, `focusout`, `focus`, `focusin`, in one settled step.
- `covered` by a later circle, the bounding-square limit witnessed; `disabled` while the dialog adjusts another circle,
  read before `covered`.
- The declared `selected` reads `selected` / `unselected` / no word in the tree, the snapshot and its text.
- C1–C6 each read their named case red on the mutated tree.
- CI on the final HEAD is green, and its windows job log holds neither warning (`last line warnings 0`).

## Touchpoints
- **blitz-dom:** `custom_widget.rs`.
- **dioxus-native-dom:** `dioxus_document.rs`, `snapshot.rs`, `virtual_node.rs`.
- **escher-driver:** `execute.rs`, `refusal.rs`.
- **seven_guis:** `circle_drawer.rs`, `tests/host_binary.rs`.
- **blitz-tests:** `fixtures/mod.rs`, `common/mod.rs`, `virtual_nodes_declared.rs`, `virtual_nodes_as_elements.rs`,
  `stand_circle_nodes.rs`, `stand_keyboard.rs`, `stand_handler_coverage.rs`, `stand_graph.rs`, `stand_non_lean_tasks.rs`,
  `stand_canvas_commands.rs`, `stand_canvas_cross_os.rs`, `stand_act_error_text.rs`.
- **docs:** `docs/capabilities.md`.

No new file, no manifest or lock line, no new verb, argument, key name, cause or carrier.

## Test commands (18 entries)
- the lock probe;
- the `build` leg;
- the unit tests of escher-driver, seven_guis and dioxus-native-dom;
- `stand_circle_nodes`; the three `virtual_nodes_*` targets; `stand_keyboard` with `stand_accessibility_ids`; sixteen
  stand companions; `host_binary`;
- the `a11y`, `stand-repeat`, `fast` and `doc` legs;
- the operator's hygiene, push, CI read and windows job-log read.

Baselines read green on the untouched tree: entries 1, 4, 6, 7, 8, 9 and 10. The windows-log read's known positives:
CI#45700323454 reads `warnings 2` at exit 1, and an empty read `log unread` at exit 2. HEAD `255a586fb`'s CI#45767502508
reads green, 17/17, wall 2526 s.

## Capabilities
- **v040-11 — not claimed.** Under your claims rule every clause must read on the stand, and `off-screen` cannot. A dated
  notes line records the clauses this chunk proves on the stand (keyboard reach, the click naming its circle, `covered`,
  `disabled`, `removed`, handler coverage) and the one it does not. No concretization preview: nothing is claimed.

## Rows the wrap meets
- F3 B's four `covered` rows and the rows resting on F1 A, F2 A, side B and F4 B land PROVISIONAL: your answers are a
  delegate's word (inputs#I4).
- This wrap closes Epoch 4 and lists every standing PROVISIONAL mark for the founder's batch word (`rul-3ndrhilc`).

## Beyond the plan
1. **An owner for v040-11's `off-screen` stand clause:** «Pan and zoom». A panned canvas puts a virtual node out of
   view on the stand, with no layout made for it.
2. **A residual, opened by the wrap with its incubator line:** under F1 A an assistive technology learns the current
   circle from `selected` alone, beside the CRUD list's recorded limit (a11y-plan · decision/active-descendant-unread),
   as inputs#I4's F1 A asks.

Apply? (yes / review / cancel)
