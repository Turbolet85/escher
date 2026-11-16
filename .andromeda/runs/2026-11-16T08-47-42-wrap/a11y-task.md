@task_prose part=session
The session made each acting call's answer (click, type, press, advance, scroll) carry a new `caused` field: what its step caused, one item per kind (panic, error, log) and level, tied to the reached handler's node id, its owner as the UI graph reads it, and its event name, with a count; absent when nothing was caused; no message, payload or error text crosses.
The Dioxus document's reached-handler record gained a per-step drain (take_step_reached, each handler once, first-reached order) and a read of the root error boundary's mark (root_error_mark); a session now records reached handlers on every held session from its start.
escher-telemetry gained a scoped capture (capture, CaptureLayer, Captured) that counts records by level during a step and prints nothing; the sink's subscriber now filters only the sink layer by its EnvFilter with the capture layer beside it unfiltered; the sink's output, its allowlist and the panic hook are unchanged.
The Dioxus framework targets (dioxus*, generational_box, warnings) count only at warn and error; every other outside target counts at every level; a handler Err taken by the root default boundary replaces the app with the boundary's own rendering of the error's Display, which reaches the screen as content, never `caused`.
A run-path panic is caught and the session keeps answering; exploration reads an answer carrying a panic item as a Panicked path whose kept check now holds the panicked step.
The `caused` crossing (closed words, ids, counts) stands on the founder's own word; the text of errors for every source is owed to a follow-up chunk.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: new — the scoped log capture during an acting step: counts records by target and level, framework targets at warn and error alone, prints nothing; a process-wide dispatcher pair
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited — the telemetry sink's subscriber init: the EnvFilter now filters the sink layer alone, the capture layer beside it; re-exports the capture
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: new — the Dioxus document's read of the root error boundary's mark (root_error_mark), telling a new handler Err from none
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited — the Dioxus document crate's module list and re-exports (the root error mark)
@task_file n=5 path=packages/dioxus-native-dom/src/reached.rs
what: edited — the Dioxus document reached-handler record: gains the per-step list and its drain take_step_reached
@task_file n=6 path=packages/escher-driver/Cargo.toml
what: edited — the session library's manifest: takes escher-telemetry as a dependency
@task_file n=7 path=Cargo.lock
what: edited — the workspace lock file: one dependency line for the new edge
@task_file n=8 path=packages/escher-driver/src/caused.rs
what: new — the caused item of an acting answer: kind, level, handler id, owner, event, count, and their closed words
@task_file n=9 path=packages/escher-driver/src/execute.rs
what: edited — the driver's acting step and settle: drains the reached handlers, reads the root mark, runs step and settle inside the capture under a panic catch, builds `caused`
@task_file n=10 path=packages/escher-driver/src/session.rs
what: edited — the in-process session: start records reached handlers on every held session
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: edited — the driver's verb table: the `caused` field last on the five acting verbs, its help stating its limits
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: edited — the verb table's checks: acting verbs' field rows and the field kinds table
@task_file n=13 path=packages/escher-driver/src/json.rs
what: edited — the answer's written form on the carriers: `caused` after the verb's other fields, only when non-empty
@task_file n=14 path=packages/escher-driver/src/explore.rs
what: edited — exploration: a run-path panic caught, an answer with a panic item read as a Panicked path
@task_file n=15 path=packages/escher-driver/src/fixture_tests.rs
what: edited — the driver's fixture checks: matches on acting outcomes gain the `caused` field
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: edited — the session recording checks: an acting outcome gains the `caused` field
@task_file n=17 path=packages/escher-driver/src/replay.rs
what: edited — kept-check replay: an acting outcome's `caused` compared on replay
@task_file n=18 path=packages/escher-driver/src/lib.rs
what: edited — the session library's root: re-exports Caused, CausedKind, CausedLevel
@task_file n=19 path=tests/blitz-tests/tests/stand_act_errors.rs
what: new — the stand check of `caused`: panic, handler Err on its own session, logging click, type and press Enter, both layout modes, the MCP server in process, kept checks, the command span's eight fields
@task_file n=20 path=tests/blitz-tests/tests/stand_explore.rs
what: edited — the stand exploration check: a panicked path's kept check holds the panicked step
@task_file n=21 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the stand checks' shared session helpers: matches on acting outcomes gain `caused`
@task_file n=22 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check of the driver's command spans: an acting outcome match gains `caused`
@task_item n=1 part=touches
The Dioxus document reached-handler record (reached_handlers, record_reached): it gains a per-step list and its drain take_step_reached, and a session records on every held session from its start; reached_handlers itself is unchanged.
@task_item n=2 part=touches
The acting verbs' answers (click, type, press, advance, scroll) gain an optional `caused` field; any row stating an acting answer's fields, or that an answer carries no handler-level reading, is reached.
@task_item n=3 part=touches
Handler coverage and listeners read from the vdom walk: `caused` ties an item to the first reached handler's node id and event, the same handlers the reached record holds.
@task_item n=4 part=touches
UI graph carries no values / one node per snapshot node: a `caused` item's owner is the owner id as the UI graph reads it; it carries ids and closed words, no values.
@task_item n=5 part=touches
MCP tool results: the MCP server's acting tool results carry the same `caused` member, from the verb table.
@task_item n=6 part=touches
The obs log format described in section 3's overview: the sink's EnvFilter now filters the sink layer alone and a capture layer sits beside it, counting but printing nothing; the printed line classes and allowlist are unchanged.
@task_item n=7 part=touches
Harness settle and the changed set: an acting step's settle now runs inside the capture under a panic catch, a panic in the settle running it once more; what settle leaves in the changed set is unchanged.
@task_item n=8 part=touches
Keyboard activation (Enter and Space activate as a click): press Enter on a focused logging button is now checked to answer `caused` for its click handler.
@task_item n=9 part=touches
Password never revealed and password value masked, as rules of what text crosses an answer: `caused` crosses no message, payload or error text; a handler Err the root boundary takes reaches the screen and the snapshot as the boundary's rendering of its Display.
@task_item n=10 part=touches
Stand checks repeated three rounds: a new stand check joins the repeat leg by its stand_ prefix, no leg script changed.
@task_item n=11 part=touches
Session library always enables accessibility: the session library gains a dependency on escher-telemetry; its accessibility feature is unchanged.
@task_fact n=1 slug=keyboard-navigation
fact: The Dioxus document's reached-handler record also keeps a per-step list, drained by take_step_reached: each handler reached since the last drain, once, in first-reached order; empty with the record off.
@task_fact n=2 slug=keyboard-navigation
fact: The Dioxus document reads the root error boundary's mark (root_error_mark): a new mark after a handler's Err the root default boundary takes, none where the app's own error boundary catches it.
@task_fact n=3 slug=screen-reader-support
fact: An acting call's answer carries `caused` only when its step caused something: one item per kind (panic, error, log) and level, with the first reached handler's id, its owner and event, and a count; no message, payload or error text; Dioxus framework targets counted at warn and error alone.
@task_fact n=4 slug=screen-reader-support
fact: A handler Err the root default boundary takes replaces the app with the boundary's rendering of the error's Display, read by the snapshot as ordinary screen content; `caused` holds only an error item.
@task_item n=1 part=untouched
Accessibility tree roles, names, states and hidden or inert content: no role mapping, ARIA read, name precedence or exclusion rule moved.
@task_item n=2 part=untouched
Snapshot node keys, the snapshot text form, the snapshot diff and select list readings: no node key, state token or diff entry added.
@task_item n=3 part=untouched
Visual design: contrast, the lint's contrast finding, select list colours and motion, scrollbars and zoom are not reached.
@task_item n=4 part=untouched
Focus rules, Tab order, select list keys, range input keys and the seven_guis CRUD list and Cells grid keyboard contracts: no key handling or focus move changed.
@task_item n=5 part=untouched
The platform accessibility adapter, AccessKit tree building and the accessibility feature defaults: no adapter or feature changed.
@task_item n=6 part=untouched
The actions verb and the driver's refusal causes: no verb, cause, session error or node key added; the actions answer is unchanged.
@task_item n=7 part=untouched
The a11y CI leg and the a11y CI gate, cognitive accessibility and the a11y tier: not reached.
