@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`size` on a `multiple` select) — added the `size_rows` read (`parse_unsigned_integer`, capped 65534, `0`/non-numeric no hint, a `height` of `size` × 1.2em hint), `default.css` `overflow-y: auto` and row `line-height: 1.2`, `scroll_into_view` only where `carries_valid_size`; narrowed the `select` row's limit to «a single select's `size` is not read»; ratified as an engine change for every document and a boundary widening (F8 + Q1). This is the very read and limit this chunk extends to single selects: lifting it reverses part of F8 and re-opens decision/multiple-select-size-rows, decision/select-size-hint-cap and the `except` of decision/single-select-selectedness, so it rides a fresh ratification.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (his words on the select list's widening) — records the founder's F1–F10 verbatim, including F8 «Читать на multiple (Рекомендую)» (the source of «a single select's `size` stays unread») and F9 «По-прежнему covered», plus Q1 «Утверждаю». The chunk's partial reversal of F8 must be recorded against this entry, and the F9 `covered` rule for a closed select's option must be re-read for a sized single select whose rows are real boxes.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `select` row's rules, limits and pins) — options include `optgroup` children's, a `multiple` write settles via the recorded select, stated limits incl. «an option of a closed single select has no click arm and stays refused `covered`» and «`size` is not read», `size` left to the popup chunk by the founder at P5. The limits this chunk rewrites; pins `select_interaction`, `stand_act_select` are the guards to extend.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys, clicks and chosen set) — introduced `press_multiple_select_key` (closed key match), `click_select_row` behind `handle_click`'s `option` arm, `apply_select_choice` as the one write path, and the chosen-set exits; an engine change for every document and a boundary widening (F6). The list-box key/click model this chunk reuses for a sized single select: any single-select variant must stay inside the closed key match and the one write path (decision/multiple-select-keys-and-clicks, decision/select-choice-one-write-path, decision/select-keys-closed-match).

@history_item n=5 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the `select` single-choice row) — the choice held as `CHECKED`, settled by HTML's selectedness algorithm, closed keys and the 1 s private typeahead, `input` then engine `change`, the choice's exits inside the ratified crossings; a boundary widening and engine change for every document, ratified. Grounds decision/single-select-selectedness, decision/select-typeahead-private and decision/select-choice-exits, which a list-box single select must keep (one chosen row, same exits, same typeahead).

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `select` row: Enter, Space and a click open the list) — a click on a single select now opens/closes its list; Enter and Space (no typed text) open it via `press_select_key`; retired «no popup» limits; F1, Q1. The behaviour a sized single select must NOT have (no popup): the chunk changes this row's gate per HTML's list-box rule and must restate it.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (which select opens a list, what a row takes, and hostile markup) — `select_list_allowed` gate (disabled, inert, hidden, no row, non-invertible transform), disabled row/group header choose nothing, hostile `size` / 2,000 options / 5,500-char labels keep every box finite, `MAX_ROWS` 20; ratified widenings. The gate this chunk extends (a sized single select opens no list) and the hostile-markup guard a single-select list box must also pass (decision/select-list-open-gate, decision/select-list-hostile-markup-finite, decision/select-list-row-choice).

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's state, keys, pointer and close) — one `select_list` field read via `live_select_list`, `press_open_list_key`, Tab chooses and closes, release chooses, press outside/scroll/box move closes; ratified widenings. The open-list rows (decision/select-list-live-reader, -closing, -pointer-release-chooses, -tab-chooses-and-closes, -wheel) must stay correct when a sized single select never opens a list.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses the carriers) — an open list row crosses only as its own `bounds`; no `expanded` node key (operator's call); highlighted row in no structured carrier (later changed). Precedent for decision/select-list-crossing: a list box's rows as real boxes must cross within this closed set, any new reading being a fork.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded`, and an author's `open` and `aria-expanded`) — a single select writes `set_expanded(open)` from engine state and reads no `aria-expanded`; `open` on a select steers nothing. A sized single select (a list box, never expanded) changes what this engine-state `expanded` and the accessibility role mean (decision/select-expanded-from-engine-state); the snapshot's `expanded` reading is named in the scope.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a label's click on a bound single select) — a label's click on a bound single select focuses it and opens nothing, asserted by `select_popup`. The bound-label path must still hold for a sized single select.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the select verbs' help) — `covered` meaning names an option of a closed select; `click`, `press`, `type` help restated on select behaviour, fixed literals under the 2026-10-12 rule. A sized single select's options are clickable rows, so these fixed texts (contract/refusal-causes/covered) need restating, not new causes.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `focus_select` focuses any select with no click, clears typeahead, closes any list, a text opening on a space types nothing; `Target` holds `select`. decision/driver-type-on-select stands for a sized single select too.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select readers and the list's public surface) — `closed_select_of` answers none for an open list; lint `in_select_box`; read-only `open_select`/`select_list_bounds`/`select_list_view`/`select_row_layout`, one writer `close_select_list`; `SelectListRow.label` never logged. Readers whose "closed select" notion a sized single select changes (decision/select-readers-resolve-node-closed-select-none, decision/select-list-label-never-logged).

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's readers and its close add no panic) — readers via `get_node`, an unstyled row reads `None`; closing on a select removed/disabled/hidden/inert. Grounds decision/select-row-layout-none-without-styles and decision/select-list-closes-when-select-unfit.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region, paint and scroll) — visible region of an open row is the list's scrollport; `paint_select_list` with engine-constant colours, non-finite boxes unpainted; `scroll_into_view` writes the list's own offset. A list-box single select is painted and scrolled as page boxes instead, so these rows must stay scoped to the popup.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a shot of an open list holds labels as pixels) — option and group labels as pixels in a shot, a ratified widening. A sized single select paints its option labels in-flow as pixels on every shot (as a multiple select does); check it stays inside this ratified pixel class.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`optgroup`'s `label` painted as a header) — a group's label painted as a header in an open list, F7. A list box shows group headers too; the painted-header ratification covers the popup only as written.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's markup reaches the select list; the select list in the log) — threat model names remote markup reaching the list incl. `size` on a `multiple` select as a capped number; the list holds no log site, `host_log`/`mcp_log` 0 option needles. The threat-model line must name `size` on a single select; the silence checks should cover the list box.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup` and its `label`; `type` on a `multiple` select and the public select readers; a non-invertible transform takes no hit) — options walk through groups, `option_disabled` one rule, group label as name; `focus_multiple_select` and `select_of`/`closed_select_of`/`is_multiple_select` via `get_node`; `hit_inner` none under a singular transform. Grounds decision/optgroup-options-and-label, decision/option-disabled-one-rule and the open gate's transform clause.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the key-press panic after a hidden ancestor) — a key press panicked in `position_in_containing_block` after a focused control's anonymous block was dropped, reproduced on a `select`. Trap for an in-flow list box's keys; retired later (n=22).

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — `deallocate_anonymous_block` clears children's `layout_parent`; `select_popup` presses keys in every row. Origin of decision/dropped-anonymous-block-clears-parent, which a list box's key path relies on.

@history_item n=23 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Input Validation 3 rows; Error Handling 1 row) — post-layout silent focus clear for a node that no longer shows; decision/programmatic-focus-events except names «the driver's select focus fire none»; decision/dropped-anonymous-block-clears-parent restated to «a re-focused control». The latest state of rows this chunk's key/focus model rests on.

@history_item n=24 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (Threat Model 2 rows; Input Validation 3 rows) — decision/select-list-crossing and decision/snapshot-state-readings gain the closed `highlighted` bool on an open single select's highlighted row; contracts diff-node/highlighted and snapshot-text/highlighted added. A sized single select has no open list: whether a list box carries `highlighted` (or not) is a reading that could differ from a `multiple` list box's — a fork per the scope.

@history_item n=25 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Threat Model 2 rows; Input Validation 4 rows; Data Protection 1 row) — decision/select-list-crossing gains the layout answer's `select-list` placing word; `contract/actions-answer/verbs` lists `layout` with no cause; the layout verb and decision/session-layout-read-in-process on every carrier; decision/kept-check-layout-answer. A list box's rows read as ordinary boxes in a layout answer, not `select-list` — the placing word must not leak to it.

@history_item n=26 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings; the closed-field rule) — `selected`/`unselected`, `pressed`, `invalid` as closed words/booleans, and the founder's rule that a closed-value field inside a ratified carrier is no widening. The rule (decision/boundary-widening-rule, decision/closed-value-field-no-widening) that decides whether any new list-box reading needs the founder.

@history_item n=27 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (control states row) — `aria-selected`/`aria-pressed`/`aria-invalid` trimmed, lower-case exact, closed exits only. Grounds decision/aria-state-lowercase-exact-match (a select's option reads its chosen state, never `aria-selected`) and decision/control-states-closed-values-only for list-box rows.

@history_item n=28 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§Input Validation: the `inert` read and the toggle per role; `disabled` on form controls) — `inert` focus/hit/tree exclusion, toggle per role, `disabled` presence on form controls alone. Gates the list box's focus and row click like the open gate's inert/disabled clauses (decision/disabled-read-by-presence).

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus an inert subtree loses, and the tree's `focus`) — tree `focus` names only a built node, else the window. Origin of decision/accessibility-focus-built-nodes-only; the list box vs combo box role change in `accessibility.rs` must keep it.

@history_item n=30 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — `aria-expanded`/`aria-disabled`/`aria-level` closed parse; descriptions as a fourth adapter content class. Grounds decision/merged-aria-states-closed-parse and decision/accessibility-adapter-content-classes touched by the select's role change.

@history_item n=31 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stable id as each element node's AccessKit `author_id`, leaving only through the adapter. Origin of decision/author-id-on-accessibility-nodes; list-box rows as tree nodes keep it.

@history_item n=32 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — platform tree delivered on change; the diff as an id reader. Origin of decision/accessibility-tree-refresh-on-change (later moved to the redraw, n=33).

@history_item n=33 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Input Validation 5 rows) — refresh on redraw after style resolution; decision/event-is-trusted-by-source and decision/programmatic-focus-events added. A list-box choice fires user-agent events read trusted, and focus events follow the programmatic-focus rule.

@history_item n=34 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation (Enter keydown, Space keyup) on a closed element match, a button click focuses, `disabled` by presence. Origin of decision/space-keyup-activates-focused-control; a list box's Space must stay out of that arm, as on a multiple select.

@history_item n=35 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — after `validate`, an unknown id refused `stale`/`not-found`, then `disabled`, `off-screen`, `covered` in fixed order with nothing run. Origin of decision/target-refusal-order and decision/unknown-id-refused-stale-or-not-found; a list-box row click must refuse in that order, no new cause.

@history_item n=36 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — closed refusal set, fixed texts holding no call content. Grounds decision/closed-refusal-cause-set and decision/refusal-closed-cause-set: the chunk may restate texts but adds no cause.

@history_item n=37 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (what the driver logs) — one span, only a refusal's cause name logged. Origin of decision/refusal-log-cause-name-only and decision/refusal-logged-by-cause-name.

@history_item n=38 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — an id taken in, looked up anew by `element_ids()` and `get_node` on every call, never by index. Origin of decision/id-lookup-anew-each-call.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, and `off-screen`'s meaning) — `visible_region` reads clipping boxes through transforms; `off-screen` text restated. A list box is a scrolling box: rows scrolled out read `off-screen` with `scroll` as remedy (contract/refusal-causes/off-screen).

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region) — for an open list's row the visible region answers the list's scrollport (decision/visible-region-open-list-row); an in-flow list box's rows take the ordinary clipping path instead.

@history_item n=41 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (threat model, socket, verb table, validation, shared target check, one actionable rule, unpaged size) — the `actions` answer crossing (ids, owner ids, `keyed`, verb and cause words), one `check_target` shared by verbs and `Session::actions`, `target_of` calling `is_select`, the one actionable rule, the unpaged bound. A list box's options become click targets: the actions read must list exactly what a click would take (decision/actions-read-shares-target-check, decision/one-actionable-rule).

@history_item n=42 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** (Input Validation 2 rows; Data Protection 1 row) — `keyed` ratified: decision/actions-keyed-field, contract/actions-answer/keyed and decision/kept-check-actions-answer go standing.

@history_item n=43 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the kept actions step's replay now exercised) — `stand_replay`'s kept-actions test guards decision/kept-check-actions-answer; a changed actions answer from list-box rows will stop a kept replay.

@history_item n=44 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint and the text extent reader; the gated part) — lint findings hold ids, closed words, numbers only and cross every carrier; `text_extent` read-only, none when unresolved. The lint's select-box reading must handle list-box rows (decision/lint-findings-crossing, decision/lint-findings-closed-content, decision/text-extent-read-only-none-when-unresolved).

@history_item n=45 marker=2026-10-12-self-description
**2026-10-12-self-description** — the program's fixed help text crossing is ratified with the rule that fixed text inside a ratified carrier is no widening. Covers restating `click`/`press`/`type` help for a sized single select.

@history_item n=46 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screen's pixels in band on the carriers (contract/screenshot-answer/png), a ratified widening. A list box's painted rows are page pixels in this class.

@history_item n=47 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the snapshot's one text form, `id=` and names quoted in Debug form. Origin of contract/snapshot-text/id-field and name-field that list-box rows are written under.

@history_item n=48 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `NodeState.clickable` closed bool, `differs` can name it; a closed value inside the ratified carrier. contract/snapshot-text/clickable and contract/diff-node/clickable read on list-box rows.

@history_item n=49 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names and values leave in an answer) — ids, names, masked values cross on the socket and stdout; nothing of them in a log. Grounds decision/socket-carries-screen-reading, decision/accessible-name-exits and decision/stable-id-exits.

@history_item n=50 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing) — MCP as the third exit for ids, names, states, bounded results. Grounds contract/mcp-server/tool-result and the stable-id exits.

@history_item n=51 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped whole, no id or name at any level. Grounds decision/stable-id-not-logged, decision/accessible-names-not-logged and decision/telemetry-sink-target-allowlist; list-box labels must reach no log.

@history_item n=52 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (Input Validation 2 rows) — decision/socket-carries-screen-reading reworded to «caused text scrubbed by its own field names, error kept».

@history_item n=53 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring 1 row) — decision/no-screen-content-in-logs now lists a step's caused text; together with 2026-11-16-layout-explained (layout answer) the current list the chunk's log checks read.

@history_item n=54 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Input Validation 1 row; Logging & Monitoring 1 row) — decision/session-layout-read-in-process added; no-screen-content-in-logs names the layout answer.

@history_item n=55 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (Input Validation 1 row) — contract/driver-session-reply/no-attribute-text gains its author-text exception; a list box's `size` must not cross as attribute text.

@history_item n=56 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check's call content, `MASKED_STAND_IN` for masked types. Origin of decision/kept-check-call-content (extended by 2026-11-20-driver-debts with ctrl and graph parts).

@history_item n=57 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Data Protection 2 rows) — decision/kept-check-call-content gains the ctrl flag and graph parts; latest form of the row a list box's Ctrl key or click would be kept under.

@history_item n=58 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — eight causes; `advance` on every session; scrollbar fade on the document's clock. Origin of decision/scrollbar-fade-on-document-clock: a list box's overlay scrollbar fade must read the document clock so shots stay deterministic.

@history_item n=59 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting step's bounded load wait; snapshot and screenshot wait on none. Bears on decision/time-moves-only-on-advance for list-box steps.

@history_item n=60 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Input Validation 2 rows) — added decision/presentational-hints-typed-parse; the `size` × 1.2em height hint in `presentational_hints.rs` that this chunk widens to single selects is read under that typed-parse rule.

@history_item n=61 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`multi_doc`'s inputs, …, the moved `inline` path) — the inline-box height guard re-cited; decision/inline-box-heights-finite holds against a hostile `size` hint on a single select.
