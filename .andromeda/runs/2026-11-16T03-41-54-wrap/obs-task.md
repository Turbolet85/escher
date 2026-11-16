@task_prose part=session
The session added an in-process layout read: why an element sits where it does at its size, read from the style and layout chain (v030-11, built and not claimed).
The engine gained a read-only reader on every document, BaseDocument::layout_reason, answering a placing word (PlacedBy), shifts (Scroll, Transform) and a size cause per axis (SizeCause), or None for an unresolved, unstyled or boxless element; it has no log site and no panic path.
The driver gained Session::layout, which resolves an id through the run path's resolve (refusing stale or not-found) and answers String ids, closed words and numbers, never a NodeId; it runs no step, pump, tick or settle, opens no span and emits no event.
No verb, MCP tool, socket call, kept-check key, argument, result field, refusal cause, log field or span field was added; VERBS still holds fourteen; the absence of every carrier of the read is PROVISIONAL, for the founder's word.
Measured: the read changes nothing (snapshot, focus, diff, clock unchanged, delivered Timer ticks unapplied) and a TRACE capture during it reads 0 escher_ lines; escher-driver line coverage 97.99 % with every crate at or above its floor.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: new; the engine's layout reason reader on every document and its closed words (LayoutReason, PlacedBy, ShiftBy, SizeCause, Length, SizeKeyword) — no log site, no panic path
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: edited; the engine document crate's root, now declaring the layout reason module and re-exporting its types
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: edited; the native DOM crate's root re-exports, now carrying the layout reason types and BoundingRect beside NodeId
@task_file n=4 path=packages/escher-driver/src/layout.rs
what: new; the driver's in-process layout read Session::layout and its answer types (LayoutExplanation, Placement, Placer with the reserved App canvas, ShiftReading)
@task_file n=5 path=packages/escher-driver/src/layout_tests.rs
what: new; the driver's unit tests of the layout answer (stable ids, unexplained element, reserved App placer)
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: edited; the driver's run path, its id resolve (now crate-visible, with stale and not-found refusals) and the command target
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: edited; the driver library root, declaring the layout module and re-exporting its answer types
@task_file n=8 path=tests/blitz-tests/tests/layout_reason.rs
what: new; the engine integration check reading every placing word and size cause on fixtures in both layout modes
@task_file n=9 path=tests/blitz-tests/tests/stand_layout.rs
what: new; the stand check of the layout read on the served screens, its no-change and no-pending-pass cases, its stale and not-found refusals, and its TRACE capture counting escher_ lines; in the stand-repeat set
@task_file n=10 path=docs/capabilities.md
what: edited by hand; the capability statement, gaining the layout read, its size rule and the absence of any carrier for it
@task_item n=1 part=touches
decision/driver-emits-no-event and span-trace-coverage: a new driver surface, Session::layout, emits no event and opens no span, measured by a TRACE capture reading 0 escher_ lines; it belongs beside the rule.
@task_item n=2 part=touches
decision/one-span-per-command and the command span contracts (command-span/span, escher-driver-command-span/span): the layout read is a driver read outside any command, with no span of its own; the command span itself is unmoved, a read beside the one-span rule.
@task_item n=3 part=touches
contract/refusal-cause/stale and contract/refusal-cause/not-found: Session::layout now refuses with these two causes through the run path's resolve; no new cause was added.
@task_item n=4 part=touches
decision/actions-read-changes-nothing: a second read that changes nothing in the document now stands beside it — the layout read runs no step, pump, tick or settle and leaves delivered Timer ticks unapplied.
@task_item n=5 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand for the layout read, its size rule and the absence of its carrier; its pin stays green in fast.
@task_item n=6 part=touches
decision/driver-library-installs-no-subscriber: the driver library gained a module and still installs no subscriber; only a test-side capture subscriber reads the TRACE count.
@task_item n=7 part=touches
decision/debug-types-never-logged and decision/session-logs-carry-no-page-content: a new answer-bearing type, LayoutExplanation, holds ids, closed words and numbers and is never logged.
@task_item n=8 part=touches
decision/engine-target-safe-fields: the engine gained a reader with no log site, so no new engine target field.
@task_item n=9 part=touches
contract/command-span/verb and contract/command-span/field-count: no verb was added (VERBS still fourteen) and no span field, so the verb set the command span carries is unmoved.
@task_item n=10 part=touches
contract/tool-call-span/tool and contract/host-wire/v3: no MCP tool and no socket message reaches the layout read; the tool set and the host wire are unmoved.
@task_item n=11 part=touches
decision/graph-and-actions-not-counted-verbs and the cold-agent counted verbs (advance, lint, screenshot): the layout read cannot be counted by a cold agent while it stays in process; no counted verb moves.
@task_item n=12 part=touches
decision/hosted-call-through-session-run: Session::layout is a session method reached by no hosted call, so the rule holds; it resolves ids through the same resolve as the run path.
@task_item n=13 part=touches
decision/driver-spans-carry-no-content and decision/command-span-no-call-content: the layout read adds no span, so no span carries its id or answer.
@task_fact n=1 slug=4-span-trace-coverage
fact: The driver's in-process layout read opens no span and emits no event; it runs outside the command span, and a TRACE capture during it reads 0 escher_ lines.
@task_fact n=2 slug=log-coverage
fact: The layout read changes nothing: it runs no step, pump, tick or settle; the snapshot, both focus readings, the diff and the clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied.
@task_fact n=3 slug=pii-scrubbing-compliance
fact: The layout read's answer holds String ids, closed words (placer, shift, size cause, length) and numbers only — no text, no value, no NodeId — and is never logged.
@task_fact n=4 slug=new
fact: The layout read has no carrier: no verb, MCP tool, socket call or kept check reaches it, and VERBS stays fourteen — a PROVISIONAL absence awaiting the founder's word.
@task_fact n=5 slug=log-coverage
fact: The cold-agent bar cannot count the layout read while it stays in process.
@task_fact n=6 slug=7-error-capture-reporting
fact: The engine's layout reason reader has no log site and no panic path; it answers None for an unresolved id, a non-element, an unstyled element or a boxless one (display none, display contents, or a display none ancestor).
@task_item n=1 part=untouched
Telemetry backend, metric emission and metrics exporter absences, and the WPT score publishing decisions: no exporter, metric or backend was added.
@task_item n=2 part=untouched
escher's sink log line format, its RUST_LOG, stream, service.name and service.version, the target allowlist and the scrub set: the sink was not changed.
@task_item n=3 part=untouched
The tool_call span's fields and endings and the closed span line's field order, quoting and timing: no span was added or altered.
@task_item n=4 part=untouched
The agent-run and cold-agent harness events, verdict fields and transcript decisions: no harness or verdict field moved.
@task_item n=5 part=untouched
Script error surface, window error event, onerror, WPT report and escher-telemetry panic hook rows: no error path was touched.
@task_item n=6 part=untouched
CI integration, uploaded artifacts, recording files, session state directory and recording bounds: no CI step, recording or state file changed.
@task_item n=7 part=untouched
escher-session commands, exit codes, the coverage role line, the mcp role stdout and the error object: no binary role or exit status changed.
@task_item n=8 part=untouched
The select, optgroup and open-list logging and pixel decisions: the layout read names a select's box by a placing word but logs no label, chosen set or row.
