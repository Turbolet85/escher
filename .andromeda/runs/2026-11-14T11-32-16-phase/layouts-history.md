@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: it skips Home and mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans (row `decision/headless-stand-pinned-viewport`). A session started in a fixture state must still read as this mount at this viewport; a state reached inside the boot must not turn into a second way into TaskShell.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls and displays their author ids (the `contract/stand-element-ids/*` rows), adding no class, style, wrapper or order, in a markup-only gate. A fixture state's screen has to read the same ids as the state reached by hand. Any state table or initial-state hook the chunk adds must not add markup.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — keyed CRUD rows by `{person.id}`: fixture people 0–2, and Create assigns ids from 3. A CRUD fixture state (for example "after N Creates") must give the same person ids as Creates run by hand from boot, or the first-screen comparison fails. This bears directly on whether a state is seeded as an initial state or reached by steps.

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the six stand inputs (`crud-filter`, `crud-name`, `crud-surname`, `timer-duration` by label `for`; `flight-start`/`flight-return-date` by `aria-label`) by attributes alone. These names are part of the snapshot text that a state's first screen is compared against.

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows read author ids `crud-person-{person.id}` (row `contract/stand-element-ids/crud-person-id`), from the founder's rule that every actionable element reads an author key. A seeded CRUD state's rows must carry the same keys as rows created by hand.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — made the session host the third consumer of the `task_in_shell` mount. It boots only through `seven_guis::stand` and adds no markup; a held instance reads as a fresh boot, and the mount and viewport hold after commands (row `decision/held-instance-reads-as-fresh-boot`). Starting in a state changes what "fresh boot" means for that session, so this decision's wording and its guard `stand_session_fresh` are what the chunk extends.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the host gained lines that hand the Timer's session its time step at boot. This is the precedent for a boot-time, app-supplied parameter on the served boot, next to the "app-supplied initial state" option the scope leaves open. It also notes that session_host citations move when the boot grows.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — measured the CRUD list's state after twelve Creates: the last row is refused `off-screen` and a driver `scroll` brings it in. This is one of the states stand checks reach today by clicking from boot, and a candidate fixture state for closing the observed gap.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the command form to `escher-session <verb> [arguments] --session <dir>`, with `start <task>`, the host role `serve <task>` spawned by `start`, and four endings: 0 accepted, 1 refused, 2 usage, 3 session error. The former form reads `unknown-verb` (rows `contract/escher-session-command/form`, `contract/escher-session-exit-codes/*`, `contract/escher-session-refusals/unknown-verb`). `start {task} --state {name}` extends this, and `start` must pass the state on to `serve`. An unknown state must take one of these endings with nothing started.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — measured `crud-list` after 12 and 14 Creates, and Home's Cells card, which lies below the fold and is hit only once scrolled. A standing check that clicked from boot went red here. These are clicked-to states that a fixture would replace, so their readings set the by-hand side of the proof.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role over stdio, local to the user with no listener (row `decision/mcp-server-local-stdio`). It boots one task in process on a `start` tool call through the same boot and adds no markup (row `decision/session-roles-share-stand-boot`). The crossing is the founder's ratified one of 2026-10-10, the closed-value FIELD rule the scope cites. The `start` tool's new optional argument sits on this carrier, and whether an argument is a widening is the P4 fork.

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` and `describe()`'s seven-key fixed-table answer. `--session` appears exactly once on every command except `help`, the usage line is built from the verb table, and each MCP tool description is the verb's help plus its result fields (rows `contract/escher-session-help/answer`, `contract/escher-session-mcp/tools`). A `--state` argument on `start` moves `describe()`, the usage line and `start`'s tool schema.

@history_item n=13 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes focus and Arrow/Home/End keys that select rows and fill `crud-name`/`crud-surname`. Selection and focus are reachable CRUD states whose first screen a fixture would have to reproduce, focus included.

@history_item n=14 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — named `crud-list` "People" by `aria-label` alone (row `contract/stand-element-ids/crud-list`). This is part of the CRUD snapshot text a seeded state must match.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the non-lean tasks (Cells `cell-{A..Z}{0..25}`, `cell-input`) with `id` lines alone, so `unkeyed_actionable()` reads 0 on all seven tasks (rows `decision/every-stand-control-keyed`, `contract/stand-element-ids/cell-*`). A state that opens Cells' editor or a dialog must still read every actionable element keyed.

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>` on `start`, the host role and `mcp`. It writes owner-only `check-{n}.json` (0600 in a 0700 directory) and has two session-error endings, `record-dir-not-private` and `recording-failed`. `start` checks the directory before it spawns, and the change was a founder-ratified widening (rows `decision/session-recording-replayable-check`, `contract/escher-session-record/*`). A session started in a state under `--record` must keep that state in the check; this is the scope's P4 fork.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, answer digest and masked answer, and is replayed by `Check::replay`. This was a founder-ratified change to at-rest content (F3 B), which is the precedent for the fork on adding the state name to the check's closed key set and `Check::from_bytes`. Nothing the caller typed is written back (row `decision/no-caller-input-echoed`).

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — measured the stand after twelve Creates: `crud-person-14` reads `clipped` inside `crud-list`, which agrees with the `off-screen` refusal. This is a clicked-to state whose lint and snapshot readings a fixture state would have to reproduce.

