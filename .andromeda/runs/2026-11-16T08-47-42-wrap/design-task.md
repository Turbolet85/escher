@task_prose part=session
An acting call of escher-session (click, type, press, advance, scroll) now answers what its step caused: a new caused result field, last among the verb's fields, written only when non-empty.
Each caused item carries a closed kind word (panic, error, log), a closed level word (error, warn, info, debug, trace), the reached handler's node id, its owner id as the UI graph reads it, its event name and a count; no message, payload or error text crosses.
A panic in an acting step's handler is caught and the session keeps answering; a panic while settling runs the settle once more and a second reads busy render; an exploration ends a panicking path Panicked and its kept check holds the panicked step.
Log records from the Dioxus framework targets (dioxus, generational_box, warnings) count only at warn and error, every other target outside the engine at every level; an Err the app's own error boundary catches leaves no mark.
An Err the default root boundary takes replaces the app on screen with the error's Display, crossing as screen content and never in caused; the founder's word lifted the provisional mark on the closed words, the message text left to a follow-up.
No verb, session error cause, exit status, node key or command span field was added; the telemetry sink prints what it printed.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: added — the telemetry crate's scoped log capture: counts the records an acting step caused by level, holding no message, field or target, through one process-wide dispatcher pair
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited — the telemetry crate's init: the sink's subscriber, its EnvFilter now over the sink layer alone with the capture layer beside it; the crate doc
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: added — the root error boundary mark: reads whether an Err reached the app's root default error boundary
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited — the dioxus-native document crate's module list and re-exports
@task_file n=5 path=packages/dioxus-native-dom/src/reached.rs
what: edited — the reached-handler record: the handlers an event reached, now also drained per step in first-reached order
@task_file n=6 path=packages/escher-driver/Cargo.toml
what: edited — the driver crate's manifest: its dependencies, now the telemetry crate
@task_file n=7 path=packages/escher-driver/src/caused.rs
what: added — the caused result item: its closed kind and level words and how a step's reading becomes caused entries
@task_file n=8 path=packages/escher-driver/src/execute.rs
what: edited — the driver's acting step and settle: runs the step under capture and a panic catch, answers caused on the acting outcomes
@task_file n=9 path=packages/escher-driver/src/session.rs
what: edited — the held session's start: the reached-handler record switched on for every session
@task_file n=10 path=packages/escher-driver/src/schema.rs
what: edited — escher-session's verb table: each verb's result fields and help, carried to help and the MCP listing
@task_file n=11 path=packages/escher-driver/src/schema_tests.rs
what: edited — the verb table's checks: the acting verbs' field rows and the field kinds table
@task_file n=12 path=packages/escher-driver/src/json.rs
what: edited — the answer line's written form: each outcome as one JSON line, now with the caused member
@task_file n=13 path=packages/escher-driver/src/explore.rs
what: edited — exploration: walks the actions and ends a path Panicked, now also from an answer carrying a panic item
@task_file n=14 path=packages/escher-driver/src/fixture_tests.rs
what: edited — the driver's fixture checks over acting outcomes
@task_file n=15 path=packages/escher-driver/src/recording_tests.rs
what: edited — the session recording checks over acting outcomes
@task_file n=16 path=packages/escher-driver/src/replay.rs
what: edited — kept-check replay: compares a replayed answer with the kept one
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: edited — the driver crate's public surface and re-exports
@task_file n=18 path=tests/blitz-tests/tests/stand_act_errors.rs
what: added — the stand check of caused: panic, Err, logging click, type and press in both layout modes, serve_mcp, kept checks, and the command span's eight fields around a panicking call
@task_file n=19 path=tests/blitz-tests/tests/stand_explore.rs
what: edited — the stand check of exploration: a panicked path and its kept check
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the shared helpers of the session stand checks, matching acting outcomes
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check of the command span's fields around acting calls
@task_file n=22 path=Cargo.lock
what: edited — the workspace lockfile: one dependency line for the driver's telemetry edge
@task_item n=1 part=touches
contract/driver-verbs-and-virtual-clock/click: the click answer gains the caused field when the step caused a panic, an error or a log record.
@task_item n=2 part=touches
contract/driver-verbs-and-virtual-clock/type: the type answer gains caused; a logging handler typed twice reads count 2.
@task_item n=3 part=touches
contract/driver-verbs-and-virtual-clock/press: the press answer gains caused, tied to the focused handler's id, owner and event.
@task_item n=4 part=touches
contract/driver-verbs-and-virtual-clock/advance: the advance answer gains caused; the clock behaviour is unchanged.
@task_item n=5 part=touches
decision/driver-scroll-instant: the scroll answer gains caused; scroll stays instant.
@task_item n=6 part=touches
decision/settle-does-not-wait-on-animation: settle now runs inside the step's capture and panic catch; a panic while settling runs it once more and a second reads busy render.
@task_item n=7 part=touches
contract/escher-session-commands/help: help states the caused field from the verb table — its keys, closed words, absence when nothing was caused, the first-handler, app-boundary and log-crate limits, and no message or payload.
@task_item n=8 part=touches
contract/escher-session-mcp/other-results: the acting tool results over MCP carry the same caused member as the command line.
@task_item n=9 part=touches
contract/escher-session-stdout/command-line-line and contract/escher-session-stdout/mcp-line: an acting answer line may now end with a caused member; an answer that caused nothing reads as before.
@task_item n=10 part=touches
decision/driver-output-for-agents and decision/escher-session-agent-read-surface: a new agent-read member on the answer line, closed words and ids only.
@task_item n=11 part=touches
contract/ui-graph-line/content: a caused item's owner is the handler's owner id as the UI graph reads it.
@task_item n=12 part=touches
contract/escher-session-commands/snapshot and contract/escher-session-commands/screenshot: an Err the root default boundary takes replaces the app with its Display, so that text reaches the screen content those verbs read.
@task_item n=13 part=touches
contract/escher-session-mcp/err-ending and contract/escher-session-session-error/remedy: a refused call carries no caused; a panicking step no longer ends the session, a repeated settle panic reads the existing busy render cause.
@task_item n=14 part=touches
decision/exit-statuses-from-schema-table: the verb table gained a field kind (field kinds 29 to 30) but no exit status.
@task_fact n=1 slug=surface
fact: An acting answer (click, type, press, advance, scroll) carries caused, last and only when non-empty: one item per kind and level, each with kind (panic, error, log), level (error, warn, info, debug, trace), the handler's id, owner and event where known, and a count — never a message, payload or error text.
@task_fact n=2 slug=surface
fact: caused counts the first reached handler only, an Err only where it reaches the app's root default boundary (an Err the app's own boundary catches leaves no mark), and no record of the log crate.
@task_fact n=3 slug=surface
fact: The Dioxus framework targets (dioxus, generational_box, warnings) count in caused only at warn and error; every other target outside the engine and escher prefixes counts at every level.
@task_fact n=4 slug=surface
fact: An Err the root default boundary takes replaces the app with the error's Display on screen; that text crosses as screen content, never in caused.
@task_fact n=5 slug=surface
fact: A panic in an acting step is caught and the session keeps answering; an exploration ends such a path Panicked with no payload, its kept check holding the panicked step.
@task_fact n=6 slug=surface
fact: The closed words of caused stand on the founder's own word, not provisional; the error's message text is owed to a later crossing, not carried now.
@task_item n=1 part=untouched
Colour palette, typography, spacing, depth, border radius and iconography rows: no colour, font, paint order, hit-testing or cursor changed.
@task_item n=2 part=untouched
The select list rows (open, choose, highlight, its colours, placement and accessibility): no select behaviour changed.
@task_item n=3 part=untouched
Motion rows on the virtual clock, scrollbar fades and smooth scroll: no timing moved.
@task_item n=4 part=untouched
escher-session tasks (counter, crud, timer and the rest) and the unknown task: no task added or changed.
@task_item n=5 part=untouched
escher-session exit statuses and exit codes 0 to 3, stderr usage and session errors: no exit, cause or error object added.
@task_item n=6 part=untouched
The actions, coverage and graph commands and their lines and arguments: unchanged.
@task_item n=7 part=untouched
WPT runner output and result colours, the shell provider, the JS window object, MountedData during a render and the dioxus-native config rows: not reached.
@task_item n=8 part=untouched
The web-spa and mobile targets: nothing built or tested differently.
