# a11y-plan · Visual Design Verification
Read whole. The master's broad view is .andromeda/a11y-plan.md; this file holds its Visual Design Verification detail.

## 6. Visual Design Verification

<!-- detail: values · visual-contrast -->

- A WCAG contrast ratio helper matching Chrome's GetContrastRatio is used for border bevels and scrollbar thumbs (packages/blitz-paint/src/color.rs:27-35; packages/blitz-paint/src/render/border.rs:72-78; packages/blitz-paint/src/color.rs:37-66)
- Default scrollbar thumbs get a thin contrast stroke so they read over same-coloured content (packages/blitz-paint/src/render.rs:757-759; packages/blitz-paint/src/render.rs:834-846); a test asserts default scrollbar thumbs carry a contrast stroke (tests/blitz-tests/tests/scrollbars.rs:194-202)
- A near-white author thumb must still visibly change on hover and drag (tests/blitz-tests/tests/scrollbar_drag.rs:224-292)
- Since 2026-10-13-stand-contrast-harness the stand's controls carry a measured SC 1.4.3 reading. blitz-test-harness's `Harness::text_contrast` (read-only: no mutation, input, pump or settle) takes the text-bearing element — a text input itself, else the parent element of the first non-whitespace text node under the node —, its computed `color`, `font-size` and `font-weight`, and the background behind it: each ancestor's `background-color` composited source-over down to the first opaque layer, else the root-else-`body` canvas over opaque white; it reads `None` where an element on that walk has `opacity` below 1, a non-empty `filter` or a `background-image` layer. `contrast_ratio` is the WCAG 2.x ratio over the `color` crate's relative luminance (Chrome's `GetContrastRatio`, blitz-paint's helper restated, blitz-paint untouched), and `TextContrast::required_ratio` asks 3.0 of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text. The stand check `stand_contrast` records one row per `controls(task)` row over the four lean tasks at boot (800 × 600 · scale 1 · Light · DejaVu Sans · offline), identical in both layout modes — 18 rows.
 Since 2026-10-14-stand-a11y-assertions the stand's accent is one step darker, stand-wide, in every task CSS that declares it — rest `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, white on each 5.47 · 6.83 · 8.27; the operator's F5, a stand change and no engine change (design-system §Color Palette) — and the record reads pass 14 (`back-btn` ×4, `#333333` on `#f0f0f0`, 11.09; `flight-return` 9.33; `flight-start` 17.40; `crud-filter`, `crud-name` and `crud-surname`, `#000000` on `#ffffff`, 21.00; and the five accent controls `counter-increment`, `flight-one-way` — the active toggle at boot —, `flight-book`, `timer-reset` and `crud-create`, each `#ffffff` on `#3a5ce5` at 5.47), fail 0, exempt-inactive 3 (`flight-return-date`, `crud-update` and `crud-delete`, exactly the rows whose snapshot `enabled` is `Some(false)`), no-text 1 (`timer-duration`, the range input); until then the five accent controls read `#ffffff` on `#4a6cf7` at 4.39 against 4.5, failing. No text is large, so every threshold is 4.5; no measured colour differs from its declared value, and the painted pixel matches the measured background within 1 per channel on all 17 measured rows. Beside the record one shared reader classifies two more tables, its exempt class still exactly `enabled == Some(false)` from the snapshot and a `label[for=…]` row never exempt: `TEXT_ROWS`, 14 rows — the CRUD list rows and the text outside the controls — all pass, the lowest 9.74 (`label[for=timer-duration]`); and `STATE_ROWS`, 19 colour states reached by input — return-chosen 3, start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — pass 18 and exempt 1 (`flight-book` disabled, `#ffffff` on `#a0aec0`, 2.26), the lowest pass 5.11 (`flight-start` invalid, `#c53030` on `#fff5f5`).
 `text_contrast` reads the declared `:hover` background after `move_mouse_to` and the `:active` one after `mouse_down_at`, the same in both layout modes; a button with no `:active` rule keeps its hover background while pressed, so the active rows of CRUD's buttons and `back-btn` read the hover pair. Three tests assert the three tables in both layout modes, so SC 1.4.3 is met on every measured non-exempt stand row; the check runs in the workspace `test` leg, per package and in the `a11y` leg (§9) (packages/blitz-test-harness/src/contrast.rs:13-23; packages/blitz-test-harness/src/contrast.rs:25-41; packages/blitz-test-harness/src/contrast.rs:56-91; packages/blitz-test-harness/src/contrast.rs:121-147; tests/blitz-tests/tests/stand_contrast.rs:49-137; tests/blitz-tests/tests/stand_contrast.rs:481-512; tests/blitz-tests/tests/stand_contrast.rs:514-564; tests/blitz-tests/tests/stand_contrast.rs:168-187; tests/blitz-tests/tests/stand_contrast.rs:206-279; tests/blitz-tests/tests/stand_contrast.rs:299-417; tests/blitz-tests/tests/stand_contrast.rs:577-605; tests/blitz-tests/tests/stand_contrast.rs:607-635; tests/blitz-tests/tests/stand_contrast.rs:637-661 — as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md and its evidence/pairs.md, the re-colour, the two tables and the three tests at escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md and its evidence/hover-active.md)
