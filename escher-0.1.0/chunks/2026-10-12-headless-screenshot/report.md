# Report — 2026-10-12-headless-screenshot

**Chunk:** Headless screenshot — the whole screen or one element by id with no display, the same pixels on repeat with the bundled font
**Date:** 2026-10-12
**Commits:** `5173f7e1 chore(2026-10-12-headless-screenshot): operator pre-CI commit, for the run this chunk verdict reads` (the
chunk's one commit; `git log --since=2026-10-12T09:41:25Z` also lists `938cd1b8`, the 2026-10-12-self-description wrap's own
commit, landed four minutes after that wrap's `last_wrap` stamp — not this chunk's). Basis of every diff below: `938cd1b8`, the
parent of the oldest pre-CI commit.

## Changes (structured — detectors read this)
- **Files:** 30 changed, all on research's lists (`gate.py scope`: `clean — changed 30 · listed 30 · recorded 0`):
  - engine: `packages/blitz-dom/src/document.rs` · `packages/blitz-paint/src/render.rs`
  - harness: `packages/blitz-test-harness/Cargo.toml` · `packages/blitz-test-harness/src/lib.rs` · `packages/blitz-test-harness/src/screenshot.rs` (new)
  - driver: `packages/escher-driver/Cargo.toml` · `src/{cli,client,command,describe,execute,json,lib,mcp,refusal,schema}.rs` · `src/image.rs` (new)
  - tests: `tests/blitz-tests/Cargo.toml` · `tests/blitz-tests/tests/{password_painted_masked,stand_act_screenshot,stand_screenshot_repeat}.rs` (new) ·
    `tests/blitz-tests/tests/{session_common/mod,stand_act_scroll,stand_act_spans,stand_mcp_tools}.rs` ·
    `examples/seven_guis/tests/{cli_commands,host_log,mcp_log,mcp_server}.rs`
  - `Cargo.lock` (dependency lines only)
  - untouched though listed: root `Cargo.toml` (every crate already a workspace dependency), `packages/blitz-dom/src/node/text.rs`
    (no state kept between frames), `tests/blitz-tests/tests/stand_act_diff.rs` (its `let … else` on `Outcome` is not exhaustive).
- **Symbols / APIs:**
  - blitz-dom `BaseDocument::can_paint_text(&self) -> bool` (document.rs:1022-1035) — whether the font context resolves a family for
    the `sans-serif` generic, through fontique's `generic_families`; reads only. Unit tests `font_reading_tests` (document.rs:3649-3673):
    true on `build_single_font_ctx(BULLET_FONT)`, false on 16 zero bytes.
  - blitz-dom `BaseDocument::layout_as_text_input(&self, node_id, text) -> Option<parley::Layout<TextBrush>>` (document.rs:1037-1067)
    — NOT in the plan: lays `text` out with a text input's editor styles, scale and direction in the document's own fonts, one
    unbroken line, a fresh `LayoutContext` per call; `None` for a node that is no text input. Its one caller is blitz-paint's mask.
  - blitz-paint `ElementCx::draw_masked_input_text` (render.rs:965-1039) and `ElementCx::caret_color` (render.rs:956-963), private;
    `draw_text_input_text` branches to the mask when `type` reads `password` (render.rs:902-906). Every document, every renderer
    through blitz-paint, windowed included.
  - blitz-test-harness `Harness::screenshot(&mut self) -> Result<Raster, ScreenshotError>` and
    `Harness::screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` (screenshot.rs:37-103), with the public
    types `PixelRect` (10-17), `Raster` (19-25) and `ScreenshotError::{FontsUnavailable, EmptyRect}` (27-35), re-exported from lib.rs.
  - escher-driver: `VERBS` gains `SCREENSHOT` (schema.rs:415-448), `Level::Instance`, after `scroll`; `FieldKind::Png`;
    `Cause::FontsUnavailable` (`fonts-unavailable`); `Command::Screenshot { id: Option<String> }`; `Outcome::Shot { width, height,
    png: Vec<u8> }` (execute.rs:63-71); private `crop` (execute.rs:382-403); private `image::png` (image.rs:3-15) and
    `image::base64` (image.rs:19-37); private `json::shot_without_image` (json.rs:250-254); private `ToolResult::shot` and an
    `image` field on `ToolResult` (mcp.rs).
  - Remaining callers: `Outcome`'s readers each gained the `Shot` arm where they match exhaustively — `record_result` (execute.rs),
    `to_json` (json.rs), `Server::run` (mcp.rs), `session_common::act`, `stand_act_spans::acting`; `stand_act_diff` and
    `cli_commands` read one variant through `let … else` and are unchanged.
- **Crates / modules:** escher-driver gains the private module `image` (fourteen modules); blitz-test-harness gains the private
  module `screenshot`. No crate added or removed.
- **Dependencies:** no package enters `Cargo.lock` (probe `git diff 938cd1b8… -- Cargo.lock | grep -c '^+\[\[package\]\]'` → 0 at
  exit 1); four dependency lines added inside existing packages:
  - blitz-test-harness: `blitz-paint` (`default-features = false`, so no `svg` and no `scrollbars` in a per-package build),
    `anyrender`, `anyrender_vello_cpu` (no `multithreading`) — three more;
  - escher-driver: `png` (0.18.1) — its dependencies four → five;
  - blitz-tests (dev): `png`.
  The `base64` crate and `serde_json` writing were not used: the base64 is hand-written (RFC 4648 §4, padded).
