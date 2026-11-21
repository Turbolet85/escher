@history_item n=1 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the select model, an engine change for every document) — built the single select's key model (arrows, Home/End, typeahead on the document's clock), the click focus, the one choice path `apply_select_choice` (CHECKED, changed set, `input` then `change`), the choice as CHECKED with the selectedness algorithm, and the user-agent select box; a direct engine change, upstreamable, merge surface carried. This is the key model a sized single select must reuse for "keys moving the choice", and its "one path" rule (decision/select-choice-one-path, decision/select-choice-single-path) binds any row click in the list box.

@history_item n=2 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (counts, the type help) — `type` on a select chooses the first option whose label starts with the text, and `stand_act_select` / `select_interaction` were born here; the driver's `type` on a sized single select must keep this meaning (contract/driver-verbs/type).

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a multiple select's chosen set, keys and pointer) — added `press_multiple_select_key` and `click_select_row` behind `handle_click`'s `option` arm, a row click focusing its select, and narrowed "the box holding its chosen option" to single-choice selects. These are the list-box pieces the scope names to build on; a sized single select is a list box whose clicks and keys must choose one row alone (single-select selectedness), not span or flip (decision/multiple-select-row-clicks, decision/multiple-select-row-keys).

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the select limits that stand) — recorded "`size` is unread" and that an option of a closed single select takes no click and stays refused `covered`, sending `size` to the popup chunk on the founder's P5 word. Origin of the limit this chunk closes; decision/closed-select-options-covered must be re-scoped to a select that is still a drop-down (size ≤ 1).

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's rows and chosen-row pair) — the UA sheet keys list-box row rules (`display: block`, padding, nowrap, `optgroup` block) and the #c8c8c8/#000000 chosen-row pair on `select[multiple]`, and the drop-down's `display: contents`/`option:checked { display: inline }` on `select:not([multiple])`. A sized single select's look must move from the drop-down selectors to the list-box ones (decision/multiple-select-chosen-row-colours), a trap since every `:not([multiple])` selector currently catches it.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (optgroup) — a select's options include its groups' (`is_select_option`, `option_disabled`, group `Role::Group` named by its label); the list box must show group headers and their options as rows (decision/select-options-include-optgroup, decision/option-disabled-own-or-group, decision/optgroup-group-role-named-by-label).

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the public select readers and the row rules) — `select_values`, `select_of`, `clear_select_typeahead`, `closed_select_of`/`is_multiple_select` re-exported for the driver, `SelectCursor`/`RowGesture`/`Gestured` crate-private in `node::select`, `set_option_selectedness` sparing a multiple select's others. The readers and cursor types a sized single select's list-box path extends (contract/select-readers/*, contract/form-controls/select-rules).

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a multiple select in the bridge and the snapshot) — a multiple select's rows and groups read their own client rects as bounds, while a closed select's options read the select's rect through `closed_select_of`; `differs` naming the moved rows. A sized single select's rows must read like a multiple select's (contract/snapshot/multiple-select-bounds, contract/driver-forms/select-snapshot-options), and `closed_select_of` must not treat it as closed.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (type and press on a multiple select; the lint's overlap) — `type` focuses a multiple select with no click and chooses its match alone; the lint's `overlap` skips a closed select's options by the element (`in_closed_select`) while a multiple select's rows are linted as usual. A sized single select falls on the list-box side of both rules (contract/lint-verb/overlap-select-rows).

@history_item n=10 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (native toggled; the eight engine changes) — the adapter crossing's states gain a multiple select's multiselectable, and the multiple select and optgroup are counted direct engine changes for every document, flagged upstreamable, their merge surface carried. Precedent for the list-box role/state in the tree and for recording this chunk's engine change in the tally (decision/hit-focus-form-engine-changes, decision/multiple-and-optgroup-modelled).

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the blitz-tests records' counts) — `select_interaction` 18 and `stand_act_select` 10 tests, a multiple select's rows/keys/clicks and an option under a closed group refused `covered`; the checks this chunk's tests sit beside.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`size` on a `multiple` select) — `size_rows` (parse, cap 65534, 0 or bad no hint) hints `height: size × 1.2em`, `select[multiple][size] { overflow-y: auto }`, `line-height: 1.2` on rows, a key scrolling the current row into view only with a valid `size`; "a single select's `size` is unread" kept as the limit. The founder's F8 that this chunk partly reverses (decision/select-size-hint), and the exact mechanism to extend to a single select.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded, the select model's limits retired) — a single select opens an engine-painted list by click, Enter or a Space with no typed text; what closes it, Tab choosing, the rejected designs, and the standing limit "a single select's `size` unread". A sized single select must open no list: these rows (decision/single-select-list-opening, decision/select-list-painted-overlay, decision/select-list-tab-chooses-and-closes, decision/select-list-rejected-designs) gain a size exception.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's state and placement) — `select_list_allowed` gates opening (`document.rs`), `open_select_list`, `reconcile_select_list` in resolve, `SelectList.fresh`, at most 20 rows; the scope names `select_list_allowed` as the popup gate to close for a sized single select (decision/select-list-rows-and-placement, decision/resolve-reconciles-select-list).

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's default actions) — `handle_click`'s `select` arm opens/closes the list, its `option` arm `click_list_row` behind `click_lands_on_row`, the label arm focusing a single select and opening nothing, `press_select_key` opening on Enter/Space, Tab running `choose_current_and_close`. Every arm here keyed on "single select" must branch for a sized one (contract/select-list-default-actions/*, decision/label-click-focuses-select).

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the bridge's select tests; the driver's verbs on a select) — `closed_select_of` answers None for an open list, `is_select` the driver's select test, `Target.select`, `focus_select` closing any list; `click` on a select opens its list, `type` focuses any select with no click, `covered` names the closed-select option. A sized single select's options are clickable rows, so `covered`'s meaning and `click`'s help must not apply to it (contract/driver-verbs/click, contract/refusal-causes/covered).

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select reads no `aria-expanded` and writes `expanded` from its list; an author's `open` steers nothing. A sized single select as a list box should not carry a combo box's expanded (contract/accessibility-tree/select-expanded, contract/accessibility-tree/aria-expanded, contract/html-open-attribute/select).

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's exceptions to scrolling and the hit; the list's paint) — `scroll_into_view` for an open list's row writes the list's `scroll_y` alone, the wheel over the list scrolls it, `hit_with_scrollbar` answers the list first, `visible_region` the list's scrollport, `paint_select_list` paints last with engine-constant colours. Those exceptions are for the popup only; a sized single select's rows are real in-flow boxes scrolled by the ordinary primitive (decision/one-scrolling-primitive, decision/scroll-into-view-whole-chain, decision/select-row-scroll-into-view, decision/open-select-list-hit-first, decision/select-list-colours-engine-constants).

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening; his words) — the list crosses only as a row's own bounds and pixels, no `expanded` node key; the founder's F1–F10 words, F8 «Читать на multiple (Рекомендую)» among them. The scope's fork rule (a reading differing from a multiple list box's or HTML's is the founder's) rests on this crossing (decision/select-list-crossing, decision/select-list-engine-change).

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list in the redraws and the changed set; the lint on an open list; the checks) — the list's open/close/move marks the changed set and asks redraws; `Entry.in_select_box` covers an open list's rows; `select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12 added, named in `stand-repeat`. The checks the scope places this chunk's tests beside (decision/select-list-changed-set, decision/redraw-requests).

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — a paint change to a select was green locally and red on `Test (macos)` alone; the trap behind the `embolden` witness, relevant to a single select's new painted face (decision/font-embolden-local-witness).

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` now ends on `cargo test -p blitz-tests --features blitz-paint/font-embolden --test select_popup_painted`, the local witness for any select paint change.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the merge surfaces' reading and owner; the key-press panic's fix) — the select model's and select list's merge surfaces re-owned by «Upstream sync ahead of determinism and exploration»; dropped anonymous blocks clear `layout_parent`, a select's choice still moving past its dropped block. This chunk's engine change adds to that merge surface (decision/anonymous-block-drop-clears-layout-parent, decision/direct-engine-changes-flagged-upstreamable).

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (restyle on mutation) — attribute writes restyle through `restyle_for_attribute_change`, never a blanket subtree restyle; a `size` write that flips a single select between drop-down and list box rides this path (decision/targeted-restyle-on-mutation, decision/snapshot-before-attribute-mutation).

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand checks re-pinned to the flight booker's select; the served-page checks) — the flight booker's two options refused `covered` at its select's rect, `stand_keyboard` walking its arrows; the lint reading a select's options as overlapping, and the overlay-scrollbar hazard (a press in a just-scrolled box's bar band is a thumb drag). Stand pins a sized single select must not move, and the hazard its scrolling rows meet (decision/click-after-scroll-advances-past-fade).

@history_item n=26 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs `select_interaction` and `stand_act_select` among its named targets, so the checks beside them sit in that leg too (contract/ci-leg-legs/a11y).

@history_item n=27 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read and the shared per-verb target check; stand_actions) — one `check_target` serves the id-taking verbs and `actions`; the flight booker's options read `covered` until the list opens. A sized single select's rows change these readings, a fork if they differ from a multiple list box's (contract/actions-verb/*, contract/target-check/click-type).

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader) — an option's or optgroup's graph `owner` is `select_of`'s select; the actionable rows read that owner, so list-box rows keep it.

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `PlacedBy` is a closed set holding `Select` and `SelectList`; a sized single select's rows are laid out in flow, so which placing word they read is a decision this chunk faces (contract/layout-reason/placed-by).

@history_item n=30 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Established Decisions: 2 row(s); the layout answer) — the select list's crossing gained the placing words `select` and `select-list`; the layout answer's closed words cross every carrier. Any new placing word for a list-box row would widen a ratified closed set (decision/select-list-crossing, contract/answer-json/layout-placing).

@history_item n=31 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — the highlight crosses as the closed bool node key `highlighted` (open single select list only), the tree's active descendant names the current option while the list is open, the lint reads a highlighted row at the engine's colours, and a highlight move marks the changed set. A sized single select has no open list, so it reads none of these, its chosen row instead marked by `selected` (contract/node-keys/highlighted, contract/accessibility-tree/active-descendant, contract/lint-verb/contrast, contract/test-harness/highlight-contrast, decision/select-list-changed-set).

@history_item n=32 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Standard Contracts: 4 row(s) (2 of 3)) — `press` gains `ctrl` as the host's action key, retiring the limit that non-adjacent multiple-select rows could not be built; the press contract a list box's keys are driven through (contract/driver-verbs/press).

@history_item n=33 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Cross-cutting Patterns: 1 row(s)) — the changed-set write-sites row gained the select-list scroll exception; a list box's choice marks through `apply_select_choice` and its row scroll marks nothing (decision/changed-set-write-sites).

@history_item n=34 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — the post-layout focus fixup and the Tab skip join the engine-change tally (decision/hit-focus-form-engine-changes, its why restored), the changed set marks a focus given up after a layout, and the driver's select focus stays one of the silent focus paths (contract/event-dispatch/focus-change). The tally row this chunk's engine change adds to.

@history_item n=35 marker=2026-11-16-taffy-past-1372
**2026-11-16-taffy-past-1372** (Standard Contracts: 3 row(s)) — `scroll_into_view` skips a box whose scroll min and max are both zero and still writes an open list's row alone; a list box's row brought into view goes through the chain path (contract/scroll-hit/scroll-into-view, decision/programmatic-scroll-one-box).

@history_item n=36 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Established Decisions: 5 row(s)) — scroll offsets clamped after layout, a removed subtree's scroll state reset; a list box whose rows shrink must read clamped (decision/scroll-offsets-clamped-after-layout).

@history_item n=37 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Established Decisions: 4 row(s); Cross-cutting Patterns: 4 row(s)) — user scrolls chain except where overscroll-behavior contain stops them; events queued on the document, a cancelled pointerup still clicking. A wheel over a list box and a row click ride these (decision/user-scroll-chaining, decision/events-queued-on-document, decision/default-action-event-order).

@history_item n=38 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a scrolled box reads its own bounds and a hit stops where a box clips by overflow; a list box is such a scrolling box, its rows read where they show (decision/client-rect-own-box, decision/hit-stops-at-clip-padding-box, decision/scrolled-box-bounds).

@history_item n=39 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `scroll_into_view` widened to the whole containing-block chain for every document, and `disabled`/`off-screen`/`covered` detection; the base of the scroll and refusal rows a list box's rows read (decision/scroll-into-view-whole-chain, contract/target-check/click-type).

@history_item n=40 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (seven engine changes; the geometry readers) — `disabled` read by presence on a form control alone (`ElementData::is_disabled`), the hit stopping at a `contain: paint` box; a disabled sized select's rows and the click gate follow (decision/disabled-read-on-form-control-only, decision/hit-stops-at-paint-clip).

@history_item n=41 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (keyboard activation and a click's focus) — the button-arm click focus that the select later excepted; decision/click-focuses-button's except (a select focuses through its own arm, a multiple select's row focuses its select) must cover a sized single select's row.

@history_item n=42 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the accessibility builder reads three state attributes) — `aria-selected` and friends read once on every element; later excepted for a select's option, whose `selected` is its chosen state (decision/a11y-aria-states, contract/accessibility-tree/aria-selected).

@history_item n=43 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads) — the builder reads `aria-expanded` on every element, later replaced on a single select by its list state; the list-box role choice the scope names (combo box vs list box) sits on this builder (contract/accessibility-tree/state-attributes).

@history_item n=44 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`:open` and `ElementData::flush_open_state`) — `:open` holds to `details` and `dialog`, a select never matching; a list box must not take `:open` (decision/open-pseudo-class-details-dialog).

@history_item n=45 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (change on a checkbox and a radio) — the engine dispatches `change` after `input` on a select's choice and on checkable clicks; a list box's row choice keeps that pair through the one path (contract/dom-events/change-after-input).

@history_item n=46 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (Standard Contracts: 6 row(s)) — the cold-agent verdict's `capability-unused` now needs a list opened (`list_opens`); a stand select turned into a sized single select would open no list, a trap for the stand's served pages.
