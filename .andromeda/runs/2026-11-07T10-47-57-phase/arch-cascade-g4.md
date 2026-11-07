@carried n=1 label=architecture/standard-contracts fact=fct-6qyfixyh
why: DioxusDocument's Document impl, its poll included, must take its document borrow only after the vdom returns, and still implement the trait as stated, its accessibility_tree override and stable ids reading the same after a buffered render.
quote: `Document` trait — `inner`, `inner_mut`, default `handle_ui_event` via `EventDriver` with `NoopEventHandler`, default `poll` returning `false`, `id`, and — under the `accessibility` feature — a default `accessibility_tree() -> accesskit::TreeUpdate` returning `self.inner().build_accessibility_tree()`, which a wrapper may override to enrich nodes (`DioxusDocument` alone does, with stable ids) (packages/blitz-dom/src/document.rs:188-222), implemented for `PlainDocument`, `BaseDocument`, `Rc<RefCell<BaseDocument>>` (packages/blitz-dom/src/document.rs:224-251), `HtmlDocument` (packages/blitz-html/src/html_document.rs:7-70), `ScriptDocument` (packages/blitz-vibey-script/src/document.rs:402-446) and `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:259-331); `DocGuard`/`DocGuardMut` wrap a borrowed `BaseDocument` (packages/blitz-dom/src/document.rs:136-186);
@carried n=1 run=2 label=architecture fact=fct-c5wyjq5i
quote: the changed set read by `has_changes` and drained by `take_changed_nodes`, its ids read through `get_node`, never by indexing;
@carried n=1 run=3 label=architecture fact=fct-srb7slak
quote: `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
@carried n=2 label=architecture/standard-contracts fact=fct-ivrm3he5
why: the buffer's replay must reach the document only through the existing MutationWriter's doc.mutate() calls, in recorded order, so it builds the same document as the direct write.
quote: **Mutation, query and CSSOM:** `doc.mutate()` methods `create_element`, `create_text_node`, `set_attribute`, `clear_attribute`, `append_children`, `insert_nodes_before`, `remove_node`, `remove_and_drop_node`, `set_inner_html`, `set_node_text`, `set_custom_widget`, `set_style_property`
@carried n=2 run=2 label=architecture fact=fct-4eadslhd
quote: **Mutation, query and CSSOM:** mutations through `doc.mutate()`;
@carried n=3 label=architecture/standard-contracts fact=fct-oti3svjj
why: the replayed mutations must still restyle through the mutator and insert into the changed set as the direct write did, so incremental_oracle's layout identity and the shell's refresh on change hold.
quote: Restyle on mutation: `set_attribute`, `clear_attribute`, removal and insertion restyle through `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change`, with no blanket `RestyleHint::restyle_subtree()`, and the changed-set insert in `remove_and_drop_all_children` stands first;
@carried n=4 label=architecture/standard-contracts fact=fct-oq4wpu6j
why: a polled task's set_focus must reach this focus change with no panic, which the ported test and both inverted pins assert by the focused node.
quote: focus change dispatches `blur`, `focusout`, `focus`, `focusin` (packages/blitz-dom/src/events/focus.rs:15-39), and the inert focus fixup none ([DOM semantics]);
@carried n=4 run=2 label=architecture fact=fct-3zkmzaom
quote: a focus change as `blur`, `focusout`, `focus`, `focusin`, but for the focus fixup under `inert`, which clears focus and dispatches none of them,
@carried n=5 label=architecture/standard-contracts fact=fct-qmowki7i
why: the scope names MountedData scroll_to among the calls that must reach the document during render, and it must reach this scroll_into_view unchanged.
quote: `scroll_into_view` scrolls every scrolling box on its target's containing-block chain, innermost first, and then the viewport — the CSSOM View "scroll an element into view" steps — its signature unchanged and for every caller: script `scrollIntoView`, Dioxus `MountedData::scroll_to`, fragment navigation and the test harness's input helper;
@carried n=6 label=architecture/standard-contracts fact=fct-qmowki7i
why: stale_node_mapping, re-run as the witness that the bridge's node mapping is unchanged, must still read a stale id's panic after a buffered render.
quote: `NodeTree` `get`, `get_mut`, `contains_key`, `len`, `iter`, `iter_mut`, stale-id indexing panics (packages/blitz-dom/src/tree.rs:75-117; packages/blitz-dom/src/tree.rs:120-136) with "invalid SlotMap key used" (tests/blitz-tests/tests/stale_interaction_state.rs:5-9; tests/blitz-tests/tests/stale_node_mapping.rs:12-16);
@carried n=6 run=2 label=architecture fact=fct-mjzuvlbq
quote: stale-id `NodeTree` indexing panics.
@carried n=7 label=architecture/standard-contracts fact=fct-evfmtphd
why: the ported test calls get_client_rect on a held MountedData during render, so dioxus-native-dom's get_client_rect must keep reading through get_client_bounding_rect.
quote: the readers of `get_client_bounding_rect` move with it — the snapshot's `bounds`, `node_client_rects`, the events driver's `adjust_element_coords`, the script `getBoundingClientRect` and dioxus-native-dom's `get_client_rect`.
@carried n=8 label=architecture/standard-contracts fact=fct-et27vuvt
why: the inverted pins must drive DioxusDocument::poll through the harness's pump, poll(None) then resolve, in both layout modes, so the polled task runs inside the buffered render.
quote: `Harness<DioxusDocument>` (tests/blitz-tests/tests/stale_node_mapping.rs:41-43; tests/blitz-tests/tests/stale_node_mapping.rs:60-67); `Harness::settle(&mut self) -> Result<Settled, NotSettled>` runs every piece of work that is due now and says whether anything is still outstanding: one pass is `pump`'s two calls in its order — `poll(None)`, then `resolve` at the harness's clock — with poll's answer kept;
@carried n=8 run=2 label=architecture fact=fct-gkkvjkmi
quote: **Test harness:** blitz-test-harness's `Harness`, generic over the document, with constructors, inspection and input helpers that pump and never settle;
@carried n=9 label=architecture/standard-contracts fact=fct-syf56xsm
why: the stand is named by the scope as a Dioxus document that renders through the buffer, so every stand boot's fresh VirtualDom must build through initial_build's buffer and the stand_* checks must hold.
quote: `boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>` and `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each building a fresh `VirtualDom` whose root provides a `TimerTicks` context and renders `app::task_in_shell` with a no-op back handler;
@carried n=9 run=2 label=architecture fact=fct-tewgiusr
quote: `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`)
@binding n=1
**Document core ↔ Mutation, query and CSSOM**: DioxusDocument's poll and initial_build must run the vdom with no `DocGuardMut` or borrow of the `Rc<RefCell<BaseDocument>>` held (architecture/standard-contracts fct-6qyfixyh), then replay the buffer once under one borrow through the `doc.mutate()` methods the direct write used (architecture/standard-contracts fct-ivrm3he5), so the restyle and changed-set inserts run as before (architecture/standard-contracts fct-oti3svjj) and the changed set drained by the shell reads the same (architecture fct-c5wyjq5i).
@binding n=2
**Events ↔ Scrolling, client rect and the Dioxus bridge**: the MountedData calls a polled task makes during render must reach the engine readers this group states, set_focus the focus change (architecture/standard-contracts fct-oq4wpu6j), scroll_to `scroll_into_view` (architecture/standard-contracts fct-qmowki7i) and get_client_rect `get_client_bounding_rect` (architecture/standard-contracts fct-evfmtphd), each with no borrow panic, while the stable ids and snapshot read unchanged (architecture fct-srb7slak).
@binding n=3
**Test harness ↔ Headless stand**: the inverted pins and the stand_* re-runs must drive the buffered poll through `Harness<DioxusDocument>`'s pump in both layout modes (architecture/standard-contracts fct-et27vuvt), on stand boots whose fresh VirtualDom now builds through the buffer (architecture/standard-contracts fct-syf56xsm).
