# P5 review — 2026-11-23-circle-drawer-on-a-canvas («Circle Drawer on a canvas», v040-09)

The plan's render is `.andromeda/cache/docs/2026-11-23-circle-drawer-on-a-canvas/plan.md` (rev 3); open it in an editor.
No plan stands in the chunk's folder, and a change in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-circle-drawer-on-a-canvas` is a
diff of the chunk's records.

## Goal
Circle Drawer's drawing area becomes a real canvas: an `<object>` whose widget paints the app's circles and each
circle's diameter from Rust. The diameter text is shaped from the document's own fonts (on the stand, the bundled
DejaVu Sans). It is measured in process in both layout modes, and the canvas's element shot joins the cross-OS digest
table.

The forks stand as answered (`inputs#I2`), with the resume yours (`inputs#I3`):
- **F2 A and F1 A are the founder's rulings.** Under F2 A the engine still makes the click and double click on a
  custom-widget target, after the widget has the event. Under F1 A the sample is not served.
- **F3 A, F4 A, F5 A and `data-circles` are the escher overseer's word, a delegate's,** left standing. They are the
  diameters as text, centre-only clicks, `role="group"` named «Drawing area», and one attribute carrying the list.

## Top acceptance criteria
- `stand_canvas_click`, both modes:
  - one driver click on a canvas reads `clicks 1 dbl 0`, the widget hearing `[down, up]`;
  - two clicks with no `advance` read `clicks 2 dbl 1`;
  - after `advance` 600 a third reads `clicks 3 dbl 1`.
- `stand_circle_drawer_canvas`, both modes:
  - at boot the canvas reads role `Group`, name «Drawing area», `clickable`; lint 0; `unkeyed_actionable` empty;
  - one centre click paints the ring and diameter glyphs (measured literals);
  - undo and redo give shots byte-equal to boot and to after-click;
  - two centre clicks open the dialog, the slider resizes, and Close closes it.
- `stand_canvas_cross_os` reads 6 passed: the sample's canvas element shot after one click is held to a 2-row `SAMPLE`
  table, and the CI run on HEAD reads green on its linux, windows and macOS test legs.
- `cargo test -p seven_guis --lib`: malformed `data-circles` reads as no circle, no panic; the stand boots Circle
  Drawer.
- `fast`, `build` (wasm32 with the widget's crates), `audit` and `stand-repeat` green; the lock probe reads `3+0`.

## Touchpoints
- New: `tests/blitz-tests/tests/stand_canvas_click.rs`, `tests/blitz-tests/tests/stand_circle_drawer_canvas.rs`.
- Engine (F2 A): `packages/blitz-dom/src/events/pointer.rs` (`widget_click_synthesis`, private, behind
  `custom-widget`) and one call in `packages/blitz-dom/src/events/mod.rs`'s widget arm.
- Sample: `examples/seven_guis/src/tasks/circle_drawer.rs`, `examples/seven_guis/src/stand.rs` (`boot_circle_drawer`),
  `examples/seven_guis/Cargo.toml` (`anyrender`, `parley`, `peniko`), `Cargo.lock` (three lines, no package).
- Checks and docs: `stand_canvas_cross_os.rs`, `stand_non_lean_tasks.rs`, `docs/capabilities.md` (the sample, the
  centre-click stated limit).

## Test commands
16 entries: 13 the implement runs, and the operator pass's hygiene, push and `ci.py` read.
- Baselines on the untouched tree:
  - entries 1, 3, 6, 7, 8, 11, 12 green (blitz-tests 964 passed in 326 s);
  - entries 2 and 4 red (no target yet), 5 red (5 passed of 6), 9 red (empty lock diff);
  - entries 10 and 13 seen green at the last wrap.
- The `doc` and `a11y` legs are not listed: no public item or doc comment, and the new checks read the snapshot, not
  the AccessKit tree, so no a11y leg count moves.

## Each new control with its red reading
- **Engine check, red first (step 1):** `clicks 0 dbl 0` with the widget's events heard, as the P3 probe measured
  (`evidence/probe-cd-click.log`).
- **Sample check, red first (step 6):** with the `data-circles` write held back, the after-click canvas shot reads
  byte-equal to the boot shot.
- **Five one-shot `gate.py` controls (step 10),** each with its red reading:
  - `widget-click`: the call removed → `clicks 0 dbl 0`;
  - `data-circles`: the attribute line removed → the shot equals the boot shot;
  - `diameter-text`: `draw_glyphs` removed → no dark pixel in the label box;
  - `canvas-role`: `role` and `aria-label` removed → role `Unknown`, name empty;
  - `canvas-digest`: stroke `#3a5ce5` → `#3a5ce4` → the digest differs from `SAMPLE`.
- **The parse's unit test:** each malformed input is a case that panics or reads a circle if the parse is made
  infallible.
- **The lock probe:** `0+0` (empty) on the untouched tree.

## Concretization preview — matrix#v040-09 (claimed at this chunk's word)
- Current acceptance: «On the stand, a shot of Circle Drawer shows the circles its app painted from Rust and canvas
  text drawn from the bundled fonts, in both layout modes; the canvas shot's bytes are measured on CI's Linux, macOS
  and Windows hosts and held equal or within the tolerance the founder set.»
- Proposed concrete form: «On the stand, Circle Drawer's canvas element shot after one centre click holds the circle
  ring and its diameter glyphs, painted by the app's widget from Rust with text shaped from the document's own font
  context (the bundled DejaVu Sans), in both layout modes (`stand_circle_drawer_canvas`). Its answer digest is held
  byte-equal to one committed `SAMPLE` table in both layout modes by `stand_canvas_cross_os`, read green on CI's linux
  x86_64, windows x86_64 and macOS aarch64 test legs in the run the operator's `ci.py` entry reads (byte equality, the
  founder's F1 of 2026-11-23-canvas-shot-across-hosts).»
- The outcome is not weakened: «circles and text from the bundled fonts, both modes, bytes equal on three hosts» each
  stand. It is concrete in which shot (the canvas element after one centre click) and which checks. The whole screen
  holds page text and stays outside the byte check, as the founder's rule requires.

## Reads behind it
- The row read was re-run whole on scope rev 4 (`--repin`): 729 of 3583 rows picked, no master at `picked 0`, one
  picker draw re-spawned once (`[EVERY_ROW]`) and passed.
- The rows files were read whole, 216 KB in all, under the 250 KB line, with the seven standing lists and the seven
  history files.
- CI on HEAD `a199c213e`: CI#45226928137 green 17/17, on attempt 2.
- The stale probe worktree under the cleared temp dir was pruned (`git worktree prune`).

## Expected amendments at the wrap (named in the plan)
- The F2 A row in architecture and security-plan carries your founder's word, relayed verbatim (`inputs#I2`).
- The boot contract gains Circle Drawer.
- The lint-clean, `circle-canvas` and canvas-digest rows gain the sample; F5 A is a delegate's word.
- obs-plan gains its silence row.

## Beyond the plan
none

Apply? (yes / review / cancel)
