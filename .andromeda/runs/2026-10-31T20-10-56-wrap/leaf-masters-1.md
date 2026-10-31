group: masters-1
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «hit testing (a hit stops at the padding box of a box paint clips»
  source: .andromeda/architecture.md:147 «`hit_with_scrollbar` answers an open select list first and no scrollbar with it, `hit_inner` holding none of it, so `hit`, `set_hover_to` and `handle_pointerdown` all read the list»
  replace-with: «hit testing (`hit_with_scrollbar` answers an open select list first and no scrollbar with it, `hit_inner` holding none of it, so `hit`, `set_hover_to` and `handle_pointerdown` all read the list; past it a hit stops at the padding box of a box paint clips»
- leaf: CLAUDE.md:22   class: stale
  leaf-text: «`scroll_into_view` moves every scrolling box that holds its target, then the viewport;»
  source: .andromeda/architecture.md:147 «then the viewport, but for a row of an open select list, which writes the list's `scroll_y` at once and moves no chain or viewport»
  replace-with: «`scroll_into_view` moves every scrolling box that holds its target, then the viewport, but for a row of an open select list, which writes the list's `scroll_y` at once and moves no chain or viewport;»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «`None` for a mapped box with a non-finite edge)»
  source: .andromeda/architecture.md:149 «a row or a group of an open select list reading the list's scrollport in place of its clipping boxes»
  replace-with: «`None` for a mapped box with a non-finite edge, a row or a group of an open select list reading the list's scrollport in place of its clipping boxes)»
- leaf: CLAUDE.md:22   class: stale
  leaf-text: «`aria-expanded`, `aria-disabled` (`true` sets disabled) and `aria-level`»
  source: .andromeda/architecture.md:141 «a single select's native `expanded`, true while its list stands open, with no `aria-expanded` read on one, every other element keeping that read»
  replace-with: «`aria-expanded` (on every element but a single select, which reads none and writes `expanded` true while its list stands open), `aria-disabled` (`true` sets disabled) and `aria-level`»
- leaf: CLAUDE.md:22   class: stale
  leaf-text: «focused and no disabled form control, `ArrowDown` · `ArrowUp` · `Home` · `End` take it»
  source: .andromeda/architecture-detail/established-decisions.md:78 «a focused single-choice `select` (no `multiple`), its list closed, that is no disabled form control takes a pressed `ArrowDown` / `ArrowUp`»
  replace-with: «focused, its list closed and no disabled form control, `ArrowDown` · `ArrowUp` · `Home` · `End` take it»
- leaf: CLAUDE.md:22   class: stale
  leaf-text: «a click focuses it; each change goes through `apply_select_choice`»
  source: .andromeda/architecture-detail/established-decisions.md:78 «and on a single select opens its list or closes one standing»
  replace-with: «a click focuses it and opens its list or closes one standing; each change goes through `apply_select_choice`»
- leaf: CLAUDE.md:22   class: stale
  leaf-text: «its one non-test caller the driver's `focus_multiple_select`»
  source: .andromeda/architecture.md:137 «public `clear_select_typeahead` clears the typed text, its one non-test caller the driver's `focus_select`»
  replace-with: «its one non-test caller the driver's `focus_select`»
- leaf: CLAUDE.md:22   class: retired
  leaf-text: «no popup painted, Enter and Space opening nothing on a single select, an option of a closed single select, under a group too, taking no click, `size` unread»
  source: .andromeda/architecture.md:64 «a click, Enter and a Space with no typed text standing open a single select's list, which the engine paints over the page with a group's `label` as a header»
  replace-with: «a click, Enter and a Space with no typed text standing opening its list the engine paints over the page (below), a released Space doing nothing more, an option of a closed single select, under a group too, taking no click, a single select's `size` unread»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «a `multiple` select's model (the chosen set held»
  source: .andromeda/architecture-detail/established-decisions.md:80 «a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing»
  replace-with: «a single select's list (in the crate-private `node/select_list.rs`, `SelectListView` — its colours engine constants, `HIGHLIGHT_TEXT` · `HIGHLIGHT_BACKGROUND` · `DISABLED_TEXT` · `BORDER` — and `SelectListRow` the two public read types re-exported from the crate root; public read-only `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`, public writer `close_select_list`; it opens on the chosen option firing nothing, showing at most `MAX_ROWS` (20) rows, where its rows fit and else on the side with more room, at least its select's width and at most the viewport's; a disabled, inert or hidden select or one with no row to show opens nothing, and a bound label's click focuses the select and opens nothing; focus stays on the select, the move keys and the pointer move the highlight over enabled rows and choose nothing; Enter, Space, Tab in either direction before focus moves and a click on a row choose through `apply_select_choice`, a disabled row changing nothing and leaving the list open, a group's header choosing nothing; Escape, a second click, a press outside, a wheel outside the list and a page scroll or move of the select's box close it with no event, and removal, `disabled`, `hidden` or `inert` on the select too; its limits: the thumb painted and not draggable, a highlighted row's contrast unread by the lint, a larger authored font clipping in the one row height, the highlight in no snapshot and no tree — a direct engine change for every document, remote pages included, ratified), a `multiple` select's model (the chosen set held»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «disabled rows never chosen, a removal of `multiple` settling the set to one option, Ctrl+A not modelled»
  source: .andromeda/architecture-detail/established-decisions.md:130 «A `multiple` select's `size` hints a `height` of `size` × 1.2em through `size_rows`, capped at 65534, `0` or a bad value no hint»
  replace-with: «disabled rows never chosen, a removal of `multiple` settling the set to one option, `size` hinting a `height` of `size` × 1.2em through `size_rows`, capped at 65534, `0` or a bad value no hint, a key bringing the current row into view only where the select carries a valid `size`, Ctrl+A not modelled»
- leaf: CLAUDE.md:22   class: stale
  leaf-text: «and a select's changed choice (`apply_select_choice`, nothing for a choice of the chosen option), not by node creation, hover or layout)»
  source: .andromeda/architecture.md:279 «the changed set is written at five kinds of site, the fifth a select list's open and close marking its select»
  replace-with: «a select's changed choice (`apply_select_choice`, nothing for a choice of the chosen option) and a select list's open and close, marking its select (a page scroll or a move that closes it through `reconcile_select_list` at the next resolve, a wheel elsewhere through `close_select_list`), not by node creation, hover or layout, nor by a highlight or a wheel move inside a list)»
- leaf: CLAUDE.md:26   class: omission
  leaf-text: «paints a resolved document into an anyrender `PaintScene`;»
  source: .andromeda/architecture.md:161 «`paint_scene` paints a document into a `PaintScene`, an open select's list last through `paint_select_list`, after the root's render and before the devtools overlays»
  replace-with: «paints a resolved document into an anyrender `PaintScene`, an open select's list last through `paint_select_list`, after the root's render and before the devtools overlays, read from `SelectListView` and past any clip or `opacity` on its select's ancestors, a group's `label` a header heavier than a row on every build by a synthesized bold (`stroke_text_as`, `RunPaint`);»
- leaf: CLAUDE.md:33   class: stale
  leaf-text: «its public `closed_select_of` and `is_multiple_select` are the select tests the driver reads, naming the element and never the role,»
  source: .andromeda/architecture.md:187 «the public `closed_select_of`, which answers `None` for a select whose list is open, and `is_select`, read by the driver's `locate` alone, are the select tests the driver reads»
  replace-with: «its public `closed_select_of`, which answers `None` for a select whose list is open, and `is_select`, read by the driver's `locate` alone, are the select tests the driver reads, naming the element and never the role, `is_multiple_select` staying re-exported, read within the workspace by `closed_select_of` alone,»
- leaf: CLAUDE.md:33   class: omission
  leaf-text: «a `multiple` select's rows and groups with client rects of their own, named by their labels,»
  source: .andromeda/architecture.md:187 «an open list's rows and groups reading `select_list_bounds` before the client rect»
  replace-with: «a `multiple` select's rows and groups with client rects of their own, named by their labels, an open list's rows and groups reading `select_list_bounds` before the client rect (the snapshot holding no `expanded` key and no highlight),»
- leaf: CLAUDE.md:37   class: stale
  leaf-text: «on a select the text chooses the first option whose label starts with it, on a `multiple` select its match alone, focused with no click, a text opening on a space typing nothing;»
  source: .andromeda/architecture-detail/standard-contracts.md:202 «`type` does not click such a target, a click landing on a `multiple` select's row at the centre and opening a single select's list: the crate-private `focus_select` focuses it with no click»
  replace-with: «on a select, single or `multiple`, focused with no click, firing no DOM focus event and closing any list, the text chooses the first option whose label starts with it, on a `multiple` select its match alone, a text opening on a space typing nothing;»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «a `click` or a `type` on a target that cannot take it refused `disabled`, `off-screen` or `covered` with nothing run,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:210 «a closed option and a button under the list refused `covered`, a row past the scrollport `off-screen` until `scroll` brings it in»
  replace-with: «a `click` or a `type` on a target that cannot take it refused `disabled`, `off-screen` or `covered` with nothing run (an option of a closed select `covered`, whose meaning says to click the select first, then the option; a button under an open list `covered`, a row past its scrollport `off-screen` until `scroll` brings it in, writing the list's `scroll_y` alone), a `click` on a single select opening its list or closing one standing and a `click` on a row choosing it,»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «a `screenshot`'s PNG as base64 inside its answer line (screen pixels in band, ratified by the founder, 2026-10-12)»
  source: .andromeda/architecture-detail/occupied-resources.md:17 «A shot of an open select list also carries that list's option labels and group `label`s, a page's text as painted pixels, on both carriers»
  replace-with: «a `screenshot`'s PNG as base64 inside its answer line (screen pixels in band, ratified by the founder, 2026-10-12; a shot of an open select list also carrying that list's option labels and group `label`s as painted pixels, a ratified boundary widening)»
- leaf: CLAUDE.md:44   class: omission
  leaf-text: «a shot's pixels (a PNG as base64, a password in it painted masked)»
  source: .andromeda/architecture-detail/occupied-resources.md:17 «A shot of an open select list also carries that list's option labels and group `label`s, a page's text as painted pixels, on both carriers»
  replace-with: «a shot's pixels (a PNG as base64, a password in it painted masked, an open select list's option labels and group `label`s in it as painted pixels)»
- leaf: CLAUDE.md:45   class: omission
  leaf-text: «the colour states it reaches, hover and active included (`TEXT_ROWS`, `STATE_ROWS`)»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:12 «hover 5, active 5, list-open 2 (`flight-type-one-way` and `flight-type-return`, each 17.40)»
  replace-with: «the colour states it reaches, hover, active and a select's open list included (`TEXT_ROWS`, `STATE_ROWS`)»
- leaf: CLAUDE.md:45   class: omission
  leaf-text: «so a stand colour change moves its record;»
  source: .andromeda/a11y-plan.md:146 «The select list's keyboard and focus coverage is headless: `select_popup`, `stand_act_popup` and `stand_keyboard`'s list walks assert SC 2.1.1 and 2.4.3 on it in both layout modes»
  replace-with: «so a stand colour change moves its record; an open select list is asserted headlessly — SC 2.1.1 and 2.4.3 by `select_popup`, `stand_act_popup` and `stand_keyboard`'s list walks (only `stand_keyboard` in the `a11y` leg), SC 1.4.3 by `select_popup_painted` against `SelectListView`'s engine constants, in no sheet — and what an assistive technology reads of it is not measured, the windowed list read and not run;»
- leaf: CLAUDE.md:52   class: omission
  leaf-text: «`cargo doc --workspace --no-deps --locked`) pass.»
  source: .andromeda/test-plan-detail/ci-integration.md:34 «`apple-font-embolden` made it red on macOS alone, and no local gate builds `blitz-paint/font-embolden`»
  replace-with: «`cargo doc --workspace --no-deps --locked`) pass; neither builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises, so a paint change under that feature also takes its local witness `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`.»