- **Schema / config:**
  - The verb table: eleven verbs, seven asked of the instance (`screenshot` the seventh), three of the session, `help` last.
    `screenshot` takes one optional `id` (`ArgKind::Id`) and returns `width` (count), `height` (count), `png` (`FieldKind::Png`,
    base64 text), all `always`. Its help states two measured limits: an element shot is cropped to the part of its bounds that can
    be seen; in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time.
  - Field kinds nine → ten (`png`). Causes eight → nine: `fonts-unavailable`, last in the order — meaning "the app this session
    holds has no font it can paint text with", remedy "boot the app with a font it can decode".
  - Written forms: the shot's line `{"width":…,"height":…,"png":"<base64>"}`; over MCP a shot is two content blocks — `image`
    (`data` the base64, `mimeType` `image/png`) then `text` (the line without `png`), `isError` false; a refused shot stays one text
    block, `isError` true.
  - Bounds: the wire's 1,048,576-byte answer bound unchanged; over MCP a result that holds an image is bounded by its WHOLE written
    form, a text-only result by its text as before (mcp.rs `ToolResult::bounded`).
  - The command span keeps its eight fields: a shot records `verb` (and `cause` when refused), never the image, its size or its
    dimensions. The `tool_call` span unchanged.
  - The unknown-verb stand-in in the eight unit/check sites that used `screenshot` is now `capture`.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** each basis is a pattern of `{run_dir}/report-patterns.toml` fired by `cascade.py sweep --pre-pass`
  (baseline HEAD `5173f7e1`; exit 0 over ten patterns, `total (10 patterns) · 91 rows over 18 files`); master sites are its
  `standing` rows; leaf rows are P2's.
  - verbs ten → eleven — `verbs10`: 7 standing · 7 changed: architecture.md:133 ×2 · security-plan.md:81 · security-plan.md:83 ·
    layout-templates.md:74 ×2 · test-plan.md:21 · test-plan.md:143 · registries/contracts/test-plan/session-lifecycle.md:9.
  - instance verbs six → seven — `inst6`: 9 standing · 9 changed: architecture.md:133 · security-plan.md:81 · security-plan.md:212 ·
    security-plan.md:392 · layout-templates.md:74 · test-plan.md:26 · test-plan.md:143 · test-plan.md:200 · obs-plan.md:296.
  - causes eight → nine — `causes8`: 9 standing · 9 changed: security-plan.md:81 · :346 · :392 · test-plan.md:21 · :26 · :143 · :181 ·
    obs-plan.md:106 · :296 (the tool also reports `case-variants +3`).
  - field kinds nine → ten — `kinds9`: 1 standing · 1 changed: test-plan.md:143.
  - escher-driver private modules thirteen → fourteen — `modules13`: 1 standing · 1 changed: architecture.md:257.
  - escher-driver dependencies four → five — `drvdeps4`: 1 standing · 1 changed: architecture.md:101.
  - `session_common` readers 18 → 20 (+ `stand_act_screenshot`, `stand_screenshot_repeat`) — `readers18`: 2 standing · 2 changed:
    architecture.md:263 · test-plan.md:101.
  - stand run 131 / 34 → 139 passed · 0 failed · 7 ignored over 36 files (`run.end` of the gate entry `bash scripts/agent-run.sh run
    stand`) — `stand131`: 1 standing · 1 changed: test-plan.md:116.
  - blitz-tests `.rs` files / test targets 110/109 → 113/112 (three new files, each its own target) — `targets110`: 1 standing ·
    1 changed: test-plan.md:66.
  - workspace tests: 849 passed · 0 failed · 12 ignored over 174 result lines (`target/ci-logs/test.log` of the fast leg); no
    master site swept for it.
  - the token `screenshot` — `screenshot`: 32 standing · 1 bears on this chunk: architecture.md:210 (README names the screenshot as
    still to come — true of README as it stands, see Deviations); 31 no change — each names the upstream `examples/screenshot.rs`
    example, the browser's `capture` (`blitz-screenshot-<unix-secs>`) or the `screenshot` Cargo feature (architecture.md:50 · 79 ×2 ·
    80 · 82 · 93 ×2 · 108 · 113 · 122 · 129 · 138 ×7 · 147 · 149 ×3 · 185 ×3 · 193 ×4 · 266 ×2; security-plan.md:15 ×2 · 69 · 70 · 71 · 223 ·
    303 · 369 ×4 · 393 ×3; design-system.md:377 · 384 ×2 · 394 ×5; test-plan.md:199 · 208 ×2; obs-plan.md:10 · 123 ×8 · 175).
  - zeros, by the sweep's control refusal (exit 3, `control: textblock, pwclear never fired over the seven masters' pre-pass text
    (5173f7e1)`): no master states a tool result as "one text block" in those words (`textblock`), and none states a password's
    paint ("in the clear", "painted masked") (`pwclear`) — the security-plan password row and arch Paint and shell carry no paint
    claim to correct; the new fact is an addition.
- **Dev-tool versions:** none — no host tool installed or upgraded. (The fast leg's clippy named lint `chunks_exact_to_as_chunks`,
  documented under rust-1.99.0 in its message; read, not changed.)
- **Harness / gate surface:**
  - `Harness::screenshot` / `screenshot_rect`: resolve at the harness's own time (no tick, no pump), paint the viewport at its scale
    and device size into `VelloCpuImageRenderer`, then composite over opaque white (premultiplied source-over: each channel
    `+ (255 - alpha)`, alpha 255); a rectangle is clipped to the viewport, an empty one is `EmptyRect`; a document whose
    `can_paint_text` reads false is `FontsUnavailable` with nothing painted. No path prints, panics or logs (probe over
    `image.rs` and `screenshot.rs` outside their test modules: 0 at exit 1).
  - `scripts/agent-run.sh` unchanged; `run stand` now runs 36 files.
  - No CI step changed.
- **Cross-project / external claims:**
  - `inputs.py verify`: `inputs: 4 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 ·
    uncited 2 · unparsed 0`.
    - I1 · message (the overseer, relaying the founder, P4, 2026-10-12 18:29Z) · copy message · n/a — cited plan.md (11 sites):
      fork 1 In-band, fork 2 Engine masks.
    - I2 · `../additional/escher-overseer/relays/headless-screenshot-founder-answers.md` · copy no-repo · unchanged — cited plan.md
      (7 sites): the overseer's record of the founder's two chosen labels («В самом ответе», «Движок маскирует»).
    - I3 · message (the session's user, P5, 2026-10-12) · copy message · n/a — UNCITED by the plan; carried here: the word "yes —
      apply" on the P5 card below. Its giver is superseded by I4.
    - I4 · message (delegate the overseer, 2026-10-12) · copy message · n/a — UNCITED by the plan; carried here: the giver of the
      P5 word is delegate the overseer, not the founder; I3's word stands.
  - inputs#I4 (the newest entry naming a P5 card; I3 names the same card and is superseded for its giver) →
    `.andromeda/runs/2026-10-12T18-14-09-phase/p5-card-2.md`, `## Beyond the plan`, verbatim; authority: delegate the overseer,
    2026-10-12 — route placements, not a fork the operator owns:
    | # | proposal | owner |
    |---|---|---|
    | 1 | Our merge surface in the inherited engine files this chunk edits: `packages/blitz-paint/src/render.rs` (the password mask) and `packages/blitz-dom/src/document.rs` (the font reading), plus `packages/blitz-dom/src/node/text.rs` if step 3 touches it | a `CARRY:` on the existing entry "Upstream sync ahead of polish and ship" (working-route.md:86), written at this chunk's wrap |
    | 2 | The overlay scrollbar's fade reads the wall clock (`packages/blitz-dom/src/document.rs:1782-1801`), so a shot within 0.7 s of a scroll can differ. Stated as a limit in the verb's help; moving the fade onto the harness clock is an engine change for every document, not asked here | a `CARRY:` on the existing entry "Stand requirement sweep" (working-route.md:96), written at this chunk's wrap |
    Measured since the card: step 3 did NOT touch `text.rs`; `document.rs` gained `layout_as_text_input` beside the font reading
    (row 1's merge surface is the two engine files, `document.rs` now with two methods).
  - CI: `CI#38297557417` on `5173f7e12f96` (the pre-CI commit, the chunk's final HEAD) — `verdict: green · checks 16/16 · wall 1023 s`,
    the MSRV build and the windows, macos, ios and android legs among them (`evidence/operator-pass.md`). The verdict was taken on
    that tree; this wrap's commit adds to it.
- **Reverted / negative API facts:** the harness's white ground was first written as a fill before `paint_scene` (`kurbo::Rect`,
  `peniko::{Color, Fill}`, as `examples/screenshot.rs` does), and replaced before any gate ran: neither crate is re-exported by the
  three dependencies the plan allows, and adding them would make the harness five.
- **Insufficient fixes (written, kept, not the remedy):** the password mask. It removes the typed characters from every frame — two
  equal-length values paint byte-identical frames, focused or not — but a CLICK inside a password field holding a value still
  places the caret by the editor's real text geometry (blitz-dom's pointer path, `events/pointer.rs`, unchanged), and the mask
  then paints the caret at the bullet for that index; which bullet a given click lands on can depend on the characters' widths.
  Not measured by any check (the focused-caret check clicks past the text's end). Owner: a route owner, on the direction below
  (Decisions).
- **Spec claims disproved by measurement:**
  - research.md, Sweeps: `refusal.rs:306` listed "no-change (other tables)" — it counts `CAUSES` plus the five faults and moved
    13 → 14 (measured: `refusal.rs::tests::a_refusal_reads_as_its_cause_then_its_remedy` red until moved). A research artifact, not
    a master.
  - plan.md step 5 ("fills the frame with opaque white, then runs `paint_scene`") beside the acceptance "the harness's three more"
    dependencies: jointly unsatisfiable as written (see Reverted); resolved by the composite, the acceptance kept.
  - The CARRY hypothesis "the engine lays out and paints a password input's characters like any text input's" is CONFIRMED on a
    frame, not disproved (`evidence/controls.md`, step 1).
- **Expected amendments (from plan):**
  - arch §Standard Contracts → Driver session (eleven verbs, seven instance verbs; `screenshot`, its `id`, `width` · `height` ·
    `png`; `FieldKind::Png`; nine causes; `Outcome::Shot`; the MCP image block) — carried: Schema / config, Symbols. Sites:
    `verbs10` architecture.md:133 ×2 · `inst6` architecture.md:133; `causes8` and `kinds9` 0 in architecture.md (their sites are in
    security-plan, test-plan, obs-plan).
  - arch §Standard Contracts → Test harness (`Harness`'s screenshot) — carried: Symbols, Harness / gate surface. No count pattern.
  - arch §Standard Contracts → Paint and shell (a password input paints masked) — carried: Symbols, Insufficient fixes. `pwclear`
    0 by control refusal: an addition.
  - arch §Established Decisions → Driver session (the image in-band on socket, stdout and MCP stdio — Boundary widening, the
    founder's fork 1 relayed verbatim by the overseer, `inputs#I1`, `inputs#I2`; driver dependencies five, harness raster three) —
    carried: Schema / config, Dependencies. Sites: `drvdeps4` architecture.md:101.
  - arch §Established Decisions (a password paints masked in every document — engine behaviour change, the founder's fork 2,
    `inputs#I1`, `inputs#I2`) — carried: Symbols, Insufficient fixes; plus the new public `layout_as_text_input` it rests on.
  - arch §Existing Scopes (escher-driver modules thirteen → fourteen, `image`; blitz-tests three files) — carried: Crates / modules,
    Counts. Sites: `modules13` architecture.md:257; `readers18` architecture.md:263.
  - security-plan §Input Validation (Driver command schema row — eleven verbs, nine causes; CLI arguments row — `screenshot`; MCP
    server row — the image block; password row — painted masked, measured on a frame) and §Threat Model Summary / §API Security
    (pixels in the ratified crossing, the bound unchanged) — carried: Schema / config, Coverage. Sites: `verbs10`
    security-plan.md:81 · :83; `inst6` :81 · :212 · :392; `causes8` :81 · :346 · :392.
  - test-plan §1 · §2 (Pixel tests: the repeat check) · §3 Session lifecycle (`session-binary`: the verb) · §4 · §6 — counts and new
    files — carried: Counts, Outcome. Sites: `verbs10` test-plan.md:21 · :143 and registries/contracts/test-plan/session-lifecycle.md:9;
    `inst6` :26 · :143 · :200; `causes8` :21 · :26 · :143 · :181; `kinds9` :143; `readers18` :101; `stand131` :116; `targets110` :66.
  - obs-plan §4 (`verb` and `tool` gain `screenshot`, the cause set nine) · §3 (the no-subscriber census gains the new check files) —
    carried: Schema / config, Coverage. Sites: `causes8` obs-plan.md:106 · :296; `inst6` :296.
  - a11y-plan §7 (a password painted masked, measured; the screenshot one more reader of the snapshot model's bounds) — carried:
    Symbols, Coverage. No count pattern hits a11y-plan.
  - design-system §Surface: cli → Component Patterns (the `screenshot` answer line, the MCP image block) · §Typography → Loading (no
    usable font is refused `fonts-unavailable`) — carried: Schema / config. The `screenshot` hits in design-system.md (377 · 384 ×2 ·
    394 ×5) name the upstream example: no change.
  - layout-templates §Surface: cli → Primary screens (`screenshot` in the usage line, eleven tools) — carried: Schema / config.
    Sites: `verbs10` layout-templates.md:74 ×2; `inst6` :74.
  - matrix#v010-13 notes — 2026-10-12 — ledger-note — owner P7.3. Its pins as moved: `mcp.rs::tests::the_listing_is_the_eleven_verbs_of_the_table`
    (renamed), `mcp_server.rs::the_mcp_role_completes_the_counter_flow_by_tool_calls_alone` (count eleven), `stand_mcp_tools.rs::a_scripted_mcp_session_completes_the_counter_flow_and_parents_each_command`
    (unchanged); all green.
  - matrix#v010-14 notes — 2026-10-12 — ledger-note — owner P7.3. `stand_mcp_tools.rs::every_cause_a_tool_call_reaches_is_served_with_its_remedy`
    serves eight (the no-font fixture adds `fonts-unavailable`), `describe.rs::tests::every_text_in_it_is_its_tables` and
    `cli_commands.rs::help_answers_the_tools_description_and_reaches_no_session` read eleven verbs and nine causes; all green.
- **Coverage of new surfaces:**
  - `screenshot` (in process · CLI · MCP · the socket's `call`) → validation {`validate`: optional `id`, `ArgKind::Id` bound; wrong
    kind or extra argument `malformed`✓} · instrumentation {`command` span `verb`/`cause`; `tool_call` `tool=screenshot`✓} · PII
    {the image is screen content in the ratified in-band crossing; logs carry no image byte — 0 occurrences of the first 64 base64
    characters in the host's and the `mcp` role's stderr at `trace`, both builds✓} · tests {unit + integration + binary✓} · a11y
    {n/a — reads the snapshot's bounds and paints, changes no focus or tree: a shot leaves `has_changes` false, focus and the diff
    unchanged✓} · tokens {n/a}
  - the password input's paint → validation n/a · instrumentation n/a · PII {masked✓ — byte-identical frames for equal-length
    values; the click-placed caret index unmeasured (Insufficient fixes)} · tests {integration, both layout modes✓} · a11y {the
    snapshot still reads `MASKED_VALUE`; no tree change✓; windowed paint unrunnable-here} · tokens {inherits the input's text
    colour and caret colour✓}
  - `Harness::screenshot` / `screenshot_rect` → validation {rect clipped, empty refused✓} · instrumentation {n/a — the harness
    writes no record (obs-plan §4)} · PII n/a · tests {unit 6 + integration✓} · a11y n/a · tokens n/a
  - `fonts-unavailable` → validation {checked before any paint✓} · instrumentation {`cause=fonts-unavailable` on the span✓} · PII
    {fixed text✓} · tests {stand_act_screenshot, stand_act_spans, stand_mcp_tools✓} · a11y n/a · tokens n/a

## Deviations from intent
- **White ground composited after paint, not filled before** (plan step 5): the fill needs `kurbo` and `peniko`, which none of the
  three allowed harness dependencies re-exports; `vello_cpu` writes premultiplied RGBA8 (its `Pixmap`), so source-over white is
  `c + (255 - a)` per channel, exact. Keeps the acceptance's "harness three more". Accepted by the operator's delegate, 2026-10-12.
- **The mask's layout lives in blitz-dom** (`BaseDocument::layout_as_text_input`, a second new public method in `document.rs`):
  blitz-paint holds no font context, and a host or empty context would paint other glyphs than the input's. `document.rs` is a
  listed file; step 3 named `render.rs` and `text.rs` only. Accepted, as above.
- **A password field scrolls only to keep the caret in its content box**, measured on the bullets, instead of the editor's stored
  `scroll_offset` (which follows the real glyph widths). Accepted, as above.
- **An element shot that crops to no pixel is refused `off-screen`** (the harness's `EmptyRect`); the plan named no cause for it.
- **MCP bound on an image result**: its whole written form ≤ 1,048,576 bytes; a text-only result keeps the text-length rule.
- **The PNG encoder's failure path** falls back to an empty `png` with a comment that it is unreachable (a raster always fills its
  non-empty size, the encoder writes to memory), so the driver path has no panic.
- **`cli_commands`'s shot check is stronger than the plan's**: the CLI `png` is decoded (a hand base64 reader and the PNG header,
  since `seven_guis` has no `png` dependency) and equals the PNG `Session::run` returns in process; the plan asked for 800 × 600.
- **Unknown-verb stand-in**: the eight sites take `capture`.
- **README.md is stale and out of this chunk's write set**: README.md:48 still says the MCP client "gets the ten driver commands as
  tools" and "Its screenshot is still to come" (architecture.md:210 records README naming the screenshot as still to come). Not
  edited — README was on no list. Owner: P2 / P5 to place.
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 30 · listed 30 · recorded 0 (companion 0 · mechanical
  0 · in-intent 0 · widening 0) · absorbed 0 · excluded 60`, base `938cd1b8`).

## Decisions & corrections
- The operator's delegate accepted implement's departures as reported (2026-10-12, in this session) and directed: "the
  click-vs-bullet caret mismatch goes to the wrap card for a route owner" — the caret a click places inside a password field
  holding a value is chosen by the real characters' widths (Insufficient fixes). Authority: delegate the overseer, 2026-10-12, in
  this session (the session's driver per inputs#I4).
- The operator pass ran on the delegate's direction, in plan order: hygiene green; the pre-CI commit `5173f7e1`; the push entry run
  BY HAND exactly as the plan writes it (it moves history; never through `--operator`); the CI read green (`evidence/operator-pass.md`).
- The delegate directed this wrap: P1 only, then stop; record what the leaf cascade lists on cascade.py v1.6, verbatim, in the
  run dir — written to `{run_dir}/cascade-leaves-listing.md` (whole mode: 11 groups, 25 leaves, CLAUDE.md in one group, `masters`,
  14 source files; pass mode at P1: `pass: no source changed — no group`; 1 unmapped host leaf).
- Sweep hazards found this chunk:
  - `gate.py run … --operator {n} --dry-run` WRITES the run dir's gate trail: once the trail is tracked (after the pre-CI commit), a
    dry run before the push entry dirties the tree and its `git diff --quiet` guard fails — set aside, restore, run, write back
    (operator-pass.md, entry 20).
  - `cascade.py sweep --pattern` takes ALL its ids after ONE flag (`--pattern a b c`); repeated `--pattern` flags keep only the last
    (measured: ten flags ran one pattern).
  - the token `screenshot` stands 32 times in the masters, 31 of them the upstream example, the browser's capture or a Cargo
    feature — a grep for the verb reads them as the verb.
  - the `examples/seven_guis` package has no `png` dev-dependency: a binary-side PNG check decodes by hand.
  - a `pub(crate)` blitz-dom field (`font_ctx`) is the reason a paint-side mask needs a document method: a paint crate cannot reach
    the document's fonts.
- Skill-reference drift: the wrap SKILL.md's P1 site-sweep text says to HALT on a stamp other than `cascade v1.4`; the deployed
  tool prints `cascade v1.6`, which `amendment-flow.md` §Tool names and the operator's delegate named as just deployed. Proceeded
  on v1.6.

## Outcome
- **v010-07** (with no display, an image of the whole stand screen and one cropped to an element by id) — MET against the diff:
  `screenshot` accepted on each lean task in both modes as an 800 × 600 PNG equal to the harness's raster, text painting inside
  `#task-title`, the page reading `#f5f5f5`; `--id crud-list` 369 × 420 equal to that rectangle; the binary answers status 0 with
  a `png` equal to the in-process PNG; MCP answers an image block then a text block. Matrix: `implemented`, ref
  `stand_act_screenshot.rs::the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints` · `…::an_element_shot_is_its_bounds_rounded_outward`
  · `cli_commands.rs::a_screenshot_answers_one_line_whose_png_is_the_screen` · `stand_mcp_tools.rs::a_screenshot_tool_call_answers_its_image_then_its_text_and_parents_its_command`
  · `mcp_server.rs::a_screenshot_tool_call_answers_an_image_block_then_a_text_block`.
- **v010-08** (two shots of one state in separate runs pixel-identical) — MET: two children and the parent print byte-identical PNGs
  for 12 shots (four tasks whole, `crud-list`, `counter-value`, the flight booker with `flight-start` focused by a click, both
  modes), under the per-package and the workspace build. Matrix: `implemented`, ref
  `stand_screenshot_repeat.rs::a_shot_of_one_state_is_the_same_in_two_runs_and_in_this_one`.
- (security, a11y) a typed password never leaves as pixels — MET for the typed characters (engine and driver checks, both modes);
  the click-placed caret index is the open remainder (Insufficient fixes), outside the criterion's measured cases.
- (tests, design) no-font fails and nothing skips — MET (`fonts-unavailable`, next snapshot unchanged; the font reading's unit test).
- (security) refusals hold nothing of the call — MET (`not-found`, `stale`, `off-screen` leave the instance unchanged; CLI `--id`
  on an unread id exits 1 writing nothing typed); no-panic probe 0.
- (obs) one `command` line per shot, `cause` on a refusal, `tool_call` `tool=screenshot` parenting it; 0 ids, names, sentinels and
  image bytes on stderr at `trace`, both builds — MET.
- (a11y, layouts) a shot changes nothing — MET; the outside-write-set probe green.
- (design, layouts) table order on both surfaces, `help` last, one uncoloured line, no terminal probe — MET.
- (arch, security) no package enters `Cargo.lock`, audit green, driver deps five, harness three more — MET.
- (tests, matrix) v010-13 and v010-14 stay proved at eleven verbs and nine causes under their moved pins — MET; the two dated notes
  are P7.3's.
- (tests) fast and doc legs pass; `run stand` ends failed 0 — MET.
- Gates — implement's final full firing (run dir `.andromeda/runs/2026-10-12T18-43-51-implement/`), every non-operator entry green:
  - `cargo test -p escher-driver --locked` — green (exit 0 · lacks FAILED) — 72 unit tests.
  - `cargo test -p blitz-dom --locked --lib` — green.
  - `cargo test -p blitz-test-harness --locked` — green — 17 tests.
  - `cargo build --locked -p seven_guis --bin escher-session && target/debug/escher-session help` — green (contains
    `"name":"screenshot"` and `"name":"fonts-unavailable"`) — the smoke.
  - `cargo test -p blitz-tests --locked --test password_painted_masked --test stand_act_screenshot --test stand_screenshot_repeat --test stand_act_spans --test stand_mcp_tools --test stand_act_scroll --test stand_mcp_masked` — green.
  - `cargo test -p seven_guis --locked --test cli_commands --test mcp_server --test host_log --test mcp_log` — green.
  - `cargo test --workspace --locked --test password_painted_masked --test stand_act_screenshot --test stand_screenshot_repeat --test stand_act_spans --test stand_mcp_tools --test cli_commands --test mcp_server --test host_log --test mcp_log` — green.
  - `cargo test -p blitz-tests --locked --test stand_act_refused --test stand_act_diff --test stand_act_filled --test stand_snapshot_text --test stand_session_lifecycle --test stand_accessibility_ids --test stand_actionable_keys --test stand_snapshot_state --test background_size` — green.
  - `cargo test -p seven_guis --locked --test cli_flow --test host_binary --test host_timer` — green.
  - the Cargo.lock package probe — green (exit 1 · last line 0).
  - the no-panic probe over `image.rs` and `screenshot.rs` — green (exit 1 · 0).
  - the terminal probe over the driver — green (exit 1 · 0).
  - the outside-write-set probe (`git diff --quiet 938cd1b8… -- …`) — green (exit 0).
  - `bash scripts/agent-run.sh boot` · `bash scripts/agent-run.sh run stand` — green; `run.end` passed 139 · failed 0 · ignored 7 over 36 files.
  - `bash .github/scripts/ci-leg.sh audit` — green.
  - `bash .github/scripts/ci-leg.sh fast` — green (red once at the first firing: clippy `chunks_exact_to_as_chunks` on the harness's
    composite loop, fixed with `as_chunks_mut`; then green) — 849 passed · 0 failed · 12 ignored.
  - `bash .github/scripts/ci-leg.sh doc` — green.
  - operator entries (`evidence/operator-pass.md`): `gate.py hygiene` — green (`hygiene: clean`); `bash .github/scripts/ci-leg.sh fast
    && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` — run by hand, exit 0, pushed
    `938cd1b8..5173f7e1`; `ci.py conclusion --sha HEAD --wait 1800` — green, `CI#38297557417` 16/16 on `5173f7e12f96`.
  - Smoke: the rebuilt binary answers `help` with eleven verbs and nine causes; `agent-run.sh status` truthful. No headful
    self-verify exists for the paint change; the engine check is its witness.
  - Mutation controls (`evidence/controls.md`): mask off, crop rounded inward, font check removed — each turned exactly its target
    cases red; each restore sha256-equal.
- Watches: none folded.
- Outcome basis: the operator pass's final state — commit list `5173f7e1` (`git log 938cd1b8..HEAD`) and the final HEAD's CI run
  `CI#38297557417` recorded in `evidence/operator-pass.md`; implement's report (this session) for what only it holds — the
  gate firings, the controls and the sizes (`evidence/counts.md`: largest whole-screen answer 22,404 bytes, flight-booker, 2.1 % of
  the bound). After the pass, the delegate accepted the departures and directed the caret item to the card (Decisions).
- Process hygiene: implement's census — none left running; the operator pass started the fast leg's tests and the push, both
  ended; re-measured at this P1 (`ps -eo pid,comm,args`, filtered for `escher-sessio`): none running.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 938cd1b8 (the parent of the oldest pre-CI commit 5173f7e1) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 5 line(s) in 4 range(s)
added: 1062-1063 · 1066 · 1093 · 3104
### examples/seven_guis/tests/cli_commands.rs — added 80 line(s) in 6 range(s)
added: 2 · 4-5 · 18 · 119 · 282-354 · 407-408
- 282-299 @283 «fn decode_base64(text: &str) -> Option<Vec<u8>> {»
  - 285-287 «if text.len() % 4 != 0 {»
  - 289-297 «for chunk in text.as_bytes().chunks(4) {»
- 301-308 @302 «fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {»
- 310-353 @311 «fn a_screenshot_answers_one_line_whose_png_is_the_screen() {»
  - 312-315 @313 «let mut held =»
  - 316-319 «let call = Call {»
  - 320-322 «let Ok(Outcome::Shot { png, .. }) = held.run(&call) else {»
  - 326-329 «assert!(»
  - 332-334 «let shot = string_at(&answer, "png")»
  - 336-339 «assert!(»
  - 348-351 «assert!(»
### examples/seven_guis/tests/host_log.rs — added 34 line(s) in 6 range(s)
added: 2-4 · 108 · 127-134 · 158-171 · 234-237 · 239-242
  - 158-167 @159 «let images: Vec<String> = answers[3..]»
  - 168-171 «assert!(»
  - 234-237 «let image_bytes: usize = images»
### examples/seven_guis/tests/mcp_log.rs — added 32 line(s) in 7 range(s)
added: 2-4 · 106-112 · 115 · 144-157 · 254-257 · 259 · 261-262
  - 144-153 @145 «let images: Vec<String> = replies[6..8]»
  - 154-157 «assert!(»
  - 254-257 «let image_bytes: usize = images»
### examples/seven_guis/tests/mcp_server.rs — added 54 line(s) in 4 range(s)
added: 3-5 · 125 · 128-145 · 305-336
  - 131-134 «assert!(»
  - 135-139 «let described = listing»
- 305-335 @306 «fn a_screenshot_tool_call_answers_an_image_block_then_a_text_block() {»
  - 311-316 «assert!(»
  - 317-320 «assert!(»
  - 321-326 «let (is_error, refused) = call(»
  - 327-330 «assert!(»
### packages/blitz-dom/src/document.rs — added 73 line(s) in 2 range(s)
added: 1022-1068 · 3648-3673
  - 1022-1035 @1025 «pub fn can_paint_text(&self) -> bool {»
  - 1037-1067 @1040 «pub fn layout_as_text_input(»
- 3649-3673 @3650 «mod font_reading_tests {»
  - 3654-3659 «fn doc_with(font_ctx: FontContext) -> BaseDocument {»
  - 3661-3665 @3662 «fn a_context_holding_a_decodable_font_can_paint_text() {»
  - 3667-3672 @3668 «fn a_context_holding_no_family_cannot_paint_text() {»
### packages/blitz-paint/src/render.rs — added 91 line(s) in 3 range(s)
added: 902-906 · 935 · 956-1040
  - 956-963 «fn caret_color(&self) -> peniko::Color {»
  - 965-1039 @969 «fn draw_masked_input_text(»
### packages/blitz-test-harness/Cargo.toml — added 5 line(s) in 2 range(s)
added: 18 · 24-27
### packages/blitz-test-harness/src/lib.rs — added 3 line(s) in 3 range(s)
added: 15 · 22 · 29
### packages/blitz-test-harness/src/screenshot.rs — new file · 245 line(s)
- 10-17 @12 «pub struct PixelRect {»
- 19-25 @21 «pub struct Raster {»
- 27-35 @29 «pub enum ScreenshotError {»
- 37-103 «impl<D: Document> Harness<D> {»
  - 38-42 @40 «pub fn screenshot(&mut self) -> Result<Raster, ScreenshotError> {»
  - 44-71 @46 «pub fn screenshot_rect(&mut self, rect: PixelRect) -> Result<Raster, ScreenshotError> {»
  - 73-102 «fn paint_viewport(&mut self) -> Result<Raster, ScreenshotError> {»
- 105-245 @106 «mod tests {»
  - 110-112 «const PAGE: &str = r#"<html><body style="margin:0; background:#ff0000;">»
  - 114-124 «fn harness(font_ctx: Option<blitz_dom::FontContext>) -> Harness {»
  - 126-128 «fn bullet_font() -> Option<blitz_dom::FontContext> {»
  - 130-138 «fn pixel(raster: &Raster, x: u32, y: u32) -> [u8; 4] {»
  - 140-147 @141 «fn the_whole_viewport_is_painted_at_its_device_size() {»
  - 149-170 @150 «fn what_the_page_leaves_unpainted_reads_opaque_white() {»
  - 172-189 @173 «fn a_rectangle_is_that_part_of_the_frame() {»
  - 191-222 @192 «fn a_rectangle_past_the_viewport_is_clipped_and_one_outside_it_is_an_error() {»
  - 224-238 @225 «fn a_document_with_no_font_is_refused() {»
  - 240-244 @241 «fn a_shot_is_the_same_twice() {»
### packages/escher-driver/Cargo.toml — added 1 line(s) in 1 range(s)
added: 16
### packages/escher-driver/src/cli.rs — added 13 line(s) in 5 range(s)
added: 460 · 464 · 467-474 · 673 · 787-788
### packages/escher-driver/src/client.rs — added 1 line(s) in 1 range(s)
added: 339
### packages/escher-driver/src/command.rs — added 34 line(s) in 12 range(s)
added: 138-142 · 155 · 228-231 · 457-463 · 491 · 505 · 587 · 651-660 · 750 · 788 · 822 · 832
  - 138-142 @139 «Screenshot {»
### packages/escher-driver/src/describe.rs — added 6 line(s) in 2 range(s)
added: 145 · 149-153
### packages/escher-driver/src/execute.rs — added 74 line(s) in 10 range(s)
added: 3-5 · 11 · 63-71 · 108-114 · 205-229 · 242 · 289-291 · 326 · 382-404 · 522
- 3-5 «use blitz_test_harness::{»
  - 63-71 @64 «Shot {»
- 382-403 @385 «fn crop(harness: &Harness<DioxusDocument>, target: &Target) -> Option<PixelRect> {»
  - 394-396 «if right <= left || bottom <= top {»
  - 397-402 «Some(PixelRect {»
### packages/escher-driver/src/image.rs — new file · 107 line(s)
- 3-15 @6 «pub(crate) fn png(width: u32, height: u32, rgba: &[u8]) -> Option<Vec<u8>> {»
- 19-37 @20 «pub(crate) fn base64(bytes: &[u8]) -> String {»
  - 22-35 «for chunk in bytes.chunks(3) {»
- 39-107 @40 «mod tests {»
  - 43-56 @44 «fn base64_reads_the_rfc_4648_test_vectors() {»
  - 58-65 @59 «fn base64_writes_every_sextet_of_the_alphabet() {»
  - 67-76 «fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {»
  - 78-92 @79 «fn a_png_decodes_to_the_pixels_it_was_given() {»
  - 94-100 @95 «fn a_png_holds_no_time_or_text_chunk() {»
  - 102-106 @103 «fn pixels_that_do_not_fill_the_size_write_nothing() {»
### packages/escher-driver/src/json.rs — added 35 line(s) in 7 range(s)
added: 12 · 242-255 · 295-299 · 616-625 · 627 · 638-640 · 657
- 242-248 @243 «fn shot(width: u32, height: u32) -> Object {»
- 250-254 @252 «pub(crate) fn shot_without_image(width: u32, height: u32) -> String {»
### packages/escher-driver/src/lib.rs — added 11 line(s) in 4 range(s)
added: 30-32 · 48-51 · 79-81 · 103
### packages/escher-driver/src/mcp.rs — added 75 line(s) in 18 range(s)
added: 22 · 65 · 70-72 · 460 · 464-465 · 473-482 · 490 · 498 · 503-504 · 506-510 · 518-525 · 529 · 531 · 646-648 · 962
       1066 · 1075 · 1291-1322
### packages/escher-driver/src/refusal.rs — added 19 line(s) in 12 range(s)
added: 29-30 · 34 · 43 · 58 · 85-87 · 117 · 210 · 220 · 222 · 279-283 · 285 · 320
### packages/escher-driver/src/schema.rs — added 75 line(s) in 17 range(s)
added: 187-188 · 204 · 415-449 · 571 · 574 · 629-639 · 644-645 · 650 · 669 · 698-704 · 706 · 751 · 774-778 · 834-836
       845 · 907 · 909
- 415-448 «pub(crate) const SCREENSHOT: VerbSpec = VerbSpec {»
  - 418-420 «help: "the screen, or the element an id names, as a PNG image; an element shot is cropped to \»
  - 421-427 «args: &[ArgSpec {»
  - 428-447 «fields: &[»
### tests/blitz-tests/Cargo.toml — added 1 line(s) in 1 range(s)
added: 37
### tests/blitz-tests/tests/password_painted_masked.rs — new file · 131 line(s)
- 22-39 «fn harness(kind: &str, value: &str, incremental: bool) -> Harness<HtmlDocument> {»
  - 23-28 «let html = format!(»
  - 29-38 «Harness::from_html_with(»
- 41-48 «fn paint(harness: &mut Harness<HtmlDocument>) -> Vec<u8> {»
  - 43-47 «render_to_buffer::<VelloCpuImageRenderer, _>(»
- 50-61 @51 «fn frame(kind: &str, value: &str, focused: bool, incremental: bool) -> Vec<u8> {»
  - 53-59 «if focused {»
- 63-82 @64 «fn field_pixels(kind: &str, value: &str, focused: bool, incremental: bool) -> Vec<u8> {»
  - 66-68 «if focused {»
  - 72-75 «let (x1, y1) = (»
  - 77-80 «for y in y0..y1 {»
- 84-93 @85 «fn a_text_input_paints_its_characters() {»
  - 86-92 «for incremental in [false, true] {»
- 95-104 @96 «fn two_password_values_of_one_length_paint_the_same_frame() {»
  - 97-103 «for incremental in [false, true] {»
- 106-115 @107 «fn a_password_input_paints_its_bullets() {»
  - 108-114 «for incremental in [false, true] {»
- 117-131 @118 «fn a_focused_password_input_paints_its_caret() {»
  - 119-130 «for incremental in [false, true] {»
### tests/blitz-tests/tests/session_common/mod.rs — added 28 line(s) in 3 range(s)
added: 104-129 · 181 · 184
- 104-107 @105 «pub fn screenshot() -> Call {»
- 109-112 @110 «pub fn screenshot_of(id: &str) -> Call {»
- 114-119 @115 «pub struct Shot {»
- 121-128 @123 «pub fn shot(session: &mut Session, call: &Call) -> Option<Shot> {»
  - 124-127 «match session.run(call).ok()? {»
### tests/blitz-tests/tests/stand_act_screenshot.rs — new file · 304 line(s)
- 22-25 «use session_common::{»
- 44-54 @46 «fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {»
- 56-59 «fn pixel(rgba: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {»
- 61-69 @62 «fn crop(rgba: &[u8], frame_width: u32, (x, y, width, height): (u32, u32, u32, u32)) -> Vec<u8> {»
  - 64-67 «for row in y..y + height {»
- 71-77 @72 «fn bounds(session: &Session, id: &str) -> (f64, f64, f64, f64) {»
- 79-122 @80 «fn the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints() {»
  - 81-121 «for incremental in [false, true] {»
- 124-149 @125 «fn an_element_shot_is_its_bounds_rounded_outward() {»
  - 126-148 «for incremental in [false, true] {»
- 151-196 @152 «fn an_element_shot_is_refused_as_every_id_addressed_call_is() {»
  - 153-195 «for incremental in [false, true] {»
- 198-221 @199 «fn an_app_with_no_usable_font_is_refused_and_nothing_changes() {»
  - 200-220 «for incremental in [false, true] {»
- 223-230 @224 «fn password_fixture() -> Element {»
  - 225-229 «rsx! {»
- 232-239 @233 «fn text_fixture() -> Element {»
  - 234-238 «rsx! {»
- 241-254 @242 «fn typed_shot(fixture: fn() -> Element, text: &str, incremental: bool) -> Vec<u8> {»
  - 243-245 «let mut session = Session::start("typed", || {»
  - 247-250 «assert!(»
  - 251-253 «shot(&mut session, &screenshot())»
- 256-269 @257 «fn a_typed_password_never_leaves_as_pixels() {»
  - 259-268 «for incremental in [false, true] {»
- 271-304 @272 «fn a_shot_changes_nothing() {»
  - 273-303 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_scroll.rs — added 1 line(s) in 1 range(s)
added: 505
### tests/blitz-tests/tests/stand_act_spans.rs — added 47 line(s) in 13 range(s)
added: 3-6 · 12-13 · 26 · 34-36 · 176 · 184 · 340-346 · 379-380 · 408-409 · 412-413 · 540-541 · 572 · 617-635
- 34-36 «use session_common::{»
### tests/blitz-tests/tests/stand_mcp_tools.rs — added 166 line(s) in 6 range(s)
added: 14-20 · 29 · 38 · 517-545 · 551 · 558-684
- 558-571 @559 «fn boot_without_font(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {»
  - 560-570 «move |app| {»
- 573-590 @574 «fn decode_base64(text: &str) -> Option<Vec<u8>> {»
  - 576-578 «if text.len() % 4 != 0 {»
  - 580-588 «for chunk in text.as_bytes().chunks(4) {»
- 592-608 @595 «fn shot_result(reply: &str) -> Option<(String, String, String, bool)> {»
  - 602-607 «Some((»
- 610-683 @611 «fn a_screenshot_tool_call_answers_its_image_then_its_text_and_parents_its_command() {»
  - 612-682 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_screenshot_repeat.rs — new file · 145 line(s)
- 23-31 @24 «fn plan(task: LeanTask) -> (Option<&'static str>, &'static [Option<&'static str>]) {»
  - 25-30 «match task {»
- 33-41 «fn shoot(session: &mut Session, element: Option<&str>) -> Vec<u8> {»
  - 34-37 «let call = match element {»
  - 38-40 «shot(session, &call)»
- 43-64 @45 «fn shots() -> Vec<(String, Vec<u8>)> {»
  - 47-62 «for incremental in [false, true] {»
- 66-72 «fn hex(bytes: &[u8]) -> String {»
  - 68-70 «for byte in bytes {»
- 74-97 @75 «fn child_run() -> (Vec<String>, Option<u32>) {»
  - 76-84 «let output = Command::new(std::env::current_exe().expect("the test binary has a path"))»
  - 89-95 «for line in stdout.lines() {»
- 99-136 @100 «fn a_shot_of_one_state_is_the_same_in_two_runs_and_in_this_one() {»
  - 101-104 «let parent: Vec<String> = shots()»
  - 105-109 «assert!(»
  - 113-116 «assert!(»
  - 117-120 «assert!(»
  - 121-127 «assert!(»
  - 128-135 «for (index, ((one, two), here)) in first.iter().zip(&second).zip(&parent).enumerate() {»
- 138-145 @140 «fn child_prints_its_shots() {»
  - 142-144 «for (key, png) in shots() {»
