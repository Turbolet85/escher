# Task — design · 2026-10-26-codebase-health

## The chunk
A pure refactor: three escher-written functions over the cognitive-complexity ceiling of 15 are factored into table walks or named helpers so each reads at or under 15, with no change to any behaviour, output or assertion.
The three are the visual lint's per-node walk `lint.rs::findings` (16), the MCP listing test `the_listing_is_the_twelve_verbs_of_the_table` (18) and the 0.2.0 witness sweep test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (18).
It re-measures the code audit's complexity reading (rust-code-analysis 0.0.25, cognitive > 15) before and after, and re-runs `stand_lint`, the lint's unit tests, the two touched tests, the blitz-tests target pin and the fast and doc CI legs, all unchanged and green.
The engine function `clear_interaction_state_for_removed_node`, every other over-ceiling function and every verb, field, carrier, log site and span stay as they are.

## Reached files
- packages/escher-driver/src/lint.rs · edits `findings` (the visual lint's per-node walk)
- packages/escher-driver/src/mcp_tests.rs · edits `the_listing_is_the_twelve_verbs_of_the_table`
- tests/blitz-tests/tests/stand_requirement_sweep.rs · edits `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
- tests/blitz-tests/tests/stand_lint.rs · re-reads stand_lint (passes unchanged)
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target pin (stand_requirement_sweep's name and target unchanged)
- .github/scripts/ci-leg.sh · re-reads the fast and doc legs
- .andromeda/runs/2026-10-25T21-43-01-code-audit/over_ceiling.py · re-reads the audit's complexity reading (`complexity.over_ceiling`)

## Needs
- The visual lint's per-node walk (`lint.rs::findings`): its branch ladder moved into named helpers or a table walk, each helper itself at or under cognitive 15.
- The lint's output kept identical: the same `Finding`s (stable id, closed defect word, bounds, reason numbers) in document order, then `Defect::ALL` order.
- The lint's six defects kept as they are: `overlap`, `clipped`, `truncated`, `off-screen`, `zero-size`, `contrast`, including a node both clipped or off-screen and holding text also reading `truncated`.
- The lint's `contrast` defect, read through `Harness::text_contrast` under `stand_contrast`'s exempt rule: the required ratio from the text's font size and weight (3.0 for large text, else 4.5) carried through the factoring unchanged.
- The lint's `fonts-unavailable` refusal when the font context resolves no `sans-serif` family, still raised before anything is read.
- The lint changing nothing on the held instance: no step run, nothing dispatched, the harness's `VirtualClock` not moved.
- The stand screens as `stand_lint` reads them at boot, re-read only: counter, timer, CRUD and the load page at 0 findings, the flight booker's one `overlap` on its select's options, and the last CRUD row `clipped` inside `crud-list` after twelve Creates; the stand's colours, text sizes and layout are not edited.
- The lint's answer as it crosses the agent-facing command line and the `mcp` role (`{"findings":[…]}`, one JSON line, no colour or ESC byte): unchanged, no carrier or field touched.
- The MCP `tools/list` answer as the listing test asserts it: twelve tools in `VERBS` order, `lint` after `screenshot`, `help` last, each described by its help and its result fields; the same rows asserted after the factoring.
- The 0.2.0 requirement witness sweep: each 0.2.0 requirement names its witness inside stand-repeat; the same rows asserted, the test's name and blitz-tests target unchanged.
- Each touched test still counted in the complexity population and still turning red when a row it guards is dropped from its table, `VERBS` or the witness list.
- The audit's complexity reading re-measured: the three functions at or under 15 and `complexity.over_ceiling` at most 85 at HEAD with no new entrant.

## Standing conventions
- (none)

## Not needed
- Colour tokens of the reference browser and rdme apps (`apps/browser`, `apps/readme`): no app stylesheet is reached.
- Border radius values and radius handling: no CSS or paint value changes.
- Depth strategy's shadow and elevation values: no CSS or paint value changes.
- Motion's into-view scroll and smooth-scroll behaviour: the lint runs no scroll and no animation code is touched.
- Iconography, cursors and favicons: no icon, cursor or favicon is reached.
- Typography's type scale, font families and engine text mapping as values: no font or text style changes (the lint's `fonts-unavailable` refusal is a Need).
- Spacing values in the apps, examples and engine: no layout or spacing value changes.
- The desktop-native titlebar, custom widget, devtools overlay, window and stack patterns: no windowed path is touched.
- The mobile-native surface (Android and iOS tokens, url bar, back button, capture, stack): not built or touched.
- The web-spa surface (WASM canvas and its tokens): not built or touched.
- The CLI tokens of the WPT runner (`owo-colors` output): the WPT runner is not reached.
