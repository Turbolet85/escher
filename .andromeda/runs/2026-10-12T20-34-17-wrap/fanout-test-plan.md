proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "72 inline unit tests in thirteen files (66 in twelve until 2026-10-12-headless-screenshot) — since that chunk image.rs 5 (base64 reads the RFC 4648 test vectors and writes every sextet of the alphabet; a PNG decodes to the pixels it was given, holds no time or text chunk, and pixels that do not fill the size write nothing) and one more whose file the report does not state; refusal.rs's causes nine in order, `fonts-unavailable` last; schema.rs's verbs eleven — seven instance-level, `screenshot` the seventh after `scroll`, then `start` · `status` · `stop`, then `help`; mcp.rs's listing the eleven verbs of the table (`the_listing_is_the_eleven_verbs_of_the_table`, renamed); the executor's verbs and refusals covered by the thirteen `stand_act_*` files, `stand_act_screenshot` the thirteenth; cite packages/escher-driver/src/image.rs:39-107 and the count at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md"
    sidecar: "§1 escher-driver: 66 → 72 unit tests in thirteen files (image.rs 5), eleven verbs, nine causes, thirteen stand_act_* files — 2026-10-12-headless-screenshot"
    rationale: "Report Changes → Counts: verbs10, inst6 and causes8 each name test-plan.md:21; Outcome gate `cargo test -p escher-driver --locked` reads 72 unit tests; Crates / modules adds the private module `image`, whose New-text rows hold five test functions; matrix#v010-13 note names the renamed mcp.rs test; Files lists `stand_act_screenshot.rs` (new)."
    basis: "test-plan.md:21; packages/escher-driver/src/image.rs:39-107"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, bump, examples, accesskit_xplat)"
    change: "seven_guis' `cli_commands.rs` 11 tests (10 until 2026-10-12-headless-screenshot: since that chunk a `screenshot` answers one line whose `png`, decoded by a hand base64 reader and its PNG header, equals the PNG `Session::run` returns in process); `mcp_server.rs` 5 tests (4 until that chunk: `tools/list` offering eleven verbs — the `ten since that chunk` reading retired — and a screenshot tool call answering an image block then a text block, a refused one a single text block with `isError`); `host_log` and `mcp_log` each also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under both builds"
    sidecar: "§1 seven_guis tests: cli_commands 11, mcp_server 5 (eleven verbs), host_log/mcp_log read no image byte on stderr — 2026-10-12-headless-screenshot"
    rationale: "Report Outcome v010-07 names `cli_commands.rs::a_screenshot_answers_one_line_whose_png_is_the_screen` and `mcp_server.rs::a_screenshot_tool_call_answers_an_image_block_then_a_text_block`; matrix#v010-13 note: the Counter-flow test now counts eleven; Coverage of new surfaces: 0 occurrences of the first 64 base64 characters in the host's and the mcp role's stderr at trace, both builds; Deviations: the CLI png decoded by hand. Same retired claim (ten verbs) as the §1 escher-driver primary."
    basis: "examples/seven_guis/tests/cli_commands.rs:282-354; examples/seven_guis/tests/mcp_server.rs:305-335; examples/seven_guis/tests/host_log.rs:158-171; examples/seven_guis/tests/mcp_log.rs:144-157"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-dom (document, mutator, net)"
    change: "34 `#[test]` functions: 18 in document.rs (16 until 2026-10-12-headless-screenshot added the two of `font_reading_tests`), 12 in mutator.rs, 4 in net.rs"
    sidecar: "§1 blitz-dom: 32 → 34 tests, document.rs 16 → 18 (font_reading_tests) — 2026-10-12-headless-screenshot"
    rationale: "Report Symbols: `BaseDocument::can_paint_text` with unit tests `font_reading_tests` — true on `build_single_font_ctx(BULLET_FONT)`, false on 16 zero bytes; New-text rows show the module with two test functions beside the helper `doc_with`."
    basis: "packages/blitz-dom/src/document.rs:3649-3673"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-dom covered behaviours"
    change: "add: the font reading `can_paint_text` — true on a context holding a decodable font, false on one holding no family (packages/blitz-dom/src/document.rs:3649-3673); `layout_as_text_input`, public since 2026-10-12-headless-screenshot, has no unit test and is read only through the password mask's frames (`password_painted_masked`, §1 tests/blitz-tests)"
    sidecar: "§1 blitz-dom covered behaviours: + the font reading; layout_as_text_input read through the mask's frame check only — 2026-10-12-headless-screenshot"
    rationale: "Report Symbols: `can_paint_text` (document.rs:1022-1035) with its two unit tests; `layout_as_text_input` (document.rs:1037-1067), NOT in the plan, whose one caller is blitz-paint's mask; no unit test of it is listed in New text."
    basis: "packages/blitz-dom/src/document.rs:1022-1067"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-paint"
    change: "add: the password mask (`draw_masked_input_text`, `caret_color`, render.rs:956-1039), painted for every document since 2026-10-12-headless-screenshot, has no unit test — it is read on frames by blitz-tests' `password_painted_masked` (4, both layout modes) and through the driver by `stand_act_screenshot`; the caret a click places inside a password field holding a value is read by no check (owned by a route entry)"
    sidecar: "§1 blitz-paint: the password mask covered by integration checks only; the click-placed caret unmeasured — 2026-10-12-headless-screenshot"
    rationale: "Report Symbols: `ElementCx::draw_masked_input_text` (render.rs:965-1039) and `caret_color` (956-963), every renderer through blitz-paint; Insufficient fixes: the click-placed caret not measured by any check, owner a route owner; Coverage: the password paint tests {integration, both layout modes}."
    basis: "packages/blitz-paint/src/render.rs:956-1040; tests/blitz-tests/tests/password_painted_masked.rs:84-131"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-test-harness"
    change: "its unit tests are 17: 11 inline in `settle.rs` (as before) and, since 2026-10-12-headless-screenshot, 6 in `screenshot.rs` (packages/blitz-test-harness/src/screenshot.rs:105-245)"
    sidecar: "§1 blitz-test-harness: 11 → 17 unit tests (screenshot.rs 6) — 2026-10-12-headless-screenshot"
    rationale: "Report Outcome gate `cargo test -p blitz-test-harness --locked` — 17 tests; Coverage: `Harness::screenshot` / `screenshot_rect` tests {unit 6 + integration}; Files: screenshot.rs (new)."
    basis: "packages/blitz-test-harness/src/screenshot.rs:105-245"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "add the chunk's files and moves: `password_painted_masked` 4 (an engine check on parsed pages in both layout modes — a text input paints its characters, two password values of one length paint the same frame, a password input paints its bullets, a focused one its caret; no `stand_` prefix); the thirteenth driver-action file, since 2026-10-12-headless-screenshot, `stand_act_screenshot` 6 (the whole screen a PNG of the harness raster with text painting, an element shot its bounds rounded outward, an element shot refused as every id-addressed call is, an app with no usable font refused `fonts-unavailable` with nothing changed, a typed password never leaving as pixels, a shot changing nothing — each in both layout modes); `stand_screenshot_repeat` 1 and its `#[ignore]` child (two child runs and the parent print byte-identical PNGs for 12 shots, both modes, under the per-package and the workspace build); `stand_act_spans` over the seven instance-level verbs and a refused call for each of the nine causes (its call count not stated by the report); `stand_mcp_tools` 5 (4 until that chunk: a screenshot tool call answers its image then its text and parents its command, and every cause a tool call reaches now served eight through MCP, the no-font fixture adding `fonts-unavailable`)"
    sidecar: "§1 tests/blitz-tests: + password_painted_masked 4, stand_act_screenshot 6, stand_screenshot_repeat 1 (+1 child); stand_act_spans seven verbs · nine causes; stand_mcp_tools 5 — 2026-10-12-headless-screenshot"
    rationale: "Report Counts: inst6 and causes8 name test-plan.md:26; Files: three new test files; Outcome v010-07 / v010-08 refs and the (security, a11y), (tests, design), (a11y, layouts) criteria; matrix#v010-14 note: stand_mcp_tools serves eight; Coverage lists stand_act_spans for fonts-unavailable. Same retired claims (six instance verbs, eight causes) as the §1 escher-driver primary."
    basis: "test-plan.md:26; tests/blitz-tests/tests/stand_act_screenshot.rs:79-304; tests/blitz-tests/tests/stand_screenshot_repeat.rs:99-145; tests/blitz-tests/tests/password_painted_masked.rs:84-131; tests/blitz-tests/tests/stand_mcp_tools.rs:610-683"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Harness-driven tests"
    change: "add: since 2026-10-12-headless-screenshot the harness also paints its document with no display — `Harness::screenshot` / `screenshot_rect` into `VelloCpuImageRenderer`, composited over opaque white, at the harness's own time with no tick or pump (packages/blitz-test-harness/src/screenshot.rs:37-103)"
    sidecar: "§2 Harness-driven tests: + the harness's headless screenshot — 2026-10-12-headless-screenshot"
    rationale: "Report Symbols and Harness / gate surface: `Harness::screenshot` / `screenshot_rect`, re-exported from lib.rs, resolving at the harness's own time and compositing over white; §2 lists the harness's capabilities and names none that paints."
    basis: "packages/blitz-test-harness/src/screenshot.rs:37-103"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Pixel tests"
    change: "add: since 2026-10-12-headless-screenshot pixels are also compared whole — a shot's PNG equal to the harness's raster (`stand_act_screenshot`), byte-identical across two child runs of the test binary's own `#[ignore]` child, whose stdout the check reads, and the parent, 12 shots in both layout modes under both builds (`stand_screenshot_repeat`), and a password field's frame equal for two values of one length (`password_painted_masked`, through `render_to_buffer::<VelloCpuImageRenderer, _>`); a check decodes PNGs through blitz-tests' `png` dev-dependency, or in seven_guis, which has none, by a hand base64 reader and the PNG header"
    sidecar: "§2 Pixel tests: + whole-frame and cross-process PNG equality checks — 2026-10-12-headless-screenshot"
    rationale: "Report Outcome v010-07 (PNG equal to the harness's raster) and v010-08 (two children and the parent byte-identical for 12 shots, both builds); Dependencies: blitz-tests (dev) gains `png`; Deviations and Sweep hazards: seven_guis has no png dev-dependency, decoded by hand."
    basis: "tests/blitz-tests/tests/stand_screenshot_repeat.rs:74-136; tests/blitz-tests/tests/password_painted_masked.rs:41-82"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Font-dependent tests"
    change: "add: since 2026-10-12-headless-screenshot a shot of a document whose font context resolves no family for `sans-serif` (`BaseDocument::can_paint_text` false) is refused — the harness's `ScreenshotError::FontsUnavailable`, the driver's `fonts-unavailable` — with nothing painted and no skip path, read by `stand_act_screenshot` and `stand_mcp_tools` on an app booted with no usable font and by blitz-dom's `font_reading_tests`"
    sidecar: "§2 Font-dependent tests: the no-font shot refused, never skipped — 2026-10-12-headless-screenshot"
    rationale: "Report Outcome: (tests, design) no-font fails and nothing skips — MET (`fonts-unavailable`, next snapshot unchanged; the font reading's unit test); Harness / gate surface: `FontsUnavailable` with nothing painted; §2 states only the pre-existing skip path and the stand's no-skip font."
    basis: "tests/blitz-tests/tests/stand_act_screenshot.rs:198-221; tests/blitz-tests/tests/stand_mcp_tools.rs:558-571"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Directory pattern"
    change: "113 `.rs` files, 112 test targets and the unbuilt `all.rs` (110 and 109 until 2026-10-12-headless-screenshot added `password_painted_masked.rs`, `stand_act_screenshot.rs` and `stand_screenshot_repeat.rs`, each its own auto-discovered target); `tests/session_common/mod.rs` read through `mod session_common;` by 20 checks (+ `stand_act_screenshot`, `stand_screenshot_repeat`), the thirteen `stand_act_*.rs` among them; the `mod common;` reader count (17) and the files declaring both re-measured at this wrap, the report stating no `common` count"
    sidecar: "§2 Directory pattern: blitz-tests 110/109 → 113/112, session_common readers 18 → 20, thirteen stand_act_* — 2026-10-12-headless-screenshot"
    rationale: "Report Counts: targets110 (test-plan.md:66) 110/109 → 113/112, three new files each its own target; readers18 (test-plan.md:101) 18 → 20 (+ stand_act_screenshot, stand_screenshot_repeat). Same retired claims as the §3 session-checks site."
    basis: "test-plan.md:66"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (Harness) → Core"
    change: "add `screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `screenshot_rect(&mut self, PixelRect)`, in the private module `screenshot`, with `PixelRect`, `Raster` and `ScreenshotError::{FontsUnavailable, EmptyRect}` re-exported: each resolves at the harness's own time (no tick, no pump), paints the viewport at its scale and device size into `VelloCpuImageRenderer` and composites over opaque white (premultiplied source-over); a rectangle is clipped to the viewport, an empty one is `EmptyRect`; a document whose `can_paint_text` reads false is `FontsUnavailable` with nothing painted; no path prints, panics or logs (packages/blitz-test-harness/src/screenshot.rs:37-103)"
    sidecar: "§3 Harness Core: + screenshot / screenshot_rect, PixelRect, Raster, ScreenshotError — 2026-10-12-headless-screenshot"
    rationale: "Report Symbols (screenshot.rs:37-103, types 10-35, re-exported from lib.rs), Crates / modules (private module `screenshot`) and Harness / gate surface (semantics, the no-panic probe 0)."
    basis: "packages/blitz-test-harness/src/screenshot.rs:10-103"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: "'six of the twelve driver-action checks' → of the thirteen driver-action checks, with whether `stand_act_screenshot` and `stand_screenshot_repeat` declare `mod common;` (and so the reader count 17) measured at this wrap — the report states no `common` reader count"
    sidecar: "§3 stand-check helpers: twelve → thirteen driver-action checks — 2026-10-12-headless-screenshot"
    rationale: "Report Files: `stand_act_screenshot.rs` (new) makes the `stand_act_*` files thirteen; the `twelve` claim retired at the §1 escher-driver primary stands here too."
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: "add the builders `screenshot()` and `screenshot_of(id)` and the reader `shot(session, call)` returning an `Option<Shot>` (tests/blitz-tests/tests/session_common/mod.rs:104-128), `act` reading the `Shot` outcome; readers 20 (18 until 2026-10-12-headless-screenshot) — the thirteen driver-action checks `stand_act_*`, `stand_mcp_tools` and `stand_screenshot_repeat`, which is no `stand_act_*` file, beside the five lifecycle-side readers"
    sidecar: "§3 session-check helpers: + screenshot builders and the shot reader; readers 18 → 20 — 2026-10-12-headless-screenshot"
    rationale: "Report Counts readers18 (test-plan.md:101) 18 → 20; New text rows for session_common/mod.rs (screenshot, screenshot_of, Shot, shot); Remaining callers: `session_common::act` gained the Shot arm."
    basis: "test-plan.md:101; tests/blitz-tests/tests/session_common/mod.rs:104-128"
    dependent-of: D-tests-coverage
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: "add: since 2026-10-12-headless-screenshot `screenshot` is a word of `command`'s `verb` and `tool_call`'s `tool` closed sets, no span field added — a shot's `command` line records `verb` (and `cause` when refused), never the image, its size or its dimensions — and the host's and the `mcp` role's stderr at `trace` hold 0 occurrences of a shot's first 64 base64 characters under both builds (`host_log`, `mcp_log`), in step with obs-plan §4"
    sidecar: "§3 Stand log format: screenshot joins the verb/tool closed sets; no image byte on stderr — 2026-10-12-headless-screenshot"
    rationale: "Report Schema / config: the command span keeps its eight fields, a shot records verb (and cause), never the image, size or dimensions; tool_call unchanged; Expected amendments: obs-plan §4 (`verb` and `tool` gain `screenshot`); Coverage: 0 image bytes on stderr at trace, both builds. test-plan §3 lists `help` as the last word added to those closed sets, so a one-sided obs-plan edit would leave the two §3s apart."
    basis: "examples/seven_guis/tests/host_log.rs:158-171; examples/seven_guis/tests/mcp_log.rs:144-157"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "append: re-counted at 2026-10-12-headless-screenshot, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 139 · failed 0 · ignored 7 over 36 files — +8 passed (`stand_act_screenshot` 6 and `stand_screenshot_repeat` 1, two new files picked up by their `stand_` prefix with no script change, `stand_mcp_tools` +1) and +1 ignored, the repeat check's child — `password_painted_masked` carries no `stand_` prefix and is not selected, and `run all` was not re-read — as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md"
    sidecar: "§3 agent-run Proof: run stand 131/6/34 → 139/7/36 — 2026-10-12-headless-screenshot"
    rationale: "Report Counts stand131 (test-plan.md:116): 139 passed · 0 failed · 7 ignored over 36 files; Harness / gate surface: agent-run.sh unchanged, run stand now 36 files. The +8 split is the report's New-text test functions (six in stand_act_screenshot, one in stand_screenshot_repeat plus its child, one in stand_mcp_tools), which sum to its totals."
    basis: "test-plan.md:116"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle → session-start"
    change: "add: since 2026-10-12-headless-screenshot `run` takes `screenshot`, the seventh instance verb, its `id` optional, and returns `Outcome::Shot { width, height, png }` — a PNG of the harness's raster, the whole screen or the element's bounds rounded outward and cropped to the part that can be seen; it returns no diff and a shot changes nothing (`has_changes` false, focus and the diff unchanged); an element shot's id is refused `stale` or `not-found` as every id-addressed call is, and one that crops to no pixel `off-screen`; an app with no font it can paint text with is refused `fonts-unavailable`, the ninth cause, with nothing painted (packages/escher-driver/src/execute.rs:63-71; packages/escher-driver/src/execute.rs:382-403)"
    sidecar: "session-start: + screenshot and Outcome::Shot, fonts-unavailable — 2026-10-12-headless-screenshot"
    rationale: "Report Symbols: `Command::Screenshot { id: Option<String> }`, `Outcome::Shot` (execute.rs:63-71), private `crop` (382-403), `Cause::FontsUnavailable`; Deviations: an element shot that crops to no pixel refused `off-screen`; Coverage: a shot leaves `has_changes` false, focus and the diff unchanged. The key enumerates `run`'s outcomes and names no shot."
    basis: "packages/escher-driver/src/execute.rs:63-71; packages/escher-driver/src/execute.rs:382-403"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle → session-wire"
    change: "add: since 2026-10-12-headless-screenshot a `screenshot` call's answer carries the screen's pixels out — a PNG as padded base64 in the one JSON line `{width, height, png}` — in band, the founder's fork 1 («В самом ответе», relayed by the overseer, inputs#I1 · inputs#I2 of that chunk), inside the unchanged 1,048,576-byte answer bound (the largest whole-screen answer 22,404 bytes, flight-booker, 2.1 % of it)"
    sidecar: "session-wire: a shot's PNG crosses in band on its answer line, bound unchanged — 2026-10-12-headless-screenshot"
    rationale: "Report Schema / config: the shot's written line and the wire's answer bound unchanged; Expected amendments: arch §Established Decisions → Driver session records the image in band on socket, stdout and MCP stdio as a boundary widening on the founder's fork 1 (inputs#I1, inputs#I2); Outcome basis: largest whole-screen answer 22,404 bytes. The key lists what an answer carries out and names no image."
    basis: "packages/escher-driver/src/json.rs:242-254"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle → session-binary"
    change: "eleven verbs — the seven instance-level ones (`screenshot` the seventh, its `--id` optional) with their arguments as flags spelt as the schema spells them, then `start <task>`, `status`, `stop`, then `help`; since 2026-10-12-headless-screenshot a `screenshot`'s accepted answer is one line `{width, height, png}`, `png` padded base64, status `0`"
    sidecar: "session-binary: ten → eleven verbs, six → seven instance-level — 2026-10-12-headless-screenshot"
    rationale: "Report Counts verbs10 names registries/contracts/test-plan/session-lifecycle.md:9; Schema / config: eleven verbs, seven of the instance, the shot's written line; Outcome v010-07: the binary answers status 0 with a png equal to the in-process PNG. Same retired claims as the §1 escher-driver primary."
    basis: "registries/contracts/test-plan/session-lifecycle.md:9"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle → session-proof"
    change: "seven_guis `cli_commands` 11 (10 until 2026-10-12-headless-screenshot) · `mcp_server` 5 (4 until it) · `stand_mcp_tools` 5 (4 until it) · since that chunk `stand_act_screenshot` 6 and `stand_screenshot_repeat` 1 (+1 ignored child) · the thirteen driver-action files (46; 40 over twelve until that chunk) · escher-driver's 72 unit tests (image.rs 5 and one more whose file the report does not state; 66 until that chunk) — and the fork's CI run 38297557417 on `5173f7e1` (16 of 16 checks green, wall 1023 s, the MSRV, windows, macos, ios and android legs among them) of the screenshot verb and the password mask on them"
    sidecar: "session-proof: counts moved (cli_commands 11, mcp_server 5, stand_mcp_tools 5, thirteen driver-action files 46, crate 72) and CI#38297557417 — 2026-10-12-headless-screenshot"
    rationale: "Report Outcome (v010-07 / v010-08 refs, gate `cargo test -p escher-driver` 72), Files (stand_act_screenshot, stand_screenshot_repeat new), Cross-project claims (CI#38297557417 green 16/16, wall 1023 s, on 5173f7e12f96). Same retired counts as the §1 sites."
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → blitz-test-harness"
    change: "add: since 2026-10-12-headless-screenshot `screenshot.rs` pins the shot — the whole viewport painted at its device size, what the page leaves unpainted reading opaque white, a rectangle being that part of the frame, a rectangle past the viewport clipped and one outside it an error, a document with no font refused, and a shot the same twice (packages/blitz-test-harness/src/screenshot.rs:105-245)"
    sidecar: "§4 blitz-test-harness: + screenshot.rs's six unit tests — 2026-10-12-headless-screenshot"
    rationale: "Report Coverage: `Harness::screenshot` / `screenshot_rect` tests {unit 6}; New-text rows name the six test functions of screenshot.rs's test module."
    basis: "packages/blitz-test-harness/src/screenshot.rs:140-244"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "schema.rs pins eleven verbs (seven instance-level, `screenshot` after `scroll`), field kinds ten (`FieldKind::Png` the tenth), refusal.rs nine causes (`fonts-unavailable` last); add image.rs — the hand base64 against RFC 4648's vectors and every sextet, and the PNG encoder: decoded pixels equal those given, no time or text chunk, pixels that do not fill the size write nothing; `cli_commands` 11 (10 until 2026-10-12-headless-screenshot); the thirteen `stand_act_*` files; and the three mutation controls of that chunk (mask off, crop rounded inward, font check removed), each turning exactly its target cases red, each restore sha256-equal (escher-0.1.0/chunks/2026-10-12-headless-screenshot/evidence/controls.md)"
    sidecar: "§4 escher-driver: eleven verbs, ten field kinds, nine causes, image.rs, thirteen stand_act_*, three controls — 2026-10-12-headless-screenshot"
    rationale: "Report Counts: verbs10, inst6, causes8 and kinds9 each name test-plan.md:143; Symbols: `FieldKind::Png`, `Cause::FontsUnavailable`, private `image::png` and `image::base64`; Outcome: mutation controls in evidence/controls.md. Same retired claims as the §1 escher-driver primary."
    basis: "test-plan.md:143; packages/escher-driver/src/image.rs:39-107"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: "add `screenshot` to the calls a check runs through `Session::run` and `fonts-unavailable` to the refused causes (any of the nine); a shot returns the screen or the element as a PNG equal to the harness's raster and changes nothing — `has_changes` false, focus and the diff unchanged; an element shot's id refused `not-found` or `stale`, and one cropping to no pixel `off-screen`, the instance unchanged; a typed password never leaves as pixels; the thirteen `stand_act_*` files (§1)"
    sidecar: "§5 Session ↔ held instance: + screenshot, nine causes, thirteen stand_act_* — 2026-10-12-headless-screenshot"
    rationale: "Report Counts causes8 names test-plan.md:181; Outcome: (security) refusals hold nothing of the call — not-found, stale, off-screen leave the instance unchanged; (a11y, layouts) a shot changes nothing; (security, a11y) a typed password never leaves as pixels. Same retired claims as the §1 escher-driver primary."
    basis: "test-plan.md:181"
    dependent-of: D-tests-coverage
  - detector: D-tests-obs-harness
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ what it writes"
    change: "add: since 2026-10-12-headless-screenshot `host_log` and `mcp_log` also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under the `-p seven_guis` and the workspace build"
    sidecar: "§5 Session host ↔ what it writes: + no image byte on the host's or the mcp role's stderr — 2026-10-12-headless-screenshot"
    rationale: "Report Coverage (screenshot → PII): logs carry no image byte — 0 occurrences of the first 64 base64 characters in the host's and the mcp role's stderr at trace, both builds; Outcome (obs) 0 image bytes on stderr. Same claim (what the log carries) as the §3 Stand log format primary."
    basis: "examples/seven_guis/tests/host_log.rs:158-171; examples/seven_guis/tests/mcp_log.rs:144-157"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Drivers per surface → Command line"
    change: "`cli_commands` (11, 10 until 2026-10-12-headless-screenshot): each of the seven instance-level verbs is accepted and answers exactly its fields; since that chunk a `screenshot` answers status `0` and one line whose `png`, decoded by a hand base64 reader and checked by its PNG header, equals the PNG `Session::run` returns in process"
    sidecar: "§6 Command line: cli_commands 11, seven instance verbs, the shot's png equal in process — 2026-10-12-headless-screenshot"
    rationale: "Report Counts inst6 names test-plan.md:200; Outcome v010-07 ref `cli_commands.rs::a_screenshot_answers_one_line_whose_png_is_the_screen`; Deviations: the CLI png decoded by hand and equal to the in-process PNG. Same retired claims as the §1 sites."
    basis: "test-plan.md:200; examples/seven_guis/tests/cli_commands.rs:282-354"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Drivers per surface → MCP"
    change: "`mcp_server` (5, 4 until 2026-10-12-headless-screenshot): `tools/list` offering the verbs in order — eleven since that chunk — and a `screenshot` tool call answering an image block (`image/png`) then a text block, `isError` false, a refused one a single text block, `isError` true; in process `stand_mcp_tools` (5, 4 until that chunk)"
    sidecar: "§6 MCP: mcp_server 5 and stand_mcp_tools 5, eleven tools, the image-then-text result — 2026-10-12-headless-screenshot"
    rationale: "Report Schema / config: over MCP a shot is two content blocks, a refused shot one text block isError true; Outcome v010-07 refs in mcp_server.rs and stand_mcp_tools.rs; matrix#v010-13: count eleven. Same retired claim (ten verbs) as the §1 escher-driver primary."
    basis: "examples/seven_guis/tests/mcp_server.rs:305-335; tests/blitz-tests/tests/stand_mcp_tools.rs:610-683"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline"
    change: "append to the `cargo test --workspace` entry: re-counted at 2026-10-12-headless-screenshot: 174 result lines, 849 passed · 0 failed · 12 ignored (+28 passed: three new result lines — blitz-tests' `password_painted_masked` 4, `stand_act_screenshot` 6 and `stand_screenshot_repeat` 1, +1 ignored its child — escher-driver 6 to 72, blitz-test-harness 6 to 17, blitz-dom 2 (`font_reading_tests`), `stand_mcp_tools` 1 and seven_guis 2 (`cli_commands` 1, `mcp_server` 1) in existing lines), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)"
    sidecar: "§9 Local baseline: workspace 171 lines / 821 · 0 · 11 → 174 / 849 · 0 · 12 — 2026-10-12-headless-screenshot"
    rationale: "Report Counts: workspace tests 849 passed · 0 failed · 12 ignored over 174 result lines (`target/ci-logs/test.log` of the fast leg), no master site swept for it. The split is from the report's gate counts (72, 17) and its New-text test functions, which sum to +28."
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Engine features by runner"
    change: "add: since 2026-10-12-headless-screenshot blitz-test-harness depends on blitz-paint with `default-features = false`, so a per-package build of the harness paints no `svg` and no overlay scrollbars (the workspace build's features for it not read by the report); `stand_screenshot_repeat` reads byte-identical shots under the per-package and the workspace build, and the `screenshot` verb's help states the limit that, in a build that paints overlay scrollbars, a thumb painted within 0.7 s of a scroll fades by wall time — no check shoots inside that window (owned by a route entry)"
    sidecar: "§9 Engine features by runner: the harness's blitz-paint without default features; shots equal under both builds; the scrollbar-fade limit — 2026-10-12-headless-screenshot"
    rationale: "Report Dependencies: blitz-test-harness gains blitz-paint (`default-features = false`, so no svg and no scrollbars in a per-package build); Schema / config: the verb's help states the 0.7 s fade limit; Outcome v010-08: identical under the per-package and the workspace build; inputs#I4 card row 2 places the wall-clock fade as a CARRY on the route."
    basis: "packages/blitz-test-harness/Cargo.toml:24-27; packages/escher-driver/src/schema.rs:418-420"
note: "D-tests-framework fired nothing: the chunk runs cargo's built-in harness throughout, `png` is a decoding dev-dependency (no test framework), and seven_guis decodes by hand. Per-file counts not in the report (stand_act_screenshot 6, password_painted_masked 4, image.rs 5, cli_commands +1, mcp_server +1, stand_mcp_tools +1, the +1 ignored child) are read from its New-text test functions and sum exactly to its stated totals (stand 139/7, workspace 849/12, crate 72, harness 17); escher-driver's sixth new test has no file in the report. The `mod common;` reader count (17) is unstated — measure before applying the §3 stand-checks proposal. The session-wire proposal records the image in band, a boundary widening the report attributes to the founder's fork 1 (inputs#I1, inputs#I2) — validate under the widening rule. Citation re-pointing for the shifted test-file ranges in §1/§5/§6 is not proposed here."
