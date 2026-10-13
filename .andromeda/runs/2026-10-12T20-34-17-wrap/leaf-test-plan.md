group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:13   class: omission
  leaf-text: «inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`). See»
  source: .andromeda/test-plan.md:47 «a paint with no display — `Harness::screenshot` / `screenshot_rect` into `VelloCpuImageRenderer`, composited over opaque white, at the harness's own time with no tick or pump»
  replace-with: «inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`), and since 2026-10-12-headless-screenshot a paint with no display — `screenshot` / `screenshot_rect` into `VelloCpuImageRenderer`, composited over opaque white, at the harness's own time with no tick or pump, a document with no font refused `FontsUnavailable`. See»

- leaf: .claude/docs/tests-summary.md:17   class: omission
  leaf-text: «and since 2026-10-12-self-description a `help` call, answered by the server, one `tool_call` line with no `command` line beneath it —»
  source: .andromeda/test-plan.md:103 «`screenshot` a word of the same two closed sets, no span field added — a shot's `command` line records `verb` (and `cause` when refused), never the image, its size or its dimensions»
  replace-with: «and since 2026-10-12-self-description a `help` call, answered by the server, one `tool_call` line with no `command` line beneath it; since 2026-10-12-headless-screenshot `screenshot` a word of the same two closed sets, no span field added — a shot's `command` line records `verb` (and `cause` when refused), never the image, its size or its dimensions, and the host's and the `mcp` role's stderr at `trace` hold 0 occurrences of a shot's first 64 base64 characters under both builds —»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «and the driver's calls run on a held session by stable id (`stand_act_*`, twelve files:»
  source: .andromeda/test-plan.md:26 «and, the thirteenth, since 2026-10-12-headless-screenshot, `stand_act_screenshot` 6 (the whole screen a PNG of the harness's raster with text painting, an element shot its bounds rounded outward»
  replace-with: «and the driver's calls run on a held session by stable id (`stand_act_*`, thirteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot — a shot a PNG of the harness's raster, the whole screen or an element's bounds rounded outward, refused `fonts-unavailable` with no usable font, a typed password never leaving as pixels, a shot changing nothing:»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a step returning the node whose pressed, selected or invalid reading it changed) — asserted unconditionally, no font skip;»
  source: .andromeda/test-plan.md:26 «since 2026-10-12-headless-screenshot `stand_screenshot_repeat` 1 and its `#[ignore]` child — the test binary re-run twice, two children and the parent printing byte-identical PNGs for 12 shots»
  replace-with: «a step returning the node whose pressed, selected or invalid reading it changed), since 2026-10-12-headless-screenshot `stand_screenshot_repeat` (one `#[ignore]` child: the test binary re-run twice, two children and the parent printing byte-identical PNGs for 12 shots, both layout modes and both builds) and `password_painted_masked` (an engine check with no `stand_` prefix: two password values of one length paint the same frame, a password input its bullets and a focused one its caret) — asserted unconditionally, no font skip;»

