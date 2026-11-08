@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — registered the stable element id grammar on the Dioxus DOM bridge (no `NodeId`, `ElementId` or `ScopeId` in an id); the coverage answer must key handlers on this grammar's ids (with the graph's optional `owner`), never on a `NodeId`.

@history_item n=2 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the anchored tier and `unkeyed_actionable()`; standing trap: `UnkeyedActionable.node` is a process-local `NodeId`, so the first wire form must not carry it — the same rule binds any coverage record or answer built beside the actionable readers.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session registered) — escher-driver depends on no app; standing rule that the first request carrying an id, a name or screen content is a new crossing question — the root of the boundary a coverage verb, argument or result field would widen.

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built the one `'static` verb table and closed `Cause`; rule for later chunks: a later entry adds its verbs to `VERBS`, never a second table; `Command`/`Call`/`ArgValue` print ids under `Debug`, so nothing fields them — a coverage verb follows this.

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` the one entry point, validating first; rules: nothing public runs a `Command` not from `validate`, nothing resolved outlives a call, nothing fields an `Outcome` — every coverage read and replayed step must go through it.

@history_item n=6 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the sixth verb `scroll` ratified as a verb-set widening by the founder; restated rule "a verb joins `VERBS`, never a second table"; added the session's record of ids (`SeenIds`), which a coverage read must decide whether to feed.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span) — every call to `Session::run` leaves one hand-built `command` span (`info_span!`, no `#[instrument]`, ungated `tracing`), with a closed list of never-fielded values; a coverage verb records `verb` alone and fields none of its list.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries a call; nine verbs, written forms, `command_line`) — the ratified crossing that ids, names and values leave by design and the next surface asks again; written forms hand-written in `json`, no ending writing back caller input; the CLI verb set read from `VERBS` alone — the surfaces a new verb moves.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — `serde_json` admitted as a reader only (founder's «serde_json, только чтение»), the `tool_call` span with `tool` and `ending` only; standing rule: no span field holds what a call supplied or a screen reads; `tools/list` lists `VERBS` — a new verb moves `mcp/listing.rs`.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — `describe()` writes counts from the tables and the fixed `'static` table text rides the ratified carriers (ratified with the rule "text from a page, a call or a screen, a new carrier or a new exit still widens"); a coverage verb moves `describe()`'s count and its help text must stay table text.

@history_item n=11 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (keyboard activation) — Enter/Space dispatch a queued Click so the app's handlers run before `handle_click`; the exercised record must count that click handler as reached where the bridge dispatches, never inferred from the driver verb.

@history_item n=12 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — a range input fires one `input` per change and none for a no-change move; the coverage count must follow events the engine actually dispatched.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — `time-unavailable` retired, `CAUSES` closed at eight (ratified 2026-10-18); the coverage read adds no cause, any refusal one of the eight.

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check: launcher-named `--record <dir>`, owner-only `0700`/`0600`, never named by a tool call, `Check::from_bytes` strict, unix only (`recording_into` `Unsupported` off unix); a ratified boundary widening — a verb or argument naming a check path would be a new input class; the replay test records into this medium.

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole; the replay API) — answers kept whole beside digests, `Check::replay` → `Replay`/`Change` (`step`, `keys`), run through `Session::run` alone, `serde_json` reading the live line; the committed Counter checks the one owner-only exception; same-host limit: a check compared across OSes holds no `screenshot` step — the exact API and trap the owed `actions`-step replay test rests on.

@history_item n=16 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — `recording::tests` moved to `recording_tests.rs` under `#[cfg(all(test, unix))]`, `off_unix_tests` for windows; a replay test placed there is witnessed on linux/macOS only.

@history_item n=17 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — stand checks share tables and helpers through non-target modules under `tests/` (one behaviour per test-target file); the convention a coverage stand check and replay test follow.

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — precedent for a no-argument read verb (one argument refused `malformed`), a no-step outcome that dispatches and settles nothing, recorded `verb` alone on the span, counted into `describe` — the pattern a coverage verb would follow.

@history_item n=19 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — no-step verbs wait on nothing; a kept check whose app meets a load can replay `settled` differently (recorded, not established) — the replay test should record on an app with no load.

@history_item n=20 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a select's choice dispatches `input` then `change` through `apply_select_choice` only, a no-change choice firing nothing; the exercised record must reflect exactly those dispatches.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (change on a checkbox and a radio) — the engine dispatches `change` after `input` in `handle_click`'s checkbox/radio arms (radio only when not checked before); the coverage record must count that (node, `change`) as reached.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — paging added only to `snapshot` (`part`, `MAX_PART`) as a ratified widening; precedent that paging another answer is a new argument kind, while `graph`/`actions` stay unpaged under the answer bound.

