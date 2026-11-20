@history_item n=1 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows, each verified against the code; several picked rows carry their proof by test name in their `why` (e.g. decision/open-select-list-hit-first pinned by `a_button_under_the_list_is_refused_covered` and the covered fixture in `stand_actions`; decision/select-list-opens-without-motion pinned by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance` in the `stand_act_popup` driver tests; decision/default-ua-sheet-via-default-config pinned by `default_config_uses_the_default_sheet`) — a helper refactor must keep every such test's name, file and assertion so these rows' pins still resolve.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the seven_guis headless stand registers the bundled DejaVu Sans for every generic, system fonts off, through `HarnessOptions.font_ctx` (`build_single_font_ctx(DEJAVU_SANS)`), so layout is the same on dev host and CI — a shared boot helper must keep each stand boot on that font context, or text measurement (and the screenshot rows) shift.

@history_item n=3 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's font registration is now held in architecture §Standard Contracts → Headless stand beside `font_ctx()`, not in design-system; a helper that moves the boot rests on that architecture contract, not a typography row.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — origin of the picked screenshot rows: a driver `screenshot` whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted, the stand's bundled font passing; the cli answer line `{"width","height","png"}` and the MCP image block — helpers that boot fixtures for screenshot checks must keep the font context that decides pass vs `fonts-unavailable`.

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves the app's own time through the step a session's caller hands it (on the stand whole Timer ticks), `click`, `type`, `press` leaving the clock alone; measured on the stand's Timer and a running-animation fixture — the precedent behind the picked driver-verbs-and-virtual-clock rows; a shared act helper must not add or drop an `advance` around a verb.

@history_item n=6 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — replaced the two-clock reading: the harness drives animation from the `VirtualClock` its documents read, moved by `tick`, `advance_time` and the driver's `advance` by exactly its `ms` together with the app's time; click/type/press leave it; after a driver `scroll`, `animating` reads true until 700 ms pass — the current form of decision/driver-advance-moves-one-clock and the advance/click/press/type contract rows.

@history_item n=7 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — ratified one-virtual-clock's `advance` clause (founder, 2026-10-18), which is why decision/driver-advance-moves-one-clock reads «Ratified»; the clock contract the stand checks assert is settled, not open to a test-side reinterpretation.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle returns `Settled` with `animating` reading the document's flag, never advancing the clock; the caller holds the clock — a shared act-and-read helper reads the settled answer as it is and must not wait or tick to make it settle.

@history_item n=9 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait (at most `LOAD_WAIT`, 5 s wall) moves no clock — the `VirtualClock` stands still while a step waits on a load; stand checks on the `load` task depend on it, so a helper folding their boot/act must not move the clock around the wait.

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — escher-session served six tasks (Cells and the load page added in `TaskShell`), the bundled font for every boot incl. `boot_cells` and `boot_load`; trap: after a scroll the overlay scrollbar takes a pointerdown in its band, so a driver `click` on a Cells cell there takes an `advance` past the fade first, and a `click` on `load-fetch` waits out `LOAD_ANSWER_DELAY` (200 ms) — per-check setup quirks a shared helper must preserve, not normalize away.

@history_item n=11 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — the escher-session-tasks rows moved six → seven tasks and contract/escher-session-tasks/order was added (`order` the seventh, in the usage line's `apps:` and the MCP `start` enum) — the latest change to the picked task rows; `stand_order_served.rs` (one of the first candidate clone family) is that task's check, so a helper shared with it must keep `order`'s boot as served.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` prints one uncoloured JSON line per command (answer, refusal or session error's object, schema's words), no ESC byte, no terminal test; exit 0 accepted · 1 refused · 2 usage · 3 session error — the source of decision/driver-output-for-agents, decision/escher-session-agent-read-surface and the stdout/exit-status/exit-code rows; a shared reader of answers/refusals must keep asserting these same shapes and statuses.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC lines only, no ESC byte asserted on every reply by the shared `Speaker`, no terminal test — an existing shared test helper precedent on the cli surface that the new helpers sit beside.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — the exit status's four classes are read from the schema's `EXIT_STATUSES` table and a session error's object carries its fixed `remedy` — a refusal/exit reader moved into a helper reads the same table, not a restated copy.

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `record-dir-not-private` and `recording-failed` join the exit-3 session errors, no new status — the exit-status class set a shared refusal-reading helper must still cover unchanged.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — origin of contract/escher-session-commands/snapshot: `{"text":…}` whole, `{"text":…,"part":n,"parts":m}` in parts, status 0; over `mcp` a paged result is the JSON line — the snapshot-text read a helper takes over must keep both shapes.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line) — origin of decision/actions-share-the-verbs-checks and the actions-line rows (nodes, content, stale, actions-answer): the answer decided by the same `validate` and `check_target` every id-taking verb meets, so a listed cause equals the call's refusal; the line carries ids, owner ids, verb words, cause words and booleans only — stand checks comparing an `actions` cause to the call's refusal are exactly the act-then-read-refusal setup this chunk folds; a helper must keep that pairing and add no value or typed text to messages.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `escher-session graph` answers one uncoloured JSON line on socket, command line, `mcp` and in a kept check, carrying names, closed flag words and booleans only, never a value or typed text — the sibling command whose answer reads stand checks share setup with; same content rule as the boundary on assertion messages.

@history_item n=19 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer and layout-arguments beside actions and graph (a missing id refused `malformed`, not a usage ending) — the most recent escher-session-commands rows; their checks are candidates for shared boot/act/read helpers.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk; the open list's wheel and key, and no motion) — origin of decision/open-select-list-hit-first (`hit_with_scrollbar` answers the list first, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` read it; a button under the list is refused `covered`) and decision/select-list-opens-without-motion (a screen read after a step reads the new state with no `advance`) — the stand popup/actions checks pin both; a helper must not insert an `advance` or change the act order those checks rely on.

@history_item n=21 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once through `DocumentConfig::default()` (upstream #1189 removed `DioxusDocument::new`'s own call) — origin of decision/default-ua-sheet-via-default-config; a shared boot helper must build documents through the default config, not add a sheet itself.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed by the chunk's line moves) — design-system citations name test files' lines (`stand_cells_served.rs` among them), re-pointed when that chunk added lines there — trap: moving setup out of stand check files shifts their line numbers, so any design citation into a touched test file must be re-read at wrap.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the citations re-pointed past the take) — re-pointed citations including into `select_popup.rs` (−4, one range ending sooner) and `settle.rs` — further precedent that design citations reach into test files, so a test-only refactor still owes a citation re-read.
