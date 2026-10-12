# Codebase Research — 2026-10-12-headless-screenshot

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 21 · code-graph queries: 2 (trace `.andromeda/runs/2026-10-12T18-14-09-phase/tree-query-2026-10-12-headless-screenshot.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition applied (an
  entry exiting 124/137 reads as the gate's own bound — no stay-up smoke is planned); `.claude/rules/testing.md` — read in
  full, 7 Session Additions applied (no deleting key in a fixture; sha-checked mutation restores; `--no-fail-fast` for red
  counts; one merged stream for per-target attribution; search `tests/` and `examples/*/tests/` for a table's readers
  before calling a check unaffected — done, Sweeps below)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `examples/screenshot.rs` (1-175) — the CPU raster path: `paint_scene` into `render_to_buffer::<VelloCpuImageRenderer, _>`
  after an explicit white fill (108-129), PNG through `png::Encoder` RGBA 8-bit (151-170); `unwrap`s and `println!` timings
  throughout (not carried into the driver)
- `tests/blitz-tests/tests/background_size.rs` (1-60) — the same raster inside a test, reading RGBA pixels by index (43-49)
- `examples/seven_guis/src/stand.rs` (full) — pinned 800 × 600 · scale 1.0 · Light (42-49), `font_ctx()` =
  `build_single_font_ctx(DEJAVU_SANS)` (51-54), `options(incremental)` offline (58-69), `boot` over `Harness::from_vdom` (73-107)
- `examples/seven_guis/src/lib.rs:7` — `DEJAVU_SANS` is `include_bytes!` of a woff2; `examples/seven_guis/Cargo.toml:30-41` —
  native `dioxus-native` features `system-fonts` + `woff`
- `packages/blitz-dom/src/lib.rs` (132-157) — `build_single_font_ctx`: system fonts off, registers what `decode_font_bytes`
  yields for four generics; an undecodable font registers no family
- `packages/blitz-dom/src/util.rs` (17-32) — woff/woff2 decode only under `feature = "woff"`, else the bytes pass through
- `packages/blitz-dom/src/document.rs` (254, 381-405, 1782-1811, 2883-2930) — `font_ctx` is `pub(crate)`; scrollbar opacity
  from `Instant::elapsed` (wall clock); the engine tests' "no usable font" skip guard
- `packages/blitz-dom/src/node/scrollbar.rs` (12-26) — `FADE_DELAY` 500 ms, `FADE_DURATION` 200 ms, `opacity_at`
- `packages/blitz-dom/src/layout/construct.rs` (440-460, 857-890) — `password` builds the same text editor as `text`; the
  editor's text is the `value` attribute, typed text edits it
- `packages/blitz-dom/src/layout/mod.rs` (345-370) — `password` lays out as `text`
- `packages/blitz-dom/src/node/text.rs` (64-75) — `TextInputData` holds a `parley::PlainEditor`, no mask
- `packages/blitz-dom/src/mutator.rs` (1310-1338) — a file input builds a `Browse` button and a `No File Selected` label
  under `feature = "file-input"` only; the stand's graph names `blitz-dom` with `default-features = false`
  (`packages/dioxus-native-dom/Cargo.toml:29`)
- `packages/blitz-paint/src/render.rs` (890-945) — text-input paint: selection and caret when focused (no timer, no
  blink — a static caret), then the editor's layout stroked; no masking branch
- `packages/blitz-paint/Cargo.toml` (`[features]`) — `scrollbars` is off by default; `tests/blitz-tests/Cargo.toml:20`
  names it, so a workspace build paints scrollbar thumbs where a per-package build of the driver does not
- `packages/blitz-test-harness/Cargo.toml`, `src/harness.rs` (85-217) — no paint or raster dependency; `base_mut()`
  hands out the document; `HarnessOptions` default has no `font_ctx`
- `packages/escher-driver/Cargo.toml` — four dependencies (blitz-test-harness, dioxus-native-dom +accessibility,
  serde_json, tracing)
- `packages/escher-driver/src/session.rs` (full) — `Session` holds `harness`, `time`, `seen`; `with_time` is the
  caller-supplied hook pattern
- `packages/escher-driver/src/execute.rs` (18-60, 73-192, 262-336) — `Outcome` (four variants), `run` → `Result<Outcome,
  Refusal>`, `resolve` (stale / not-found), `action_point` (disabled → off-screen → covered), `in_view` (centre inside
  `visible_region`)
- `packages/escher-driver/src/mcp.rs` (490-505) — a tool result is one `text` content block plus `isError`

## Graph impact (from the code-graph query)
- **VERBS** — referenced in 10 files: `escher-driver` `schema.rs` (9), `mcp.rs` (5), `describe.rs` (4), `cli.rs` (3),
  `command.rs` (2), `execute.rs` (2), `lib.rs` (1); `examples/seven_guis/tests/cli_commands.rs` (2),
  `examples/seven_guis/tests/mcp_server.rs` (2); `tests/blitz-tests/tests/stand_act_scroll.rs` (3),
  `tests/blitz-tests/tests/stand_act_spans.rs` (2) — every one reads the table back; a new row moves each count pin.
- **Outcome** — referenced in `execute.rs` (12), `json.rs` (11), `mcp.rs` (2), `lib.rs` (1),
  `tests/blitz-tests/tests/stand_act_spans.rs` (9), `session_common/mod.rs` (5), `stand_mcp_tools.rs` (3),
  `stand_act_diff.rs` (2), `examples/seven_guis/tests/cli_commands.rs` (2) — a new variant reaches every `match` on it.
- **Command** — `command.rs` (33), `execute.rs` (7), `lib.rs` (1).
- **crate edges** — `seven_guis` and `blitz-tests` → `escher-driver` → `blitz-test-harness`, `dioxus-native-dom`,
  `blitz-dom`, `blitz-traits`; `blitz-test-harness` → `blitz-dom`, `blitz-html`, `blitz-traits`, `dioxus-native-dom`. A
  raster in the harness reaches both consumers through the driver with no new edge into either.

## Patterns detected
- **Caller-supplied capability** (`packages/escher-driver/src/session.rs:139-147`): `with_time` hands the session what
  it cannot know itself; a session without it refuses the verb (`time-unavailable`).
- **One shared id path** (`execute.rs:285-301`): every id-addressed verb reads the screen, records it, then refuses
  `stale` / `not-found` before anything else.
- **Fresh-process identity** (`tests/blitz-tests/tests/stand_id_persistence.rs`, its `ids_in_a_child_process`): the test
  binary re-runs its own `#[ignore]` child and compares what the child printed.
- **Masked-value proof** (`tests/blitz-tests/tests/stand_mcp_masked.rs`): an in-file password fixture beside the stand,
  a typed sentinel counted 0 times in what crosses, both layout modes.
- **Hand-written written forms** (`packages/escher-driver/src/json.rs`, `mcp.rs:490-505`): every answer and tool result
  is built by the crate's own writer; `serde_json` only reads.

## Conventions to follow
- **Typed errors, no panics on the driver path**: the raster example `unwrap`s (`examples/screenshot.rs:42,51,139`); the
  driver's equivalent returns a `Refusal` or an `Outcome`, per `refusal.rs` and `execute.rs:110`.
- **Count pins move together**: verbs `10`, instance verbs `6`, causes `8`, session errors `13`, node keys `13` are each
  asserted at the sites listed under Sweeps.
- **Both layout modes**: `for incremental in [false, true]` in every stand check (`stand.rs:136`).

## Mechanisms re-derived (the equalities the plan leans on)
- **Raster equality across runs**: the same `BaseDocument` state painted by `paint_scene` at the same scale and size into a
  fresh `VelloCpuImageRenderer` yields the same RGBA bytes — holds on everything this research read: no paint input reads
  a clock except scrollbar opacity (`document.rs:1782-1793`, wall clock since the last scroll activity, painted only under
  the `scrollbars` feature) and the caret is static (`render.rs:927-940`). NOT measured: run-to-run identity of the CPU
  renderer itself, single-threaded (per-package build) and with `multithreading` (the root package's dev-dependency,
  `Cargo.toml:274`, unified into a `--workspace` build) — the plan's determinism check is that measurement, under both
  builds.
- **Password in the clear**: the paint of a focused or unfocused `password` input strokes the editor's layout of the typed
  text (`construct.rs:449-455`, `873-882`; `render.rs:890-945`) — code read at HEAD, not a frame. The plan measures it on a
  frame before any fix.
- **No usable font**: a font context built from bytes `decode_font_bytes` cannot decode registers no family
  (`lib.rs:143-156`), so every generic resolves to nothing and text lays out 0×0 (`document.rs:2883-2888`). The driver has
  no reader of the context (`document.rs:254`, `pub(crate)`).

## New files to create
- `packages/blitz-test-harness/src/screenshot.rs` — the harness's CPU raster of its document: the whole viewport, or a pixel rectangle of it, as RGBA
- `packages/escher-driver/src/image.rs` — the PNG encoding and the answer's encoded form of an image
- `tests/blitz-tests/tests/stand_act_screenshot.rs` — the verb on the stand: whole screen and element, refusals, the password mask, no font, nothing changed by a shot
- `tests/blitz-tests/tests/stand_screenshot_repeat.rs` — the same stand state screenshotted in separate processes (a re-run `#[ignore]` child), both layout modes
- `tests/blitz-tests/tests/password_painted_masked.rs` — the engine's paint of a password input on a frame (the engine branch of the password fork)

## Files to modify
- `Cargo.toml` — `[workspace.dependencies]` lines for any raster crate the harness or the driver takes
- `Cargo.lock` — the delta of those lines
- `packages/blitz-test-harness/Cargo.toml` — blitz-paint, anyrender, anyrender_vello_cpu
- `packages/blitz-test-harness/src/lib.rs` — the screenshot module and its re-export
- `packages/blitz-dom/src/document.rs` — a public reading of whether the document's font context holds a family
- `packages/blitz-paint/src/render.rs` — a password input's characters painted masked (the engine branch of the password fork)
- `packages/blitz-dom/src/node/text.rs` — a masked layout's state beside `TextInputData`, if the paint needs one kept between frames
- `packages/escher-driver/Cargo.toml` — `png`
- `packages/escher-driver/src/lib.rs` — the new module and any new public item
- `packages/escher-driver/src/schema.rs` — the `screenshot` row, its argument and result fields, count pins
- `packages/escher-driver/src/command.rs` — `Command::Screenshot`, validation, the unknown-verb pins
- `packages/escher-driver/src/execute.rs` — the executor arm, `Outcome`'s new variant, span recording
- `packages/escher-driver/src/refusal.rs` — a cause for a screenshot that cannot be taken, if P4 adds one
- `packages/escher-driver/src/json.rs` — the new outcome's written form, count pins
- `packages/escher-driver/src/mcp.rs` — the tool result for an image, the unknown-tool pin, count pins
- `packages/escher-driver/src/cli.rs` — the unknown-verb pins, usage line rows
- `packages/escher-driver/src/describe.rs` — the self-description's counts
- `packages/escher-driver/src/client.rs` — the unknown-verb pin
- `tests/blitz-tests/Cargo.toml` — `png` as a dev-dependency, to decode what a check reads
- `tests/blitz-tests/tests/session_common/mod.rs` — a `screenshot` call builder and an image reader
- `tests/blitz-tests/tests/stand_act_diff.rs` — its `match` on `Outcome`, when exhaustive
- `tests/blitz-tests/tests/stand_act_spans.rs` — the instance-verb table, its span line
- `tests/blitz-tests/tests/stand_act_scroll.rs` — the verb-count pin
- `tests/blitz-tests/tests/stand_mcp_tools.rs` — the tool call of `screenshot` in process
- `examples/seven_guis/tests/cli_commands.rs` — the unknown-verb rows, the verb-count pin, a hosted `screenshot`
- `examples/seven_guis/tests/mcp_server.rs` — the tool listing's count, a `screenshot` tool call through the binary
- `examples/seven_guis/tests/host_log.rs` — a hosted `screenshot` in the measured flow
- `examples/seven_guis/tests/mcp_log.rs` — a `screenshot` tool call in the measured flow

## Sweeps
- `"screenshot"` over `packages/escher-driver/src`, `examples/seven_guis/{src,tests}`, `tests/blitz-tests/tests`: 8 hits ·
  8 changed (each stands as the unknown verb: `command.rs:494`, `command.rs:768`, `client.rs:339`, `cli.rs:460`,
  `cli.rs:464`, `mcp.rs:929`, `cli_commands.rs:333`, `cli_commands.rs:334`) · 0 no-change
- `VERBS.len()|len(), 10` over the same three trees: 9 count pins · 9 changed (`command.rs:802`, `mcp.rs:1042`,
  `schema.rs:598`, `schema.rs:622`, `json.rs:452`, `cli_commands.rs:118`, `mcp_server.rs:123`, `stand_act_scroll.rs:505`,
  `mcp_server.rs:3` doc line "ten verbs") · `accessibility_states.rs:163` no-change (its own state table)
- `CAUSES.len()|len(), 8)|len(), 6)|len(), 13)` over the same trees: 18 hits · changed only where the plan adds a cause or
  an instance verb (`refusal.rs:213`, `refusal.rs:271`, `stand_act_spans.rs:401`, `execute.rs:444`, `execute.rs:451`,
  `mcp.rs:1184`, `mcp_log.rs:107`) · no-change for `session.rs:197`, `command.rs:497` (unknown-verb list length, kept at 8 by
  swapping the name), `command.rs:571`, `wire.rs:374`, `schema.rs:830`, `schema.rs:858`, `error.rs:271`, `refusal.rs:306`,
  `json.rs:624`, `json.rs:667`, `stand_act_range.rs:99` (other tables)

## Open questions
- Where the image goes and in what form (in-band in the JSON answer and an MCP image block, or a file at a caller-named
  path), and whether pixels may cross at all → blocks: plan-decision (the founder's word — Boundary widening)
- How a password stays out of the pixels (the engine paints it masked for every document, or the screenshot refuses while a
  password with a value is in frame) → blocks: plan-decision
- Where the raster lives (the harness, the driver, or a caller hook like `with_time`) and how "no usable font" is read →
  blocks: plan-decision
