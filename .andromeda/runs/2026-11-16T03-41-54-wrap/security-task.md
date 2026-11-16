@task_prose part=session
The session added a layout read that explains why an element sits where it does at its size: an engine reader on every document, BaseDocument::layout_reason, and a driver reader, Session::layout, both in process only.
The engine reader is read-only: it answers None for an unresolved id, a non-element, an unstyled element or a boxless element, has no log site and no panic path; a closed select's option reads Select at its select's box.
The driver reader takes an id from Rust code alone, resolves it through the run path's resolve (refusing stale or not-found), records the screen's ids, runs no step, pump, tick or settle, opens no span, and answers String ids, closed words and numbers, never a NodeId.
No carrier reaches the read: no verb, argument, MCP tool, socket message, kept-check key, span field, env var or refusal cause was added; VERBS still holds fourteen. That absence is PROVISIONAL on the delegate's answer to the founder fork, pending the founder's word.
An element the engine cannot explain reads Placer::Engine(PlacedBy::Block) in process; the wrap directive owes it a closed word of its own before the answer crosses any carrier. Placer::App with a canvas string is reserved and built only by unit tests.
Measured: escher-driver line coverage 97.99 percent against a 98.05 percent baseline, coverage floors all held; a pump on a settled screen was disproved as a control of a changes-nothing read.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: new file; the engine's read-only layout reader on every document and its closed placing, shift, size-cause and length words
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: edited; the engine crate root, its module list and public re-exports
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: edited; the Dioxus native bridge crate root and its re-exports of engine types beside NodeId
@task_file n=4 path=packages/escher-driver/src/layout.rs
what: new file; the driver's in-process layout read on a session, resolving a caller id and answering stable ids, closed words and numbers
@task_file n=5 path=packages/escher-driver/src/layout_tests.rs
what: new file; the driver's unit tests of the layout answer and the reserved app placer
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: edited; the driver's command run path, its single target check and id resolution refusing stale or not-found
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: edited; the driver crate root, its Session and its public exports
@task_file n=8 path=docs/capabilities.md
what: edited; the committed capability statement, what each carrier crosses and what is absent
@task_file n=9 path=tests/blitz-tests/tests/layout_reason.rs
what: new file; the engine-level integration test of every placing word and size cause on fixtures
@task_file n=10 path=tests/blitz-tests/tests/stand_layout.rs
what: new file; the stand integration test of the driver layout read, that it changes nothing, refuses stale and not-found ids and writes no escher line at TRACE
@task_item n=1 part=touches
The boundary widening rule and closed-value-field-no-widening: a new public in-process read was added that crosses no carrier; the rule over what counts as a widening now has a reader that answers closed words and numbers but reaches no carrier.
@task_item n=2 part=touches
One validation path for every carrier and the single command validation gate: the driver's id resolver was made crate-visible so the layout read resolves its id through the same path as the run path's id-taking arms.
@task_item n=3 part=touches
Unknown id refused as stale or not-found, caller-supplied id looked up anew, and the driver id refusal contract items stale and not-found: a new caller of the same resolution, refusing with those two causes.
@task_item n=4 part=touches
Bounded id record fed by the session and the bounded in-process record of read ids: the layout read records the screen's ids into that record.
@task_item n=5 part=touches
No NodeId kept between calls, no process-local handle crosses with an id, and the stable element id's NodeId contract item: the layout answer holds String stable ids and no NodeId; the engine reader's own types do hold NodeIds and stay in process.
@task_item n=6 part=touches
Ratified exits of the stable id and the stable element id's carriers: the stable id now also leaves through an in-process layout answer, which no ratified carrier carries.
@task_item n=7 part=touches
Fixed order of target refusals and the actions read sharing one target check: the target type gains a crate-visible node accessor used only by the layout read.
@task_item n=8 part=touches
Actions read runs nothing and moves no session state: the layout read makes the same promise (no step, pump, tick or settle; snapshot, focus, diff, clock and open list unchanged; delivered Timer ticks stay unapplied).
@task_item n=9 part=touches
Text extent reader read-only with none answers, stale node ids never panic and no panic on call-supplied input: a second engine reader of the same kind, answering None for an unresolved, non-element, unstyled or boxless node, with no log site and no panic path.
@task_item n=10 part=touches
Select readers resolve the node and skip an open list, and select row layout reads none for an unstyled row: the layout reader reads a closed select's option as Select at its select's box, and a closed select's optgroup reads None.
@task_item n=11 part=touches
Visible region read through transforms and non-invertible transform takes no hit: the layout reader reports transform and scroll shifts, including a scale(0) transform, from the unrounded geometry.
@task_item n=12 part=touches
Closed driver verb schema and the driver verbs contract: VERBS still holds fourteen; no layout verb or argument kind was added, and the layout read is reached by no verb.
@task_item n=13 part=touches
Committed statement of the MCP crossing: the committed capability statement gained the layout read, its size rule and the absence of any carrier for it; its pin stays green.
@task_item n=14 part=touches
One command span per call, command span holds no content, driver installs no subscriber and driver outcomes never logged: the layout read opens no span and emits no event; TRACE capture reads zero escher lines.
@task_item n=15 part=touches
Per-crate line coverage floors: escher-driver measured 97.99 percent against the 98.05 percent baseline, still at or above its floor; where a row holds a floor value or baseline for escher-driver, it is the one to check.
@task_item n=16 part=touches
Threat Model Summary crossings (actions, graph, coverage, MCP result, select list): a new reading of each element's placement and size exists in process, held off every crossing until the founder rules.
@task_fact n=1 slug=input-validation
fact: The engine layout reader BaseDocument::layout_reason is read-only and in process; it answers None for an unresolved id, a non-element, an element with no computed styles or one with no box, reads the unrounded geometry, and has no log site and no panic path.
@task_fact n=2 slug=input-validation
fact: The driver layout read Session::layout takes its id from Rust code alone, resolves it through the run path's resolve (stale or not-found), records the read ids, runs no step, pump, tick or settle, and answers String stable ids, closed words and numbers, with no NodeId, text or value.
@task_fact n=3 slug=input-validation
fact: No carrier reaches the layout read: no verb, MCP tool, socket call or kept check; this absence is PROVISIONAL on the delegate's option 2 to the founder fork, pending the founder's word at the epoch boundary.
@task_fact n=4 slug=input-validation
fact: An element the engine cannot explain reads Placer::Engine(PlacedBy::Block) in process only; before the layout answer crosses any carrier it must take a closed word of its own.
@task_fact n=5 slug=input-validation
fact: Placer::App with a canvas string is a reserved placer word that no path outside the driver's unit tests builds; no app-supplied string reaches the layout answer.
@task_fact n=6 slug=logging-monitoring
fact: The layout read opens no span and emits no event; at TRACE it writes no escher line.
@task_item n=1 part=untouched
Authentication and authorization: CI tokens, publish guards and the absence of application authentication are not reached; no listener, peer or credential changed.
@task_item n=2 part=untouched
Data protection: history, cookies, HTTP cache, clipboard, file dialog, TLS, kept-check files and cold-agent recordings are not reached; the layout read writes nothing to disk.
@task_item n=3 part=untouched
Dependency security: no dependency, pin, audit ignore, licence or CI action changed.
@task_item n=4 part=untouched
Secret management: no environment variable, secret or signing material was added or read.
@task_item n=5 part=untouched
API security's outbound side: outbound HTTP, per-host limits, load waits, idle expiry, script fetch methods, CORS and same-origin absences are not reached.
@task_item n=6 part=untouched
The session socket and MCP server wire: request grammar, bounds, replies, protocol errors, tool results and exit codes are unchanged, since nothing crosses them.
@task_item n=7 part=untouched
Password and file value masking, recorded checks, recording directories, cold-agent runs and the select list's keys, paint, wheel and choice paths are not reached.
@task_item n=8 part=untouched
WPT runner, upstream sync and CI workflow rows are not reached.
