# P5 review — 2026-11-23-canvas-placement-and-text («Canvas placement and text»)

The plan: `.andromeda/cache/docs/2026-11-23-canvas-placement-and-text/plan.md` (rev 2, open it in an editor). No plan
stands in the chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-canvas-placement-and-text`.

## Goal
A widget painted from Rust is placed at its content box and draws its text from the document's own fonts — the
founder's F1 A+C and F2 A (`inputs#I5`, relayed verbatim; resumed on the operator's word, `inputs#I4`):
- the paint: `paint` handed the content-box size × scale, the scene at `M · T(content origin)`, unclipped (upstream #1578);
- the pointer: a widget's pointer events in its content-box frame (border + padding past its border-box origin,
  untransformed); a sub-document's unmoved;
- the text: a defaulted `Widget::fonts` hands the document's font context before each `paint`; a public
  `BaseDocument::font_ctx()`, both behind `custom-widget`.

## Top acceptance criteria
- `stand_canvas_placement` (3 tests): padded+bordered widget handed `(100, 60)`, paints its content box only; under
  `scale(2)` inside its transform; a plain widget unchanged — both layout modes.
- `stand_canvas_pointer_frame`: the padded widget hears pointer down/move from its content box (predicted `(27, 27)`,
  `(97, 57)`; today `(42, 42)`, `(112, 72)`), literals pinned from the red run.
- `stand_canvas_text` (+ under `font-embolden`): the widget is handed the document's own context (`Arc::ptr_eq`), its «Hg»
  non-blank and byte-equal to the same text shaped from bundled DejaVu Sans — no rasterized pixel count pinned.
- Each new check red before its fix, kept in evidence, plus four `gate.py control`s (`paint-size`, `paint-origin`,
  `pointer-frame`, `font-handoff`) reading `holds`.
- Unchanged: `stand_canvas_shot`, `custom_widget_layout`, all blitz-tests, `fast`, `doc`, `audit`, `stand-repeat`;
  lock `1+0`, no package; CI green on the pushed HEAD.

## Touchpoints
- New: `tests/blitz-tests/tests/stand_canvas_{placement,pointer_frame,text}.rs`.
- Modify: `packages/blitz-paint/src/{lib,render}.rs`, `packages/blitz-dom/src/events/mod.rs`,
  `packages/blitz-dom/src/node/custom_widget.rs`, `packages/blitz-dom/src/document.rs`, `tests/blitz-tests/Cargo.toml`
  (`parley` dev-dependency), `Cargo.lock` (one line).
- Each addition to an upstream function is an escher helper with one call (`content_box_paint_size`,
  `custom_widget_content_origin`); all three changes flagged upstreamable.

## Test commands (16 entries)
1–2 blitz-dom / blitz-paint unit (baseline green 177 / 12) · 3–6 the three new checks + text under `font-embolden`
(baseline red by absence) · 7 `stand_canvas_shot` + `custom_widget_layout` (green 8) · 8 all blitz-tests per package
(green 954, 363 s) · 9 lock `1+0` (red on the untouched tree; must-pass over a clone green, `p5-controls.md`) · 10
`fast` · 11 `doc` (green 71 s) · 12 `audit` · 13 `stand-repeat` · 14–16 operator: hygiene, push, CI.

## Capabilities
- None claimed. `v040-09` (its acceptance: Circle Drawer on a canvas, canvas text, the cross-host measurement) stays
  pooled — «Canvas shot across hosts» and «Circle Drawer on a canvas» (working-route lines 27–28) carry its other legs.
  After the word, one dated `matrix#v040-09 notes` line records this chunk's part (placement, pointer frame, canvas
  text from the document's fonts).

## Mechanical checks
All pass: sections, modify paths exist, new-file parents exist, the fence parses (16 entries), planlint 0 hits,
research lists parse (3 new · 7 modify), every `new` entry baselined, inputs verify 0 unparsed, no host reach, step
order read. Validation-1: aligned with the scope (rev 4, the ruling folded).

## Beyond the plan
none
