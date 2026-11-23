@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set up escher-telemetry's stderr sink and the allowlist scrub. Engine prefixes (`blitz` and others) print only `node_id`, `status`, `waiting_nodes`, `property` and the `log.*` fields, and the content-named fields are redacted. This is where `decision/engine-target-safe-fields` and `decision/tracing-for-telemetry` come from. Any log site near the blitz-dom events change (F2 A) is judged under this scrub.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand `seven_guis::stand` and its in-process checks install no subscriber, print nothing and read no env var. This is the origin of `decision/headless-stand-no-sink`. Circle Drawer's new in-process stand boot in `examples/seven_guis/src/stand.rs` (F1 A) has to keep to it.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the headless-stand silence census was widened to cover the checks' shared module `tests/blitz-tests/tests/common/mod.rs`. Circle Drawer's rows move into that module (scope: `common/mod.rs`), so the census reads it too.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became a second sink installer, and the scrub's reach was scoped to what was measured. Trap recorded there: a zero read from a host that installs no sink says nothing about one that does. Circle Drawer stays in-process only in this chunk (F1 A), so no host reading of it exists.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — records from outside targets are dropped whole at every level. Rule for later chunks: a target is admitted only together with a scrub rule for it. This is the origin of `decision/sink-target-allowlist`. A new log target in the engine events change or the widget's crates would need its own rule.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor is silent. Rule kept: never field a `Command`, `Call`, `ArgValue` or `Outcome` with `Debug`. This is the basis of `decision/debug-types-never-logged`. Circle state and the `data-circles` text must stay out of any `Debug`-logged type.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span as built: eight fields, entered before `validate`, and never fielding an argument, `Outcome` or screen text. `tracing` is ungated in escher-driver. A workspace build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. This is where `decision/engine-tracing-by-invocation`, `decision/escher-crates-always-trace` and the `contract/command-span/*` rows come from.

@history_item n=8 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line (one line per closed span whose target is admitted). The diff's three sizes may reach a log but its content never does. This stands behind `contract/command-span/line` and `added` / `removed` / `changed`, which a driver click on the canvas records.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — what a command prints is its answer, never a log line. Standing rule: no span field or log field is added, and the next surface adds none that holds what a call supplied or what a screen reads. This binds the canvas sample: circles and diameters cross only as answers and pixels.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot` to the closed sets and the causes `fonts-unavailable` and `off-screen` (an element shot that crops to no pixel). A shot records `verb` alone and never the image, its byte size or its dimensions. `stand_screenshot_repeat`'s re-executed child prints to stdout, and the parent reads it into assertions only. This is the origin of `contract/command-span/screenshot`, `contract/command-answer/shot`, `decision/shot-image-leaves-only-in-answer` and the refusal-cause rows. These rows bind Circle Drawer's canvas shot and its cross-OS element shot.

@history_item n=11 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — an `advance` is accepted on every session and records the acting fields with no cause (ratified by 2026-10-18-engine-debts). This is the origin of `decision/advance-accepted-every-session`. Two centre clicks with no `advance` read as a double click (F4 A), so the sample's flow depends on it.

@history_item n=12 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing. Added the at-rest Recording directory row, ratified. This is the basis of the `contract/kept-check/*` rows (`accepted`, `answer-digest`) that a kept Circle Drawer check would carry.

@history_item n=13 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is held whole as `answer`, a shot's without `png`, and masked as on every carrier. This is the origin of `contract/kept-check/answer` and `contract/recording-file/answer`. A recorded canvas shot keeps width and height only.

@history_item n=14 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the stand checks' third shared module `tests/blitz-tests/tests/fixtures/mod.rs` is silent, and the «their shared modules» clause of `decision/checks-silent` covers all three modules. New Circle Drawer stand checks that read it stay inside the census.

@history_item n=15 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the gated part: the `lint` verb runs inside the command span with no field, and a lint line holds no finding or count (F2, the founder: no new field). This is the origin of `contract/command-span/lint` and `contract/command-answer/linted`. Circle Drawer's boot lint (0 findings, both modes) is read through it.

