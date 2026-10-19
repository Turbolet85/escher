# design extract

## Relevance
partial — the chunk moves test fixtures and a network stand-in and adds no surface, token or style; the design plan binds it only through what the moved fixture pages render (bundled font, stand colours, hit and paint behaviour the checks read) and must leave unchanged.

## Constraints
- A fixture booted with the stand's options takes its text from the one bundled DejaVu Sans registered for every generic with system fonts off; design-system §Typography (Loading) requires the stand's headless boot to register it through `HarnessOptions.font_ctx`, so a shared fixture boot must pass the stand's own font context, never a fresh or system one (per design-system §Typography).
- design-system §Typography (Loading) requires a driver `screenshot` of an app whose font context resolves no `sans-serif` family to be refused `fonts-unavailable` with nothing painted; a shared fixture boot used by `stand_act_screenshot` must keep the bundled font so a fixture shot still paints, and any fixture built to exercise the refusal must stay distinct from the shared one (per design-system §Typography).
- design-system §Color Palette (Core Colors, seven_guis accent row) pins the stand's colours (accent `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, page `#f5f5f5`, text `#1a1a1a`) as measured by `stand_contrast`; fixture pages are not stand tasks, so the chunk must move no stand CSS and no `stand_contrast` row (per design-system §Color Palette).
- design-system §Depth Strategy (Engine depth behavior) states the hit walk the obstructed, transformed and scrolled fixtures rest on (stacking order, the stop at a clipping box's padding box, a transformed-and-scrolled box hit where its rows show); a fixture page consolidated into the shared module must keep the markup and CSS each check's hit reading rests on — whether the per-file copies differ in such markup is research's question (per design-system §Depth Strategy).
- design-system §Motion (Animation runtime) requires settle to read animation time only through the harness's `VirtualClock`, never advancing it, and names `stand_settle`'s CSS-animation member; moving `stand_settle`'s held network stand-in into the shared module must leave that check's animation fixture and its reading unchanged (per design-system §Motion).

## Patterns to follow
- Boot every fixture through `seven_guis::stand::options(incremental)` — the stand's pinned viewport and `build_single_font_ctx(DEJAVU_SANS)` — as design-system §Typography (Loading) records the stand doing, so fixture text measures and paints as the stand's does.
- Keep the password fixture's value painting masked through blitz-paint, as the plan's CLAUDE.md-cited behaviour and design-system §Typography (Loading)'s screenshot note assume; a single shared password page must preserve whatever markup each of the seven readers' masked readings rest on (whether the seven copies are one page is research's question).

## Anti-patterns to avoid
- No colour, font or spacing value introduced into a shared fixture that a stand check then reads as a stand value — design-system §Color Palette ends `NOT YET MEASURED` for a project-wide token set, so there is no token to cite, and a fixture must not become a de facto one.
- No fixture boot that turns system fonts on or leaves the font context unset: design-system §Typography (Loading) records that without fonts text measures 0x0 and font-dependent assertions pass vacuously.

## Contract bindings
- design ↔ a11y: the stand colour pairs bind to a11y-plan §Contrast (SC 1.4.3) through `stand_contrast`; the chunk must leave that check's tables untouched (per design-system §Color Palette).
- design ↔ tests: the bundled-font boot (design-system §Typography) is the precondition of every screenshot and text-bound fixture reading the shared module serves — test-plan §7 (Test Data & Fixtures) is where the shared boot is recorded.

## Acceptance criteria contributions
- (design) Every shared fixture boot registers the stand's bundled DejaVu Sans through the stand's options, and a fixture `screenshot` check still reads an accepted shot, not `fonts-unavailable` (per design-system §Typography).
- (design) `stand_contrast`'s `TEXT_ROWS` and `STATE_ROWS` and every stand task's CSS are byte-unchanged by the chunk (per design-system §Color Palette).
- (design) Each moved hit-dependent fixture check (obstructed, transformed, scroll) asserts the same acceptance or refusal in both layout modes before and after the move (per design-system §Depth Strategy).
