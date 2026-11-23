# P5 review — 2026-11-23-canvas-in-a-headless-shot «Canvas in a headless shot» (v040-09)

The plan: `doc:2026-11-23-canvas-in-a-headless-shot/plan` rev 4. Its render, to open in an editor:
`.andromeda/cache/docs/2026-11-23-canvas-in-a-headless-shot/plan.md`. No plan stands in the chunk's folder. Writing
in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff
--root . 2026-11-23-canvas-in-a-headless-shot` diffs the chunk's records.

## Goal
A widget an app paints from Rust, handed to an `<object data>`, shows in a harness shot and in the driver's
`screenshot`, whatever packages cargo builds. Measured on the stand, in process, in both layout modes.

**The fix, one manifest line (T1, `inputs#I3`):** blitz-test-harness's `blitz-paint` names `custom-widget`.

**The witness, one new check (T2):** `stand_canvas_shot`, with its widget in the check. It is booted by
`stand::boot_alone` in a session held in process, on no carrier (F2 A, `inputs#I4`).

**F1 A (`inputs#I4`) needs no code.** Its two master rows go to the wrap as Expected amendments under `rul-izni57qv`,
on the founder's word relayed verbatim by the overseer.

## Measured
- **Today, per package** (P3): the widget's pixel reads white, `[255,255,255,255]`.
- **With the feature, or in a workspace build** (P3): it reads red, `[255,0,0,255]`.
- **Through the driver, feature on** (P4, a throwaway probe in the take-up's worktree):
  - the `<object>` is a snapshot node at (8, 8, 200 × 100);
  - the whole driver shot equals the harness raster;
  - the element shot is 200 × 100, every pixel red;
  - the same in both modes.
- **blitz-dom's own `custom-widget`** is already on under the harness, the driver and seven_guis. So only
  blitz-paint's widget pass moves.

## Steps (7)
1. Write the check: harness shot, then driver whole and element shots, both modes.
2. Add `peniko` as a blitz-tests dev-dependency. The lock gains one line and no package.
3. Run the check before the fix. It is red: `evidence/red-before-fix.log`.
4. The harness manifest line.
5. Green.
6. One `gate.py control`: remove the feature text, and the harness case reads red.
7. Run the wider gates.

## Top acceptance criteria
- **Per package:**
  - a harness shot reads `[255,0,0,255]` inside the object and white outside;
  - the driver's whole shot equals the raster;
  - the element shot is 200 × 100, all red;
  - all in both modes.
- **Every build selection green:**
  - blitz-tests and seven_guis per package;
  - `fast` (the workspace);
  - `stand-repeat`;
  - blitz-paint reads `custom-widget` under harness, driver and seven_guis (gate 5: 3).
- **Red readings shown:**
  - the check before the fix, and under the control;
  - gate 5 red (0) at baseline;
  - gate 7 no line at baseline.
- **Dependencies:** lock +1 line, no package; no platform adapter; audit green.
- **CI:** green on the pushed HEAD.

## Touchpoints
- new: `tests/blitz-tests/tests/stand_canvas_shot.rs`
- modify: `packages/blitz-test-harness/Cargo.toml`, `tests/blitz-tests/Cargo.toml`, `Cargo.lock`

## Test commands — 13 entries
- **Gate 1** — harness unit tests. Baseline green: 25 passed.
- **Gate 2** — the new check. Baseline red: the target is absent; its real red is step 3.
- **Gate 3** — blitz-tests whole, per package. Baseline green: 952 passed.
- **Gate 4** — seven_guis whole, per package. Baseline green: 57 passed.
- **Gate 5** — the feature in 3 selections. Baseline red: 0.
- **Gate 6** — the adapter census. Baseline green: 0.
- **Gate 7** — the lock delta. Baseline red: no line.
- **Gates 8–10** — `fast` · `audit` · `stand-repeat`, each seen green at the last wrap.
- **Gates 11–13** — operator entries: hygiene · push · CI read.

**Must-pass readings** over a scratch clone carrying the change (`p5-controls.md` in this run dir):
- gate 5 reads 3;
- gate 7 reads `1+0`;
- gate 6 still reads 0.

## Capabilities — concretization preview
- **None is claimed.** `v040-09`'s acceptance needs:
  - Circle Drawer on a canvas;
  - canvas text from the bundled fonts;
  - the cross-host measurement.
- Three later route entries carry those legs (working-route lines 26–28). After the word, one dated
  `matrix#v040-09 notes` line records this chunk's part, and the cap stays pooled.

## Expected amendments (the wrap's)
- **security-plan:** a canvas's painted content crosses as pixels in a shot on every ratified carrier.
  - Pattern `rul-izni57qv`. Authority: the founder, 2026-11-23, relayed verbatim by the overseer (`inputs#I4`).
  - The wrap halts on it once, as the answer card.
- **security-plan:** the harness's feature and `peniko` as reviewed pinning facts.
- **architecture:** a canvas crossing row beside the select list's (same pattern and authority).
- **architecture:** the harness names `custom-widget`.
- **test-plan:** the `stand_canvas_shot` witness.

## Beyond the plan
none

Apply? (yes / review / cancel)
