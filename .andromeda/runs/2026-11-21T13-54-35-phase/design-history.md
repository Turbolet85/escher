@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay) — was «no popup, `size` not read»; now a single select's list is painted, opened by click, Enter or Space, and `size` is read on a `multiple` select and stays unread on a single select; on desktop-native the list is an engine overlay through `paint_scene`, not rows as boxes. This is the clause this chunk reverses (founder's F8 in part): it must re-word surface's select line and decision/select-list-engine-overlay and decision/single-select-open-gestures so they hold only for an unsized (or `size` ≤ 1) single select.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a `multiple` select with `size`) — added the §Spacing item: `size_rows` hints a `height` of `size × 1.2em`, capped at 65534, `0` or a bad value giving no hint, `select[multiple][size]` taking `overflow-y: auto`, and the §Typography row `blitz-dom default select[multiple][size] rows` line-height 1.2, on F8. This is the mechanism and the register the sized single select extends: the selectors name `multiple` and widen to a sized single select; HTML's 4-row default for `multiple` stays unadopted.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (no select popup is painted, and who owns it) — first recorded «`size` is not read» on a select as a stated limit, with the Select popup route entry as owner. It is the origin of the limit residual `res-ysz67imk` now closes, superseded in part by the select-popup entry above.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and a select's groups) — added `select[multiple] > option` / `> optgroup > option` `display: block`, `white-space: nowrap`, padding 0 2px, `select[multiple] > optgroup` `display: block`, and for a single select `select:not([multiple]) > optgroup:not([hidden])` `display: contents` with its chosen option `display: inline`. A sized single select must lay out as rows like the `multiple` list box, so these `:not([multiple])` / `[multiple]` selectors are the ones the chunk re-keys (F6/F7, the founder's).

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair) — added the Core Colors row `blitz-dom default chosen row` `#c8c8c8` / `#000000` (Chromium's inactive selected pair, hardcoded, 12.56:1 measured by `select_interaction`), coloured by the `select[multiple] > option:checked` sheet rule. The sized single select's chosen row is marked the same way, so the rule's selector and decision/select-chosen-row-chromium-pair widen to it; no token may be added.

@history_item n=6 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the user-agent `select` box (`1px solid #999`, padding 2px, white, `inline-block`), and a single-choice select showing its chosen option alone (`select:not([multiple]) > option:checked { display: inline }` beside `option { display: none }`), engine defaults, no token (F2, founder-ratified). This row (decision/single-select-box-shows-chosen-label) becomes false for a sized single select and needs its exception.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how a single select's list opens, moves, chooses and closes) — the open/choose/close/highlight event rules and «a disabled, inert or hidden select, one with no row to show, and a `multiple` select open no list». The contract/select-list-events/* rows and decision/selects-that-open-no-list come from here; a sized single select joins the selects that open no list, and its key/click choosing (one `input` then `change`, chosen option firing nothing) must follow the `multiple` model or be a fork.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — the tree reads an open select `ComboBox` and `expanded`, its chosen option `selected`; an author's `open` steers nothing; a row of an open list carries its own `bounds` and a shot of a row crops to it. A sized single select is a list box, not a combo box: its role and the absence of `expanded` must be decided against these rows (scope: any reading that differs from a `multiple` list box's is a fork).

@history_item n=9 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — re-worded contract/select-accessibility/highlighted-row (the snapshot's `highlighted` bool on a single select's open list alone), contract/select-accessibility/open-select (active descendant while the list stands open, `aria-activedescendant` not read) and contract/select-list-events/highlight-move (a move to another row marks its select changed). These are the newest select rows; a sized single select carries no open list, so `highlighted` and the active descendant must not appear on it.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion) — a wheel over the open list scrolls it, elsewhere closes it; a key brings the current row into view at once; the list has no motion (F5). Rows decision/select-list-wheel-and-key-scroll and decision/select-list-opens-without-motion; the list box's key moves must likewise bring the chosen row into view instantly, but through the ordinary scrolling box rather than the list's `scroll_y`.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (into view on a row of an open list) — the into-view rule's exception: on a row of an open select list `scroll_into_view` writes that list's `scroll_y` at once, moving no other box or viewport. A list box's rows are real boxes, so they fall under the general rule (every holding box instant, Smooth on the viewport alone), not this exception.

@history_item n=12 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an into-view scroll never animates a nested box: every scrolling box holding the target is written at once, innermost first, Smooth applying to the viewport alone (founder-ratified). Governs bringing a list box's chosen row into view on a key.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — a row of an open list is answered by `select_list_hit` before the page's walk, so a box clipping the select clips no row (the except of decision/hit-testing-stops-at-paint-clip). A list box's rows are hit by the page's walk and are clipped by the list box's own scrollport, the padding-box clip rule.

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the hit stops at a `contain: paint` box) — widened the hit stop to a computed `contain` including `paint`, ratified at the Epoch 2 close (2026-10-18-engine-debts, the PROVISIONAL batch ratified); with 2026-10-10-scrolling-box-bounds-and-hit (the overflow padding-box stop) it makes decision/hit-test-stops-at-paint-clip. A `size`d list box with `overflow-y: auto` clips its rows, so a click on a scrolled-out row reaches nothing.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`visible_region` on a row of an open list) — `visible_region` answers the list's scrollport for a row of an open list, and the driver's lint takes an open row's region from it alone. A list box's rows need the ordinary clipping-box reading instead (see 2026-10-31-engine-debts `visible_region` through transforms), so the lint reads a scrolled-out row as hidden.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants) — `SelectListView` constants `HIGHLIGHT_BACKGROUND`/`HIGHLIGHT_TEXT` `#c8c8c8`/`#000000`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`, not in `default.css`, no token (F6); colours in `default.css` for the list stand rejected. The rows decision/engine-colours-hardcoded and decision/select-list-colours-engine-constants; a list box's chosen and disabled rows are styled by the sheet, as a `multiple` select's, and add no token.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's plain pair is measured, its value unstated; a group in the open list, a header heavier than a row) — the plain row's pair is measured by `select_popup_painted`, and a group's `label` paints as a bold header through `RunPaint { synthesize_bold }`. A sized single select shows its groups as headers in-flow, like a `multiple` select's `optgroup`, not by the popup's painter.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's row cap, placement and row height; the open list's thumb; paints last) — `MAX_ROWS` 20, `place`, the row-height rule, a non-draggable thumb under `scrollbars`, and paint-last past clips and opacity: all popup-only. A list box `size` rows tall uses none of these; it scrolls as a normal box with the overlay scrollbar.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade (decision/shown-scrollbar-takes-thumb-drag); it also put the flight booker's `flight-type` select on the stand. A key-scrolled list box's rows under the thumb band meet this trap in the driver checks.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block) — a focused select whose anonymous block was dropped keeps focus and a key still moves its choice; retired by 2026-11-21-engine-debts (− decision/boxless-focused-select-keeps-focus): such a select now gives focus up after the next layout, a11y-plan's decision/focus-fixup-gives-up-unshown-focus. The list box's key model must keep to the newer rule.

@history_item n=21 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once, through `DocumentConfig::default()` (decision/default-ua-sheet-via-default-config). The list box's look lands in that sheet, so it reaches every document, remote pages included.

@history_item n=22 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot of an app with no `sans-serif` family is refused `fonts-unavailable`, nothing painted, and the shot's answer line and MCP image block (rows contract/driver-screenshot*/fonts-unavailable, escher-session screenshot). The list box's screenshot checks rest on these unchanged.

@history_item n=23 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography) — + decision/unavailable-families-use-default-font: font-relative units measure against the default font when no family is available. The `size × 1.2em` height of the list box is font-relative and reads through it.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `click`, `type` and `press` leave the `VirtualClock` where it was (ratified 2026-10-18; it narrowed 2026-10-07-act-by-id's two-clock rule). Rows contract/driver-verbs-and-virtual-clock/press and /type: the list box's key moves add no animation that would need the clock.

@history_item n=25 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` answer line and that a node's verbs are decided by the verbs' own `validate` and `check_target` (rows decision/actions-share-the-verbs-checks, contract/actions-line/*, actions-answer). A list box's rows become actionable nodes like a `multiple` select's, so their listed causes must match what `click` returns.

@history_item n=26 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — + contract/escher-session-commands/layout-answer (and its arguments row): `layout --id` answers bounds, laid_out, placed, shifts. A sized single select's height change and its rows read through it like any box.

@history_item n=27 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot` answer line on the cli surface and the MCP snapshot result (rows contract/escher-session-commands/snapshot, escher-session-mcp/snapshot-result). A list box's rows join the snapshot under these shapes, with no new field.

@history_item n=28 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows, each verified against the code; later changes name a row by handle (`+` / `~` / `−`). The chunk's design amendment must re-word the picked select rows by handle (e.g. ~ decision/select-list-engine-overlay, ~ decision/single-select-box-shows-chosen-label, ~ decision/selects-that-open-no-list).