- leaf: CLAUDE.md:75   class: omission
  leaf-text: «the select's rules `packages/blitz-dom/src/node/select.rs` (unit tests inline; checks `tests/blitz-tests/tests/{select_interaction,stand_act_select}.rs`)»
  source: .andromeda/architecture-detail/existing-scopes.md:49 «select_list (the crate-private state and geometry of a single select's open list, with 6 unit tests»
  replace-with: «the select's rules `packages/blitz-dom/src/node/select.rs` (unit tests inline; checks `tests/blitz-tests/tests/{select_interaction,stand_act_select}.rs`), a single select's open list `packages/blitz-dom/src/node/select_list.rs` (unit tests inline), painted by `packages/blitz-paint/src/render/select_list.rs` (checks `tests/blitz-tests/tests/{select_popup,select_popup_painted,stand_act_popup}.rs`)»
- leaf: CLAUDE.md:98   class: stale
  leaf-text: «fifteen named blitz-tests targets»
  source: .andromeda/test-plan.md:378 «the `stand-repeat` leg runs the stand checks, seventeen named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds»
  replace-with: «seventeen named blitz-tests targets»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «on a select it chooses the first option whose label starts with the text, on a `multiple` select, focused with no click, its match alone»
  source: .andromeda/architecture-detail/standard-contracts.md:202 «`type` on a select, single or `multiple`: `Target` carries `select`, and `type` does not click such a target»
  replace-with: «on a select, single or `multiple`, focused with no click, it chooses the first option whose label starts with the text, on a `multiple` select its match alone»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «`scroll` brings its target into view through every scrolling box that holds it»
  source: .andromeda/architecture-detail/standard-contracts.md:204 «an instant scroll of every scrolling box that holds the element and then of the viewport, a row of an open select list writing that list's `scroll_y` alone»
  replace-with: «`scroll` brings its target into view through every scrolling box that holds it, a row of an open select list by writing that list's `scroll_y` alone»
- leaf: CLAUDE.md:110   class: omission
  leaf-text: «a `multiple` select's rows reading their own client rects and linted as usual)»
  source: .andromeda/architecture-detail/standard-contracts.md:233 «an open list's rows read their own bounds in the list and `overlaps` skips them too, so they read no `overlap` on the stand»
  replace-with: «a `multiple` select's rows reading their own client rects and linted as usual, an open list's rows reading their own bounds in the list and `overlaps` skipping them too, a highlighted row's contrast unread)»
- leaf: CLAUDE.md:110   class: omission
  leaf-text: «pinned by `stand_lint` in both layout modes).»
  source: .andromeda/architecture-detail/established-decisions.md:80 «a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing»
  replace-with: «pinned by `stand_lint` in both layout modes), and the select list (built — a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing, a direct engine change for every document, remote pages included, ratified: one `BaseDocument` field `select_list`, answered first in `hit_with_scrollbar` and painted by `paint_scene` after the root's render, a row's own `bounds` crossing in the snapshot and the diff and an option's and a group's label leaving as pixels in a `screenshot`, a ratified boundary widening, with no new carrier, exit, cause, session error, node key, log field or span; proven headlessly in both layout modes by `select_popup`, `select_popup_painted` and `stand_act_popup` and through the host and the `mcp` role, what an assistive technology reads of it unmeasured and the windowed list read and not run).»