- Since 2026-10-19-visual-lint `Harness::text_contrast` has its first non-test caller: the `contrast` reading of escher-driver's visual lint (`Session::lint` in process and the `lint` verb on every carrier, recorded; packages/escher-driver/src/execute.rs:293-295) — a node with direct non-whitespace text or a text control, read through `text_contrast` and reported below `required_ratio()`, exempt exactly where the snapshot's `enabled` reads `Some(false)`, a `None` reading not measured — the exempt rule `stand_contrast` asserts, with no second formula or threshold; a `contrast` finding is written `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only (packages/escher-driver/src/json.rs:280-308). The stand at boot reads no `contrast` finding on the four lean tasks in both layout modes, the nodes outside `stand_contrast`'s rows included; a planted low-contrast row reads 2.8490272 against 4.5, equal to `text_contrast`'s own reading; the `a11y` leg reads 43 tests over 6 targets, unmoved (tests/blitz-tests/tests/stand_lint.rs:137-207; tests/blitz-tests/tests/stand_lint.rs:308-330 — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md and its evidence/readings.md)
- The user-agent `select` lays out as an `inline-block` box holding its chosen option, with the `input, textarea` border (`1px solid #999`), padding (`2px`) and background (`white`) (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159); its text/background pair is not measured — the stand holds no select, so `stand_contrast`'s tables are unmoved

<!-- /detail -->

<!-- detail: absence · visual-contrast -->

- Observed absent — contrast checks · searched: `contrast` over the 15 s05 files
- Observed absent — contrast or user-preference media handling · searched: `contrast|prefers-` over the 21 listed s11 files

<!-- /detail -->

<!-- detail: values · visual-focus-ring -->

- Browser urlbar focus shows a `#5E9ED6` border and 1px outline (apps/browser/assets/browser.css:172-175)
- New-tab search input removes the outline and changes only border color on focus (apps/browser/assets/about-newtab.css:30-34)
- todomvc sets `:focus` outline to 0 (examples/todomvc/src/todomvc.css:36-38)
- seven_guis inputs replace the outline with a border color or box-shadow on focus (examples/seven_guis/src/tasks/temp_converter.rs:77-81; examples/seven_guis/src/tasks/flight_booker.rs:186-189)
- counter and transparent buttons show a 4px focus outline (examples/counter/src/app.rs:79-81; examples/transparent/src/app.rs:180-182)
- rdme stylesheet shows a 2px outline in its focus-outline color custom property on focus-visible (apps/readme/assets/github-markdown.css:356-363)
- blitz-dom default stylesheet gives inputs a 2px `#4D90FE` focus outline and suppresses outlines on iframe/body/html focus-visible (packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:843-849)
- The google fixture switches focus outlines to currentcolor under @media (forced-colors:active) (examples/assets/google.html:475-479)
- Focused text inputs paint a caret honouring caret-color and a selection highlight (packages/blitz-paint/src/render.rs:930-950); since 2026-10-12-headless-screenshot a focused password input paints its caret at the bullet of its caret index in the input's caret colour (`ElementCx::caret_color`) and its bullets in the input's text colour, the field scrolling only to keep that caret in its content box, measured on the bullets (packages/blitz-paint/src/render.rs:966-973; packages/blitz-paint/src/render.rs:975-1049; tests/blitz-tests/tests/password_painted_masked.rs:117-131 — as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)

