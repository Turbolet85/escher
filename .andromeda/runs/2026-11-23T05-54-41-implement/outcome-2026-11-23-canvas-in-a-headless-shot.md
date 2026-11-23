# Implement outcome — 2026-11-23-canvas-in-a-headless-shot

**Outcome:** green (gates 1–10 green, 1 iteration) · smoke skipped — no boot-path or UI-surface change, no `smoke` / `self-verify` entry.
**Next:** the OPERATOR PASS — operator entries 11 (`gate.py hygiene`), 12 (the push of `build/escher-0.4.0`), 13 (`ci.py conclusion --sha HEAD --wait 2700`); then `/andromeda-wrap-session`.

## Files changed
- `tests/blitz-tests/tests/stand_canvas_shot.rs` — new (165 lines): the stand check, widget `RedFill`, component `PaintedCanvas`, tests `a_widget_painted_from_rust_shows_in_a_harness_shot` and `a_driver_shot_carries_the_widget_whole_and_cropped`.
- `packages/blitz-test-harness/Cargo.toml` — `blitz-paint = { workspace = true, default-features = false, features = ["custom-widget"] }` with a one-line rationale comment (lines 18-19).
- `tests/blitz-tests/Cargo.toml` — `peniko = { workspace = true }` under «Other dependencies» (line 40).
- `Cargo.lock` — `"peniko"` added to the `blitz-tests` entry (line 1153), written by `cargo tree --offline -p blitz-tests --depth 0` (exit 0); `git diff --numstat` reads `1	0`.

## Red readings
- Step 3, before step 4 (gate 2, firing 2): `incremental=false: row 58 reads the widget's paint inside the canvas, read [255, 255, 255, 255]`; both cases FAILED — `.andromeda/cache/chunks/2026-11-23-canvas-in-a-headless-shot/evidence/red-before-fix.log` (copied from `$TMPDIR/andromeda-gate/2026-11-23-canvas-in-a-headless-shot/implement-2026-11-23T05-54-41/2.2.log`, sha8 f92886af equal; then `gate.py respell` ×1 in-root path, f92886af→1508057e). Firing 1 was red the same way, its message printing no pixel (see Deviations).
- Step 6, the one-shot control (`.andromeda/runs/2026-11-23T05-54-41-implement/control-1.toml`, `--timeout 1200`):
  `control harness-custom-widget · entry 2 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 43.74s · 4.68s → 2.without-harness-custom-widget.log`
  `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
  The red case's line: `incremental=false: row 58 reads the widget's paint inside the canvas, read [255, 255, 255, 255]` — `evidence/control-1.txt`.

## Gates (the full block, one firing)
- `cargo test -p blitz-test-harness --locked --no-fail-fast` — green
- `cargo test -p blitz-tests --locked --no-fail-fast --test stand_canvas_shot` — green (firing 3)
- `cargo test -p blitz-tests --locked --no-fail-fast` — green, 327.04 s
- `cargo test -p seven_guis --locked --no-fail-fast` — green
- the `custom-widget` probe over three per-package selections — green, last line 3
- the platform-adapter probe — green, exit 1, last line 0
- `git diff --numstat ac8e6d58… -- Cargo.lock` — green, last line `1+0`
- `bash .github/scripts/ci-leg.sh fast` — green, 606.31 s
- `bash .github/scripts/ci-leg.sh audit` — green
- `bash .github/scripts/ci-leg.sh stand-repeat` — green, 898.45 s
- the three `leg = 'operator'` entries — not run (the operator's)
Summary: `entries 13 · green 10 · red 0 · recorded 0 · timeout 0 · not-run 3`. No deferral; no `gate.py guard` entry.

## Ledger
No capability row names this chunk (`select * from coverage where chunk = '2026-11-23-canvas-in-a-headless-shot'`: 0 rows); no ref written, as the plan states (`v040-09` stays pooled).

## Deviations
- The harness case's inside-pixel failure message also prints the pixel it read (`read {inside:?}`), and the check's `//!` doc says a failure message carries «a layout mode, a row and the pixel it read only» where step 1 said «a layout mode and a row only». Why: step 3 asks for the red log to show the inside pixel white; a pixel's RGBA is a measured number, no page text.
- Scope: `scope: clean — changed 4 · listed 4 · recorded 0` (`gate.py scope`); scope record: none.

## Surfaced / stuck
none

## Hygiene — the land held for the operator's word
`hygiene: refused 4 files — P1 4 · P2 0 · P3 0 · read 199 (runs 184 · evidence 9 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- `evidence/p3-probe-feature-on.log:1 ×8 · tmp,in-root`
- `evidence/p3-probe-per-package.log:241 ×16 · tmp,in-root`
- `evidence/p3-probe-workspace.log:242 ×16 · tmp,in-root`
- `evidence/p4-probe2-feature-on.log:1 ×2 · tmp,in-root`
The phase run's probe captures (cargo output), cited by the plan and research; their `tmp` hits are outside the root, so `respell` cannot clear them. Already in the store (`rec.py files` restored them at this run's start). `rec.py land` not fired — the operator's word decides: move to the scratchpad with their figures kept in a record, or keep.
- `evidence/red-before-fix.log` (this run's): respelled ×1, cleared.
- The operator's word: «Neither: keep the four logs in evidence, and make them meet the hygiene rule. In each, replace every temp path outside the repository by the placeholder <scratch> …, change no other byte, and add one first line saying the temp paths were respelled to <scratch> on 2026-11-23 for hygiene, the readings unchanged.» Done by a scratch script (the prefix `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/f1c01967-…/scratchpad` → `<scratch>`: ×7, ×15, ×15, ×1; a reverse check restored each original byte for byte) — hygiene then named one `in-root` line per file.
- The operator's word: «Yes: run gate.py respell --file on each of the four logs, re-run hygiene, land.» — respelled ×1 each (94e0882e→5fa58698, e2bcd4ff→3e458a3f, 0cee1e07→687065a9, fc39e9f8→a990a755).
- `hygiene: clean — read 199 (runs 184 · evidence 9 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1` — land fired.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (gates 1–10, the control's two firings) | this run (through `gate.py run` / `gate.py control`) | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --comm seven_guis_nati`: `census: none running` |

## Decisions & corrections
- The operator's hold, mid-run: «A hold for the host: let the gate block in flight finish … When it ends, start NO further cargo run or build of any kind … Wait for my «go» before any cargo run.» — honoured: no cargo run between the block's end and the word.
- The operator's word: «go — the host is free. Run step 6's control now and finish the implement.» — the control fired after it.
- Guard refusal: the Bash guard refused `cd` into the skill's references dir (cwd move); re-issued once as Read calls.
