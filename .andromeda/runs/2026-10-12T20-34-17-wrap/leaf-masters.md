group: masters
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «not by node creation, hover or layout)»
  source: .andromeda/architecture.md:121 «since 2026-10-12-headless-screenshot two public read-only methods: `BaseDocument::can_paint_text(&self) -> bool` — whether the font context resolves a family for the `sans-serif` generic»
  replace-with: «not by node creation, hover or layout), and two read-only readers, `can_paint_text` (the font context resolves a `sans-serif` family) and `layout_as_text_input` (a text laid out as a text input lays out, one line — the password mask its one caller)»
- leaf: CLAUDE.md:26   class: omission
  leaf-text: «paints a resolved document into an anyrender `PaintScene`»
  source: .andromeda/architecture.md:129 «since 2026-10-12-headless-screenshot a password `input` (its `type` reading `password`) paints its value masked, in every document and through every renderer that paints with blitz-paint»
  replace-with: «paints a resolved document into an anyrender `PaintScene`; a password `input` paints its value as bullets in every document and renderer through it, windowed included — equal-length values paint identical frames, the caret at its bullet (an engine change ratified by the founder, 2026-10-12)»
- leaf: CLAUDE.md:35   class: omission
  leaf-text: «and no time is advanced), input synthesis, inspection»
  source: .andromeda/architecture.md:130 «`Harness::screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `Harness::screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` resolve at the harness's own time»
  replace-with: «and no time is advanced), input synthesis, inspection, screenshot (`Harness::screenshot` / `screenshot_rect` → `Raster`, at the harness's own time, painted through blitz-paint into `VelloCpuImageRenderer` and composited over opaque white; a `PixelRect` clipped to the viewport, an empty one `ScreenshotError::EmptyRect`, a document whose `can_paint_text` reads false `ScreenshotError::FontsUnavailable` with nothing painted)»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «snapshot text and diff crossing out (ratified by the founder, 2026-10-10)»
  source: .andromeda/architecture.md:146 «Since 2026-10-12-headless-screenshot both carriers also take a shot of the screen out: the socket's answer to a `screenshot` call holds its PNG as base64 inside the JSON line»
  replace-with: «snapshot text and diff crossing out (ratified by the founder, 2026-10-10) and a `screenshot`'s PNG as base64 inside its answer line (screen pixels in band, ratified by the founder, 2026-10-12)»
- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`VERBS`, ten verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance»
  source: .andromeda/architecture.md:133 «`VERBS` lists eleven verbs in three levels (`Level` is `Instance` · `Session` · `Tool`»
  replace-with: «`VERBS`, eleven verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` asked of the held instance»
