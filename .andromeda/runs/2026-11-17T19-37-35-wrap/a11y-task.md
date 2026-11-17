@task_prose part=session
The session split the driver's answer writer, the crate-private module that writes every driver answer by hand as JSON, by answer family into seven private child modules: snapshot, diff, caused, layout, graph, actions and lint.
Every carrier's answers stay byte for byte unchanged: the 49 literal JSON pins pass, and the stand replay compares both committed Counter checks whole with every kept answer, 5 passed.
No public symbol, crate re-export, caller path, dependency, schema key, field order or written byte moved; the engine document was left whole and the stand checks and flows are untouched.
The writer's 9 unit tests moved out of line into a #[path] test file under the same json::tests names, so escher-driver's inline tests drop by 9 and its #[path] test files gain one file and 9 tests.
CI read green 17/17 at attempt 3, after two attempts in which the iOS job was cancelled with no runner, and escher-driver coverage reads 97.68 % against its 97 % floor.
@task_file n=1 path=packages/escher-driver/src/json.rs
what: edited — the driver's answer writer, the crate-private module that writes every driver answer by hand: it keeps the string and number primitives, the screenshot answer without its image, the handler coverage line, the outcome dispatch per verb, the refusal from its causes' fixed strings, the session error and the session lines
@task_file n=2 path=packages/escher-driver/src/json/snapshot.rs
what: new, moved out of the answer writer — the snapshot node writer: a node's schema fields (parent, enabled, checked, value, selected, pressed, bounds, differs) and its reading word, the password value masked as before
@task_file n=3 path=packages/escher-driver/src/json/diff.rs
what: new, moved out of the answer writer — the snapshot diff writer: the acted answer with settled, busy and the diff's entries and differs
@task_file n=4 path=packages/escher-driver/src/json/caused.rs
what: new, moved out of the answer writer — the caused writer: the causes and past causes an answer carries (panic, error, log, their source, target, file, line, boundary, owner, event), text cut where long
@task_file n=5 path=packages/escher-driver/src/json/layout.rs
what: new, moved out of the answer writer — the layout explanation writer: placer, shift and size-cause words, lengths, placement, shift and size readings of the layout answer
@task_file n=6 path=packages/escher-driver/src/json/graph.rs
what: new, moved out of the answer writer — the UI graph writer: graph nodes with owner and component, graph states with their declared, confirmed and flagged write readings
@task_file n=7 path=packages/escher-driver/src/json/actions.rs
what: new, moved out of the answer writer — the actions answer writer: one actionable node with its owner and fields
@task_file n=8 path=packages/escher-driver/src/json/lint.rs
what: new, moved out of the answer writer — the visual lint finding writer: a finding's id, defect, bounds and reason, the contrast finding's ratio and required among them
@task_file n=9 path=packages/escher-driver/src/json_tests.rs
what: new, moved out of line — the answer writer's unit tests json::tests, 9 tests holding the 49 literal JSON pins of every driver answer, each verb's fields in order
@task_item n=1 part=touches
The rule that every driver answer is written by hand in a crate-private module still holds: the writer and its seven children are all crate-private, children widened to pub(super) at most.
@task_item n=2 part=touches
The snapshot node and snapshot text form contracts (clickable, enabled, checked, value, select value, rows and bounds) are now written from the snapshot child; their written keys, order and bytes are unchanged.
@task_item n=3 part=touches
The snapshot diff contract (entries, differs) is now written from the diff child; unchanged on the wire.
@task_item n=4 part=touches
The actions answer contract (fields, causes) and the rule that the actions verb changes nothing: the actions answer now comes from the actions child and its causes from the caused child; unchanged on the wire.
@task_item n=5 part=touches
The lint contrast finding contract (id, defect, bounds, ratio, required) and the visual lint finding content: now written from the lint child; unchanged on the wire.
@task_item n=6 part=touches
The UI graph rules (one graph node per snapshot node, read-only and carrying no values) and the UI graph state registration contract (undeclared, unconfirmed, no registered state): now written from the graph child; unchanged on the wire.
@task_item n=7 part=touches
The password value masked and never revealed: the masked node is written by the snapshot child and still pinned by the writer's tests.
@task_item n=8 part=touches
The handler coverage answer, the driver screenshot answer without its image and the driver refusal causes (disabled, covered) stay written by the parent writer, unchanged.
@task_item n=9 part=touches
The keyed field on the carriers and the rule that the driver schema defines no role or name set: the schema's verbs and fields are unchanged, the tests still check each verb's fields in order.
@task_item n=10 part=touches
The MCP snapshot tool result and the snapshot model as single source with many readers: the readers are unchanged, only the writer's module layout moved.
@task_item n=11 part=touches
The coverage floor of the driver crate (97 %) reads 97.68 % after the split; the floor value is unchanged.
@task_item n=12 part=touches
Process: a CI job cancelled with no runner recurred (the iOS job twice); it was re-run, not fixed, and the CI gate read green at attempt 3.
@task_item n=1 part=untouched
The accessibility tree itself (roles, states, names, hidden and inert content, AccessKit and the platform adapter): no reader, role mapping or ARIA read changed.
@task_item n=2 part=untouched
Keyboard navigation and focus rules (Tab order, activation keys, range input, single and multiple select, the seven_guis Cells grid and CRUD list): no event, focus or key handling changed.
@task_item n=3 part=untouched
Visual design and contrast rules (contrast formula, thresholds, stand accent colour, select list colours and motion, scrollbar thumbs, scheme and zoom): no computation changed, only how a finding is written's module layout.
@task_item n=4 part=untouched
The accessibility feature gates and default-on decisions, and the absences of a platform adapter on the stand binaries: no crate, feature or dependency changed.
@task_item n=5 part=untouched
The a11y CI leg, the slow-tier accessibility test leg and the stand checks repeated three rounds: no leg, gate or harness changed, and the committed stand checks are untouched.
@task_item n=6 part=untouched
Cognitive accessibility rows (invalid state beyond colour and class, the shared stand tab order table): not reached.
@task_item n=7 part=untouched
The Dioxus document's reached-handler record and the listeners read from the vdom walk: the engine document and its handler record were left whole.
