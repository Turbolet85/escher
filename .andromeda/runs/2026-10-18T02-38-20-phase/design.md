# design extract

## Relevance
partial — the chunk is engine readings (focusability, accessibility reads, hit, position, caret) with no new rendered surface; design binds only where a fix touches the stand's painted colours, the engine's hit and stacking behaviour, the into-view scroll's motion, the bundled font a caret measure rests on, or the driver's agent-read text output.

## Constraints
- Any stand markup change (debt 9's `task-header-spacer`, if «the box» is chosen in `examples/seven_guis/src/app.rs`) keeps the stand's declared palette — the accent, page and text values and their measured pairs — design-system §Color Palette (Core Colors) requires every measured stand colour to equal its declared value; no new colour enters the stand without a measured pair.
- A hit or position fix (debts 4, 5, 8) keeps the engine's stated hit walk — stacking order of hoisted and paint children, the stop at the padding box of a box that clips by `overflow` (root excepted), the node itself still answered inside its border box, and the measured own-box answers (`img`, `iframe` not entered, filled text `input`, a transformed-and-scrolled box hit where its rows show) — per design-system §Depth Strategy (Engine depth behavior); extending the stop to `contain: paint` is a change to that stated behaviour, so the section's reading moves with it.
- A `contain: paint` stop (debt 5) must agree with what paint clips: design-system §Depth Strategy (Engine depth behavior) frames the `overflow` stop as matching paint's clip; whether blitz-paint clips `contain: paint` content today is research's question.
- A change to `Node::absolute_position` (debt 4) must leave the into-view scroll's motion as stated — every scrolling box written at once, innermost first, the requested behaviour applied to the viewport alone, the driver's `scroll` instant within one settled step — per design-system §Motion (This project's values); `scroll_into_view` is one of the reader's seven callers and adds the offset back, so whether a fix double-counts there is research's question.
- A password-caret fix (debt 6) measures against the stand's own text setup — the bundled DejaVu Sans for every generic, system fonts off — per design-system §Typography (Loading), and the editor's fixed size per design-system §Typography (the `Text input editors` row); the caret's index readings (narrow vs wide characters) are font-dependent, so a fixture must pin its font context the same way.
- Any rewritten driver text (debt 5's or 9's `covered` meaning or remedy, schema help) still leaves `escher-session` and its `mcp` role as an agent-read surface: one uncoloured line of JSON per answer, no ESC byte, no terminal test, the `help` self-description written from fixed tables — per design-system §Surface: cli (Component Patterns).
- Narrowing `disabled` focusability to form controls (debt 7) concerns focus only; the engine's disabled checkbox/radio paint is a separate reading per design-system §Color Palette (Semantic Colors) — whether that painter reads `disabled` by the same presence rule is research's question, and a focus-only fix should not move it.

## Patterns to follow
- Stand colours as declared values in each task's CSS, measured as text/background pairs by the contrast harness, the re-colour precedent moving every use (non-text and `rgba` alpha kept) together — per design-system §Color Palette (Core Colors).
- Engine behaviour changes recorded as dated readings «since {chunk}» against a measured report, with the pin that holds them — per design-system §Depth Strategy (Engine depth behavior).
- A headless check boots under a single bundled font context (`build_single_font_ctx`) so text-dependent readings are not vacuous — per design-system §Typography (Loading).
- Driver output as fixed schema words on one line, the no-ESC reading asserted by the shared readers — per design-system §Surface: cli (Component Patterns).

## Anti-patterns to avoid
- Adding colour, an ESC byte or a terminal test to any `escher-session` answer or `help` line when the `covered` text is reworded — per design-system §Surface: cli (Component Patterns).
- Hardcoding a new stand colour (e.g. for a resized or reworded header spacer) outside the declared, measured palette — per design-system §Color Palette (Core Colors).
- Letting a nested into-view scroll animate as a side effect of a position-reader fix — per design-system §Motion (This project's values).

## Contract bindings
- design ↔ a11y: the stand palette's pairs are the `stand_contrast` record (`TEXT_ROWS`, `STATE_ROWS`, SC 1.4.3); a stand markup change for debt 9 moves through it (per design-system §Color Palette (Core Colors)).
- design ↔ tests: the no-ESC, one-line answer reading is asserted by the seven_guis checks' shared reader and the MCP `Speaker`; reworded `covered` help passes through them (per design-system §Surface: cli (Component Patterns)).
- design ↔ tests: the hit walk's measured own-box answers are pinned in `tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs`; debts 5 and 8 rewrite or extend those pins in both layout modes (per design-system §Depth Strategy (Engine depth behavior)).

## Acceptance criteria contributions
- (design) If any stand markup or CSS changes, every `stand_contrast` row still reads its declared colour and the white-on-accent pair still reads ≥ 4.5 at rest, hover and active (per design-system §Color Palette (Core Colors)).
- (design) After any hit or position fix, the existing hit-walk readings — the `overflow` padding-box stop, the `img` / `iframe` / text `input` own-box answers and the transformed-and-scrolled box hit where its rows show — still hold in both layout modes, and any new `contain: paint` stop is stated in §Depth Strategy's terms (per design-system §Depth Strategy (Engine depth behavior)).
- (design) The driver's `scroll` stays instant in every box and the viewport after any `absolute_position` change, its into-view check reading unchanged (per design-system §Motion (This project's values)).
- (design) Any reworded `covered` meaning or remedy is served by `help` and on every refusal as one uncoloured JSON line with no ESC byte, on the command line and over MCP (per design-system §Surface: cli (Component Patterns)).
