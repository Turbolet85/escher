# design extract

## Relevance
partial — a sync adds no surface and no token, but its two upstream moves touch design-plan ground: Taffy main's float placement can shift the stand's painted layout and its contrast/screenshot readings, and `fb8d6c15e` changes the engine cursor reading that design-system §Iconography (Cursors and favicons (engine)) records.

## Constraints
- design-system §Color Palette (Core Colors, seven_guis accent) requires the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) on page `#f5f5f5` with text `#1a1a1a`, and requires every measured stand colour to equal its declared value. The take must leave every task CSS's colour declarations unchanged, the three `rgba(58, 92, 229, …)` uses included. Whether the merged tree still reads each declared value is the standing `stand_contrast` check's question.
- design-system §Color Palette (Core Colors, seven_guis accent) records white-on-accent at 5.47, white on hover 6.83 and white on active 8.27 against SC 1.4.3's 4.5. A Taffy rev move may shift a box but must not move a colour pair: a pair that reads a new ratio is re-measured and recorded, never loosened to pass.
- design-system §Iconography (Cursors and favicons (engine)) records the cursor order as CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise. It also records that `None` hides the cursor and resets it to Default in the shell. `fb8d6c15e` (`get_cursor` returning `Some(CursorIcon::Default)` over no box) moves that reading. Where the take carries it, the §Iconography line and its `document.rs` citation range must be restated at the wrap. Whether any escher reader (snapshot, driver, stand, harness) reads the cursor is research's question.
- design-system §Typography (Loading) requires the headless stand to register the bundled DejaVu Sans for every generic, with system fonts off. It also requires a driver `screenshot` to be refused `fonts-unavailable` when `can_paint_text` reads false. A sync must keep the stand's font context as built. Whether upstream's delta touches `stand.rs`, `build_single_font_ctx` or `can_paint_text` is research's question (the scope's diff names neither).
- design-system §Surface: cli (Component Patterns) requires `escher-session` and its `mcp` role to write uncoloured lines with no ESC byte, to add no colour dependency and to make no terminal test. The take must add no colour crate to the lock and no terminal-detection call to escher's binaries.
- design-system §Depth Strategy (Engine depth behavior) records the hit-testing walk order and the padding-box stop at a box that paint clips (PROVISIONAL `contain: paint` arm included). A Taffy float-placement change must leave those readings standing, proven by `hit_clipped_at_scrolling_box`.

## Patterns to follow
- A moved painted number is re-measured and recorded against the chunk's report, the way design-system §Color Palette (Core Colors, seven_guis accent) cites each ratio "as measured at" the chunk that took it. The record names the old value, the new value and the commit that moved it.
- A behaviour that upstream changed is restated with its upstream commit as the cause, the way design-system §Surface: desktop-native (Tokens (platform-specific)) records upstream #1189's move of `DEFAULT_CSS` into `DocumentConfig::default()`. `fb8d6c15e` / #1220's cursor change takes the same form in §Iconography.
- design-system §Typography (Loading) and §Surface: cli (Component Patterns) cite file:line ranges. Where the take shifts lines in a cited file (`packages/blitz-dom/src/document.rs`: 2260-2316 for the cursor, 1049-1062 for `can_paint_text`), the range is re-pointed at the wrap and never left stale.

## Anti-patterns to avoid
- Do not adjust a stand colour, shade or alpha so that a pair moved by layout reads green again. design-system §Color Palette (Core Colors, seven_guis accent) ties each colour to its measured ratio, so a colour change is a recorded re-colour, as 2026-10-14-stand-a11y-assertions' was.
- Do not let a merged dependency bring colour or terminal detection into the escher binaries' output path (design-system §Surface: cli (Component Patterns)).

## Contract bindings
- design ↔ a11y: the stand accent's ratios (design-system §Color Palette (Core Colors, seven_guis accent)) are SC 1.4.3 surface. `stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS` are the binding, re-proven after the take (a11y-plan §Contrast).
- design ↔ tests: a Taffy-moved box can change a whole-shot screenshot's pixels. A whole shot is an 800 × 600 PNG equal to the harness raster (design-system §Typography (Loading) for the font it paints with). That binds to the stand screenshot checks (`stand_screenshot_repeat`, the session screenshot checks) and their byte-identity readings.
- design ↔ engine cursor: the §Iconography (Cursors and favicons (engine)) cursor reading binds to upstream's `tests/blitz-tests/tests/pointer_leave.rs` (`!cursors().contains(&None)`) and to `hover_state_tests` in `document.rs`.

## Acceptance criteria contributions
- (design) After the take, `stand_contrast` passes in both layout modes with every stand colour reading its declared value (`#3a5ce5` / `#2a4cd3` / `#1f3fc1` on `#f5f5f5`, text `#1a1a1a`). Any ratio that moved is recorded with its new value and not loosened (per design-system §Color Palette).
- (design) Where the take carries `fb8d6c15e`, the §Iconography (Cursors and favicons (engine)) cursor line is restated: over no box the cursor reads `Default`, not none. Its `document.rs` citation is re-pointed. Where the take does not carry it, the line stands unchanged (per design-system §Iconography).
- (design) Every `escher-session` answer line and every `mcp` reply still carries no ESC byte, and the lock gains no colour or terminal-detection crate, as the shared readers assert (per design-system §Surface: cli).
- (design) The stand's whole-screen screenshot stays 800 × 600 and painted with the bundled DejaVu Sans, `can_paint_text` true. A pixel change caused by Taffy is recorded as re-measured, never accepted silently (per design-system §Typography).
