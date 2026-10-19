# design extract

## Relevance
partial — an upstream sync with no new UI surface, but `38ca4e777` (`:focus-within`) changes selector matching and focus redraw in every document, so the stand's measured colours, the user-agent stylesheet, the default cursor and the stand's painted pixels are the design readings it could move.

## Constraints
- design-system §Color Palette (Core Colors, seven_guis accent row) requires the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) under white text, page `#f5f5f5` and text `#1a1a1a`, with every measured stand colour equal to its declared value. A take that restyles a focused element's ancestors must leave these unmoved. Whether any stand rule or UA rule keyed on `:focus-within` reaches a stand control's colour is research's question; the scope reads none in the stand's CSS.
- design-system §Color Palette (Border Progression) records the engine's default focus colour as the blitz-dom default input outline `#4D90FE` (`packages/blitz-dom/assets/default.css`). The take must not move it unrecorded. Whether upstream's delta touches `default.css`, for example with a `:focus-within` rule, is research's question.
- design-system §Surface: desktop-native (Tokens) says every document gets the blitz `DEFAULT_CSS` user-agent stylesheet once, through `DocumentConfig::default()`. Any UA-sheet rule the take carries therefore applies to every document, the stand included, and is named as upstream's behaviour.
- design-system §Typography (Loading) requires the headless stand to paint with bundled DejaVu Sans for every generic and system fonts off, and requires a `screenshot` to be refused `fonts-unavailable` when no `sans-serif` family resolves. Neither may change with the take.
- design-system §Iconography (Cursors and favicons) records the cursor rule as: CSS keyword, else Text, Pointer or Default, with Default over no box (upstream #1220, already on the branch by cherry-pick `275950dc0`). The take must not duplicate or regress it.
- design-system §Depth Strategy (Engine depth behavior) records the hit-test walk: stop at the padding box of a box paint clips (`overflow` or `contain: paint`), with the root excepted. `38ca4e777` adds methods in `src/node/node.rs` beside `hit_inner`, so this reading must stay as recorded (it is CARRY 3's F4 row).
- design-system §Motion (Animation runtime) requires a harness settle to neither wait on nor advance any member of the document's animating set. Upstream's new redraw request on a focus move must not hold a settle open. Whether that request joins the animating set or only asks the shell for a redraw is research's question.

## Patterns to follow
- Re-read every measured colour against its declared value, as design-system §Color Palette (Core Colors) records `stand_contrast` doing: row for row, with hover and active states included.
- Record a moved painted reading as re-measured, never loosened. design-system §Color Palette (Core Colors) cites the chunk report where each measurement was taken, and a moved value takes the same form.
- Treat an upstream engine behaviour taken by a sync as upstream's, cited by its PR number, as design-system §Iconography (Cursors and favicons) records #1220 and design-system §Surface: desktop-native (Tokens) records #1189.

## Anti-patterns to avoid
- Adjusting a stand colour, or a `stand_contrast` row, to absorb a reading the take moved. Per design-system §Color Palette (Core Colors), every measured stand colour equals its declared value, and a change moves its record.
- Adopting `:focus-within` in the stand's CSS or in escher's own styling within this chunk. That is out of scope, and per design-system §Color Palette (Core Colors) any new stand colour state would become a `stand_contrast` row.

## Contract bindings
- design ↔ a11y: the stand's colour pairs (design-system §Color Palette, Core Colors) are the SC 1.4.3 rows `stand_contrast` asserts, and the focus outline (design-system §Color Palette, Border Progression) is focus-visibility surface beside `stand_keyboard`'s Tab tables (SC 2.4.3).
- design ↔ tests: the stand's painted pixels rest on the bundled font (design-system §Typography, Loading). `stand_screenshot_repeat`'s per-host byte identity is the reading that would show a moved paint or layout number, Taffy's included if the pin fork takes it.

## Acceptance criteria contributions
- `stand_contrast` passes unchanged after the take, in both layout modes. The accent pair reads 5.47, hover 6.83 and active 8.27, and every measured stand colour equals its declared value (per design-system §Color Palette (Core Colors)).
- The take's diff of `packages/blitz-dom/assets/default.css` is recorded. Any added or changed UA rule, a `:focus-within` one included, is named as upstream's and the stand rows are re-read. Otherwise the file is recorded as untouched (per design-system §Surface: desktop-native (Tokens)).
- The pointer over no box still reads the Default cursor after the take, with no second copy of #1220's change (per design-system §Iconography (Cursors and favicons)).
- A stand whole-screen `screenshot` stays byte-identical across processes on one host. Any moved pixel is explained by the take and re-measured, never loosened. A font-less app is still refused `fonts-unavailable` (per design-system §Typography (Loading)).