<!-- /detail -->

<!-- detail: values · visual-motion -->

- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)

<!-- /detail -->

<!-- detail: absence · visual-motion -->

- Observed absent — prefers-reduced-motion and prefers-color-scheme rules · searched: `prefers-reduced-motion|prefers-color-scheme` over the 21 s02 files

<!-- /detail -->

<!-- detail: values · visual-state-color -->

- Disabled icon buttons render at 0.35 opacity (apps/browser/assets/browser.css:191-198)
- Each WPT runner result line prints its status word as text alongside its color (wpt/runner/src/main.rs:372-401)
- A comment in the form example notes the input color "Should be accent-color" (examples/form.rs:126-129)

<!-- /detail -->

<!-- detail: values · visual-preferences -->

- The viewport carries a light/dark color scheme and a document zoom level (`1.0` unzoomed) (packages/blitz-traits/src/shell.rs:68-82; packages/blitz-traits/src/shell.rs:134-150)
- Color-scheme changes on the viewport are tracked as a device change that rebuilds the stylist device (packages/blitz-dom/src/stylo_device.rs:37-38; packages/blitz-dom/src/stylo_device.rs:55-57)
- Total scale changes from hidpi or zoom are tracked and invalidate text shaping (packages/blitz-dom/src/stylo_device.rs:33-36; packages/blitz-dom/src/stylo_device.rs:52-54)
- Page zoom is adjustable from the keyboard in 0.1 steps and resettable to 1.0 (packages/blitz-shell/src/window.rs:654-662)
- rdme follows `prefers-color-scheme` for dark/light tokens and backgrounds (apps/readme/assets/github-markdown.css:13-124; apps/readme/assets/blitz-markdown-overrides.css:11-35)
- The TodoMVC page declares `color-scheme: light`; the reference page declares `color-scheme: light dark` (examples/preact/index.html:8; examples/preact/core_dom_apis.html:8)
- blitz-dom default stylesheet un-inverts images and video under `inverted-colors` (packages/blitz-dom/assets/default.css:1087-1098)
- Touch panning honours the `touch-action` property per axis (packages/blitz-dom/src/events/pointer.rs:158-204; packages/blitz-dom/src/events/pointer.rs:247-260)

<!-- /detail -->

<!-- detail: values · visual-visibility -->

- Nodes with `display: none` or `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree, and so from the snapshot — a select's option excepted from `display: none` (§4 Hidden content), so a select's options are its children in the tree and the snapshot; a hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit (the `inert` read ratified by the founder (2026-10-18)) (packages/blitz-dom/src/accessibility.rs:19-28; packages/blitz-dom/src/accessibility.rs:254-274; packages/blitz-dom/src/node/node.rs:1437-1446; tests/blitz-tests/tests/inert_subtree.rs:95-194); a hit also skips a node dropped since the last resolve (architecture §Standard Contracts → Scrolling, selection, tree)
- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1448-1456)
- `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1458-1462; packages/blitz-dom/src/node/node.rs:1662-1670)
- `scrollbar-width: none` suppresses overlay scrollbars (packages/blitz-dom/src/node/scrollbar.rs:76-87); a test asserts it paints no scrollbar (tests/blitz-tests/tests/scrollbars.rs:162-175)
- Devtools layout outlines and hover/node highlight overlays exist as settings (packages/blitz-traits/src/devtools.rs:5-34)

<!-- /detail -->
