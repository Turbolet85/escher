@task_prose part=chunk
Builds a probe, in test or example code only, of the UI graph's four links (node to component, node to handlers, handler to state, state to reading nodes) as Dioxus 0.7.3 and escher's bridge dioxus-native-dom expose them with no fork-level change, each link given one of three verdicts.
Each verdict rests on a headless run on the seven_guis stand: a lean task where a handler writes state another node reads (Counter or CRUD) beside one served task, in both layout modes where layout touches the reading.
Link (2)'s answer for click says whether a node's click listener is readable from the listener map today (the deferred "reacts to a click" reading).
Re-verifies at HEAD the MountedData hazard's mechanism and coordinates (a task spawned from a key handler first polled inside render_immediate, panicking at events.rs:162) and names it in a plain-words reading for the founder, its fix brought as his fork with options, not decided or built.
Changes no Dioxus crate, no engine path for every document, and adds no verb, field, listener, log line or span; the UI graph itself is not built.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: the poll lines where a spawned task is first polled inside render_immediate while the document's RefCell is held
@task_file n=2 path=packages/dioxus-native-dom/src/events.rs mark=re-reads
what: the MountedData call's RefCell already borrowed panic at events.rs:162
@task_file n=3 path=.andromeda/cache/chunks/2026-10-31-engine-debts/evidence/readings.md mark=re-reads
what: §A hazard measured beside step 15, the headless poll(None) measurement of the MountedData panic
@task_item n=1 part=needs
The bridge's DioxusDocument element-id and listener reads, the surface every link of the probe reads through: in a Dioxus document every element node carries its stable element id as AccessKit author_id, and the probe addresses a node by that id rather than a new key.
@task_item n=2 part=needs
The actionable-key rule and its check DioxusDocument::unkeyed_actionable, which already reads whether an element carries an event listener; link (2)'s click answer is the reading that rule's residual line owns (no snapshot field says an element reacts to a click), and the probe answers it without adding that field.
@task_item n=3 part=needs
The snapshot model reads role, name and focus only off accessibility_tree, never a second role or name mapping; any role or name the probe reads of a node comes from the tree or the shared module's tables, and the probe adds no snapshot field, part or diff key.
@task_item n=4 part=needs
The engine's changed set (changed_nodes, take_changed_nodes, has_changes) as a witness for link (4), state to the nodes that read it: a harness settle neither reads nor drains it and a fresh harness document's set is non-empty, so a probe reading it as this step changed something drains first; node creation, hover and layout do not mark it.
@task_item n=5 part=needs
The accessibility tree's refresh on a poll that takes a non-empty changed set, through Document::accessibility_tree and DioxusDocument's override: the same poll path where the MountedData task is first polled inside render_immediate, re-verified headlessly and read, not run, on the windowed shell.
@task_item n=6 part=needs
The keyboard harness press and press_with on the stand, through which a key handler is reached headlessly to drive the hazard's path (a key handler spawning a task that writes a signal), and the stand_keyboard and stand_snapshot_state checks that already drive keys through them.
@task_item n=7 part=needs
Enter and Space dispatch of a Click to the focused button or control, where the app's handlers run before handle_click: the handler path whose written state links (3) and (4) read on the Counter or CRUD task.
@task_item n=8 part=needs
The CRUD list as a stand surface for the handler-to-state links: crud-list, named People, a listbox whose option rows carry aria-selected and whose selection a row's click or the list's arrow keys write.
@task_item n=9 part=needs
Values masked, never logged: nothing the probe reads (a component name, a handler, a signal's value) is printed, logged or put in a field; escher's log allowlist that redacts text, value, html and attrs for escher_* targets and the password MASKED_VALUE rule stay as they are, the probe adding no target, field or span.
@task_item n=10 part=needs
The accessibility feature reaching seven_guis through escher-driver and no stand binary building a platform adapter: the probe stays headless, so the windowed path and any assistive-technology reading are read, not run, unless a reading needs them.
@task_item n=11 part=needs
Both layout modes, the a11y plan's proof pattern for every stand reading: each verdict whose reading layout touches is measured in both.
@task_standing n=1
convention: a new stand check is asserted headlessly in both layout modes
home: ## 2. A11y Strategy and ## 3. A11y Assertion Harness Contract
@task_standing n=2
convention: the roles and names a check expects are stated once, in the shared module's tables, never a second role or name mapping
home: screen-reader-support
@task_standing n=3
convention: a new accessibility integration test file joins the a11y leg's named file list in ci.yml and its count, and a stand_* file also runs three rounds in the stand-repeat leg under the zero-failure budget
home: ci-integration
@task_standing n=4
convention: a stated table in a check asserts its own row count
home: ## 3. A11y Assertion Harness Contract
@task_item n=1 part=not_needed
Color contrast: Harness::text_contrast, contrast_ratio, stand_contrast's control, text and state tables and the visual lint's contrast reading; the probe reads no colour pair.
@task_item n=2 part=not_needed
The single and multiple select's list behaviour: typeahead, the open list's keys and clicks, SelectListView colours, select_popup and stand_act_popup; the probe reads no select list.
@task_item n=3 part=not_needed
The range input's keys, pointer value setting and step sanitizing.
@task_item n=4 part=not_needed
Role derivation from the role attribute and the HTML-AAM element mapping, and the landmark roles inventory; the probe names no role.
@task_item n=5 part=not_needed
The builder's ARIA name and description reads (aria-labelledby, aria-describedby, aria-description, label for) and the accessibility_label and accessibility_description checks.
@task_item n=6 part=not_needed
Focusability, the inert focus fixup, disabled-ness keyed on the disabled attribute's presence, and the inert_subtree check.
@task_item n=7 part=not_needed
Hidden content and visibility exclusion from the tree (aria-hidden, hidden, display none, visibility hidden) and hit-testing through inert.
@task_item n=8 part=not_needed
The bridge's falsy-value removal of its 28 boolean attributes; the probe writes no attribute path.
@task_item n=9 part=not_needed
The AccessKit platform adapter's creation before the window is shown and its windowed assistive-technology witness owned by a residuals line; no adapter is built.
@task_item n=10 part=not_needed
Cells' grid keyboard model and its focus outline, and the load page's Tab order.
@task_item n=11 part=not_needed
Cognitive cues: the flight booker's invalid dates and aria-invalid.
@task_item n=12 part=not_needed
Motion, focus ring, colour scheme and user-preference tokens.
@task_item n=13 part=not_needed
The not-yet-measured a11y violation schema, screen-reader test specs, live regions and skip links; the probe defines none of them.
