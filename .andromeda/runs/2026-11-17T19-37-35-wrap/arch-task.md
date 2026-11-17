@task_prose part=session
The session split the driver's hand-written answer JSON writer by answer family into seven private child modules (snapshot, diff, caused, layout, graph, actions, lint); the parent keeps the primitives, the shot, coverage, the outcome dispatch, refusal, session error and session lines.
Every answer on every carrier is byte for byte unchanged: the 49 literal JSON pins pass, and stand replay compares both committed Counter checks whole with the checks untouched.
No public symbol, crate re-export, caller path, dependency, schema key or written order moved; moved items are widened to pub(super) at most, the rest stay private to their child.
The writer's 9 inline unit tests moved out of line into a #[path] test child of the writer module, their test paths json::tests::* unchanged; inline driver tests drop by 9 and #[path] test files gain one file and 9 tests.
No log site, print or env read was added, info_span! sites stay 2; escher-driver coverage reads 97.68 % against its 97 % floor; CI went green 17/17 after two iOS jobs cancelled with no runner were re-run, nothing in CI changed.
@task_file n=1 path=packages/escher-driver/src/json.rs
what: edited — the driver's crate-private answer JSON writer, now the parent holding the string and number primitives, the screenshot answer, coverage, the outcome dispatch, refusal, session error and session answer lines, and the public to_json entry points
@task_file n=2 path=packages/escher-driver/src/json/snapshot.rs
what: new — the answer writer's private family module for snapshot nodes (node keys, readings, bounds, differs)
@task_file n=3 path=packages/escher-driver/src/json/diff.rs
what: new — the answer writer's private family module for the acting verbs' snapshot diff (added, removed, changed, settled, busy)
@task_file n=4 path=packages/escher-driver/src/json/caused.rs
what: new — the answer writer's private family module for the caused and caused_past items of acting verb results
@task_file n=5 path=packages/escher-driver/src/json/layout.rs
what: new — the answer writer's private family module for the layout answer (placing, shift, size cause, length and fallback words, placement, size reasons)
@task_file n=6 path=packages/escher-driver/src/json/graph.rs
what: new — the answer writer's private family module for the graph verb's nodes and states (writes, readers, write flags)
@task_file n=7 path=packages/escher-driver/src/json/actions.rs
what: new — the answer writer's private family module for the actions verb's nodes
@task_file n=8 path=packages/escher-driver/src/json/lint.rs
what: new — the answer writer's private family module for lint findings and their bounds
@task_file n=9 path=packages/escher-driver/src/json_tests.rs
what: new — the answer writer's unit tests moved out of line as a #[path] test child, holding the 49 literal answer JSON pins for every outcome, refusal, session error, session answer, node, layout word and coverage line
@task_item n=1 part=touches
The decision that driver answers are written by hand in the driver's own crate-private JSON writer: the writer is now a parent module with seven private family children, still crate-private and still by hand.
@task_item n=2 part=touches
The rule that moved unit-test modules keep their test paths: the writer's tests moved out of line and keep json::tests::* unchanged.
@task_item n=3 part=touches
The convention that driver unit tests live in #[path] children and that a split test module stays a child of its module: the writer's tests join that set as a new #[path] child.
@task_item n=4 part=touches
The unit-test placement convention (tests at the end of the source file, inline): the writer no longer holds its tests inline.
@task_item n=5 part=touches
The restricted crate internals convention: moved items are widened to pub(super) and no further, the rest private to their family child.
@task_item n=6 part=touches
The answer JSON contract items (node, outcome, paged snapshot, screenshot, string escaping, lint finding, actions, layout and its length, placing, shift, size cause and fallback words): their writing moved into family modules with every byte unchanged.
@task_item n=7 part=touches
The acting verb results, snapshot diff, graph verb, actions verb and lint verb answer contracts whose written form the moved writers produce: unchanged in key, order and byte.
@task_item n=8 part=touches
The refusal JSON and session error JSON shapes and the session answer lines: still written by the parent, unchanged.
@task_item n=9 part=touches
The screenshot answer and the hand-written base64 with no base64 crate: still written in the parent, unchanged.
@task_item n=10 part=touches
The per-crate coverage floor for escher-driver at 97 %: held at 97.68 % after the split.
@task_item n=11 part=touches
The driver-logs-no-content, driver-spans-only and no-snapshot-in-logs rules: the new family modules add no log site, print or env read, and info_span! sites stay 2.
@task_item n=12 part=touches
The lint verb's own rectangle type: the layout family module now imports it for box bounds, as the parent did.
@task_item n=13 part=touches
The committed stand-Counter checks and kept check replay: replayed with every kept answer compared whole, the checks untouched.
@task_item n=14 part=touches
The driver public API and serde_json-as-reader-only rules: no public symbol, re-export or serde_json type moved, and the writer stays hand-written.
@task_fact n=1 slug=standard-contracts
fact: The driver's answer JSON writer is split by answer family into private child modules (snapshot, diff, caused, layout, graph, actions, lint), each item pub(super) at most, while the parent keeps the primitives, the shot, coverage, the outcome dispatch, refusal, session error, session lines and every item reached from outside the writer.
@task_item n=1 part=untouched
The rendering engine's document core, DOM, style, layout, paint and hit testing: the engine's document module was left whole and nothing in the engine moved.
@task_item n=2 part=untouched
The session socket wire, the session host, the MCP role and the command line: no verb, argument, role, version or bound changed.
@task_item n=3 part=untouched
The kept check file format, the recording directory and the check reader: no kept verb, key or digest changed.
@task_item n=4 part=untouched
Telemetry, the escher-telemetry sink and its scrub rules: no span, field or target changed.
@task_item n=5 part=untouched
The stack, dependencies, workspace manifests and the lockfile: no crate or dependency was added or removed.
@task_item n=6 part=untouched
CI workflows, ci-leg.sh legs, runners and actions: the two iOS jobs cancelled with no runner were re-run, nothing in CI changed.
@task_item n=7 part=untouched
The accessibility tree, stable element ids, the UI graph registry and the snapshot readers that build what the writer writes: no reader moved.
@task_item n=8 part=untouched
The browser, WPT runner, examples and the seven_guis stand apps: none reached.
@task_item n=9 part=untouched
Occupied names (crates, binaries, servers): no crate, binary or name taken or freed; the new modules are private to the driver crate.
