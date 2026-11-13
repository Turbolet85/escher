@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the flight booker's mode list, the engine's select list in every document) added the open select list: a click, Enter or Space opens `select#flight-type`'s list; a click on a row chooses; a second click, a click outside, Escape, the page's scroll or an outside wheel closes it with the value unchanged; focus stays on the select; while open each row reads its own bounds, back at the select's client rect after a close (rows decision/open-list-rows-own-bounds, decision/flight-type-select-boot-reading). These are the select arms of `handle_click` this chunk restructures: their ordering and close/choose semantics are founder forks F1–F4/F7 and must survive the move unchanged, held by `select_interaction` / `select_popup` / `stand_act_popup`.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list's rows, the exception to the scrolling boxes rule) a row or group of an open list is seen through the list's scrollport alone and `scroll_into_view` on a row writes the list's `scroll_y` at once, no scroller or viewport moving (rows decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, their `except` clauses). The driver's `check_target` / off-screen refusal in `execute` reads `visible_region`; a restructure of the executor must keep this exception path and its order intact.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the lint's `overlap` exclusion, `Entry.in_select_box`) an open list's rows read with no `overlap`, a closed select's options at the select's client rect; `at_boot` `[0, 0, 0, 0]` in both layout modes kept (row decision/stand-lints-clean-at-boot). A behaviour-preserving move of the click/select arms must leave open/closed select state, and so these lint and snapshot readings, unchanged.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the Cells grid, a keyboard region and one Tab stop) a click inside a cell's own open editor now returns early so typed text stays — a pointer path that moved and left a pointer-opened editor's earlier reading unmet, ruled by the operator at P2 Validate as restated, not by the founder's word. A trap for this chunk: a pointer path changed silently once; the restructure of `handle_click` must keep every pointer outcome exactly, with a check per moved arm (scope's witness).

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the load page's Tab sequence and its button's keys) `load-fetch` takes Enter and Space, walked by `stand_keyboard` in both layout modes (row contract/stand-element-ids/load-fetch). Button keyboard activation and focus are among the button arm's default actions this chunk moves; `stand_keyboard` is a named held check.

@history_item n=6 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — the CRUD list took `tabindex="0"` and an `onkeydown`: the arrow, Home and End keys fill the name fields "as a row's click does"; focus stays on the list, never a row; CRUD's Tab sequence six stops (row decision/crud-list-keyboard-region). A click on a CRUD row and the focus a click gives are behaviour the restructure must not alter.

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — (the stand's nested scroller) an into-view scroll moves every holding scroller then the viewport, `visible_region` reads what an element is seen through; after twelve Creates the last CRUD row's driver `click` is refused `off-screen` until a driver `scroll` brings it in (rows decision/scroll-into-view-moves-every-scroller, decision/visible-region-through-clipping-boxes, contract/escher-session-refusals/off-screen). This is the executor's target check and refusal order that `execute`'s restructure must keep; founder-ratified (2026-10-07).

@history_item n=8 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a driver `click` naming `crud-list` lands on the row at its centre at 12 and 14 Creates; Home's seventh card (Cells) is hit only once scrolled into view, found when a standing check went red on the fixed hit. Precedent that hit/click outcomes are pinned by standing checks; a reordered click path or executor branch would show here.

@history_item n=9 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served screens (counter, flight-booker, timer, crud, cells, load); the flight booker's mode became a select (`flight-type` · `flight-type-one-way` · `flight-type-return`, was toggle buttons); the load page's `LoadNet` answers from its own thread so a driver `click` on `load-fetch` waits it out (rows decision/load-page-only-network, contract/stand-element-ids/flight-type*, contract/escher-session-served-tasks/*). The click-then-wait path in `execute` and the select's click arms are pinned here.

@history_item n=10 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — each acting tool's `settled` field states that a load in flight is waited for up to 5 s, fixed help text in a ratified carrier. The executor's settle/wait step after an acting verb is part of the per-verb dispatch `execute` holds; no help text, field or span may move.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` became the driver's command line: four endings (accepted exit 0, refused exit 1, usage exit 2, session error exit 3), the former two-argument form refused `unknown-verb` (rows contract/escher-session-exit-codes/0..3, contract/escher-session-refusals/unknown-verb, decision/no-caller-input-echoed). The refusal order and endings the restructured `execute` must preserve.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `screenshot` verb: one line `{"width","height","png"}`, `--id` cropping to visible bounds; refused `fonts-unavailable` (no font) and `off-screen` (element shot of no pixel), exit 1 (rows contract/escher-session-screenshot/answer, contract/escher-session-refusals/fonts-unavailable, contract/escher-session-refusals/off-screen). A per-verb arm and refusal pair inside `execute`.

@history_item n=13 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: `escher-session lint`) the `lint` verb: no argument, one line `{"findings":[…]}`, refused `fonts-unavailable` with no font; the stand reads 0 findings at boot through the verb as through the reader (rows contract/escher-session-lint/answer, decision/stand-lints-clean-at-boot). Another dispatch arm and refusal of `execute`.

@history_item n=14 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` (1 to 1000) answering `{"text"}` or `{"text","part","parts"}`, a part past the last empty text, outside the bound refused `malformed` (rows contract/escher-session-snapshot/answer, contract/escher-session-snapshot/part). Argument validation and refusal order inside `execute`'s snapshot arm.

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (escher-session gains the graph verb) the ninth instance verb `graph` answers `{"nodes","states"}` carrying ids, component names, handler event names, state names, closed flag words and booleans only, never a value; a named argument refused `malformed` (row contract/escher-session-graph/answer). This is the content `DioxusDocument::graph` produces and `execute` dispatches; the restructure of `graph` must leave its nodes, states and order byte-identical (held by `stand_graph`). PROVISIONAL (delegate the overseer) at its wrap.

@history_item n=16 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the screens' source citations after the graph registrations) source citations into the task files, `dioxus_document.rs` and `schema.rs` re-pointed after the graph registrations shifted lines, no claim moved. Precedent: a line-shifting edit near the graph code re-points citations; now the masters are rows (see the rows entry) the restructure moves no claim.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (escher-session's tenth instance verb actions) `actions` answers `{"nodes"}` of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or `NodeId`, unpaged (row contract/escher-session-actions/answer). A further arm of `execute`'s per-verb dispatch whose content and field omission rules must survive the restructure.

@history_item n=18 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (escher-session's coverage role) `coverage <app> <check>...` replays each kept check on a fresh boot and answers `{"unexercised":[{id,owner,event}],"checks","held"}`; a failed boot is the session error, exit 3 (row contract/escher-session-coverage/answer). Its unexercised list reads the graph's handler events, so `stand_handler_coverage` holds the `graph` restructure; PROVISIONAL (delegate overseer) at its wrap.

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role a further consumer of the TaskShell mount) the coverage role boots through the same stand boot, adding no markup; a held instance reads as a fresh boot, the mount and pinned viewport hold after commands in both layout modes (rows decision/session-roles-share-stand-boot, decision/held-instance-reads-as-fresh-boot). Behaviour the executor's restructure must keep.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121 rows: decision 21 · contract 99 · absence 1), each verified against the code at edd0cfaea7f0 / fc2d17afd. Every picked row this chunk rests on (the click, select, scroll, refusal and verb-answer rows) is from this conversion; a behaviour-preserving restructure should amend no row, and any row it would move signals a behaviour change outside the scope.