@history_item n=16 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `stand_non_lean_tasks` joined the silence census, which this chunk extends. The workspace-built host prints the `blitz_dom::document` `focussed node` INFO line twice, and why is recorded but not established.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — several entries, all precedents for an engine change in blitz-dom events:
- Citations into blitz-dom `events/pointer.rs` and `document.rs` were re-pointed after the added lines.
- A `no site added` row holds for engine changes, including the engine's checkable `change` synthesis.
- The `focussed node` INFO line was re-read beside the focus fixup: once per focus move under the workspace build, carrying a node number only.
- The five new engine checks were recorded silent in a §6 observed-absent row, not in §3.
The precedents for this chunk: F2 A's click synthesis in `events/pointer.rs` moves cited lines, a canvas `<object>` takes focus on a click, and a new engine check of the click on a canvas needs its silence row.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — several entries:
- Citations into `events/pointer.rs` and `document.rs` were re-pointed.
- `click`'s help and `covered`'s meaning are fixed `'static` text that adds no span field.
- New checks were recorded silent, and the `session_common` reader count moved.
- A shot of an open list carries page labels as pixels. This was an engine change for every document and a ratified boundary widening.
These are precedents for canvas text painted into a shot, for an engine behaviour change across every document behind `custom-widget`, and for the stated click limit in help text.

@history_item n=19 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — the take is silent. `set_custom_widget`'s replay and the `flush_queued_mounted_events` guard open no span. Citations into blitz-dom `document.rs` and `mutator.rs` were re-pointed. This is the custom-widget path the canvas `<object>` rests on, and the event dispatch change has to stay spanless too.

@history_item n=20 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the mutation buffer holds attribute values in process memory only. Its replay re-fires `MutationWriter`'s own `trace!` calls, which print under an engine target as safe fields only. The `data-circles` attribute write is one such mutation, so its value must stay out of every log.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — several entries:
- The graph answer is never logged or fielded, and its carried set was ratified (by 2026-11-15T05-21-42-wrap).
- A page with no registered state reaches no signal registry.
- The snapshot's `clickable` reading is a closed bool that carries no page text.
This is the origin of `contract/command-answer/graphed`, `contract/graph-answer/*`, `decision/no-state-page-skips-registry` and the `clickable` rows. The canvas `<object>` reads `clickable`, and the circles leave the graph.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the actions answer carries ids, owner ids, verb words, cause words and booleans, never a value (ratified). The actions read changes nothing, and an actions line holds no node. This is the origin of `contract/actions-answer/*`, `contract/command-span/actions` and `decision/actions-read-changes-nothing`. Circle Drawer's `unkeyed_actionable` reading of 0 and its canvas's actions rest on these.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — dioxus-native-dom's reached record is written in `DioxusEventHandler::handle_event` and logs nothing. `stand_handler_coverage` is silent, and it opens Circle Drawer from Home by its card. Once F2 A makes clicks on the canvas reach the app's `onclick` / `ondoubleclick`, the reached handlers change, and that check reads them.

@history_item n=24 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (decision / contract / absence / overview), each verified against the code. This is the basis of every picked row handle that this chunk's plan cites.

@history_item n=25 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` except: a re-run child may print ids or digests to its parent. This covers digest-printing children like those of the cross-OS digest table that Circle Drawer's canvas element shot joins (`stand_canvas_cross_os`).

@history_item n=26 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `~ decision/checks-silent` and `~ decision/headless-stand-no-sink`: the layout stand check is named as an exception that installs its own recorder. Precedent: a new check that installs any subscriber has to be named in both rows.

@history_item n=27 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `~ decision/headless-stand-no-sink`, `~ decision/driver-library-installs-no-subscriber` and `~ decision/driver-emits-no-event`: acting steps run under escher-telemetry's scoped capture, which prints nothing. A driver click on the canvas runs under that capture.

@history_item n=28 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — several row changes:
- `~ decision/checks-silent`, `~ decision/headless-stand-no-sink` and `~ decision/driver-library-installs-no-subscriber` (chaining panic hook), and `~ decision/driver-emits-no-event` (the capture keeps the step's records and panics).
- `+ decision/engine-sites-fixed-messages`: engine warn and error sites write fixed messages with named fields.
- `~ decision/escher-target-content-named-set`: `password` added.
Any engine log site added near the click synthesis must follow fixed-message form.

@history_item n=29 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/command-span/verb` (fifteen words) and `~ contract/command-span/settled` (layout reads verb alone). These are the current forms of the picked command-span rows. The chunk adds no verb (F4 A), so the counts must not move.

@history_item n=30 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/checks-silent` except «the order page logs»: a stand page that adds a log site is named as an exception. Also `~ contract/escher-session-exit-codes/2`: the usage line is built from the served task names. Circle Drawer is not served (F1 A), so the served list and that line do not move.

@history_item n=31 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/command-answer/graphed`, `~ contract/graph-answer/shape`, `~ contract/command-span/graph` and `~ contract/graph-answer/content`: graph part reads. These are the current forms of the picked graph rows that Circle Drawer's graph reads rest on.

@history_item n=32 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of these claims, so a change there is made by hand. This is the origin of `decision/capability-statement-updated-by-hand`. The chunk edits that statement for the canvas sample and its stated click limit.
