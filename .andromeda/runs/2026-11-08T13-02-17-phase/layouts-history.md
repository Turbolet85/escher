@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's TaskShell mount (`main#main > #task-shell`, header above `#task-body`) at the pinned 800 x 600, scale 1.0, Light viewport; the read is proven against exactly this mount.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids (counter-*, flight-*, timer-*, crud-*), attributes only, no class/style/wrapper/order; these are the stable ids the read must list nodes under, and the precedent that ids are added without moving the stand.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows keyed by person id rather than index so an id follows its person under filter, Create and Delete; the read must not key a row on DOM position.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows read author id `crud-person-{id}` and Home's cards `task-card-{slug}`, on the founder's rule that every element an agent can act on reads an author key; the basis of the actionable-key check (L12) this chunk may fold in.

@history_item n=5 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the non-lean tasks' author ids (`temp-*`, `circle-*`, `cell-{A..Z}{0..25}`, `cell-input`) and `unkeyed_actionable()` reading 0 on all seven tasks; the actionable-key baseline the L12 fold must reproduce.

@history_item n=6 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — `crud-list` takes `role="listbox"` and rows `role="option"` with `aria-selected` (the flight toggle part later superseded); interactive roles bear on which elements the actionable set counts.

@history_item n=7 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"` and `onkeydown`; CRUD's Tab sequence is six stops with focus on the list, never a row; a focus-order fact the read must leave unchanged and a focusable/listener element in the actionable set.

@history_item n=8 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by attribute alone, Tab sequence unchanged; part of the snapshot/tree state the read must leave the same.

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the stand's nested scroller: `crud-list` clips its rows, `task-body` does not; after twelve Creates the last row is refused `off-screen` until a driver `scroll`; `visible_region` reads what an element is seen through; the pinned viewport is scrolled only on an in-file fixture. The off-screen acceptance case and the fixture precedent rest on this (engine change ratified by the founder 2026-10-07).

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — what scrolled `crud-list` reads and that a click lands at 12 and 14 Creates; Home's `#home` scrolls and the Cells card (615 to 684) is off-screen until scrolled; a second off-screen case on Home the read's causes must agree with (a standing check went red when this went unstated).

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — a row or group of an open select list is seen through the list's scrollport, `visible_region` answering it; `scroll_into_view` on a row moves no box or viewport; the read's off-screen cause must use the same reading.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — the open list painted by the engine, no row a layout box, focus staying on the select, rows reading their own `bounds` while open and the select's client rect after close (founder's forks F1-F4, F7); the owner-served case and the anti-pattern against keying on layout boxes rest on this.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_select_box`) — `closed_select_of` reads the element never the role, so CRUD's `role="option"` rows are not a select's; a closed option reads its select's client rect, an open row its own `bounds`; the owner relation (select -> option) the read must reproduce without misreading CRUD rows.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served screens (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`) with their boots; the flight booker's mode became `select#flight-type` with options `flight-type-one-way` · `flight-type-return` (was toggle buttons); `load-fetch` answered after `LOAD_ANSWER_DELAY` 200 ms; the screens the read is proven on and the owner-served case's ids.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — boot lint `[0, 0, 0, 0]` in both layout modes, a closed select's options skipped via `closed_select_of`; lint findings the read must leave unchanged, and the closed-select option reading the owner-served case relies on.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — `cells-grid` one Tab stop (tabindex 0, onkeydown, no role) over 676 data cells, 677 reacting elements measured (plan's forecast of 676 falsified); `CELLS_TAB_ORDER`; the count trap and the baseline for the actionable set on Cells.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence and its button's keys) — `LOAD_TAB_ORDER` and `load-fetch` taking Enter and Space; a focus-order fact the read must leave unchanged.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines) — the load page holds two keyed elements, `load-fetch` and `load-done`, and a driver `click` on `load-fetch` waits out `LoadNet`; the in-flight load case the read must answer without dispatching.

@history_item n=19 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting tools' `settled` field states a load in flight is waited up to 5 s, fixed table help text; a read with the load in flight must not wait or dispatch, and any new help text is fixed table text in a ratified carrier.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the stand reads clean) — after twelve Creates `crud-person-14` reads `clipped` (visible 367 x 11.203125) inside `crud-list`, agreeing with the driver's `off-screen` refusal; the model for this read's reason-agreement check.

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — `lint` added as a read-only verb on every carrier answering one uncoloured line `{"findings":[…]}`, kept under `--record`, listed in MCP after `screenshot`, on the founder's ratification; the precedent shape and boundary widening a new read verb follows.

@history_item n=22 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the lint's PROVISIONAL «in process only» mark dropped on the founder's word, its verb owed to a gated part; precedent that an in-process reader and its carrier verb are separate founder decisions.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — `VERBS` 12 to 13, instance verbs 8 to 9, `graph` after `lint` answering `{"nodes","states"}` with ids, closed words and booleans only, usage line and MCP list from the table; PROVISIONAL (delegate the overseer) until the founder's word at the epoch boundary. The counts a new verb would extend and the graph this read reads.

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations after the graph registrations) — task-file, `dioxus_document.rs` and `schema.rs` citations re-pointed after the graph registration lines; any line this chunk adds to `schema.rs` or the task files shifts these citations again.

@history_item n=25 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the ratified `escher-session <verb> [arguments] --session <dir>` shape with one uncoloured JSON line and the four endings (0 · 1 · 2 · 3); the client command boots nothing and the stand's layout is unchanged; the shape a new verb must follow.

@history_item n=26 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` and the MCP tool descriptions built from the verb table (each tool's description its help plus result fields), statuses from `EXIT_STATUSES`; a verb-table change here flows into `help` and `tools/list`.

@history_item n=27 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` with optional `--id` and its `off-screen` refusal for an element shot of no pixel; a prior verb whose refusal cause words the read's causes must stay consistent with, and a precedent of a founder-ratified verb addition.

@history_item n=28 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role lists the verbs as its tools and is a further consumer of the TaskShell mount adding no markup; a new verb reaches MCP through this role.

@history_item n=29 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record` keeps calls as a replayable check; `help` and `tools/list` the same with and without it; a new read verb's step would be kept here (founder-ratified boundary widening).

@history_item n=30 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check holds each call's answer, masked as on every carrier, replayed by `Check::replay`; the read's answer, if kept, enters a recording, a boundary question for the founder.

@history_item n=31 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host consumes the TaskShell mount adding no markup, id, class, style, wrapper or order, and a held instance reads as a fresh boot; the read-only constraint the read keeps.
