# P5 review — 2026-11-24-canvas-lint «Canvas lint»

Plan `doc:2026-11-24-canvas-lint/plan` rev 3. Open it in an editor at `.andromeda/cache/docs/2026-11-24-canvas-lint/plan.md`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-canvas-lint` diffs the
  chunk's records.

## Goal
The visual lint reads inside a canvas. A label a widget paints from Rust whose ink runs past its region is named by
`Session::lint` and the `lint` verb. It is built on the forks as answered (inputs#I2):
- **F1 A, the founder's own word.** The finding reuses `truncated` inside the `lint` answer: `text` is the label's
  ink box, `visible` its part in the region. No new word or key; the defect set stays six.
- **F2 A.** The region is the content box, cut by every pushed clip, layer or mask path's bounding box and by the
  element's clip region.
- **F3 B.** The extent is the ink box: each glyph's outline bounds through skrifa from the run's own font.
  escher-driver takes `skrifa = { workspace = true }`.
- **F4 A.** The label overflow alone.
- **F5 A.** Circle Drawer reads clean at boot, after a click, and at 12 and 100 circles, read in the engine.

F2–F5 are the escher overseer's delegate answers, so their rows land PROVISIONAL.

## Top acceptance criteria
- The planted page, in both layout modes, reads a literal table of 5 findings:
  - `fx-past-edge`, `fx-past-clip` and `fx-ascent` read `truncated`;
  - `fx-in-clipped-box` reads `clipped`, then `truncated`;
  - `fx-inside` and `fx-after-pop` read nothing.
  - `fx-ascent` is the ink-box case: every glyph origin lies inside the canvas and only the ascent passes the edge.
- Circle Drawer reads no finding at its four states. Its glyph-run count is pinned per state first, so the clean
  reading is never vacuous.
- The `lint` verb's answer carries the findings keyed `id`, `defect`, `bounds`, `text`, `visible` alone. It holds no
  glyph id, position or label text, and `Defect::ALL` is unchanged.
- The lock moves by exactly one line. A `--locked` workspace build and the audit read green.
- `evidence/label-readings.md` holds the engine's per-state readings in place of the research model's.
- The `fast`, `doc` and `stand-repeat` legs read green, and CI on the final HEAD reads `verdict: green`.

## The lock step — its whole diff, measured at P5 in a scratch clone (`runs/2026-11-24T21-28-19-phase/control-lock.md`)
```
diff --git a/Cargo.lock b/Cargo.lock
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -3216,2 +3216,3 @@ dependencies = [
  "serde_json",
+ "skrifa",
  "tracing",
```
- `--numstat` reads `1 0 Cargo.lock`. skrifa 0.49.0 is already one package in the lock, so no package, version or
  checksum moves.
- Gate entries 1–2 hold it, controlled both ways:
  - the untouched tree reads red;
  - the clone with the step applied reads green;
  - a second moved lock line reads red (`2 1`).
- Entry 3 is the `--locked` workspace build (`ci-leg.sh build`), green on the untouched tree.
- A diff other than this one is a stop for /implement, never a hand trim.

## Touchpoints
- New:
  - `packages/escher-driver/src/canvas_lint.rs`
  - `canvas_lint_tests.rs`
  - `tests/blitz-tests/tests/stand_canvas_lint.rs`
- Modify:
  - `packages/escher-driver/Cargo.toml` and `Cargo.lock`
  - `lib.rs`, `lint.rs` and `schema.rs` (the `lint` help)
  - `docs/capabilities.md`
  - `session_common/mod.rs`, which takes Circle Drawer's helpers, moved in unchanged
  - `stand_canvas_commands.rs`

## Test commands (15 entries)
- The lock pair and the `--locked` build.
- escher-driver's unit tests.
- `stand_canvas_lint`: red at baseline, an absent target.
- The seven companion targets: green at baseline.
- seven_guis' four process checks.
- The capability pin.
- The `fast`, `doc`, `audit` and `stand-repeat` legs.
- The operator's hygiene, push and `ci.py` entries. The CI note names the windows and macOS legs as the planted
  literals' witness and the third run of WATCH `cry-uqifds5z`.

## Controls — each with the red reading it must produce (step 7)
Each is a one-shot `gate.py control` under `--timeout 900`. A `timeout` reading is no hold.

| Control | Edit | Red reading |
|---|---|---|
| C1 | the ink box replaced by the origins box | `fx-ascent`'s row missing, 4 of 5 |
| C2 | the pushed clip ignored | `fx-past-clip`'s row missing |
| C3 | the element's clip region not cut | `fx-in-clipped-box` reads `clipped` alone |
| C4 | a pop leaves the stack | `fx-after-pop` gains a `truncated` row |
| C5 | the same edit as C4, on the unit entry | the pop-restores unit case `failed` |
| C6 | the lint no longer reads the canvas | the page reads 1 row of 5 |

## The engine re-read (inputs#I3)
The research model's «0 labels past the content box at 1, 12 and 100 circles» is a prediction in the plan, never a
reading. Step 5's test measures it, and step 6 records it in evidence. The model's past-circle and overlap counts
belong to options not built; they are named as not re-measured.

## Capabilities — concretization preview
- **v040-10 is not claimed and stays pooled.** Its acceptance also holds «the commands grouped under the virtual
  node that covers them». «Circle Drawer's circles as virtual nodes» lists v040-10 and builds that part.
- After the word, step 4 writes one dated `notes` line: the canvas lint is built here, not claimed, and the grouping
  is still open.
- No acceptance is changed.

## Expected amendments (the wrap's)
- **On F1 A, the founder's own word.** Pattern `rul-izni57qv`: the wrap halts once, as the answer card.
  - architecture: `contract/lint-verb/truncated`, `contract/lint-verb/finding`, `contract/driver-verbs/lint`;
  - security-plan: `decision/canvas-content-crossing`, `decision/lint-findings-closed-content`;
  - obs-plan: `decision/canvas-read-opens-no-span`;
  - a11y-plan: `decision/circle-drawer-canvas-pixels-only`, a founder-ratified row;
  - layout-templates: `decision/circle-drawer-circles-painted-on-canvas`.
- **PROVISIONAL, delegate the escher overseer.**
  - The region row (F2 A).
  - The skrifa stack row and the dependency row (F3 B).
  - The proof row in the test plan, and `decision/stand-lints-clean-at-boot` (F5 A).

## Validation
- Mechanical: sections present; modify paths exist, no master among them; `lib.rs` in the set for the new module.
- `gate.py run --dry-run`: 15 entries parse.
- `gate.py baseline`: 6 new entries, each baselined; 6 seen green; 3 operator.
- `planlint`: 0 hits. Its 12 (d) listing was read, and each of its seven claims holds after the steps.
- `inputs.py verify`: I1–I3, 0 drift.
- Research lists parsed, revised to the answered options (rev 2).
- Validation-1: aligned with the scope and the working entry.
- Warnings:
  - Plan size: about 440 prose lines, a little over the ~400 guide.
  - No code-graph trace in the run dir: research recorded `derived-without-graph`, since no existing signature
    changes but `findings`'s, which is private.

## Beyond the plan
- **A deferral with no owner.** The lint-like checks left out on F4 A have no route entry: labels overlapping one
  another, a label's contrast against the fill under it, a command wholly outside its region, and a shape with no
  area. v040-10's acceptance needs none of them. No entry is proposed; your word leaves them unowned.

Apply? (yes / review / cancel)
