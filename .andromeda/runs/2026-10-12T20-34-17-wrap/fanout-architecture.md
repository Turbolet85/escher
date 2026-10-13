proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`VERBS` lists eleven verbs in three levels — first the seven asked of the held instance, `screenshot` the seventh after `scroll` (the row `SCREENSHOT`: one optional `id`, `ArgKind::Id`; result `width` (count), `height` (count) and `png` (`FieldKind::Png`, base64 text), all `always`; its help states two measured limits — an element shot is cropped to the part of its bounds that can be seen, and in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time) — then the three of the session and `help` last; `Command` gains `Screenshot { id: Option<String> }`; `FieldKind` gains `Png` — ten kinds"
    sidecar: "arch §Standard Contracts → Driver session: verbs ten → eleven, instance verbs six → seven (`screenshot`), `Command::Screenshot`, field kinds nine → ten (`png`) — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (`VERBS` gains `SCREENSHOT`, `Level::Instance`, after `scroll`; `FieldKind::Png`; `Command::Screenshot { id: Option<String> }`) and Schema / config (eleven verbs, seven of the instance; field kinds nine → ten); Counts `verbs10` architecture.md:133 ×2 and `inst6` architecture.md:133."
    basis: "packages/escher-driver/src/schema.rs:415-448; packages/escher-driver/src/command.rs:138-142"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`describe()`'s self-description counts 11 verbs, 13 node keys, 12 key names, 3 busy classes, 9 causes, 4 exit statuses and 13 session errors (was 10 verbs and 8 causes)"
    sidecar: "arch §Standard Contracts → Driver session: the self-description's counts — 11 verbs, 9 causes — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Schema / config (eleven verbs; causes eight → nine) and Outcome smoke (`help` answers eleven verbs and nine causes); the same sentence of architecture.md:133 restates the verb and cause counts."
    basis: "packages/escher-driver/src/schema.rs:415-448"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "the MCP server's `tools/list` answers the eleven verbs of `VERBS` in order, `help` last (`screenshot` after `scroll`)"
    sidecar: "arch §Standard Contracts → Driver session: MCP tool listing ten → eleven verbs — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Schema / config (eleven verbs) and Expected amendments (matrix#v010-13 pin renamed `the_listing_is_the_eleven_verbs_of_the_table`); architecture.md:133 states the listing as ten."
    basis: "packages/escher-driver/src/mcp.rs"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → CLIs"
    change: "`escher-session`'s verbs are the eleven of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, since 2026-10-12-headless-screenshot `screenshot [--id]`, `start <task>`, `status`, `stop` and `help`"
    sidecar: "arch §Standard Contracts → CLIs: `escher-session` verbs ten → eleven, `screenshot [--id]` — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Schema / config (eleven verbs; `screenshot` takes one optional `id`) and Outcome v010-07 (the binary answers `screenshot` status 0); architecture.md:138 lists the ten."
    basis: "packages/escher-driver/src/cli.rs"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Cause` is closed at nine, listed in `CAUSES` — the eight as before, then `fonts-unavailable` last (meaning \"the app this session holds has no font it can paint text with\", remedy \"boot the app with a font it can decode\"); all nine are returned by code — `validate` returns `unknown-verb` and `malformed`, the executor the other seven, `fonts-unavailable` from `screenshot` alone, checked before any paint"
    sidecar: "arch §Standard Contracts → Driver session: refusal causes eight → nine (`fonts-unavailable`), the executor returning seven — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (`Cause::FontsUnavailable`) and Schema / config (causes eight → nine, `fonts-unavailable` last, its meaning and remedy); Coverage (`fonts-unavailable` checked before any paint, `cause=fonts-unavailable` on the span); architecture.md:133 states the set closed at eight and the executor returning six."
    basis: "packages/escher-driver/src/refusal.rs"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Outcome` is exactly `Screen` · `Acted` · `Advanced` · `Scrolled` · `Shot { width: u32, height: u32, png: Vec<u8> }`; `screenshot` with no `id` returns the whole screen and with an `id` resolves it as `click`, `type` and `scroll` do (`stale` / `not-found`, nothing run) and crops the frame to the part of the element's bounds that can be seen, rounded outward to whole pixels (private `crop`), a crop of no pixel refused `off-screen`; an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted; the PNG is written by the private `image::png` (no time or text chunk), and a shot changes nothing — `has_changes` stays false, focus and the next snapshot unchanged; on the stand a whole shot is an 800 × 600 PNG equal to the harness's raster"
    sidecar: "arch §Standard Contracts → Driver session: `Outcome::Shot` and the executor's `screenshot` (id resolution, crop, `off-screen` on an empty crop, `fonts-unavailable`) — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (`Outcome::Shot { width, height, png: Vec<u8> }`, private `crop`, `image::png`), Deviations (an element shot that crops to no pixel is refused `off-screen`), Harness / gate surface (`FontsUnavailable` with nothing painted) and Outcome (v010-07; a shot changes nothing; refusals hold nothing of the call)."
    basis: "packages/escher-driver/src/execute.rs:63-71; packages/escher-driver/src/execute.rs:382-403; packages/escher-driver/src/image.rs:3-15"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "the written forms gain the shot's line, `{\"width\":…,\"height\":…,\"png\":\"<base64>\"}`, written by `Outcome::to_json` through the crate's `json` writer, the base64 hand-written in the private `image::base64` (RFC 4648 §4, padded) — no base64 crate, and `serde_json` still writes nothing"
    sidecar: "arch §Standard Contracts → Driver session: the shot's JSON written form and the hand-written base64 — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Schema / config (written forms: the shot's line) and Dependencies (the `base64` crate and `serde_json` writing not used: the base64 is hand-written, RFC 4648 §4, padded)."
    basis: "packages/escher-driver/src/json.rs:242-248; packages/escher-driver/src/image.rs:19-37"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "an MCP tool result is `{content:[…], isError}` — an accepted `snapshot` the screen text itself, an accepted `screenshot` two blocks, `image` (`data` the base64, `mimeType` `image/png`) then `text` (the shot's line without `png`, private `json::shot_without_image`), `isError` false, any other accepted call its JSON line, and a refusal (a refused shot included) or a session error one text block with `isError: true`; a result that holds an image is bounded at 1,048,576 bytes by its whole written form, a text-only result by its text as before (`ToolResult::bounded`)"
    sidecar: "arch §Standard Contracts → Driver session: the MCP image block and the bound on an image result — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (private `ToolResult::shot`, an `image` field on `ToolResult`, `json::shot_without_image`), Schema / config (over MCP a shot is two content blocks, image then text; bound by the whole written form) and Deviations (MCP bound on an image result); architecture.md:133 states every result as one text block bounded by its text."
    basis: "packages/escher-driver/src/mcp.rs; packages/escher-driver/src/json.rs:250-254"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "the `command` span keeps its eight fields: a `screenshot` records `verb` (and `cause` when refused) and none of the other six, and the image, its size and its dimensions are never fielded; the `tool_call` span is unchanged, `tool` reading `screenshot`"
    sidecar: "arch §Standard Contracts → Driver session: what a shot records on the command span — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Schema / config (the command span keeps its eight fields; a shot records `verb`, and `cause` when refused, never the image, its size or its dimensions; `tool_call` unchanged) and Outcome (obs: 0 image bytes on stderr at `trace`, both builds)."
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Test harness"
    change: "blitz-test-harness also exports `PixelRect`, `Raster` and `ScreenshotError::{FontsUnavailable, EmptyRect}` from its private module `screenshot`; `Harness::screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `screenshot_rect(&mut self, PixelRect)` resolve at the harness's own time (no tick, no pump), paint the viewport at its scale and device size through blitz-paint into `VelloCpuImageRenderer` and composite the frame over opaque white (premultiplied source-over: each channel + (255 − alpha), alpha 255); a rectangle is clipped to the viewport and an empty one is `EmptyRect`; a document whose `can_paint_text` reads false is `FontsUnavailable` with nothing painted; no path prints, panics or logs; the harness's dependencies gain three — `blitz-paint` (`default-features = false`, so no `svg` and no `scrollbars` in a per-package build), `anyrender` and `anyrender_vello_cpu` (no `multithreading`)"
    sidecar: "arch §Standard Contracts → Test harness: `Harness::screenshot` / `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError`, three raster dependencies — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (the two methods and three public types, re-exported from lib.rs), Harness / gate surface (resolve at the harness's own time, `VelloCpuImageRenderer`, composite over white, clip and `EmptyRect`, `FontsUnavailable`, no print, panic or log) and Dependencies (blitz-test-harness: `blitz-paint` default-features off, `anyrender`, `anyrender_vello_cpu` without `multithreading`)."
    basis: "packages/blitz-test-harness/src/screenshot.rs:10-17; packages/blitz-test-harness/src/screenshot.rs:19-25; packages/blitz-test-harness/src/screenshot.rs:27-35; packages/blitz-test-harness/src/screenshot.rs:37-103"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Document core (blitz-dom)"
    change: "`BaseDocument` gains two public read-only methods since 2026-10-12-headless-screenshot: `can_paint_text(&self) -> bool` — whether the font context resolves a family for the `sans-serif` generic, through fontique's `generic_families` — and `layout_as_text_input(&self, node_id, text) -> Option<parley::Layout<TextBrush>>` — `text` laid out with a text input's editor styles, scale and direction in the document's own fonts, one unbroken line, a fresh `LayoutContext` per call, `None` for a node that is no text input; its one caller is blitz-paint's password mask"
    sidecar: "arch §Standard Contracts → Document core: `BaseDocument::can_paint_text` and `layout_as_text_input` — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (blitz-dom `can_paint_text` and `layout_as_text_input`, the latter not in the plan) and Deviations (the mask's layout lives in blitz-dom because blitz-paint holds no font context)."
    basis: "packages/blitz-dom/src/document.rs:1022-1035; packages/blitz-dom/src/document.rs:1037-1067"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Paint and shell"
    change: "a password `input` (`type` reading `password`) paints its value masked in every document and every renderer through blitz-paint, windowed included: `draw_text_input_text` branches to the private `draw_masked_input_text`, which paints bullets laid out by `BaseDocument::layout_as_text_input`, so two values of one length paint byte-identical frames, focused or not; the caret is painted at the bullet of its index in the input's caret colour (private `caret_color`), and the field scrolls only to keep the caret in its content box measured on the bullets; a click inside a field holding a value still places the caret by the real characters' widths (blitz-dom's pointer path, unchanged), so which bullet it lands on can depend on them — unmeasured, left to a route owner"
    sidecar: "arch §Standard Contracts → Paint and shell: a password input paints masked, with the click-placed caret limit — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Symbols / APIs (`draw_masked_input_text`, `caret_color`, the branch in `draw_text_input_text`; every document, every renderer through blitz-paint), Deviations (the field scrolls on the bullets), Insufficient fixes (the click-placed caret index) and the `pwclear` zero: architecture.md:129 carries no paint claim, so this is an addition."
    basis: "packages/blitz-paint/src/render.rs:902-906; packages/blitz-paint/src/render.rs:965-1039; tests/blitz-tests/tests/password_painted_masked.rs:84-131"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → Painting"
    change: "a fourth direct engine change for every document, beside the three under Scrolling and input: a password input paints masked — blitz-paint's mask over blitz-dom's `layout_as_text_input` — on the founder's fork 2 «Движок маскирует» (2026-10-12, relayed verbatim by the overseer, inputs#I1, inputs#I2); the hypothesis that the engine painted a password's characters like any text input's was confirmed on a frame before the change; its merge surface is the two engine files `render.rs` and `document.rs`"
    sidecar: "arch §Established Decisions → Painting: a password paints masked in every document — engine behaviour change, founder fork 2 — 2026-10-12-headless-screenshot"
    rationale: "Report Expected amendments (arch §Established Decisions: a password paints masked in every document — engine behaviour change, the founder's fork 2, inputs#I1, inputs#I2), Spec claims (the CARRY hypothesis confirmed on a frame) and Cross-project claims (inputs#I4 row 1: the merge surface, `document.rs` now with two methods)."
    basis: "packages/blitz-paint/src/render.rs:965-1039; packages/blitz-dom/src/document.rs:1037-1067"
  - detector: D-arch-decisions
    severity: escalate
    section: "§Established Decisions → Driver session"
    change: "since 2026-10-12-headless-screenshot a shot's pixels cross in-band — its PNG as base64 inside the answer on the session socket and the command line's stdout, and as an MCP `image` block on the `mcp` role's stdio — screen content, an open value set, on the ratified carriers, the 1,048,576-byte answer bound unchanged (largest whole-screen answer 22,404 bytes, 2.1 % of it): a boundary widening on the founder's fork 1 «В самом ответе» (2026-10-12, relayed verbatim by the overseer, inputs#I1, inputs#I2)"
    sidecar: "arch §Established Decisions → Driver session: the shot's image in-band on socket, stdout and MCP stdio — boundary widening, founder fork 1 — 2026-10-12-headless-screenshot"
    rationale: "Report Expected amendments (arch §Established Decisions → Driver session: the image in-band on socket, stdout and MCP stdio — Boundary widening, the founder's fork 1 relayed verbatim by the overseer, inputs#I1, inputs#I2), Schema / config (bounds unchanged) and Outcome basis (22,404 bytes, 2.1 % of the bound); pixels are no closed value set, so the 2026-10-10 closed-field rule does not cover them and the widening needs Validate's halt."
    basis: "packages/escher-driver/src/json.rs:242-248; packages/escher-driver/src/mcp.rs"
  - detector: D-arch-decisions
    severity: escalate
    section: "§Occupied Resources → Network ports and listeners"
    change: "the session socket's answer to a call also carries a shot — its PNG as base64 inside the JSON line — beside ids, names, values, snapshot text and diff; still the one listener and no port, the wire's bounds unchanged"
    sidecar: "arch §Occupied Resources → Network ports and listeners: the socket answer carries a shot's PNG — 2026-10-12-headless-screenshot"
    rationale: "The crossing claim of §Established Decisions → Driver session is restated in the listener registry; Report Changes → Schema / config (the shot's line; the wire's 1,048,576-byte answer bound unchanged) and Coverage (`screenshot` through the socket's `call`)."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: escalate
    section: "§Occupied Resources → Network ports and listeners"
    change: "the `escher-session mcp` role's tool result also carries a shot — an `image` block (`image/png`, base64) then a text block — beside ids, names, values, snapshot text and diff; still no listener, socket or port, an image result bounded by its whole written form"
    sidecar: "arch §Occupied Resources → Network ports and listeners: the MCP role's result carries a shot's image block — 2026-10-12-headless-screenshot"
    rationale: "Second restatement of the crossing in the listener registry (the MCP role's sentence); Report Changes → Schema / config (over MCP a shot is two content blocks; bound by its whole written form)."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → Driver session"
    change: "the library depends on five crates — blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and, since 2026-10-12-headless-screenshot, `png` (the workspace's, resolved 0.18.1, already in the graph — no package entered `Cargo.lock`), its encoder of a shot, the base64 written by hand"
    sidecar: "arch §Established Decisions → Driver session: escher-driver dependencies four → five (`png`) — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Dependencies (escher-driver: `png` 0.18.1 — its dependencies four → five; no package enters `Cargo.lock`) and Counts `drvdeps4` architecture.md:101."
    basis: "packages/escher-driver/Cargo.toml"
  - detector: D-arch-decisions
    severity: warning
    section: "§Occupied Resources → Names"
    change: "escher-driver depends on blitz-test-harness, on dioxus-native-dom with its `accessibility` feature named, on `tracing`, on `serde_json` and, since 2026-10-12-headless-screenshot, on `png` (the workspace's, each with no feature)"
    sidecar: "arch §Occupied Resources → Names: escher-driver's dependency list gains `png` — 2026-10-12-headless-screenshot"
    rationale: "The escher-driver dependency set is restated in the crate names registry; Report Changes → Dependencies (escher-driver: `png`)."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Existing Scopes → escher-driver"
    change: "it depends on blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and `png` (each with no feature)"
    sidecar: "arch §Existing Scopes → escher-driver: dependencies gain `png` — 2026-10-12-headless-screenshot"
    rationale: "The escher-driver row of architecture.md:257 restates the four dependencies; Report Changes → Dependencies (`png`, four → five)."
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → escher-driver"
    change: "fourteen private modules — the thirteen as before and, since 2026-10-12-headless-screenshot, image (the crate-private PNG encoder `png` and the hand-written padded `base64`); execute also holds `Outcome::Shot` and the shot's crop, and mcp the image block"
    sidecar: "arch §Existing Scopes → escher-driver: private modules thirteen → fourteen (`image`) — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Crates / modules (escher-driver gains the private module `image`, fourteen modules) and Counts `modules13` architecture.md:257."
    basis: "packages/escher-driver/src/image.rs:3-15; packages/escher-driver/src/image.rs:19-37"
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-test-harness"
    change: "Modules harness, input, inspect, settle and, since 2026-10-12-headless-screenshot, the private screenshot (`Harness::screenshot`, `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError`, re-exported from the crate root)"
    sidecar: "arch §Existing Scopes → blitz-test-harness: module `screenshot` — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Crates / modules (blitz-test-harness gains the private module `screenshot`) and Symbols / APIs (its types re-exported from lib.rs)."
    basis: "packages/blitz-test-harness/src/screenshot.rs:37-103"
  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: "add three test files, each its own target: `password_painted_masked` (4 tests, both layout modes — a text input paints its characters, two password values of one length paint the same frame, a password input paints its bullets, a focused one its caret), `stand_act_screenshot` (the whole screen a PNG of the harness raster with text painted, an element shot its bounds rounded outward, refused as every id-addressed call, a no-font app refused with nothing changed, a typed password never leaving as pixels, a shot changing nothing) and `stand_screenshot_repeat` (a shot of one state the same in two child runs and in the parent); the `stand_act_*` files twelve → thirteen; `session_common` read by twenty (+ `stand_act_screenshot`, `stand_screenshot_repeat`, measured at 2026-10-12-headless-screenshot, where eighteen stood), holding since then the `screenshot` builders and the `shot` reader"
    sidecar: "arch §Existing Scopes → blitz-tests: three new targets, `stand_act_*` twelve → thirteen, `session_common` readers 18 → 20 — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Files (three new test files), Counts `readers18` architecture.md:263 (18 → 20) and `targets110` (110/109 → 113/112, three new files, each its own target), Outcome v010-07 / v010-08 and the a11y/security criteria; the thirteen files' test count is not stated by the report."
    basis: "tests/blitz-tests/tests/password_painted_masked.rs:84-131; tests/blitz-tests/tests/stand_act_screenshot.rs:79-304; tests/blitz-tests/tests/stand_screenshot_repeat.rs:99-136; tests/blitz-tests/tests/session_common/mod.rs:104-128"
  - detector: D-arch-resources
    severity: warning
    section: "§Conventions → Tests"
    change: "the checks that hold a `Session` — the session checks, `stand_settle`, the thirteen driver-action checks `stand_act_*`, `stand_mcp_tools` and, since 2026-10-12-headless-screenshot, `stand_screenshot_repeat` — keep what they share in `tests/session_common/mod.rs`"
    sidecar: "arch §Conventions → Tests: `session_common`'s readers restated — thirteen `stand_act_*` and `stand_screenshot_repeat` — 2026-10-12-headless-screenshot"
    rationale: "The `session_common` reader set is restated in Conventions as twelve `stand_act_*` and `stand_mcp_tools`; Report Counts `readers18` (+ `stand_act_screenshot`, `stand_screenshot_repeat`)."
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads"
    change: "five blitz-tests integration binaries re-execute their own test binary under `cargo test`: since 2026-10-12-headless-screenshot `stand_screenshot_repeat` runs `std::env::current_exe()` twice with a fixed argv, each child running its ignored `child_prints_its_shots` and printing its shots, output captured — test-only, no socket or port"
    sidecar: "arch §Occupied Resources → Process-wide state and threads: `stand_screenshot_repeat` re-executes its test binary twice — 2026-10-12-headless-screenshot"
    rationale: "Report Outcome v010-08 (two children and the parent print byte-identical PNGs for 12 shots) and New text (`child_run`, `child_prints_its_shots`): a new child-process spawn by a blitz-tests binary, which the registry lists per binary."
    basis: "tests/blitz-tests/tests/stand_screenshot_repeat.rs:74-97; tests/blitz-tests/tests/stand_screenshot_repeat.rs:138-145"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Testing"
    change: "blitz-tests' dev-dependencies gain `png` (the workspace's), read by `stand_act_screenshot` and `stand_screenshot_repeat` to decode a shot"
    sidecar: "arch §Stack and Technologies → Testing: blitz-tests dev-dependency `png` — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Dependencies (blitz-tests (dev): `png`) and Files (`tests/blitz-tests/Cargo.toml`); the Testing row enumerates blitz-tests' dev-dependencies."
    basis: "tests/blitz-tests/tests/stand_act_screenshot.rs:44-54"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Media"
    change: "`png` \"0.18\" (resolved 0.18.1) is since 2026-10-12-headless-screenshot also taken by escher-driver, its encoder of a shot, and by blitz-tests as a dev-dependency — no package entered `Cargo.lock`; no `base64` crate is taken"
    sidecar: "arch §Stack and Technologies → Media: `png` taken by escher-driver and blitz-tests — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Dependencies (no package enters `Cargo.lock`; escher-driver `png` 0.18.1; blitz-tests (dev) `png`; the `base64` crate not used); the Media row names `serde_json`'s escher-driver use and no other escher consumer of `png`."
    basis: "packages/escher-driver/src/image.rs:3-15"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → Renderer backends"
    change: "blitz-test-harness's `screenshot` also renders through `VelloCpuImageRenderer` (anyrender_vello_cpu, no `multithreading`) and composites the frame over opaque white after paint, where the `screenshot` example fills before `paint_scene` — the fill would need `kurbo` and `peniko`, which none of the harness's three raster dependencies re-exports"
    sidecar: "arch §Established Decisions → Renderer backends: the harness's CPU renderer and white composite — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Harness / gate surface (`VelloCpuImageRenderer`, composite over opaque white), Reverted (the fill before `paint_scene` replaced) and Deviations (white ground composited after paint); architecture.md:79 names the `screenshot` example's renderer only."
    basis: "packages/blitz-test-harness/src/screenshot.rs:73-102"
note: "No arch drift at architecture.md:210 — README.md still names the screenshot as still to come and the MCP tools as ten (report Deviations: README stale, out of the write set, owner P2/P5); when README moves, :210 moves with it. The four proposals marked escalate are one boundary widening (pixels on the ratified carriers, founder fork 1) and need Validate's halt."
