# Implement outcome — 2026-11-23-circle-drawer-on-a-canvas

**Outcome:** green (gates 1–8 and 10–13 green; gate 9 red by the plan's own wording, below) · smoke skipped — no
smoke or self-verify entry · run `.andromeda/runs/2026-11-23T21-22-31-implement`

## Files changed
- new: `tests/blitz-tests/tests/stand_canvas_click.rs`, `tests/blitz-tests/tests/stand_circle_drawer_canvas.rs`
- modified: `packages/blitz-dom/src/events/pointer.rs` (`widget_click_synthesis`), `packages/blitz-dom/src/events/mod.rs`
  (its one call in the widget arm), `examples/seven_guis/src/tasks/circle_drawer.rs` (`CircleCanvas`, the `<object>`
  canvas, `malformed_circle_text_reads_as_no_circle`), `examples/seven_guis/Cargo.toml`, `Cargo.lock`,
  `examples/seven_guis/src/stand.rs` (`boot_circle_drawer`, `circle_drawer_boots_in_the_shell`),
  `tests/blitz-tests/tests/stand_canvas_cross_os.rs` (`SAMPLE`, `circle_drawers_canvas_reads_its_digest`),
  `tests/blitz-tests/tests/stand_non_lean_tasks.rs`, `docs/capabilities.md`

## Deviations from the plan, with justifications
1. **A fourth seven_guis dependency, `blitz-dom` (step 4; gate 9 reads `4+0` for `3+0`).** `Widget::paint` names
   `blitz_dom::node::ComputedStyles` (`style::properties::ComputedValues`), which dioxus-native does not re-export, so
   the widget cannot be written against anyrender, parley and peniko alone. `blitz-dom = { workspace = true }` joins
   the target-free table; the lock's `seven_guis` entry gains four lines, no package added (`git diff -- Cargo.lock`:
   `+anyrender`, `+blitz-dom`, `+parley`, `+peniko`). Both files are on research's list, so no scope-record line;
   gate 9's red is a red by the plan's own wording, the plan unedited. `audit` and `build` legs read green on it.
2. **The centre is `348,198`, not the predicted `350,200`.** `.cd-canvas` is border-box sized on the stand (700 × 400
   border box, 696 × 396 content box). The app now reads a click against the canvas's content box
   (`CANVAS_BORDER`, 2 px, subtracted in `onclick` and `ondoubleclick`), so the widget paints the circle under the
   pointer; the circle's centre in the element shot is (350, 200). Pinned as measured.
3. **Step order.** Step 5's `boot_circle_drawer` was written before step 6's check, so step 6's first red reading
   («does not build») was not taken; the second, with the `data-circles` line held back (bytes saved, restored,
   sha256 `a570ff52…` equal), read red: 1 passed, 3 failed (`evidence/red-circle-drawer.log`). The click test
   stops at its `data-circles` assertion, ahead of the shot comparison the plan named; the undo test reads the
   shot equal to the boot shot there.
4. **The engine check's red log was taken with `assert_eq!`**, which prints the readings (`clicks 0 dbl 0`); the
   assertions were then changed to `assert!` so a failure message names a layout mode and an index only
   (`evidence/red-canvas-click.log`). The control `widget-click` re-read the red on the final file.
5. **The widget clips its scene to its box** (`push_clip_layer` over the paint size), since a widget's scene is
   painted unclipped; the old `div` canvas clipped by `overflow: hidden`.
- scope record: none — `gate.py scope` clean, changed 11 · listed 11 · recorded 0.

## Gates (gate tool, `gate-2026-11-23-circle-drawer-on-a-canvas.json`)
- `cargo test -p blitz-dom --locked --no-fail-fast` — green
- `… --test stand_canvas_click` — red before step 2 (`clicks 0 dbl 0`, the widget `[down, up]`), green after
- `cargo test -p seven_guis --locked --no-fail-fast --lib` — green, 10 passed
- `… --test stand_circle_drawer_canvas` — red with `data-circles` held back, green, 4 passed
- `… --test stand_canvas_cross_os` — green, 6 passed
- the gate-6 readers — red once on `stand_non_lean_tasks`' `CLICKABLE` Circle Drawer row (predicted 1 → 0, measured
  0), re-pinned; `MEASURED` Circle Drawer predicted 1 → 0, measured 0; green
- `cargo test -p blitz-tests --locked --no-fail-fast` — green, 327 s
- `cargo test -p seven_guis --locked --no-fail-fast` — green
- `git diff --numstat a199c213e… -- Cargo.lock | awk …` — red · `last line 3+0` (last `4+0`): by the plan's own wording, deviation 1
- `bash .github/scripts/ci-leg.sh fast` — green, 631 s
- `bash .github/scripts/ci-leg.sh build` — green
- `bash .github/scripts/ci-leg.sh audit` — green, `advisories ok, licenses ok`
- `bash .github/scripts/ci-leg.sh stand-repeat` — green, `stand-repeat: 3 of 3 rounds green`
- operator entries (hygiene, push, `ci.py conclusion`) — not fired: the operator pass's
- controls (`control-1.toml`, `evidence/control-1.txt`): `control: 5 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
- matrix: v040-09 `planned → implemented`, ref `stand_circle_drawer_canvas.rs::a_click_paints_a_circle_and_its_diameter_on_the_canvas; stand_canvas_cross_os.rs::circle_drawers_canvas_reads_its_digest`
  — its acceptance's three-host CI read is still owed by the operator pass

## Surfaced / stuck
- none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / escher-session / seven_guis_native | this run's gates and controls | terminated — `gate.py census`: `census: none running` |

## Decisions & corrections
- The Bash guard refused a `cd` into the skill's references dir twice (re-issued with absolute paths) and a `cat`
  heredoc writing a scratch script (re-done through the Edit tool).

## Next
The OPERATOR PASS: the plan's operator entries 14 (hygiene), 15 (push) and 16 (`ci.py conclusion`, the sample
digest on CI's linux, windows and macOS test legs), then `/andromeda-wrap-session`.
