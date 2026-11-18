@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand mounting one task in TaskShell (`task_in_shell`) at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans; this is the origin of `decision/headless-stand-pinned-viewport`, the viewport and font every keyboard and contrast re-read in this chunk runs on.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations to the `Default` impl holding 800x600, scale 1, light; the pinned-viewport values the re-read's checks inherit are cited there.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls and displays author ids as attributes alone (no class, style, wrapper or order, no CSS selecting by them); origin of most `contract/stand-element-ids/*` rows and of `decision/stand-ids-as-attributes-alone`, the ids the Tab sequences and contrast rows name.

@history_item n=4 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows keyed by `{person.id}` (fixture 0–2, Create from 3) instead of index, so a row's id follows its person under filter, Create and Delete; behind `contract/stand-element-ids/crud-person-id`.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the six task inputs by attributes alone (`for` on labels of timer-duration, crud-filter, crud-name, crud-surname; `aria-label` "Departure date"/"Return date"); origin of `decision/stand-a11y-by-attributes-alone` — the precedent any naming fix this chunk finds must follow (attributes only, no structure move).

@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows carry `crud-person-{person.id}` and Home's seven `task-card-{slug}` ids, attributes only, on the founder's rule that every actionable element reads an author key; behind the `task-card-*` and `crud-person-id` rows.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the `escher-session` host became a third consumer of the TaskShell mount, booting through the stand boot only and adding no markup; a held instance reads as a fresh boot; origin of `decision/session-roles-share-stand-boot` and `decision/held-instance-reads-as-fresh-boot` — why stand checks speak for the served pages.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — into-view scroll moves every holding scroller then the viewport, and `visible_region` reads what an element is seen through (founder-ratified for every document 2026-10-07); `crud-list` the stand's nested scroller; origin of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes` — engine rules a focus-order fix must not widen.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` became the driver's command line (`<verb> [arguments] --session <dir>`, exits 0/1/2/3, host role `serve`), the third consumer named as the binary's host role adding no markup; base of `contract/escher-session-command/form`.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a scrolled `crud-list` reads true bounds and hits; Home's `#home` is a scrolling box at 800 × 600 with the Cells card below the fold, shown and hit only once scrolled — a trap for any Home keyboard/focus walk or contrast read.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — states by attributes alone: `aria-invalid` on the flight date inputs, `crud-list` `role="listbox"`, rows `role="option"` with `aria-selected`, the state classes staying (the `aria-pressed` toggle part later retired by 2026-10-25-cold-agent-bar); behind `decision/stand-a11y-by-attributes-alone`.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role a further consumer of the TaskShell mount, booting through the same boot and adding no markup; part of `decision/session-roles-share-stand-boot` for the served pages.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `screenshot` verb: `{"width","height","png"}`, `--id` cropping to visible bounds, refusals `fonts-unavailable` and `off-screen` (exit 1); origin of `contract/escher-session-screenshot/answer` and the two refusal rows — the font gate bears on the bundled-font contrast read.

@history_item n=14 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"` and `onkeydown` (ArrowDown/Up/Home/End select and fill the name fields); CRUD's Tab sequence becomes six stops; tree focus is the list, never a row, no `aria-activedescendant` — origin of `decision/crud-list-keyboard-region`, the SC 2.1.1/2.4.3 assertion this chunk re-reads; the tree-focus gap is a stated CARRY, not a defect to fix.

@history_item n=15 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone; the current-option gap stays stated and the six-stop Tab sequence unchanged; the last assertion of the SC criteria this chunk re-asserts at HEAD.