@history_item n=19 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting tools state that a load in flight is waited for up to 5 s, as fixed table text. This bears on any load-page state (for example "Loaded") and on whether a state reached by steps inside `start` must wait out `LoadNet`.

@history_item n=20 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot --part`, answering `{"text"}` or `{"text","part","parts"}` (row `contract/escher-session-snapshot/answer`). The chunk's proof compares snapshot text by digest or whole, so a large state's screen (such as Cells) may read in parts.

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — set the six served tasks `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`. Lean tasks boot by their boot, Cells by `boot_cells` and load by `boot_load`, and the `start <task>` usage `apps:` list and the MCP `start` enum name all six (rows `contract/escher-session-served-tasks/*`). It also made the flight booker's mode a select (`flight-type` with its options). A per-app state table attaches to these six boots and the same enumerations.

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines) — the load page holds `load-fetch` and `load-done`, and `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out (row `decision/load-page-only-network`). Only the load page's boot has a network provider, so a load-page state must not add one elsewhere.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence; the Cells grid) — `load-fetch` takes Enter and Space, and `LOAD_TAB_ORDER` lives in the tests' shared `common/mod.rs`. `cells-grid` became one Tab stop and keyboard region named "Cells" (row `contract/stand-element-ids/cells-grid`), with `current`, Enter opening the editor, and `CELLS_TAB_ORDER`. Cells' editor-open and `current` states are clicked-to states a fixture may name. The shared test module is where states reached by hand are stated.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — added the flight booker's open select list. While it is open each row reads its own bounds in the snapshot and diff, and after a close the rows read back at the select's client rect (row `decision/open-list-rows-own-bounds`). The boot snapshot's `ComboBox "Flight type"` with two `ListBoxOption`s is unchanged (row `decision/flight-type-select-boot-reading`). A flight-booker state's first screen must read the select as the by-hand path does, and an open list is a state reached only by clicking.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — added the `graph` verb (thirteen verbs) and every task's signals registered through `use_graph_signal`, answering registered state names and never a value. The registered signals are a likely seam for an app-supplied initial state. Re-pointed schema and task-file citations show how adding to `schema.rs` moves cited lines.

@history_item n=26 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added the `actions` verb: fourteen verbs, ten instance verbs, MCP tools in table order with `help` last (rows `contract/escher-session-command/form`, `contract/escher-session-mcp/tools`). It also added `ACTIONABLE_AT_BOOT` to `tests/blitz-tests/tests/common/mod.rs`, which moved citations. An argument added to `start` changes no verb count but moves `start`'s help and tool schema, and new shared-module state tables move the cited lines again.

@history_item n=27 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, both entries) — added `escher-session coverage <app> <check>...`. Each check is read bounded and parsed by `Check::from_bytes`, then replayed on a fresh boot through the same boot `main` hands to the command line, adding no markup (rows `contract/escher-session-coverage/*`, `decision/session-roles-share-stand-boot`). The ratification is PROVISIONAL (delegate overseer). A check kept from a session started in a state must replay from that state here as well; the scope names `coverage.rs`, a check's replay boot.

@history_item n=28 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (121 rows), each verified against the code at edd0cfaea7f0, with fates approved by the founder. The picked rows' handles and their guards (`stand_session_fresh`, `stand_load_page`) date from here, so the chunk amends these rows by handle.
