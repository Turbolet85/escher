# P5 review — 2026-11-23-canvas-shot-across-hosts «Canvas shot across hosts»

The plan to open in an editor is `.andromeda/cache/docs/2026-11-23-canvas-shot-across-hosts/plan.md` (rev 2). No plan
file stands in the chunk's folder, and editing that file changes no record until a `put`. To diff the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-canvas-shot-across-hosts`.

## Goal
The founder ruled F1: «A: требовать побайтовое равенство (Recommended)» (the founder, 2026-11-23, relayed verbatim
by overseer1 and the escher overseer, inputs#I3). This chunk builds what that answer asks for: canvas-only shots held
byte-equal on linux x86_64, macOS aarch64 and windows x86_64 by one committed digest table, with no pixels committed.

## The work
- One new check, `tests/blitz-tests/tests/stand_canvas_cross_os.rs`. It uses the probe's three canvas fixtures:
  a solid fill, an antialiased edge, and glyphs from the document's fonts.
- Each fixture is shot as the whole screen and as an element shot cropped to the canvas, in both layout modes. That
  makes 12 `answer_digest` rows, held to literals.
- The check has five tests. One of them proves that each fixture holds only the canvas (no named node, no lint
  finding).
- `docs/capabilities.md` is restated by hand. It gives the measured document-text difference on macOS, the canvas
  exception and its witness, and the conditions still unmeasured.

## Top acceptance criteria
- `stand_canvas_cross_os` passes 5 tests per package and under `blitz-paint/font-embolden`.
  `stand_canvas_shot` and `stand_cross_os_readings` stay green.
- Each of the four controls under `gate.py control` reads `holds`, with its red reading on the dev host recorded in
  `evidence/`. The four controls are fill colour 255→254, circle centre moved, glyph size 40→41, and page text beside
  the canvas.
- The operator pass's CI run on the pushed HEAD reads `verdict: green`. In that run, `Test (macos)`,
  `Test (windows)` and the linux `test` leg read the one table, and that run is the cross-host reading.
- No carrier, verb, field, cause, exit, log field or span is added. Nothing is printed, no pixel is committed, and no
  kept or committed check gains a `screenshot` step.

## Test commands
1. per-package check (baseline red: file absent)
2. the same under font-embolden (red: absent)
3. `stand_canvas_shot` + `stand_cross_os_readings` (green, 10 passed)
4. `fast` (seen green)
5. `audit` (green)
6. `stand-repeat` (seen green)
7. operator hygiene
8. operator push
9. operator `ci.py conclusion --wait 2700`

## Red readings of the new controls
The red readings are taken on the dev host (linux) by implement's one-shot `gate.py control`. A red on macOS and
Windows for the same edits is predicted, not measured: it would take a mutated push to the build branch, which the
plan does not make. The CI run reads the green side on all three hosts.

## Capability claims
None. v040-09's acceptance names a shot of Circle Drawer, which «Circle Drawer on a canvas» builds. After the word,
v040-09 gets one `notes` line: this chunk proved its cross-host leg on canvas-only fixtures, and Circle Drawer's
element shot joins the same table.

## Expected amendments at the wrap
The no-shot cross-OS rows gain the founder's canvas-only exception, and their `why` is restated to the measured
cause. The rows are:
- test-plan `decision/cross-os-checks-hold-no-shot`, `decision/no-shot-in-cross-os-byte-checks` and
  `decision/no-screenshot-in-cross-os-checks`;
- architecture `decision/no-screenshot-in-cross-os-byte-check`.

A new test-plan row, `+ decision/canvas-shots-cross-os-by-digest`, is added. The authority is `relayed`, the
founder's own words. Pattern `rul-izni57qv` (Boundary widening) is named on the entry: the fork read it as not met,
and if the wrap reads it met, it halts once as an answer card.

## Beyond the plan
- WATCH, no owner change: «Circle Drawer on a canvas» (the working-route entry «Circle Drawer on a canvas — the first
  sample, circles painted from Rust; skeleton, keys, boot lint clean; checks in both layout modes, cross-OS digests
  (v040-09)»). Its element shot of the canvas joins `stand_canvas_cross_os`'s table. Its whole screen holds page
  text, so it stays outside under the rule. This is a direction to that entry's plan, written as a `WATCH:` at the
  wrap.

Apply? (yes / review / cancel)
