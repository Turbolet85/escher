@task_prose part=chunk
Ports upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge: DioxusDocument's two render sites, initial_build (vdom.rebuild) and poll (vdom.render_immediate), run the vdom into a buffer of 0.7 WriteMutations calls with the document unborrowed, then borrow the document once, replay the buffer through the existing MutationWriter and flush the queued mounted events as before.
So user code run during render (a component body, a polled task) can call MountedData (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size) with no RefCell already borrowed panic; an engine change for every DioxusDocument, the stand and the windowed dioxus-native launch included.
Ports upstream's test task_polled_during_render_can_access_document as the witness: a task spawned during a re-render calls set_focus(true) and get_client_rect() and the focused node is the input it named.
Inverts dioxus_runtime_probe's two should_panic pins into tests that a key-handler-spawned task's set_focus moves focus without panic, in both layout modes, the handle held and the key target focused by Tab still asserted first.
Re-asserts unchanged behaviour through the existing checks: the document each render builds, element ids, snapshot and diff, mounted events' order and incremental/non-incremental layout identity (stand_*, stale_node_mapping, incremental_oracle, dioxus_runtime_probe's other eight tests).
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: the two render sites initial_build and poll, and the ported test task_polled_during_render_can_access_document in its mod tests
@task_file n=2 path=packages/dioxus-native-dom/src/mutation_buffer.rs mark=edits
what: the new buffer module recording 0.7 WriteMutations calls and replaying them into MutationWriter
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs mark=edits
what: the module list
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_writer.rs mark=edits
what: DioxusState, if the buffer lives on it as upstream's does; otherwise kept byte-identical to upstream at 434e0661
@task_file n=5 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the two should_panic pins at lines 581 and 587 inverted, one per layout mode, and its module text re-said
@task_item n=1 part=needs
DioxusDocument's render path (initial_build and poll): the document is borrowed once after the vdom renders, so every mutation of a render reaches the engine through MutationWriter in recorded order.
@task_item n=2 part=needs
Focus moved by MountedData set_focus from a task polled during render: the engine's focused node must become the named input, with no panic, in both layout modes.
@task_item n=3 part=needs
The engine's changed set (changed_nodes): an in-document mutation and a focus change mark it; the replayed buffer and a render-time set_focus must still mark it so a poll that takes a non-empty set rebuilds the accessibility tree.
@task_item n=4 part=needs
Accessibility tree refresh through the Document::accessibility_tree trait method, so DioxusDocument's override (its stable ids) still reaches the tree on InitialTreeRequested and on change, headless on the stand in both layout modes.
@task_item n=5 part=needs
Stable element ids as AccessKit author_id on every element node of a Dioxus document: the buffer carries ids and must leave each element's id as the direct write gave it.
@task_item n=6 part=needs
The snapshot model reading role, name, focus and states from accessibility_tree, and its diff: unchanged by the buffered render.
@task_item n=7 part=needs
The actionable-key rule and DioxusDocument::unkeyed_actionable returning nothing on every stand task: author keys must survive the replay unchanged.
@task_item n=8 part=needs
Falsy Dioxus values of the bridge's 28 boolean attributes (disabled, hidden, inert among them) removing the attribute on both attribute paths of MutationWriter: the replay must route set-attribute calls through the same paths.
@task_item n=9 part=needs
HTML focus fixup on the mutator flush: a focused node coming under inert gives up focus as the mutator flushes; the replay must still flush so the fixup runs once per applied buffer.
@task_item n=10 part=needs
The tree's focus falling back to the window where the focused node was not built: a set_focus issued during render, before the buffer is applied, must not leave the tree's focus on an unbuilt node.
@task_item n=11 part=needs
Tab key focus as the inverted pins' precondition: the key target focused by Tab through the harness's press, read back before the spawned task's set_focus.
@task_item n=12 part=needs
Mounted events (flush_queued_mounted_events) flushed after the buffer is applied, their order unchanged, so a MountedData handle is held only for nodes in the document.
@task_item n=13 part=needs
Both layout modes (incremental and non-incremental) and their identity: the inverted pins run one per mode and the layout identity stays as it is.
@task_item n=14 part=needs
The windowed dioxus-native launch renders through the same buffer; its accessibility tree refresh to the platform adapter is read, not run, as the probe's windowed path was.
@task_item n=15 part=needs
The buffer holds ids, tag and attribute names and text in process only, never printed or logged: no new log field or span for blitz* or escher_* targets.
@task_standing n=1
convention: a focus check reads the engine's focus, the snapshot's focused node and the tree's focus and asserts they agree at each step
home: keyboard-navigation
@task_standing n=2
convention: a headless stand or bridge check of focus or the tree runs in both layout modes
home: ## 3. A11y Assertion Harness Contract
@task_standing n=3
convention: a check reading has_changes() as "this step changed something" drains the changed set first, a fresh harness document's set being non-empty
home: a11y-strategy
@task_item n=1 part=not_needed
Colour contrast, Harness::text_contrast, stand_contrast's tables and the select list's painted colours: the chunk changes no style, colour or paint.
@task_item n=2 part=not_needed
Role derivation and ARIA state attributes (aria-selected, aria-pressed, aria-checked, aria-invalid, aria-expanded, aria-disabled, aria-level): the builder is not touched.
@task_item n=3 part=not_needed
Accessible names and descriptions (aria-label, aria-labelledby, label binding, aria-describedby): the builder's name reads are not touched.
@task_item n=4 part=not_needed
The select control's keys, list opening, typeahead and optgroup, and the range input's keys: no control's key or pointer handling changes.
@task_item n=5 part=not_needed
Per-surface keyboard shortcuts of seven_guis CRUD and Cells, and the CRUD listbox's unread aria-activedescendant: no stand app changes.
@task_item n=6 part=not_needed
Password and file input masking (MASKED_VALUE) and the driver's disabled refusal of click and type: no driver verb or snapshot field changes.
@task_item n=7 part=not_needed
Platform adapter creation before the window is shown (accesskit_xplat) and the windowed AT witness of residual res-llkgqygv: no adapter is built or moved.
@task_item n=8 part=not_needed
Hidden content and inert hit-testing rules (aria-hidden, hidden, display none, visibility hidden): no visibility rule changes.
@task_item n=9 part=not_needed
Cognitive error recovery (the flight booker's invalid dates) and motion: no stand cue or animation changes.
@task_item n=10 part=not_needed
Language and direction, text alternatives in fixtures and IME: no fixture or text input path changes.
@task_item n=11 part=not_needed
Escher's visual lint (Session::lint) and its contrast reading: not touched.