- leaf: CLAUDE.md:37   class: stale
  leaf-text: «and eight refusal `Cause`s each with a fixed remedy»
  source: .andromeda/architecture.md:133 «`Cause` is closed at nine, listed in `CAUSES` — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable` · `fonts-unavailable`»
  replace-with: «and nine refusal `Cause`s each with a fixed remedy (`fonts-unavailable` the ninth, returned by `screenshot` alone)»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «`scroll` adding whether its target is then in view;»
  source: .andromeda/architecture.md:133 «`screenshot` (since 2026-10-12-headless-screenshot) dispatches nothing and is no step: with no `id` it returns the whole screen, and with an `id` it crops the frame to the part of the element's bounds»
  replace-with: «`scroll` adding whether its target is then in view, and `screenshot` — no step, dispatching nothing, changing nothing — `Outcome::Shot { width, height, png }`, the whole screen or, given an id resolved as `click`'s is, the part of that element's bounds that can be seen rounded outward, as a PNG (a crop of no pixel refused `off-screen`, an app with no `sans-serif` family `fonts-unavailable`, before anything is painted);»
- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`tools/list` the ten verbs of `VERBS`, `help` last»
  source: .andromeda/architecture.md:133 «`tools/list` answers the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot), `help` last»
  replace-with: «`tools/list` the eleven verbs of `VERBS`, `help` last»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «a `snapshot` result the screen text, a `help` result the self-description's line, any other the JSON line»
  source: .andromeda/architecture.md:133 «since 2026-10-12-headless-screenshot an accepted `screenshot` two blocks, an `image` block (`data` the PNG's base64, `mimeType` `image/png`) then a text block holding the shot's line without `png`»
  replace-with: «a `snapshot` result the screen text, a `help` result the self-description's line, an accepted `screenshot` an `image` block (`image/png`, base64) then a text block, its line without `png`, any other the JSON line»
- leaf: CLAUDE.md:37   class: stale
  leaf-text: «a result at most 1,048,576 — that crossing ratified»
  source: .andromeda/architecture.md:133 «since 2026-10-12-headless-screenshot a result that holds an image is bounded by its whole written form instead, a text-only result by its text as before (`ToolResult::bounded`)»
  replace-with: «a result at most 1,048,576 — a text-only one by its text, one holding an image by its whole written form — that crossing ratified»
- leaf: CLAUDE.md:38   class: stale
  leaf-text: «`start <task>`, the six instance verbs, `status`, `stop`»
  source: .andromeda/layout-templates.md:74 «the seventh `screenshot` after `scroll` since 2026-10-12-headless-screenshot, taking an optional `--id`»
  replace-with: «`start <task>`, the seven instance verbs, `status`, `stop`»
- leaf: CLAUDE.md:44   class: omission
  leaf-text: «it carries the lifecycle and driver calls with their answers — ids, names, values, snapshot text and diff — each widening ratified by the founder;»
  source: .andromeda/security-plan.md:78 «since 2026-10-12-headless-screenshot an answer also carries the screen as an image — an accepted `screenshot`'s line `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen»
  replace-with: «it carries the lifecycle and driver calls with their answers — ids, names, values, snapshot text and diff and, since 2026-10-12, a shot's pixels (a PNG as base64, a password in it painted masked) — each widening ratified by the founder;»
- leaf: CLAUDE.md:47   class: omission
  leaf-text: «0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace`, under both builds»
  source: .andromeda/obs-plan.md:296 «A shot's image is screen content that leaves only in its answer: 0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`»
  replace-with: «0 id needles, 0 name needles, 0 occurrences of a typed sentinel and 0 of a shot's first 64 base64 characters at `trace`, under both builds»
