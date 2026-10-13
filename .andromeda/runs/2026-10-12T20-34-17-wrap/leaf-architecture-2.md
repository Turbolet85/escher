group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:6   class: stale
  leaf-text: «the **MCP server** that serves the ten verbs as MCP tools»
  source: .andromeda/architecture.md:133 «`tools/list` answers the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot), `help` last»
  replace-with: «the **MCP server** that serves the eleven verbs as MCP tools»

- leaf: .claude/docs/services/escher-driver.md:11   class: omission
  leaf-text: «it was already in both graphs through dioxus-html, so the lock changed by one line and no package.»
  source: .andromeda/architecture.md:101 «since 2026-10-12-headless-screenshot, `png` (the workspace's, resolved 0.18.1, already in the graph — no package entered `Cargo.lock`), the encoder of a shot»
  replace-with: «it was already in both graphs through dioxus-html, so the lock changed by one line and no package — and, the fifth since 2026-10-12-headless-screenshot, `png` (the workspace's, no feature, resolved 0.18.1, already in the graph — no package entered `Cargo.lock`), the encoder of a shot, its base64 written by hand with no crate taken for it.»

- leaf: .claude/docs/services/escher-driver.md:12   class: omission
  leaf-text: «The executor, the session, the schema, the validation and the JSON writer read no clock and print nothing»
  source: .andromeda/architecture.md:133 «its executor, session, schema, validation and JSON writer and, since 2026-10-12-headless-screenshot, its private `image` encoder print nothing and read no clock or environment»
  replace-with: «The executor, the session, the schema, the validation, the JSON writer and, since 2026-10-12-headless-screenshot, the private `image` encoder read no clock or environment and print nothing»

- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «`click`, `type` and `scroll` look their id up on the screen as it reads NOW»
  source: .andromeda/architecture.md:133 «`click`, `type`, `scroll` and, since 2026-10-12-headless-screenshot, a `screenshot` given an `id` resolve their id first, on the screen as it reads now»
  replace-with: «`click`, `type`, `scroll` and, since 2026-10-12-headless-screenshot, a `screenshot` given an `id` look their id up on the screen as it reads NOW»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «it is never refused for that. Each acting verb is one settled step»
  source: .andromeda/architecture.md:133 «`screenshot` (since 2026-10-12-headless-screenshot) dispatches nothing and is no step: with no `id` it returns the whole screen, and with an `id` it crops the frame»
  replace-with: «it is never refused for that; `screenshot` (since 2026-10-12-headless-screenshot) dispatches nothing and is no step — with no `id` it returns the whole screen, with an `id` it crops the frame to the part of the element's bounds that can be seen, rounded outward to whole pixels, a crop of no pixel refused `off-screen`; an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted; the frame is the harness's `screenshot`, written as a PNG by the private `image::png`, and a shot changes nothing (`has_changes` stays false, focus and the next snapshot unchanged). Each acting verb is one settled step»

- leaf: .claude/docs/services/escher-driver.md:19   class: stale
  leaf-text: «`Scrolled { settled, busy, diff, in_view }`; `diff` is dioxus-native-dom's»
  source: .andromeda/architecture.md:133 «`Shot { width: u32, height: u32, png: Vec<u8> }` (the last since 2026-10-12-headless-screenshot, packages/escher-driver/src/execute.rs:63-71)»
  replace-with: «`Scrolled { settled, busy, diff, in_view }` · `Shot { width, height, png }` (the last since 2026-10-12-headless-screenshot); `diff` is dioxus-native-dom's»

- leaf: .claude/docs/services/escher-driver.md:20   class: omission
  leaf-text: «`start` answers `label`, `pid`, `idle_expiry_s`, `status` adds `served`»
  source: .andromeda/architecture.md:133 «a `screenshot`'s outcome, since 2026-10-12-headless-screenshot, `{"width":…,"height":…,"png":"<base64>"}`, the PNG's base64 written by hand in the private `image::base64`»
  replace-with: «a `screenshot`'s outcome `{"width":…,"height":…,"png":"<base64>"}` (since 2026-10-12-headless-screenshot), the PNG's base64 written by hand in the private `image::base64` (RFC 4648 §4, padded) — no base64 crate taken; `start` answers `label`, `pid`, `idle_expiry_s`, `status` adds `served`»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «`VERBS` — the verb table, ten verbs in three levels»
  source: .andromeda/architecture.md:133 «`VERBS` lists eleven verbs in three levels (`Level` is `Instance` · `Session` · `Tool`»
  replace-with: «`VERBS` — the verb table, eleven verbs in three levels»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «`scroll` (`id`; the sixth, ratified by the founder, 2026-10-07), then `start`»
  source: .andromeda/architecture.md:133 «`screenshot` (one optional `id`, `ArgKind::Id`; result `width` (count), `height` (count) and `png` (`FieldKind::Png`, base64 text), all always present)»
  replace-with: «`scroll` (`id`; the sixth, ratified by the founder, 2026-10-07), `screenshot` (one optional `id`; result `width` (count), `height` (count) and `png` (`FieldKind::Png`, base64 text), all always present; the seventh, since 2026-10-12-headless-screenshot, its help stating two measured limits — an element shot is cropped to the part of its bounds that can be seen, and in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time), then `start`»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «`help`'s fields are of the kinds `Words` · `Entries` (nine `FieldKind`s)»
  source: .andromeda/architecture.md:133 «then `Png`, the kind of `screenshot`'s `png` field (base64 text), since 2026-10-12-headless-screenshot — ten kinds»
  replace-with: «`help`'s fields are of the kinds `Words` · `Entries`, and `screenshot`'s `png` of `Png` (base64 text, since 2026-10-12-headless-screenshot) — ten `FieldKind`s»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «10 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes and 13 session errors with their remedies, 4 exit statuses»
  source: .andromeda/architecture.md:133 «11 verbs, 13 node keys, 12 key names, 3 busy classes, 9 causes, 4 exit statuses, 13 session errors (10 verbs and 8 causes until 2026-10-12-headless-screenshot)»
  replace-with: «11 verbs, 13 node keys, 12 key names, 3 busy classes, 9 causes and 13 session errors with their remedies, 4 exit statuses (10 verbs and 8 causes until 2026-10-12-headless-screenshot)»

- leaf: .claude/docs/services/escher-driver.md:25   class: stale
  leaf-text: «`Command` is `Snapshot` · `Click` · `Type` · `Press` · `Advance` · `Scroll`»
  source: .andromeda/architecture.md:133 «`Command` is exactly `Snapshot` · `Click { id }` · `Type { id, text }` · `Press { key, shift }` · `Advance { ms: u32 }` · `Scroll { id }` · `Screenshot { id: Option<String> }`»
  replace-with: «`Command` is `Snapshot` · `Click` · `Type` · `Press` · `Advance` · `Scroll` · `Screenshot { id: Option<String> }` (the last since 2026-10-12-headless-screenshot)»

- leaf: .claude/docs/services/escher-driver.md:26   class: stale
  leaf-text: «`CAUSES` is closed at eight: `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable`»
  source: .andromeda/architecture.md:133 «`Cause` is closed at nine, listed in `CAUSES` — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable` · `fonts-unavailable`»
  replace-with: «`CAUSES` is closed at nine: `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable` · `fonts-unavailable` (the ninth since 2026-10-12-headless-screenshot: meaning "the app this session holds has no font it can paint text with", remedy "boot the app with a font it can decode")»

- leaf: .claude/docs/services/escher-driver.md:26   class: stale
  leaf-text: «All eight are returned today — `unknown-verb` and `malformed` by `validate`, the other six by the executor.»
  source: .andromeda/architecture.md:133 «All nine are returned by code: `validate` returns `unknown-verb` and `malformed` only, and the executor returns the other seven»
  replace-with: «All nine are returned today — `unknown-verb` and `malformed` by `validate`, the other seven by the executor, `fonts-unavailable` from `screenshot` alone, checked before anything is painted.»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «`tools/list` → the ten verbs of `VERBS` in order, `help` last»
  source: .andromeda/architecture.md:133 «`tools/list` answers the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot), `help` last»
  replace-with: «`tools/list` → the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot), `help` last»

- leaf: .claude/docs/services/escher-driver.md:27   class: omission
  leaf-text: «a result's text ≤ 1,048,576 bytes (past it `AnswerTooLarge`'s result)»
  source: .andromeda/architecture.md:133 «since 2026-10-12-headless-screenshot a result that holds an image is bounded by its whole written form instead, a text-only result by its text as before (`ToolResult::bounded`)»
  replace-with: «a result's text ≤ 1,048,576 bytes (past it `AnswerTooLarge`'s result) — since 2026-10-12-headless-screenshot a result that holds an image bounded by its whole written form instead, a text-only result by its text as before (`ToolResult::bounded`)»

- leaf: .claude/docs/services/escher-driver.md:27   class: stale
  leaf-text: «A result is `{content:[{type:"text",text}], isError}` — `snapshot` the screen text itself, any other accepted call its JSON line, a refusal or a session error its JSON with `isError: true`»
  source: .andromeda/architecture.md:133 «an accepted `screenshot` two blocks, an `image` block (`data` the PNG's base64, `mimeType` `image/png`) then a text block holding the shot's line without `png`»
  replace-with: «A result is `{content:[…], isError}` — an accepted `snapshot` one text block, the screen text itself; since 2026-10-12-headless-screenshot an accepted `screenshot` two blocks, an `image` block (`data` the PNG's base64, `mimeType` `image/png`) then a text block holding the shot's line without `png` (private `json::shot_without_image`), `isError` false; any other accepted call one text block, its JSON line; a refusal, a refused shot included, or a session error one text block, its JSON with `isError: true`»

- leaf: .claude/docs/services/escher-driver.md:29   class: stale
  leaf-text: «whose twelve `stand_act_*` checks drive the schema through `Session::run`»
  source: .andromeda/architecture.md:263 «read through `mod session_common;` by four of the session checks, by `stand_settle`, by the thirteen `stand_act_*`»
  replace-with: «whose thirteen `stand_act_*` checks drive the schema through `Session::run`»

- leaf: .claude/docs/services/escher-driver.md:32   class: omission
  leaf-text: «text from a page, a call or a screen, a new carrier or a new exit still widens.»
  source: .andromeda/architecture.md:101 «since 2026-10-12-headless-screenshot a shot's pixels cross in band on those carriers — its PNG as base64 inside the answer on the session socket and the command line's stdout»
  replace-with: «text from a page, a call or a screen, a new carrier or a new exit still widens. Since 2026-10-12-headless-screenshot a shot's pixels cross in band on those carriers — its PNG as base64 inside the answer on the session socket and the command line's stdout, and as an MCP `image` block on the `mcp` role's stdio — screen content and no closed value set, with no new carrier, exit or path argument and the 1,048,576-byte bounds unchanged, a bigger image ending `oversize` / `AnswerTooLarge`: a boundary widening ratified by the founder (2026-10-12).»

- leaf: .claude/docs/services/escher-driver.md:35   class: stale
  leaf-text: «one of the tool's own texts says a measured limit — `covered`'s meaning, the `contain: paint` reading;»
  source: .andromeda/architecture.md:133 «whose help states two measured limits — an element shot is cropped to the part of its bounds that can be seen, and in a build that paints overlay scrollbars a thumb painted within 0.7 s»
  replace-with: «two of the tool's own texts say measured limits — `covered`'s meaning, the `contain: paint` reading, and, since 2026-10-12-headless-screenshot, `screenshot`'s help, two: an element shot is cropped to the part of its bounds that can be seen, and in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time;»

- leaf: .claude/docs/services/escher-driver.md:39   class: stale
  leaf-text: «an `Outcome` is written by `to_json` alone — as a command's answer, and as an MCP tool result's text —»
  source: .andromeda/architecture.md:133 «a text block holding the shot's line without `png` (private `json::shot_without_image`, packages/escher-driver/src/json.rs:250-254)»
  replace-with: «an `Outcome` is written by `to_json` alone — as a command's answer, and as an MCP tool result's text; an accepted `screenshot` over MCP as an `image` block and its line without `png` (`json::shot_without_image`), since 2026-10-12-headless-screenshot —»

- leaf: .claude/docs/services/escher-driver.md:49   class: stale
  leaf-text: «`snapshot` records `verb` alone;»
  source: .andromeda/architecture.md:133 «`snapshot` and, since 2026-10-12-headless-screenshot, an accepted `screenshot` record `verb` alone»
  replace-with: «`snapshot` and, since 2026-10-12-headless-screenshot, an accepted `screenshot` record `verb` alone;»

- leaf: .claude/docs/services/escher-driver.md:49   class: omission
  leaf-text: «Never fielded: `in_view`, `advanced_ms`, the screen text or its length, the label,»
  source: .andromeda/architecture.md:133 «Never fielded: `in_view`, `advanced_ms`, the screen text or its length, a shot's image, its size or its dimensions, the session's label»
  replace-with: «Never fielded: `in_view`, `advanced_ms`, the screen text or its length, a shot's image, its size or its dimensions, the label,»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «twelve files, 40 tests and two ignored children, both layout modes:»
  source: .andromeda/architecture.md:263 «since 2026-10-12-headless-screenshot a thirteenth driver-action file, `stand_act_screenshot`»
  replace-with: «thirteen files — the first twelve 40 tests and two ignored children — both layout modes:»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «no id, name or typed text, none at the default level).»
  source: .andromeda/architecture.md:263 «`stand_screenshot_repeat` — a shot of one state byte-identical in two child runs and in the parent, 12 shots in both layout modes»
  replace-with: «no id, name or typed text, none at the default level), and since 2026-10-12-headless-screenshot `stand_act_screenshot` (the whole screen a PNG of the harness's raster with text painted, an element shot its bounds rounded outward, a shot refused as every id-addressed call is, an app with no usable font refused `fonts-unavailable` with nothing changed, a typed password never leaving as pixels and a shot changing nothing); beside them `stand_screenshot_repeat` (a shot of one state byte-identical in two child runs and in the parent, 12 shots in both layout modes).»

- leaf: .claude/docs/stack.md:17   class: omission
  leaf-text: «std, flexbox, grid, block_layout»
  source: .andromeda/architecture.md:48 «features std, flexbox, flexbox_balance, grid, block_layout, content_size, calc, detailed_layout_info»
  replace-with: «std, flexbox, flexbox_balance, grid, block_layout»

- leaf: .claude/docs/stack.md:22   class: stale
  leaf-text: «accesskit_xplat (windows 0.35, macos 0.27, unix 0.23, android 0.8)»
  source: .andromeda/architecture.md:53 «accesskit_xplat with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.24.0, accesskit_android 0.9.0 per target»
  replace-with: «accesskit_xplat (windows 0.35, macos 0.27, unix 0.24, android 0.9)»

- leaf: .claude/docs/stack.md:26   class: omission
  leaf-text: «png 0.18, html-escape»
  source: .andromeda/architecture.md:57 «since 2026-10-12-headless-screenshot png (resolved 0.18.1) is also taken by escher-driver, the encoder of a shot, and by blitz-tests as a dev-dependency, no package entering `Cargo.lock`»
  replace-with: «png 0.18 (since 2026-10-12-headless-screenshot also escher-driver's encoder of a shot and a blitz-tests dev-dependency, resolved 0.18.1, no package entering `Cargo.lock`; no `base64` crate taken), html-escape»

- leaf: .claude/docs/stack.md:44   class: omission
  leaf-text: «escher-driver (the driver session), test-that 0.5.2, usvg;»
  source: .andromeda/architecture.md:64 «and, since 2026-10-12-headless-screenshot, png (the workspace's), which `stand_act_screenshot` and `stand_screenshot_repeat` decode a shot with»
  replace-with: «escher-driver (the driver session), test-that 0.5.2, usvg and, since 2026-10-12-headless-screenshot, png, which `stand_act_screenshot` and `stand_screenshot_repeat` decode a shot with;»
