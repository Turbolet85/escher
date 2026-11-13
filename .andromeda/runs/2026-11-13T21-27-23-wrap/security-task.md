@task_prose part=session
A behaviour-preserving restructure for cognitive load: the engine's click default-action handler, the driver's command executor and the Dioxus document's graph builder were each split into private per-arm functions, signatures, callers and behaviour unchanged.
Cognitive readings moved: the click handler 97 to 2, the command executor 32 to 1, the graph builder 50 to 4; every new private function reads 0 to 10.
No public symbol, span field, refusal cause, verb, key name, carrier, log field, env read, listener, dependency or unwrap/expect count was added or removed; the census of log and panic sites per file held equal.
One new silent integration check was added: click default actions (link navigation and fragment links, a hrefless link walking on to its label, an unmatched click clearing focus, a disabled submit stopping the walk, a label of a multiple select, a double click), in both layout modes.
Coverage readings moved with every per-crate floor held: escher-driver 98.04 to 98.01, dioxus-native-dom 86.71 to 90.09, blitz-test-harness 96.06 to 95.74.
The operator's standing word for this wrap: the security decision unresolvable-stylesheet-href-no-load holds a FALSE why, its resolver no longer panics, and must be mended by the shorter why or retired.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's click default actions — the click walk up the tree, focus clear, the double click, and per-element arms: checkbox and radio toggling with their change event, summary/details toggling, select and option choice, label re-dispatch to its bound control, link navigation by the resolved href, button click focus, file input dialog
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the driver's command executor — the one match over verbs (snapshot, click, type including type on a select, key, advance of the virtual clock, scroll, screenshot in band, graph, lint, actions) under the one command span, feeding the session's bounded record of read ids
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the graph answer builder of the Dioxus document — state readings, handlers per node, declared and confirmed writes from the signal registry and its dispatch record, reads per scope, the graph states
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: a new silent integration check of the engine's click default actions with a recording navigation provider, in the unpublished blitz-tests crate
@task_item n=1 part=touches
decision/unresolvable-stylesheet-href-no-load: its why (resolution through the non-panicking resolver instead of the one that panics) is FALSE — the URL resolver answers none and never panics since the upstream take; its rule (no load, no render block) still holds; two leaves already state the truth; the shorter why or a retire is owed.
@task_item n=2 part=touches
decision/engine-change-event-checkable: the checkbox and radio change dispatch moved into their own arm functions, behaviour unchanged; its verified evidence cites click-handler lines that now stand elsewhere (checkbox change at 951-954, radio if-not-was-checked at 995-1000); guard checkable_change_event unchanged and green.
@task_item n=3 part=touches
decision/external-navigation-get-http-mailto and decision/external-navigation-schemes: link navigation on click moved into its own arm (resolve the href, navigate), unchanged; the new check asserts a link leaving the document navigates and a fragment link does not.
@task_item n=4 part=touches
decision/button-click-focuses and decision/enter-activates-focused-control and decision/space-keyup-activates-focused-control: the click arms they reach (button, the walk, the focus clear) were split, behaviour unchanged.
@task_item n=5 part=touches
decision/select-choice-one-write-path, decision/multiple-select-keys-and-clicks, decision/select-list-row-choice, decision/select-list-pointer-release-chooses: select and option click arms moved into their own functions, unchanged; the new check asserts a label of a multiple select focuses it and chooses nothing.
@task_item n=6 part=touches
decision/disabled-read-by-presence and decision/inert-subtree: the click walk that stops at a disabled control moved into its own function; the new check asserts a disabled submit button stops the walk.
@task_item n=7 part=touches
decision/masked-input-values and decision/password-file-value-masked: the file input click arm moved with its unwrap and expect, behaviour and panic-site count unchanged.
@task_item n=8 part=touches
decision/one-command-span-per-call and decision/command-span-content-free and the driver-command-span contract items: the command span stays opened in the session's run with its eight fields recorded where they were; no field added.
@task_item n=9 part=touches
decision/closed-driver-verb-schema, decision/single-validation-gate, decision/target-refusal-order, decision/closed-refusal-cause-set: the executor's verb match was split into one function per verb; no verb, cause or check order changed, the target check unedited.
@task_item n=10 part=touches
decision/driver-type-on-select: the type verb's acting order (select focus or click, select-all and backspace, then typing that skips a leading space on a select) became a named step function, unchanged.
@task_item n=11 part=touches
decision/advance-moves-virtual-clock-exactly and decision/time-moves-only-on-advance: the advance verb moved into its own function, unchanged.
@task_item n=12 part=touches
decision/screenshot-in-band and decision/screen-pixels-in-band and contract/screenshot-answer/png: the screenshot verb with its crop and error map moved into its own function, unchanged.
@task_item n=13 part=touches
decision/id-record-bounded-and-session-fed and decision/session-id-record-bounded and decision/no-node-id-between-calls: the snapshot and graph verbs still feed the session's bounded record of ids, now from their own functions.
@task_item n=14 part=touches
decision/graph-answer-crossing, decision/graph-answer-unpaged, contract/graph-answer/nodes-states, contract/declared-write/event, contract/declared-write/target: the graph builder was split into state readings, handlers, writes, reads and states functions; the answer is unchanged.
@task_item n=15 part=touches
decision/signal-registry-holds-no-state-value, decision/dispatch-recorded-only-with-signal-registry, decision/reached-handler-record-without-log: the graph builder now imports the registry's dispatch record privately; still compiled under the accessibility gate, no log added.
@task_item n=16 part=touches
decision/driver-outcomes-never-logged, decision/accessible-names-not-logged, decision/stable-id-not-logged, decision/no-screen-content-in-logs: no log site added in any of the three changed components; the new check holds no tracing, print or env read.
@task_item n=17 part=touches
decision/per-crate-coverage-floors: readings moved (escher-driver 98.01, dioxus-native-dom 90.09, blitz-test-harness 95.74) with every floor held; a floor value in a row is unchanged.
@task_item n=18 part=touches
decision/blitz-tests-not-published: a new test target was added to the unpublished blitz-tests crate, auto-discovered, its manifest unedited.
@task_item n=19 part=touches
decision/no-panic-on-call-input and decision/stale-node-ids-never-panic: the arm functions re-read their element with let-else rather than unwrap, keeping the panic-site census equal.
@task_item n=1 part=untouched
Authentication and authorization rows, CI tokens and publish guards: no auth, CI workflow or publish job changed.
@task_item n=2 part=untouched
Dependency security rows (audit, licence gate, pins, locked builds): no dependency added, removed or moved.
@task_item n=3 part=untouched
Secret management rows and the environment-variable contract: no env read or secret touched.
@task_item n=4 part=untouched
Data protection rows on history, cookies, HTTP cache, TLS, clipboard and kept check files: none of those stores or files changed.
@task_item n=5 part=untouched
Outbound network rows (fetch methods, per-host limits, CORS and same-origin absences, request timeouts and size limits, script fetchers, file URLs): no fetch path changed.
@task_item n=6 part=untouched
MCP server and session socket transport rows, CLI flags, roles and exit codes, session errors: the transport, the wire and the command line are unedited.
@task_item n=7 part=untouched
Recording, recorded-check, cold-agent and agent-run rows: no recorder, replay or script changed.
@task_item n=8 part=untouched
Telemetry sink allowlist and log target rows: no sink, target or field changed.
@task_item n=9 part=untouched
WPT runner, CSS parse, stylesheet loading and serialization rows other than the stylesheet-href why above: the mutator and the document were proved unedited.
@task_item n=10 part=untouched
Stable element id derivation and snapshot text contract items: no id form, state word or field changed.