@history_item n=16 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the three non-lean tasks keyed (`temp-*`, `circle-*` incl. the open dialog's `circle-diameter`/`circle-close`, `cell-{A..Z}{0..25}`, `cell-input`) as `id:` lines alone; `unkeyed_actionable()` 0 on all seven tasks — origin of `decision/every-stand-control-keyed`.

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint (incl. contrast failure) at boot in both layout modes; `crud-person-14` reads `clipped` after twelve Creates; then (gated part, same marker) `escher-session lint` with its `{"findings"}` answer and `fonts-unavailable` refusal — origin of `decision/stand-lints-clean-at-boot` and `contract/escher-session-lint/answer`.

@history_item n=18 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the visual lint's PROVISIONAL mark (in process only, as built), on the founder's word; status of the lint-clean-at-boot row.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served screens (counter, flight-booker, timer, crud, cells, load) through `task_in_shell`/`boot_cells`/`boot_load`; flight mode became a select `flight-type` with `flight-type-one-way`/`flight-type-return` (no `aria-pressed`); load page `load-fetch` white on `#3a5ce5`, `load-done` hidden until its stylesheet lands after 200 ms `LoadNet` — origin of the served-tasks rows, `decision/load-page-only-network`, `decision/flight-type-select-boot-reading` and the flight ids; defines the served pages the scope's P3 must cover.

@history_item n=20 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (PROVISIONAL, later ratified) — the stand's colour rows landed here: accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) on `#f5f5f5` with `#1a1a1a`; white on the four accent controls reads 5.47 (hover 6.83, active 8.27) against SC 1.4.3's 4.5, each a reached state of `stand_contrast`, `#4a6cf7` 4.39 a fail — origin of `decision/stand-accent-colour`, the exact ratios the bundled-font contrast re-read re-asserts.

@history_item n=21 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; F2 ratified) — load page waits `LoadNet` on a thread of its own so a click on `load-fetch` returns with the message shown; the select's styling 15px/400; invalid state `#e53e3e`/`#fff5f5`/`#c53030`; type sans-serif 36px/14px base; then F2 ratified dropping the stand rows' PROVISIONAL marker — colour pairs and sizes the contrast re-read (large-text threshold) must cover.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations; load Tab sequence; Cells grid; stand_contrast over load and Cells; boot lint) — re-pointed `stand_contrast.rs`/`stand_lint.rs`/`stand_load_page.rs` citations; load page walks `LOAD_TAB_ORDER` with `load-fetch` taking Enter/Space; `cells-grid` one Tab stop named "Cells", no role, `CELLS_TAB_ORDER`, focus outline `#3a5ce5` (founder's F13; editor-click pointer path kept on the operator's ruling); `stand_contrast` reads load page and Cells in both modes; all four lean tasks lint 0 at boot via `in_closed_select` — the engine-debts surface (multiple select, optgroup, checkbox/radio change) this chunk re-asserts against, and the rows `contract/stand-element-ids/cells-grid`, `load-fetch`, `decision/stand-lints-clean-at-boot`.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (open list bullet; citations; scrolling exception; lint `in_select_box`) — engine-painted select list opened by click/Enter/Space, focus stays on the select, Tab chooses and closes before focus moves, Tab sequence holds open and after close (founder's F1–F4, F7); open rows read own bounds, back at the select's rect after close; `stand_contrast.rs` citations re-pointed; scroll/visible-region exception for open rows; lint skips `in_select_box` (F10) — origin of `decision/open-list-rows-own-bounds` and the `except` clauses of the scroll and visible-region rows; the SC 2.4.3 "list closing" focus move and the list's engine colour constants the re-read covers.

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (screens' citations; graph verb) — re-pointed task-file and stand-colour citations after `use_graph_signal` lines, no markup moved; `graph` verb raised the verb table to thirteen — part of `contract/escher-session-command/form`'s count history.

@history_item n=25 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (tab-order citations; actions verb) — load page's and Cells grid's tab-order citation moved to `tests/blitz-tests/tests/common/mod.rs:104-109` after `ACTIONABLE_AT_BOOT`; `actions` verb with its `{"nodes"}` answer (ids, owner, keyed, verbs, never a value) — origin of `contract/escher-session-actions/answer`; the shared tab-order constants the keyboard re-read reads.

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (coverage role a TaskShell consumer; coverage role on the cli) — the `coverage` role boots through the same boot adding no markup, a held instance a fresh boot in both layout modes — completes `decision/session-roles-share-stand-boot`'s host/mcp/coverage list.

@history_item n=27 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (121 rows: decision 21, contract 99, absence 1), each verified against the code; the source of every picked handle this chunk binds.

@history_item n=28 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified graph verb; ratified coverage role entries) — the founder ratified the graph verb and both coverage-role entries, text unchanged; the session-roles and command-form rows stand on his word, not a delegate's.

@history_item n=29 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/escher-session-command/form` to fifteen verbs; `+ contract/escher-session-layout/answer`; the layout answer's id echo an exception to no-caller-input-echoed — the current command-form and layout rows; the scope bars adding any verb or field.

@history_item n=30 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirm behind the layout-on-the-carriers rows; `contract/escher-session-layout/answer` and the fifteen-verb form stand on his word.
