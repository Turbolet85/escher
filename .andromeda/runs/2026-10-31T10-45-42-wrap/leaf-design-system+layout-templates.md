group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:27   class: stale
  leaf-text: «at boot the counter, the timer and CRUD read 0 findings in both layout modes, through the verb as through the reader (no overlap, clipping, truncated text, off-screen element, zero-size element holding content or contrast failure), the flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return` — the snapshot giving each option its select's client rect, the driver lint's reading of a select's options and not a layout defect, owned by a `.andromeda/residuals.md` line — and the load page reads 0»
  source: .andromeda/layout-templates.md:61 «at boot the counter, the flight booker, the timer and CRUD read 0 findings (`stand_lint`'s `at_boot` `[0, 0, 0, 0]`) in both layout modes»
  replace-with: «at boot the counter, the flight booker, the timer and CRUD read 0 findings in both layout modes, through the verb as through the reader (no overlap, clipping, truncated text, off-screen element, zero-size element holding content or contrast failure) — `overlaps` skipping on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's), the snapshot giving each such option its select's client rect and a `multiple` select's rows linted as usual — and the load page reads 0»
- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «since 2026-10-19-visual-lint the stand's layout reads clean»
  source: .andromeda/layout-templates.md:16 «seven_guis Cells grid — a keyboard region and one Tab stop: `cells-grid` takes an author `id`, `tabindex="0"`, `aria-label` "Cells" and an `onkeydown`, with no role»
  replace-with: «since 2026-10-31-engine-debts the Cells grid is a keyboard region and one Tab stop (`cells-grid` takes an author `id`, `tabindex="0"`, `aria-label` "Cells" and an `onkeydown`, with no role, the 677th reacting element `stand_non_lean_tasks` reads; the arrows move `current`, clamped, and bring the cell into view; Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early, so typed text stays; the Tab sequence is the one `CELLS_TAB_ORDER` states; the synchronous handles measured headlessly, the windowed path not run); since 2026-10-19-visual-lint the stand's layout reads clean»
- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «answered by the in-process stand-in `LoadNet` after 200 ms)»
  source: .andromeda/layout-templates.md:14 «the load page's Tab sequence is the one `LOAD_TAB_ORDER` states, walked in both layout modes, its button `load-fetch` taking Enter and Space»
  replace-with: «answered by the in-process stand-in `LoadNet` after 200 ms; its Tab sequence the one `LOAD_TAB_ORDER` states, walked in both layout modes, `load-fetch` taking Enter and Space)»
- leaf: .claude/docs/design-summary.md:13   class: omission
  leaf-text: «accent, on page `#f5f5f5`, text `#1a1a1a`;»
  source: .andromeda/layout-templates.md:16 «`.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5; outline-offset: -2px`, the stand accent»
  replace-with: «accent (the Cells grid's current-cell focus outline included: `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5; outline-offset: -2px`), on page `#f5f5f5`, text `#1a1a1a`;»
- leaf: .claude/docs/design-summary.md:13   class: omission
  leaf-text: «8.27 on the active shade»
  source: .andromeda/layout-templates.md:17 «`stand_contrast` reads the load page and Cells too, neither a lean task, each in both layout modes»
  replace-with: «8.27 on the active shade; `stand_contrast` reads the load page and Cells too, neither a lean task, each in both layout modes»
- leaf: .claude/docs/design-summary.md:16   class: omission
  leaf-text: «button `#EFEFEF`»
  source: .andromeda/design-system-detail/color-palette.md:29 «| blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row: Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1»
  replace-with: «button `#EFEFEF`, a `multiple` select's chosen row `#c8c8c8` background with `#000000` text (12.56:1)»
- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «a programmatic into-view scroll never animates a nested scrolling box»
  source: .andromeda/design-system-detail/motion.md:11 «The stand's Cells grid brings its current cell into view with no animation: its arrow keys' handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously»
  replace-with: «the stand's Cells grid brings its current cell into view with no animation, its arrow keys' handler calling `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously; a programmatic into-view scroll never animates a nested scrolling box»
- leaf: .claude/docs/design-summary.md:36   class: omission
  leaf-text: «(an a11y gap noted in a11y-plan §4).»
  source: .andromeda/design-system-detail/surface.md:13 «No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read»
  replace-with: «(an a11y gap noted in a11y-plan §4). No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read; a `multiple` select lays out as `display: block`, `white-space: nowrap` rows padded 0 2px and a single-choice select's group is transparent to layout (`display: contents`) — engine defaults, no design token.»