@history_item n=23 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat`'s named `--test` list beside the `stand_*.rs` glob, pinned by `test_stand_repeat_leg_runs_three_rounds`; a new non-`stand_` target the chunk adds must be named there.

@history_item n=24 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — created `docs/capabilities.md` and the capability-statement pin over `VERBS`, causes, session errors, node keys and exit statuses, plus README's «the {word} driver commands as tools»; a new verb moves the statement, README and pin together.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the headless stand serves Cells and a load page) — `seven_guis::stand` boots the lean tasks, Cells and the load page in the pinned 800x600 Light viewport, `escher-session` over six served tasks; the "on the stand" ground for the coverage acceptance and replay test, the load page the one that meets a load.

@history_item n=26 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (row helpers; `stand-repeat` held jointly) — the MCP listing test and sweep test assert rows through private `#[track_caller]` helpers naming row and index only; `stand-repeat` membership held jointly by the leg pin and the sweep's `contains` check, whose reach gap is carried on «Stand requirement sweep» — a new verb row and any new target meet both.

@history_item n=27 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the README's pinned line; default branch) — the pinned phrase «the … driver commands as tools» sits at a cited README line and the front page renders from `build/escher-0.3.0`; a verb count edit moves that line and its citations.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's checks and counts) — `stand-repeat`'s named targets outside the prefix reached seventeen with their pin; the current count a new named target extends.

@history_item n=29 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component and a held signal's readers; listener attributes over-report) — the owner walk reads through Dioxus 0.7.10 public API, hook-local `use_signal` state unreachable without a fork-level change; the bridge's listener mark over-reports while the vdom walk reads current listeners exactly — the coverage record must use public API and the exact reading, never the mark.

@history_item n=30 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (buffered render contract; hazard closed; limit and first-poll paths; witness) — the bridge renders into `MutationBuffer` unborrowed and borrows once to replay; a task polled in `render_immediate`'s `Work::PollTask` arm is the one path meeting the borrow; witnesses are the probe's `mounted_data_in_a_task_a_key_handler_spawned_acts_*` — a record written on the event path in `dioxus_document.rs` must take no document borrow there and keep these green.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers; the `graph` verb) — `graph` ratified as a widening carrying stable ids, owner ids, component, state and handler event names, flag words and booleans, never a value, typed text or `NodeId`, recorded `verb` alone; `VERBS` 13, `FieldKind` 12 — the carrier content the coverage answer may not exceed, and the verb-add precedent.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — `{owner}///{key}` reserved via `VIRTUAL_KEY_SEPARATOR` / `virtual_node_id`, no served id holding it; the coverage key must stay on graph id + optional owner so the 0.4.0 virtual-node extension stays open.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader; the graph registry and the bridge's dispatch) — `DioxusDocument::graph` read-only under `accessibility`, `GraphNode.handlers`/`GraphHandler.event` from `element_listeners`; `unconfirmed` rides the registry's dispatch, which `handle_event` sets only where a `SignalRegistry` stands, `signal_registry` ungated, `mutation_writer.rs` byte-equal to upstream — the exact gap the chunk's premise names: no "exercised" reader holds on a registry-less app.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's `clickable` reading) — `clickable` and the graph read the exact listener reading `element_listeners`, `mounted` included, never the listener mark; the coverage count must settle `mounted` consistently with the actionable rule.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (seven_guis registers its state through `use_graph_signal`; async handler writes) — every stand task registers through `use_graph_signal`, so a stand-only test cannot witness a registry-less app; an async handler's task runs inside its dispatch — both bear on where and how reached handlers are recorded.

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`stand_graph`; log needles; thirteen verbs; unpaged graph) — `stand_graph` holds the graph read-only and within bound; `graph_needles` in seven_guis' shared module with 0 needles at `trace`; the verb count moved across README, recording, lint row, CLI and cold-agent pipe; Cells' graph 293,869 bytes, «An unpaged graph» owned by `res-7cc5bi4k` — the full list of sites a further verb moves and the size bound a coverage answer over Cells meets.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer crosses the ratified carriers; fourteen verbs) — `actions` ratified as the tenth instance verb (`VERBS` 14, `FieldKind` 13 `ActionNodes`), its answer kept whole in a kept check, `keyed` PROVISIONAL — the shape the owed replay test compares key by key and the current counts a coverage verb would move to fifteen.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule and `actionable_nodes`) — crate-private `Readings`: focusable, an interactive role, or current listeners holding an event other than `mounted`; `unkeyed_actionable`'s listener leg moved off the mark — the one-rule read-only form the coverage reader should follow.

@history_item n=39 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read and the shared per-verb target check; span and needles; unpaged answer) — `Session::actions` one snapshot and one map per read, `check_target`/`target_of` shared, the read changing nothing; `actions_needles` and 0 needles at `trace`; «Accepted is not acting» and «An unpaged actions answer» (`res-7cc5bi4k`) — the answer the replay test re-measures and the no-step, no-log pattern a coverage read copies.

@history_item n=40 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (`stand_actions` and the stand's shared modules) — `ACTIONABLE_AT_BOOT` in `common`, readers `common` 23, `session_common` 34, `fixtures` 32; the reader counts a new Session-holding stand check moves.
