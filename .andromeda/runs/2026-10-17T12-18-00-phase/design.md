# design extract

## Relevance
partial — the chunk is an engine merge with no new surface or token, but it lands on painted, contrast-measured and agent-read surfaces whose recorded values the plan pins (stand colours, the stand's bundled font, the CLI's output form, the UA stylesheet), and it removes a surface the plan names (`bump`).

## Constraints
- The seven_guis accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), page `#f5f5f5` and text `#1a1a1a` are the recorded stand colours, and the plan requires every measured stand colour to equal its declared value (white on accent 5.47, on hover 6.83, on active 8.27). The merge changes no stand CSS, and a moved reading is re-measured, never re-coloured to pass (per design-system §Color Palette).
- The stand's semantic state colours (invalid `#e53e3e` / `#fff5f5` / `#c53030`, success `#ebf8ee` / `#68d391` / `#276749`) stay as declared. Whether upstream's delta touches `examples/seven_guis/` at all is research's question (per design-system §Color Palette).
- The headless stand registers bundled DejaVu Sans for every generic through `build_single_font_ctx`, with system fonts off. A driver `screenshot` requires `can_paint_text`, else `fonts-unavailable`. The merge must keep that font path working so that a shot still paints text. Whether upstream's delta touches `packages/blitz-dom/src/lib.rs`'s font-context builders or `can_paint_text` is research's question (per design-system §Typography).
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet once, through `DocumentConfig::default()`. Whether the 14 upstream commits edit `packages/blitz-dom/assets/default.css` or that wiring, and so move the engine's default colours, focus outline `#4D90FE` or button background `#EFEFEF`, is research's question (per design-system §Surface: desktop-native, and the engine defaults per design-system §Color Palette).
- `escher-session` and its `mcp` role are agent-read surfaces. They must keep one uncoloured line of JSON per answer, no ESC byte, no colour dependency and no terminal test. The keyboard-types 0.7 → 0.8 migration of `press`'s key names must leave that output form and the closed key-name words unchanged (per design-system §Surface: cli).
- Into-view scrolling is instant in every nested box and the driver's `scroll` is instant throughout. The smooth-scroll timings (300 ms cubic ease-in-out) and the overlay-scrollbar fade (`FADE_DELAY` 500 ms, `FADE_DURATION` 200 ms) are the recorded engine motion. Upstream's inline-layout and Taffy edits must not move them, and whether they touch `scrolling.rs` or `node/scrollbar.rs` is research's question (per design-system §Motion).
- Hit-testing stops at the padding box of a node that clips by `overflow`, and children paint in stacking order. The `covered` refusal and the screenshot's pixels rest on that. Upstream's #1198 baseline and inline-layout edits can move what is painted where, and a moved number is re-measured and recorded (per design-system §Depth Strategy).

## Patterns to follow
- Record a moved painted value the way the stand's re-colour was recorded: the old and new values, the measured ratio for each, and the check rows (`stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS`) that moved with it, citing the chunk report (per design-system §Color Palette).
- Keep the measured-claim form `(as measured at {chunk}/report.md)` for any screenshot or bounds number the merge moves. A shot of the whole stand is an 800 × 600 PNG equal to the harness raster (per design-system §Typography Loading, and §Depth Strategy for the hit readings).
- Migrate escher's own keyboard sites in the smallest edit inside our files, and leave the driver's output form unchanged (per design-system §Surface: cli).

## Anti-patterns to avoid
- Never re-colour or loosen a stand colour or contrast row to make a merged reading pass. The recorded stand colours are measured values, and a change to them is a decision of its own (per design-system §Color Palette).
- Never let a migrated or merged path write colour, an ESC byte or a terminal-dependent branch into `escher-session`'s stdout or the `mcp` role's replies (per design-system §Surface: cli).

## Contract bindings
- Token contrast ↔ a11y §Contrast: `stand_contrast` asserts the stand's colour pairs and states row by row against SC 1.4.3. A merged paint or layout change that moves a pixel or a computed colour moves that record (per design-system §Color Palette).
- Typography ↔ the test harness: `Harness::screenshot` and the driver's `screenshot` depend on the stand's bundled font context. The screenshot checks (`stand_screenshot_repeat`, byte-identical across processes) bind the font path to tests (per design-system §Typography).
- The CLI surface ↔ tests: the no-ESC reading is asserted on every answer line by the seven_guis checks' shared reader and on every MCP reply by the shared `Speaker` (per design-system §Surface: cli).
- Docs ↔ the wrap: design-system §Surface: cli names `bump`'s binary and its printed lines (Platform and Component Patterns). With `apps/bump` leaving, that text is stale at the wrap and moves with CLAUDE.md's `apps/` line and architecture's tree (per design-system §Surface: cli).

## Acceptance criteria contributions
- `stand_contrast` passes after the merge in both layout modes. Every measured stand colour equals its declared value, and white on the accent reads ≥ 4.5 (recorded 5.47), or a moved ratio is re-measured and recorded with no colour changed (per design-system §Color Palette).
- After the key migration, every `escher-session` answer line and every `mcp` reply is still one line of uncoloured JSON with no ESC byte, as asserted by the standing shared readers (per design-system §Surface: cli).
- A driver `screenshot` of a stand task still answers an 800 × 600 PNG, not `fonts-unavailable`, and is byte-identical across processes. If the merged layout moved pixels, the new reading is recorded, not suppressed (per design-system §Typography).
- At the wrap, design-system §Surface: cli no longer describes `bump` as a present binary (its Platform line and its "bump prints …" pattern) (per design-system §Surface: cli).
