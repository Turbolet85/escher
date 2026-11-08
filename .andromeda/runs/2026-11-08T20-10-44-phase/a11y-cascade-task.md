@task_prose part=chunk
Builds one handler-coverage command (v030-07): after a set of kept recorded checks (check-{n}.json, Check::from_bytes then Check::replay) has run on the stand, it lists exactly the UI graph's handlers, a graph node's id and a handler's event name as DioxusDocument::graph reads them, that none of those checks exercised.
States the canvas limit in the acceptance: only DOM listeners on agent-visible nodes are counted, one per (node, event); handlers a canvas dispatches inside one listener are not, the command keyed on the graph node id with its optional owner, never on a NodeId, so 0.4.0 can extend it.
Adds the owed replay test (CARRY cry-sxrivdk3): a session holding an actions step is recorded, kept as a check, read back and replayed on a fresh session of the same app, the replay holding with an equal actions answer, and a differing answer naming the step and the differing key; v030-06 reads verified once it passes.
The coverage read is read-only over the screen; the UI graph, the snapshot, the stable-id grammar and the actions verb keep their shapes and are only read.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=4 path=packages/escher-driver/src/replay.rs
@task_file n=5 path=packages/escher-driver/src/recording.rs
@task_item n=1 part=needs
The read-only UI graph (DioxusDocument::graph): one node per snapshot node, names and closed words only, never a second role or name mapping; the coverage command reads its nodes' handlers and must add no mapping of its own.
@task_item n=2 part=needs
The listener rule behind the graph's handlers: the vdom walk's exact current listeners, an event other than mounted, so a listener a re-render removed is not counted; the data-dioxus-id mark serves event dispatch alone and is not the coverage key.
@task_item n=3 part=needs
The actionable-key rule and author_id: every actionable element reads a stable author key carried as its AccessKit author_id; the coverage list keys handlers on that stable graph id (with its optional owner), never on an engine NodeId.
@task_item n=4 part=needs
DioxusDocument::actionable_nodes and the actions verb: the owed replay test records and replays a kept actions step, its answer compared key by key against the kept one, the actionable rule unchanged.
@task_item n=5 part=needs
The snapshot model the replay reads (role, name, focus and states from accessibility_tree; enabled, value, checked and clickable from the element and the vdom walk): the replayed actions answer must equal the kept one under that model.
@task_item n=6 part=needs
Redaction of what the coverage result carries: no control value, typed text or page text beyond ids, component and event names; a password input's MASKED_VALUE and a file input's marker never leave as text through a new result field.
@task_item n=7 part=needs
The driver's disabled refusal: a click or type is refused as disabled where the snapshot's enabled reads Some(false), nothing dispatched; a replayed step refused that way reached no handler.
@task_item n=8 part=needs
The canvas limit as a boundary of the count: handlers dispatched inside one canvas listener are not counted; virtual nodes are 0.4.0's, no canvas renderer or virtual-node production in this chunk.
@task_standing n=1
convention: a stand check that reads the tree or snapshot states the roles and names it expects once, in the shared module's tables, never a second role or name mapping
home: screen-reader-support
@task_standing n=2
convention: a stand check asserts in both layout modes
home: §3 A11y Assertion Harness Contract
@task_standing n=3
convention: a new stand test file is named in the leg lists that run it (the a11y leg's file list, the stand-repeat leg's named --test list) where it belongs to them
home: ci-integration
@task_standing n=4
convention: a new log line or span from escher's own targets passes the obs allowlist, text, value, html and attrs redacted, ids and closed words only
home: §3 A11y Assertion Harness Contract
@task_item n=1 part=not_needed
Colour contrast: Harness::text_contrast, the stand_contrast tables, the select list's colour constants and the lint verb's contrast reading; the chunk measures no colour.
@task_item n=2 part=not_needed
Key handling in the engine: range input steps, single and multiple select keys and typeahead, Enter and Space activation; the chunk adds no key behaviour.
@task_item n=3 part=not_needed
Focus order, focusability, the inert focus fixup and the stated Tab orders walked by stand_keyboard; the chunk changes no focus.
@task_item n=4 part=not_needed
ARIA state and role derivation in the tree builder (aria-selected, aria-pressed, aria-checked, aria-invalid, aria-expanded, aria-disabled, aria-level) and the HTML-to-AccessKit role mapping; the chunk builds no tree read.
@task_item n=5 part=not_needed
Label and description naming (aria-label, aria-labelledby, label for, aria-describedby) and their upstream tests; the chunk names nothing.
@task_item n=6 part=not_needed
The AccessKit platform adapter, its windowed assistive-technology witness and residual res-llkgqygv; the chunk builds no adapter.
@task_item n=7 part=not_needed
Hidden content and visibility exclusion (aria-hidden, hidden, display none, visibility hidden, inert hit pass-through); the chunk changes no exclusion.
@task_item n=8 part=not_needed
Cells grid role and cell semantics (residual res-wvatzhpe) and the CRUD list's unread aria-activedescendant; the chunk touches neither.
@task_item n=9 part=not_needed
Cognitive accessibility: the flight booker's invalid-date cue and status cues; the chunk adds no error or status cue.
@task_item n=10 part=not_needed
Motion, focus ring, colour scheme and user-preference rules; the chunk paints nothing.
