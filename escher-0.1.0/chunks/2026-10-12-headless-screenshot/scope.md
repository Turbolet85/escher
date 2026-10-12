# Scope — 2026-10-12-headless-screenshot

**Working entry (working-route.md:83, Epoch 5 — Agent surfaces):** Headless screenshot — screen or one element by id,
no display, identical pixels on repeat with bundled fonts; fails, never skips, without fonts (v010-07, v010-08)

**Capabilities named:** v010-07 (Headless screenshot — the driver screenshots the whole screen or one element with no
display) · v010-08 (Screenshot determinism — a repeated screenshot on the same host yields the same pixels). Both
`integration`, both unclaimed. Intent §4: "the driver takes a screenshot of the screen or of one element with no display,
the same pixels on a repeated run on the same host." This entry is the last markerless one of Epoch 5; its wrap closes
the epoch.

## What it builds
- A screenshot of the held instance with no display: the whole screen (the session's pinned viewport) or one element
  addressed by its stable element id, rasterised on the CPU — the same path the screenshot example, the browser's
  `capture` and the blitz-tests reftests use (blitz-paint → anyrender → `VelloCpuImageRenderer`) — and encoded as PNG.
  (P3: verified — `paint_scene` over `&mut BaseDocument` into `render_to_buffer::<VelloCpuImageRenderer, _>`,
  `examples/screenshot.rs:108-129`, `tests/blitz-tests/tests/background_size.rs:43-47`; the harness hands out the
  document mutably, `Harness::base_mut`, `packages/blitz-test-harness/src/harness.rs:136`; the harness itself has no raster
  dependency today, `packages/blitz-test-harness/Cargo.toml`)
- `screenshot` as a driver verb, served by every surface the verb table reaches — in process (`Session::run`), the
  command line (`escher-session screenshot …`) and MCP (`tools/call`). Its argument shape (an optional `--id`) and its
  result shape (where the image goes and in what form) are decided at this chunk (CARRY below). (P3: verified — `VERBS`
  is read by `cli.rs`, `mcp.rs`, `describe.rs`, `command.rs` and `execute.rs`, code-graph trace; the result's form is a
  P4 fork for the founder, the playbook's Boundary widening rule)
- An element screenshot crops to the element's bounds as the snapshot reads them; a target the screen does not read is
  refused as every id-addressed verb is (`stale` / `not-found`); how an element out of view or partly clipped is cropped
  or refused is decided at P4. (P3: verified — the id path is `resolve`, `packages/escher-driver/src/execute.rs:285-301`;
  the in-view reading is the centre inside `visible_region`, `execute.rs:324-334`)
- Determinism: two screenshots of the same stand state taken in separate runs on the same host are pixel-identical —
  the stand already boots with a pinned viewport, its bundled font and no network (`seven_guis::stand`).
- Fails, never skips, without fonts: when no usable font can be had, the screenshot (and its check) fails with a
  typed reason; it never renders with a host font and never skips.
  [premise-corrected: the bundled font is compiled in (`include_bytes!`, `examples/seven_guis/src/lib.rs:7`), so "without
  fonts" is a font context holding no family — the woff2 decodes only under the `woff` feature
  (`packages/blitz-dom/src/util.rs:22-31`) and `build_single_font_ctx` then registers none (`packages/blitz-dom/src/lib.rs:143-156`),
  after which text lays out 0×0 (the engine's own skip guard, `packages/blitz-dom/src/document.rs:2883-2900`). The document's
  font context is crate-private (`document.rs:254`), so no driver reader of it exists today; and `Session::run` answers
  `Result<Outcome, Refusal>` (`execute.rs:110`), so a failure it returns is a refusal cause. It binds both the driver's
  answer and the checks.]

## Freight folded from the entry
- CARRY: password painted in the clear (from 2026-10-06-snapshot-state-fidelity; the direction given at that chunk's P5
  review, 2026-10-06, that each engine defect it found gets a route owner — the wrap's placement) — the snapshot masks a
  password input's value (`MASKED_VALUE`), but a screenshot is where a typed password would leave the process as pixels.
  Hypothesis, verbatim: "hypothesis: the engine lays out and paints a password input's characters like any text input's
  (read from the code at that chunk's phase — construct.rs:452, layout/mod.rs:358, no masking site — not measured on a
  rendered frame)". (P3: re-derived at HEAD by code read, not yet by a frame — a password builds the same text editor as
  text (`packages/blitz-dom/src/layout/construct.rs:449-455`), the editor holds the typed text (`construct.rs:873-882`),
  paint strokes the editor's layout (`packages/blitz-paint/src/render.rs:890-945`); `password` occurs 0 times in
  blitz-paint and 4 times in blitz-dom, none a masking site. The frame measurement is the plan's first step.) A screenshot
  this chunk ships must not carry a typed password in the clear.
- CARRY: one verb table (from 2026-10-07-command-and-refusal-schema; the operator, 2026-10-07, at that chunk's P4 forks)
  — `screenshot` is not in `escher_driver::VERBS` (a call naming it is refused `unknown-verb` today, pinned by a unit
  test); it joins the table at this entry's promotion, where its form is decided, with its argument and result shapes
  stated there. (P3: verified, and wider than one test — the name `screenshot` stands as the unknown verb in 8 sites:
  `command.rs:494`, `command.rs:768`, `client.rs:339`, `cli.rs:460`, `cli.rs:464`, `mcp.rs:929`,
  `examples/seven_guis/tests/cli_commands.rs:333-334`; each takes another unknown name)

## Surfaces and contracts it touches
- `escher-driver`: `VERBS` (ten → eleven), `NODE_FIELDS`/describe's tables as they name verbs, `validate`, `Session::run`
  (the executor), the wire's `call` answer, `command_line`, `serve_mcp`'s `tools/list` and `tools/call`, the `help`
  self-description, the `command` span (a screenshot's fields: fixed words and counts only). (P3: verified — the
  count pins are listed in research.md's Sweeps)
- The crossing: an image is screen content leaving the process — pixels that hold every name and value the screen
  paints — on the socket, the command line's stdout (or a file) and the MCP stdio. Whether that is inside the ratified
  crossing or a new widening, and the answer's byte bound against an image's size (1,048,576 on the wire and over MCP),
  is a question for the founder, asked at P4 against the playbook's escalate patterns. (P3: verified — the playbook's
  Boundary widening rule escalates it; the precedents are recorded in the security and arch histories)
- `blitz-test-harness` / the stand's headless boot (where a raster can hang off the held instance), `blitz-paint`.
  (P3: verified — the harness owns the held document and the intent's observed gap names it: "the harness has no
  screenshot")
- The stand (`examples/seven_guis`): the `escher-session` binary's verb surface; checks under `tests/blitz-tests/tests/`
  and `examples/seven_guis/tests/`.

## Boundaries (not this chunk)
- No windowed or GPU screenshot; no cross-host or cross-OS pixel identity (v010-08 is "the same host").
- No visual-diff tooling, no reference-image store, no image comparison verb.
- The other entries' work stays theirs: the stand's contrast, keyboard and a11y harnesses, the range input, the
  requirement sweep, quality gates, the cold-agent test.

## CI read at Setup (5a)
- 938cd1b8 (the last wrap's commit, the whole range from the flip through HEAD) — green, checks 16/16, wall 720 s,
  CI#38232006309. The overseer relayed the same reading (09:58Z); Setup read it by run id. Nothing to fold.
