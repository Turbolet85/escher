@task_prose part=session
Restructured three functions to bring cognitive complexity down toward the ceiling of 15, with behaviour unchanged byte for byte: the engine's click default action (handle_click 97 to 2), the driver session's command execution (Session::execute 32 to 1) and the Dioxus document's UI graph reader (DioxusDocument::graph 50 to 4).
Each function keeps its signature and callers. Its body now dispatches to new private functions, one per arm or per step: one per element kind for the click (checkbox, radio, summary, select, option, label, link, file input), one per verb for the driver, and the state readings, handlers, writes and reads for the graph.
On the operator's word, every arm of handle_click moved, including upstream's arms. The cost is a hand re-apply whenever a later upstream sync edits one of those arms.
Added one blitz-tests integration target, click_default_actions. It holds six checks in both layout modes: link navigation versus fragment, the label walk-on, a click no element takes clearing focus (focus then reads the root element), a disabled submit button stopping the walk, a label of a multiple select, and two clicks with no time between them reading as one double click. The check is silent.
Measured coverage: escher-driver 98.01, dioxus-native-dom 90.09 and blitz-test-harness 95.74. Every per-crate floor held.
No public symbol, crate, dependency, verb, refusal cause, span field, key name, carrier, env read or listener was added or removed, and nothing was retired.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: blitz-dom pointer event handling, including the click default actions (handle_click), which walk from the hit node to the first element whose kind has a default action (checkbox and radio toggling with their change, details summary, select and option, label re-dispatch to its bound control, link navigation, file input, button), then clear focus and dispatch the double click.
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the driver session's command execution: one function an accepted verb runs through (snapshot, click, type, press, advance, scroll, screenshot, graph, lint, actions), acting on the harness and feeding the session's record of seen ids.
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the Dioxus document's UI graph reader (DioxusDocument::graph). It builds the graph's nodes, their handlers, the states they read and write and the write flags, from the signal registry and its dispatch records. It compiles under the accessibility feature.
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: a blitz-tests integration test target for the click default actions on an HTML document, with a recording navigation provider and a virtual clock, run in both layout modes, plus a Dioxus double-click page.
@task_item n=1 part=touches
decision/cognitive-complexity-ceiling: three functions over the ceiling of 15 were brought under it (handle_click 2, execute 1, graph 4, every new function 0 to 10). Three functions over it remain in the pointer handling (handle_pointermove 77, handle_pointerdown 39, touch_action_pan_axes 17). The ceiling value itself is unchanged.
@task_item n=2 part=touches
The stance of leaving engine functions over the ceiling as they are now has an exception: an engine click function with upstream's arms was restructured on the operator's word. This bears on any row about the ceiling's reach into engine code.
@task_item n=3 part=touches
decision/direct-engine-changes-flagged-upstreamable, decision/upstream-take-engine-changes and decision/upstream-taken-by-cherry-pick: the engine's click default action, upstream code, was restructured in the fork. A later upstream sync that edits one of its arms must be re-applied by hand.
@task_item n=4 part=touches
decision/per-crate-coverage-floors, decision/per-crate-coverage-floor and decision/coverage-floors: the readings moved (escher-driver 98.04 to 98.01, dioxus-native-dom 86.71 to 90.09, blitz-test-harness 96.06 to 95.74) and every floor held. Name a row only if its cell holds one of these readings or a floor.
@task_item n=5 part=touches
decision/blitz-tests-one-target-per-file, decision/blitz-tests-target-per-file, decision/integration-tests-in-blitz-tests, decision/integration-test-one-behavior and decision/blitz-tests-single-test-target: a new blitz-tests target was auto-discovered. The manifest is unedited.
@task_item n=6 part=touches
decision/both-layout-modes, decision/scenarios-both-layout-modes and decision/layout-modes-identical: the new check runs every case in both layout modes.
@task_item n=7 part=touches
decision/checkbox-change-from-engine and contract/dom-events/change-on-checkable: the checkbox and radio change dispatches moved into their own arm functions with behaviour unchanged. Any row's verified evidence citing pointer lines of those dispatches is now stale (they moved to 951-954 and 995-1000).
@task_item n=8 part=touches
decision/default-action-event-order, decision/handlers-before-default-action and contract/event-dispatch/activation-click-order: the click default action's walk and arms were split into functions, with order unchanged.
@task_item n=9 part=touches
decision/double-click-window and decision/repeated-click-window: double-click dispatch now runs at the end of the restructured click function, and a new check guards two clicks with no time between them as one double click.
@task_item n=10 part=touches
decision/click-focuses-button, contract/event-dispatch/button-click-focus and decision/focus-fixup-without-events: the button arm and the focus clearing after the walk were restructured. A click no element takes leaves focus reading the root element, now checked.
@task_item n=11 part=touches
decision/label-click-focuses-select, contract/select-list-default-actions/label-click, contract/select-list-default-actions/click-select and contract/select-list-default-actions/choice: the label, select and option arms became their own functions, and a new check covers a label of a multiple select.
@task_item n=12 part=touches
decision/fragment-link-scrolls-in-page, decision/link-load-start and decision/unresolvable-url-skipped: the link arm moved into its own function, still resolving the href through the non-panicking relative resolver. A new check guards that a link leaving the document navigates and a fragment link does not.
@task_item n=13 part=touches
decision/justified-expect-unwrap, decision/unwrap-expect-justified, decision/tracing-gated-per-call-site and decision/engine-tracing-gated-per-call-site: the file input arm's unwrap and expect and the link arm's two tracing-gated lines moved with them. The new arm functions re-read their element by let-else, adding no unwrap, expect or log site.
@task_item n=14 part=touches
decision/driver-spans-only, decision/driver-hand-built-spans, decision/escher-driver-spans-ungated and the command-span contracts: the command span stays opened in the session's run, with its eight fields recorded where they were, while execution split into one function per verb.
@task_item n=15 part=touches
decision/validated-commands-only, decision/seen-ids-record-bounded and the driver-verbs contracts: per-verb execution was restructured. Validation still runs first, and the graph verb still feeds the session's record of seen ids, so behaviour is unchanged.
@task_item n=16 part=touches
decision/type-replaces-control-content and contract/dom-events/driver-type-on-select: the type verb's acting order (select focus or click, select-all and backspace, then typing that skips a leading space on a select) is now a named step function, with order unchanged.
@task_item n=17 part=touches
decision/graph-state-by-registration, decision/signal-write-recorded-against-dispatch, decision/accessibility-feature-gating and the ui-graph and graph-verb contracts: the UI graph reader was split into state readings, handlers, handler writes, node reads and graph states. It is still under the accessibility gate and its output is unchanged.
@task_fact n=1 slug=stack-and-technologies
fact: The cognitive complexity ceiling reaches the engine's click default action by exception. It was restructured with upstream's arms on the operator's word, at the cost of a hand re-apply whenever an upstream sync edits one of its arms. Every other engine function over the ceiling is still left as it is.
@task_item n=1 part=untouched
The session socket wire, the MCP role, session errors and exit codes: no verb, refusal cause, carrier, key name or wire field changed.
@task_item n=2 part=untouched
The telemetry bootstrap, its scrub, line format and target filter: no log field, env read or sink was added.
@task_item n=3 part=untouched
Paint, layout, style, CSS feature decisions and the absences of CSS features: nothing in rendering moved.
@task_item n=4 part=untouched
The WPT runner, its outputs and its CI artifacts.
@task_item n=5 part=untouched
The browser app, its history persistence and about pages.
@task_item n=6 part=untouched
CI workflows, ci-leg legs, the cold-agent and agent-run scripts, and the package install: no workflow or script changed.
@task_item n=7 part=untouched
Occupied names (crates, binaries, servers, other names) and the stack's dependency choices: no crate, binary, dependency or listener was added or freed.
