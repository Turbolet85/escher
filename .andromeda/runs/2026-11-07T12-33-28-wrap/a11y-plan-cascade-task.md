@task_prose part=session
The Dioxus 0.7 bridge now renders into a crate-private mutation buffer (MutationBuffer, new module mutation_buffer) with the document unborrowed, then replays it through one MutationWriter under one borrow; DioxusDocument::initial_build and Document::poll keep their signatures.
Retired: the RefCell already borrowed panic of a MountedData call (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size) in a task polled during render, on every Dioxus document; ported from upstream 286874cc1, the Dioxus 0.8 move still waiting.
The probe's two should_panic pins became mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental (helper spawned_focus_acts, in place of press_with_spawned_focus): after the spawned task's set_focus, the engine focus, the snapshot's one focused node and the accessibility tree's focus all name focus-target, in both layout modes; the probe keeps 10 tests.
Measured: upstream's ported test task_polled_during_render_can_access_document passes with the borrow restored, so on Dioxus 0.7 it is not the fix's witness; the probe's two pins are (they fail under that mutation). Limit by construction: a task polled during render sees the document as the previous render left it.
Wrap directive: every site naming residual res-llkgqygv or res-wvatzhpe as an open owner is re-said to its row status, re-carried at 0.3.0 (target: next); a citation of line 24 of the retired residuals file goes.
@task_file n=1 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=4 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_item n=1 part=touches
The §5 hazard record (fct-gmhtpdel): the MountedData hazard stated as standing, pinned by two should_panic tests and owned by the route entry MountedData buffered render, is now closed — fixed by the buffered render on the 0.7 bridge; the two pins assert that the spawned task's set_focus acts, with engine, snapshot and tree focus agreeing.
@task_item n=2 part=touches
Any a11y-plan text naming the probe pins mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental / _incremental, the should_panic expectation RefCell already borrowed, or the helper press_with_spawned_focus: they are renamed to the _spawned_acts_ pair and the helper spawned_focus_acts, and inverted to pass.
@task_item n=3 part=touches
Any text saying Cells calls its MountedData handles (scroll_to) synchronously because of the borrow hazard: the calls stay synchronous and are kept, but the hazard no longer requires them; the plain statement of the synchronous scroll_to stays true.
@task_item n=4 part=touches
The dioxus_runtime_probe records in a11y-plan (4 standing): wording on the hazard or the pins changes; the link readings, the legs, its 10-test count and its absence from the a11y leg, run stand and stand-repeat stay true; any probe line citation may have moved (module text 12-17, the pin region 574-613).
@task_item n=5 part=touches
Owner lines naming residual res-llkgqygv (the stand's platform accessibility adapter and its windowed AT witness) with no status — fct-kbjv4usk, fct-zyrpeqlm, fct-cixbakze, fct-ngdl2hcv, fct-p7eqwwyx — now read owned by residual res-llkgqygv, re-carried at 0.3.0 (target: next).
@task_item n=6 part=touches
Owner lines naming residual res-wvatzhpe (Cells' grid role and cell semantics) — fct-5i3imozy and fct-pofw2owv — now read owned by residual res-wvatzhpe, re-carried at 0.3.0 (target: next); fct-pofw2owv's citation of line 24 names a line of the retired residuals file and goes.
@task_item n=7 part=touches
Any statement that a Dioxus app's component body or a task it polls cannot safely reach the document through MountedData (focus, rect, scroll) during render: that is now allowed on every Dioxus document, the stand and the windowed dioxus-native launch included.
@task_item n=8 part=touches
Any claim that upstream's task_polled_during_render_can_access_document is the witness of the buffered render, or that it is absent: it is ported and passing, but insensitive to the borrow on Dioxus 0.7; the probe's two pins are the discriminating witness there.
@task_item n=9 part=touches
Focus-agreement test inventory: the probe's two pins now assert engine focus (harness.focused), the snapshot's focused node and Document::accessibility_tree focus agreeing on focus-target after a programmatic set_focus from a spawned task — a fact any section listing where the three focus readings are asserted to agree may need to carry.
@task_fact n=1 slug=keyboard-navigation
fact: A task polled during a Dioxus render acts on the document as the previous render left it, so MountedData::set_focus on a node created in the same render does not focus it (set_focus_to returns false); the probe's pins act on nodes mounted by an earlier render; the limit is recorded as an open residual row (target: next).
@task_item n=1 part=untouched
The CI a11y leg's fourteen-file list and the stand-repeat list: no CI step, leg or harness surface changed, and the probe stays outside both.
@task_item n=2 part=untouched
Role derivation, ARIA state reads, name and description reads (aria-label, aria-labelledby, label binding) and the hidden-content rules of the AccessKit builder: the builder is not changed.
@task_item n=3 part=untouched
Contrast verification (Harness::text_contrast, stand_contrast, the visual lint) and the select list colours: no styles, paint or contrast code moved.
@task_item n=4 part=untouched
Focusability, the inert focus fixup, disabled-ness, Enter and Space activation, range and select keyboard handling, and the falsy boolean-attribute bridge rule: the engine's focus and key rules and the 28-attribute list are unchanged; the replay applies the same mutations in the same order.
@task_item n=5 part=untouched
The snapshot model, stable author_id, the actionable-key rule and unkeyed_actionable: unchanged; the probe's pins only read them.
@task_item n=6 part=untouched
The generic owner lines for other owners (fct-2bz7tofn's CARRY on A11y re-read, fct-ma4gqj52): no change.
@task_item n=7 part=untouched
Platform adapter facts (accesskit_xplat, Adapter creation before the window is shown): no adapter work this session; only the residual status wording of its owner line moves.
@task_item n=8 part=untouched
The stand test counts (237 passed, 0 failed, 7 ignored over 51 files) and the stand_keyboard, accessibility_states, inert_subtree and label/description test inventories: unmoved.
