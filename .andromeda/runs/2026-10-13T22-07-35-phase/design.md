# design extract

## Relevance
partial — the chunk is test code over the existing `Harness` and snapshot readers. It touches design only where a fork answer would change stand markup (a roving `tabindex` or a selection style on the CRUD rows, inert content) or where an activation key changes a stand state that has a colour. No token, motion or iconography work is in scope.

## Constraints
- The stand's colours are recorded as declared values, not tokens: accent `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) on page `#f5f5f5`, with the five accent controls measured at 4.39 against SC 1.4.3's 4.5. Any stand markup change a fork here makes (for example CRUD row focus or selection styling) must keep these values, or move the stand contrast record with them (per design-system §Color Palette → Core Colors, seven_guis accent row).
- Stand text `#1a1a1a` and page `#f5f5f5` are the declared pair a new stand style must reuse. A new foreground/background pair on a CRUD option row would be a new contrast row (per design-system §Color Palette → Text Hierarchy, seven_guis text row; per design-system §Color Palette → Surface Scale, seven_guis / todomvc page row).
- The one engine focus style the plan records is the blitz-dom default input focus outline `#4D90FE`; the browser chrome's `#5E9ED6` is not the stand's. Whether a focused `button`, a focused CRUD `option` or content under `inert: false` paints any focus indication is research's question, because the plan records the outline for inputs only (per design-system §Color Palette → Border Progression).
- flight booker's invalid state (`#e53e3e` / `#fff5f5` / `#c53030`) and success message (`#ebf8ee` / `#68d391` / `#276749`) are the states an activation key on `flight-book` or a date field can produce. Read them back by snapshot text (`invalid`, the message's name), never by colour (per design-system §Color Palette → Semantic Colors).
- The headless stand registers bundled DejaVu Sans for every generic, with system fonts off. The keyboard harness must boot through `seven_guis::stand` so that names and bounds read the same on every OS (per design-system §Typography → Loading).
- The harness drives animation from a controlled clock that settle reads and never advances, and `click`, `type` and `press` leave it where it was. A keyboard step returns settled without waiting on an animation. The plan records no transitions or animations in seven_guis styles, so no reduce-motion override arises (per design-system §Motion → Animation runtime; per design-system §Motion → This project's values).
- A hit stops at the padding box of a node that clips by `overflow`, the root element excepted. This applies to freight item 7's hypothesis: an activation key on a scrolled box clicks off its centre. Whether a synthetic activation click lands inside that box is research's question (per design-system §Depth Strategy → Engine depth behavior).

## Patterns to follow
- The stand contrast record is asserted row for row from computed styles (`tests/blitz-tests/tests/stand_contrast.rs`). A stand style change moves that record in the same chunk (per design-system §Color Palette → Core Colors, seven_guis accent row).
- Boot the stand with one bundled font: `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx` in `examples/seven_guis/src/stand.rs` (per design-system §Typography → Loading).
- Time moves only by explicit steps: the harness's tick-advanced clock, and on the Timer whole 100 ms ticks through `advance`. The pattern is in `tests/blitz-tests/tests/stand_act_timer.rs` and `tests/blitz-tests/tests/stand_settle.rs` (per design-system §Motion → Animation runtime).
- An into-view scroll is instant in every nested scrolling box and asks for no smooth animation inside a settled step. Follow this if a focus step needs the target brought into view (per design-system §Motion → This project's values, the `scroll_into_view` row).

## Anti-patterns to avoid
- Re-colouring a stand control, or adding a new colour pair to it (a focus or selected style on CRUD rows), without the stand contrast record moving with it. The accent's 4.39 fail belongs to "Stand a11y assertions" and is out of this chunk (per design-system §Color Palette → Core Colors, seven_guis accent row).
- Asserting a keyboard-reached state (invalid, success, selected, focused) by a painted colour instead of its snapshot reading (per design-system §Color Palette → Semantic Colors).

## Contract bindings
- Stand colours ↔ a11y SC 1.4.3 contrast: `stand_contrast` binds every stand colour pair. A markup fork that adds a pair binds a11y-plan's contrast record (per design-system §Color Palette → Core Colors).
- State colour ↔ a11y SC 1.4.1 (not colour alone): flight booker's invalid and success colours are paired with `aria-invalid` and the message text (per design-system §Color Palette → Semantic Colors).
- Focus indication ↔ a11y focus visibility: the plan records only the default input outline `#4D90FE`. The chunk's target criteria are SC 2.1.1 and 2.4.3, so focus visibility is a flag for a11y, not a check this chunk owes (per design-system §Color Palette → Border Progression).
- Controlled clock ↔ test harness settle: keyboard steps run under `Harness::settle`'s never-advance rule (per design-system §Motion → Animation runtime).

## Acceptance criteria contributions
- (design) If any stand markup changes, `stand_contrast` passes with every stand colour equal to its declared value. A changed or added pair moves the record in the same commit (per design-system §Color Palette → Core Colors).
- (design) The keyboard harness boots every task through `seven_guis::stand` with the bundled DejaVu Sans and system fonts off (per design-system §Typography → Loading).
- (design) No keyboard step advances the harness's controlled clock: the Timer's elapsed reading changes only across an explicit `advance` or tick (per design-system §Motion → Animation runtime).
- (design) Every state an activation key reaches on flight booker is read back by its snapshot reading (`invalid`, the success message's name), never by a pixel colour (per design-system §Color Palette → Semantic Colors).