- leaf: CLAUDE.md:74   class: omission
  leaf-text: «| Headless `Harness` API · settle | `packages/blitz-test-harness/src/{harness,input,inspect,settle}.rs` ·»
  source: .andromeda/architecture.md:255 «Modules harness, input, inspect, settle (packages/blitz-test-harness/src/lib.rs:19-23) and, since 2026-10-12-headless-screenshot, the private screenshot»
  replace-with: «| Headless `Harness` API · settle · screenshot | `packages/blitz-test-harness/src/{harness,input,inspect,settle,screenshot}.rs` (the screenshot's unit tests inline) ·»
- leaf: CLAUDE.md:81   class: omission
  leaf-text: «| Driver executor (a call run in process · act by id · refusal detection · `scroll` · the returned diff · `advance` and the time step · the command span) | `packages/escher-driver/src/execute.rs` ·»
  source: .andromeda/architecture.md:257 «image (the crate-private PNG encoder `png` and the hand-written padded `base64` of a shot; execute holding `Outcome::Shot` and the shot's crop since then»
  replace-with: «| Driver executor (a call run in process · act by id · refusal detection · `scroll` · `screenshot` and its crop · the returned diff · `advance` and the time step · the command span) | `packages/escher-driver/src/execute.rs` · the shot's PNG encoder and base64 `packages/escher-driver/src/image.rs` ·»
- leaf: CLAUDE.md:81   class: omission
  leaf-text: «checks `tests/blitz-tests/tests/stand_act_*.rs` (call builders and refusal readers in `tests/session_common/mod.rs`) |»
  source: .andromeda/test-plan.md:52 «byte-identical across two child runs of the test binary's own `#[ignore]` child, whose stdout the check reads, and the parent, 12 shots in both layout modes under both builds»
  replace-with: «checks `tests/blitz-tests/tests/stand_act_*.rs` and `stand_screenshot_repeat.rs` (call builders, the `screenshot` builders and `shot` reader, and refusal readers in `tests/session_common/mod.rs`) |»
- leaf: CLAUDE.md:100   class: stale
  leaf-text: «`advance --ms` · `scroll --id` · `status` · `stop`»
  source: .andromeda/layout-templates.md:74 «the seventh `screenshot` after `scroll` since 2026-10-12-headless-screenshot, taking an optional `--id` (the whole screen without it, with it the element the id names»
  replace-with: «`advance --ms` · `scroll --id` · `screenshot [--id]` · `status` · `stop`»
- leaf: CLAUDE.md:101   class: stale
  leaf-text: «its tools the ten verbs, `help` last»
  source: .andromeda/security-plan.md:203 «ion mcp` role also serves the driver's verbs as MCP tools — eleven since 2026-10-12-headless-screenshot»
  replace-with: «its tools the eleven verbs, `help` last»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «one verb table of ten verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` asked of the held instance»
  source: .andromeda/architecture.md:133 «first the seven asked of the held instance, in order: `snapshot` (no argument; result `text`), `click` (`id`), `type` (`id`, `text`), `press`»
  replace-with: «one verb table of eleven verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` asked of the held instance»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and eight refusal causes each with a fixed remedy, all eight returned today»
  source: .andromeda/architecture.md:133 «All nine are returned by code: `validate` returns `unknown-verb` and `malformed` only, and the executor returns the other seven»
  replace-with: «and nine refusal causes each with a fixed remedy, all nine returned today»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «the driver's ten verbs as MCP tools over stdio»
  source: .andromeda/security-plan.md:203 «ion mcp` role also serves the driver's verbs as MCP tools — eleven since 2026-10-12-headless-screenshot»
  replace-with: «the driver's eleven verbs as MCP tools over stdio»
- leaf: CLAUDE.md:110   class: omission
  leaf-text: «a `snapshot` result the screen text itself and every other the JSON line»
  source: .andromeda/architecture.md:133 «since 2026-10-12-headless-screenshot an accepted `screenshot` two blocks, an `image` block (`data` the PNG's base64, `mimeType` `image/png`) then a text block holding the shot's line without `png`»
  replace-with: «a `snapshot` result the screen text itself, an accepted `screenshot` an `image` block then a text block, and every other the JSON line»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «bounded at a 65,536-byte request line and a 1,048,576-byte result»
  source: .andromeda/architecture.md:133 «since 2026-10-12-headless-screenshot a result that holds an image is bounded by its whole written form instead, a text-only result by its text as before (`ToolResult::bounded`)»
  replace-with: «bounded at a 65,536-byte request line and a 1,048,576-byte result — one holding an image by its whole written form —»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «, and — still to come — the headless screenshot.»
  source: .andromeda/architecture.md:146 «Since 2026-10-12-headless-screenshot both carriers also take a shot of the screen out: the socket's answer to a `screenshot` call holds its PNG as base64 inside the JSON line»
  replace-with: «, and the headless screenshot (built — `Harness::screenshot` / `screenshot_rect` paint the viewport with no display into `VelloCpuImageRenderer` over opaque white, and the driver's `screenshot`, the seventh instance verb on the command line and over MCP, returns the screen or one element's visible part as a PNG — no step, nothing changed, an app with no `sans-serif` family refused `fonts-unavailable` — its pixels in band, base64 in the answer line and an MCP `image` block, ratified by the founder (2026-10-12); a password input paints masked in every document, an engine change ratified by the founder (2026-10-12); proven on the stand in both layout modes, a whole shot an 800 × 600 PNG equal to the harness's raster and byte-identical across processes).»
