@task_prose part=session
The chunk 2026-11-04-dioxus-runtime-probe added one blitz-tests test target, dioxus_runtime_probe (607 lines, 10 tests, both layout modes), which reads the public Dioxus 0.7.10 API to measure what is exposed of component, handlers, written state and readers; nothing under packages/ or examples/ and no Cargo.toml changed.
The probe pins the MountedData hazard at HEAD as two should_panic tests (set_focus from a task a key handler spawned panics RefCell already borrowed through doc_mut while DioxusDocument::poll holds the document borrow); the fix is routed to a separate port chunk, no engine change made.
It measured the listener reading: the bridge's listener attributes over-report (a removed listener's attribute and mark stay), while the vdom walk's dynamic attributes read current listeners exactly; the reacts-to-a-click reading measured, no snapshot field added, NODE_FIELDS thirteen.
An operator fix commit bumped pulldown-cmark 0.13.5 to 0.13.6 in Cargo.lock alone (RUSTSEC-2026-0433) and added RUSTSEC-2026-0429 (zip 2.4.2) to the deny.toml advisories ignore list, now eight IDs; the zip residual res-h7acz56c revised in the store.
The obs gate: the silence census over the new file reads 0 (no print, env read, escher_telemetry, tracing_subscriber or listener bind), and no failure message or evidence document carries a read value; no log site, span, field, log field, sink or env read added anywhere.
Counts moved: blitz-tests 158 .rs files and 157 test targets; run all passed 743, failed 0, ignored 12; run stand unmoved at 237 over 51 files; CI green 17/17 on 9b3b29714.
@task_file n=1 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_item n=1 part=touches
The section 3 Logging stack and log-coverage silence rows for test checks (absence logs group): a new blitz-tests target, dioxus_runtime_probe, now stands that installs no subscriber, reads no env var, prints nothing and binds no listener, the silence census reading 0 over it; the standing rows are earlier chunks' own and stay true, the new row is new text.
@task_item n=2 part=touches
Any count of blitz-tests test files or targets: now 158 .rs files directly under tests/blitz-tests/tests/ and 157 test targets (157 and 156 before), dioxus_runtime_probe the latest.
@task_item n=3 part=touches
Any count of the agent-run run all result: passed 743, failed 0, ignored 12 (733, 0, 12 before); run stand unmoved at 237 over 51 files because the new file carries no stand_ prefix.
@task_item n=4 part=touches
Statements of which stand_* or engine checks are silent, or of the silence probe pattern (println!, eprintln!, print!, dbg!, std::env, env::var, escher_telemetry, tracing_subscriber, UnixListener, TcpListener, bind): the same pattern fired over dioxus_runtime_probe.rs as its gate 2, exit 1 and last line 0.
@task_item n=5 part=touches
The MountedData hazard as a panic class in dioxus-native-dom: a task spawned from a key handler that calls set_focus (packages/dioxus-native-dom/src/events.rs:295, through doc_mut at events.rs:162) panics RefCell already borrowed while DioxusDocument::poll holds inner.borrow_mut() (dioxus_document.rs:295) across render_immediate (:297); any error-capture or panic-hook text that names which panics the engine or a binary raises, re-measured, not fixed; the synchronous set_focus call does not panic.
@task_item n=6 part=touches
Statements over dioxus-native-dom's listener mark and listener attributes (create_event_listener, mutation_writer.rs:331-352; remove_event_listener, mutation_writer.rs:355-360): the attribute and the data-dioxus-id mark stay after a listener is removed, a mounted listener writes none; any reading that treats the mark as exact over-includes.
@task_item n=7 part=touches
Text naming pulldown-cmark's resolved version or the dependency audit: pulldown-cmark resolves 0.13.6 (0.13.5 before), RUSTSEC-2026-0433 fixed; reached only through rdme, whose tracing feature wiring is unchanged.
@task_item n=8 part=touches
Text naming the deny.toml advisories ignore list or the zip 2.4.2 ignores: the zip IDs are now four (RUSTSEC-2026-0367, 0368, 0369, 0429), the list eight IDs at deny.toml:20-30; zip reaches the graph only through anyrender_serialize 0.8.0 under apps/browser's capture feature.
@task_item n=9 part=touches
Section 9 CI Integration, the Dependency audit leg: a CI audit can go red on advisories published after a green base with Cargo.lock and deny.toml unchanged (CI#41911351081), and the local fast leg runs no audit, bash .github/scripts/ci-leg.sh audit being the local reproduction; final CI#41914044942 green 17/17; no CI step, artifact or leg list moved.
@task_item n=10 part=touches
Any line citation into Cargo.lock or deny.toml: Cargo.lock lines 6505 and 6507 changed (pulldown-cmark version and checksum), deny.toml line 26 added, shifting later deny.toml lines by one.
@task_item n=11 part=touches
Statements that a test target is reached by the workspace test leg and run all but not by run stand, stand-repeat or the a11y leg: dioxus_runtime_probe is auto-discovered, no manifest line, reached by the workspace test leg, run all and run dioxus_runtime_probe.
@task_item n=12 part=touches
Statements over what the snapshot or driver exposes of components, handlers and state readers: the probe reads, in process and into assertions only, component names (VComponent.name), current listeners (VNode.dynamic_attrs) and a GlobalSignal's subscribers (Subscribers::visit, origin_scope); hook-local use_signal state is not exposed without a fork-level change; no snapshot field, driver verb or span field added.
@task_fact n=1 slug=log-coverage
fact: The Dioxus runtime probe check is silent: dioxus_runtime_probe (blitz-tests, both layout modes, driving blitz_test_harness::Harness on Counter, CRUD, Cells and its own pages toggled_listener, shared_state and hazard) installs no subscriber, reads no env var, prints nothing and binds no listener, its readings (component names, listener names, signal subscribers, snapshot diffs) going into assertions only and its failure messages naming a layout mode, a task or page and a link or index, never a node id, signal value or listener read off a node; the silence census reading 0 over the file, no log site, span, field, log field or env read added (as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md).
@task_item n=1 part=untouched
escher-telemetry (the stderr sink, its allowlist formatter, scrub sets, service identity, line format and chaining panic hook) and its two install sites in seven_guis_native and escher-session: no file under packages/ or examples/ changed.
@task_item n=2 part=untouched
Section 4 Span / Trace Coverage: escher-driver's command and tool_call spans, their eight and two fields, and the 2 info_span! sites are unchanged; VERBS twelve, CAUSES eight, SessionError fifteen.
@task_item n=3 part=untouched
Section 2 Telemetry Strategy feature wiring and the workspace tracing dependencies: no Cargo.toml moved, Dioxus stays at the 0.7.3 requirement, no crate added or removed.
@task_item n=4 part=untouched
Section 8 values logged as-is, including dioxus-native-dom mutation_writer.rs:172, 224 and 410 debug-log citations: mutation_writer.rs was read, not edited, its citations unmoved.
@task_item n=5 part=untouched
The agent-run harness log and the cold-agent pipe log (shapes, state areas, events, verdict fields): no script, status shape or leg moved; the new target only adds result lines to the existing test events.
@task_item n=6 part=untouched
Section 5 Metric Coverage and section 7 error classes captured for blitz-dom, blitz-net, blitz-shell, blitz-vibey-script and the wpt runner: no engine source changed.
@task_item n=7 part=untouched
Section 9 telemetry artifact handling (the two uploaded CI artifacts, local state under target/, session and recording directories): unchanged; the captured backtrace was read in the session scratchpad and only its coordinates entered the evidence.
@task_item n=8 part=untouched
Rows naming upstream 286874cc1 as left out with the Dioxus 0.8 move: still true; the buffered-render port is a future chunk and changes no code here.