- leaf: .claude/docs/tests-summary.md:23   class: stale
  leaf-text: «11 inline unit tests in `settle.rs`, the crate's first»
  source: .andromeda/test-plan.md:19 «17 since 2026-10-12-headless-screenshot, 6 more in `screenshot.rs` (packages/blitz-test-harness/src/screenshot.rs:140-244)»
  replace-with: «17 inline unit tests, 6 of them in `screenshot.rs` since 2026-10-12-headless-screenshot (the whole viewport painted at its device size, what the page leaves unpainted opaque white, a rectangle clipped or refused, a document with no font refused, a shot the same twice) and 11 in `settle.rs`, the crate's first»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «66 inline unit tests in twelve files (59 in eleven until»
  source: .andromeda/test-plan.md:21 «- **escher-driver:** 72 inline unit tests in thirteen files (66 in twelve until 2026-10-12-headless-screenshot, 59 in eleven until 2026-10-12-self-description»
  replace-with: «72 inline unit tests in thirteen files (66 in twelve until 2026-10-12-headless-screenshot, 59 in eleven until»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`refusal.rs` 3 (the eight causes,»
  source: .andromeda/test-plan.md:21 «refusal.rs 3 (the causes are named in order — nine since 2026-10-12-headless-screenshot, `fonts-unavailable` last;»
  replace-with: «`refusal.rs` 3 (the nine causes, `fonts-unavailable` last since 2026-10-12-headless-screenshot,»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`schema.rs` 6 (the ten verbs at their levels — `help` at the tool level since 2026-10-12-self-description —»
  source: .andromeda/test-plan.md:21 «schema.rs 6 (the verbs are named in order, each at its level — seven instance-level (`screenshot` the seventh, after `scroll`, since 2026-10-12-headless-screenshot)»
  replace-with: «`schema.rs` 6 (the eleven verbs at their levels — `screenshot` the seventh instance verb, after `scroll`, since 2026-10-12-headless-screenshot, `help` at the tool level since 2026-10-12-self-description —»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp.rs` 10 (each line read by its rules»
  source: .andromeda/test-plan.md:21 «mcp.rs 11 (10 until 2026-10-12-headless-screenshot; each line is read by its rules»
  replace-with: «`mcp.rs` 11 (10 until 2026-10-12-headless-screenshot; each line read by its rules»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the listing as the ten verbs, nine until 2026-10-12-self-description»
  source: .andromeda/test-plan.md:21 «the listing is the eleven verbs of the table — ten until 2026-10-12-headless-screenshot, nine until 2026-10-12-self-description»
  replace-with: «the listing as the eleven verbs, ten until 2026-10-12-headless-screenshot, nine until 2026-10-12-self-description»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «a result over the answer bound replaced)»
  source: .andromeda/test-plan.md:21 «since 2026-10-12-headless-screenshot a result that holds an image is bounded by its whole written form, and a shot is written as its image block then its text block, a refused one as one text block»
  replace-with: «a result over the answer bound replaced; since 2026-10-12-headless-screenshot a result holding an image bounded by its whole written form, a shot written as its image block then its text block, a refused one as one text block)»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «every text in it is its tables'); `host.rs` holds none»
  source: .andromeda/test-plan.md:21 «since 2026-10-12-headless-screenshot, image.rs 5 (the hand base64 reads the RFC 4648 test vectors and writes every sextet of the alphabet; a PNG decodes to the pixels it was given and holds no time or text chunk;»
  replace-with: «every text in it is its tables') and, since 2026-10-12-headless-screenshot, `image.rs` 5 (the hand base64 reads the RFC 4648 test vectors and writes every sextet of the alphabet; a PNG decodes to the pixels it was given and holds no time or text chunk; pixels that do not fill the size write nothing); `host.rs` holds none»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`cli_commands` (10, 8 until 2026-10-12-self-description — since then»
  source: .andromeda/test-plan.md:200 «`cli_commands` (11, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description): each of the four tasks starts, answers and stops; each of the seven instance-level verbs»
  replace-with: «`cli_commands` (11, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description — since 2026-10-12-headless-screenshot a `screenshot` answering one line whose `png`, decoded by a hand base64 reader, equals the PNG `Session::run` returns in process; since 2026-10-12-self-description»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_server` (4, 2 until 2026-10-12-self-description:»
  source: .andromeda/test-plan.md:201 «`mcp_server` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description): the Counter flow by tool calls alone»
  replace-with: «`mcp_server` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description: since 2026-10-12-headless-screenshot a `screenshot` tool call answered with an image block then a text block, a refused one a single text block;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`tools/list` offering the ten verbs»
  source: .andromeda/test-plan.md:201 «`tools/list` offering the verbs in order — eleven since 2026-10-12-headless-screenshot —»
  replace-with: «`tools/list` offering the eleven verbs»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`stand_mcp_tools` (4, 2 until 2026-10-12-self-description:»
  source: .andromeda/test-plan.md:201 «`stand_mcp_tools` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description)»
  replace-with: «`stand_mcp_tools` (5, 4 until 2026-10-12-headless-screenshot, 2 until 2026-10-12-self-description: since 2026-10-12-headless-screenshot a `screenshot` tool call answering its image block then its text block and parenting its `command` span;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the twelve `stand_act_*` files (40 tests, 2 ignored children)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_states` 3 · `stand_act_screenshot` 6 (46; 40 over twelve files until 2026-10-12-headless-screenshot, 37 over eleven until 2026-10-10-snapshot-states»
  replace-with: «the thirteen `stand_act_*` files (46 tests, 2 ignored children; `stand_act_screenshot` 6 the thirteenth since 2026-10-12-headless-screenshot)»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «one `tool_call` line per call, under both builds).»
  source: .andromeda/test-plan.md:183 «`host_log` and `mcp_log` also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under the `-p seven_guis` and the workspace build»
  replace-with: «one `tool_call` line per call, under both builds); since 2026-10-12-headless-screenshot `host_log` and `mcp_log` also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under both builds.»

- leaf: .claude/docs/tests-summary.md:26   class: omission
  leaf-text: «Pixel tests on a CPU buffer (box-shadow expectations from Chromium, 1px AA tolerance).»
  source: .andromeda/test-plan.md:52 «since 2026-10-12-headless-screenshot frames are also compared whole — a shot's PNG equal to the harness's raster (`stand_act_screenshot`)»
  replace-with: «Pixel tests on a CPU buffer (box-shadow expectations from Chromium, 1px AA tolerance); since 2026-10-12-headless-screenshot frames are also compared whole — a shot's PNG equal to the harness's raster (`stand_act_screenshot`), byte-identical across two child runs and the parent (`stand_screenshot_repeat`), a password field's pixels equal for two values of one length (`password_painted_masked`).»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 821 · 0 · 11 over 171 result lines at 2026-10-12-self-description (»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-12-headless-screenshot: 174 result lines, 849 passed · 0 failed · 12 ignored (+28 passed: three new result lines — blitz-tests' `password_painted_masked` 4, `stand_act_screenshot` 6»
  replace-with: «`cargo test --workspace` 849 · 0 · 12 over 174 result lines at 2026-10-12-headless-screenshot (+28 passed: three new result lines — `password_painted_masked` 4, `stand_act_screenshot` 6 and `stand_screenshot_repeat` 1, +1 ignored, its child — escher-driver +6, now 72, blitz-test-harness +6, now 17, blitz-dom +2 (`font_reading_tests`), `stand_mcp_tools` +1, seven_guis' `cli_commands` +1 and `mcp_server` +1; `run stand` 139 passed · 7 ignored over 36 files, `run all` not re-read); 821 · 0 · 11 over 171 result lines at 2026-10-12-self-description (»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «the twelve driver-action checks `stand_act_*` and `stand_mcp_tools` (18 readers; `stand_mcp_masked` declares none)»
  source: .andromeda/test-plan.md:101 «`stand_screenshot_repeat`, which is no `stand_act_*` file (20 readers, measured at 2026-10-12-headless-screenshot, where 18 stood; `stand_mcp_masked` declares none)»
  replace-with: «the thirteen driver-action checks `stand_act_*`, `stand_mcp_tools` and `stand_screenshot_repeat` (20 readers; `stand_mcp_masked` and `password_painted_masked` declare none)»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «the driver's call builders `click` · `type_into` · `press` · `advance` · `scroll` · `snapshot`, the outcome reader `act`,»
  source: .andromeda/test-plan.md:101 «since 2026-10-12-headless-screenshot, `screenshot()` and `screenshot_of(id)`, `act(session, call)` returning `Acted { settled, busy, diff, advanced_ms, in_view }`»
  replace-with: «the driver's call builders `click` · `type_into` · `press` · `advance` · `scroll` · `snapshot` · `screenshot` · `screenshot_of`, the outcome reader `act`, the shot reader `shot`,»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «probe widgets).»
  source: .andromeda/test-plan.md:52 «a check decodes a PNG through blitz-tests' `png` dev-dependency, or, in seven_guis, which has none, by a hand base64 reader and the PNG header»
  replace-with: «probe widgets). A check decodes a PNG through blitz-tests' `png` dev-dependency, or, in seven_guis, which has none, by a hand base64 reader and the PNG header.»

- leaf: .claude/rules/verification-harness.md:38   class: omission
  leaf-text: «the snapshot-style assertion surface.»
  source: .andromeda/test-plan.md:88 «`screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` in the private module `screenshot`»
  replace-with: «the snapshot-style assertion surface. Since 2026-10-12-headless-screenshot `screenshot()` and `screenshot_rect(PixelRect)` return a `Raster` or a `ScreenshotError` (`FontsUnavailable`, `EmptyRect`): each resolves at the harness's own time (no tick, no pump), paints the viewport at its scale and device size into `VelloCpuImageRenderer` over opaque white, clips a rectangle to the viewport (an empty one `EmptyRect`), and refuses a document whose `can_paint_text` reads false with nothing painted.»
