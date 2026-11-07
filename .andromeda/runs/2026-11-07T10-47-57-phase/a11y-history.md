@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the shell builds and refreshes the tree through `Document::accessibility_tree`, so DioxusDocument's override reaches it, and every Dioxus element node carries its stable element id as `author_id` (no TextRun / root / Window node does); this is the id contract the buffered replay must reproduce element for element, and the trait-method path the replayed render must still feed.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — `DioxusDocument::snapshot` takes role, name and the focused node from the tree `accessibility_tree` returns, with no second mapping; nothing focused at boot, only `back-btn` after one move; the snapshot's `focused` reading the inverted pins assert must keep coming from the tree, not from a mapping of the chunk's own.

@history_item n=3 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — founded the actionable-key rule and `DioxusDocument::unkeyed_actionable` (gated by `accessibility`); trap: never satisfy the check by adding a `tabindex` or role, since that moves the pinned Tab order; the replay must carry every author key so the check stays empty.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a falsy Dioxus boolean attribute is removed on both attribute paths (dynamic and static template), so `hidden: false` keeps a node in the tree; and the Tab / Shift+Tab key-press focus check (`crud-name` → `crud-surname`, `back-btn` on first Tab) with the focused node equal to the tree's focus; the replay must route set-attribute calls through both `MutationWriter` paths, and this check is the shape of the inverted pins' Tab precondition.

@history_item n=5 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the tree refresh on change real: `View::poll` takes `changed_nodes` (outside the cfg) and rebuilds the tree on a non-empty set; `changed_nodes` marked by in-document mutation, focus or checked change, a text input, not node creation; before this the flag answered false and the tree was built once — the precedent for the trap that a buffered replay that skipped marking would silently freeze the tree.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed-set contract (and the bridge's falsy boolean clear); these are ratified engine contracts the buffered render may not move without a new ruling.

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document's set is non-empty from the boot's own mutations, so a check reading `has_changes()` as "this step changed something" drains first; binds any new check on the buffered path's changed set (the extract's anti-pattern).

@history_item n=8 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand's role and name expectations are stated once in `tests/blitz-tests/tests/common/mod.rs` (`controls`, `INPUT_NAMES`, `rendered`), never a second mapping; the ported test and inverted pins must assert through the existing tree and snapshot, not tables of their own.

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom, so both stand binaries (`seven_guis_native`, `escher-session`) compile the override, snapshot, text and diff, neither a platform adapter; the new buffer module lands in a crate both binaries build under that feature, so it must stay ungated while the gated items keep reaching them.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the stand's states entry) — the flight booker's date inputs state `aria-invalid` and the invalid cue (Book's `disabled`) is read without colour; these reach the snapshot only through the replayed set-attribute paths, the falsy removal included.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — autofocus focuses the latest mounted focusable node whose `autofocus` is present and not "false", applied on flush, and the bridge removes a falsy `autofocus`; the buffered render's single flush after the replay must keep that flush-time focus step.

@history_item n=12 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the falsy predicate became upstream's 28-name `is_bool_attr`, the bridge's own list retired, leaving `mutation_writer.rs` byte-identical to upstream at `434e0661` (the baseline the scope keeps); a falsy `inert` is removed; a change to `mutation_writer.rs` (e.g. hosting the buffer on `DioxusState`) breaks that identity and must be named with its reason.

@history_item n=13 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (both entries) — Enter / Space dispatch a Click whose Dioxus `onclick` runs then `handle_click`, `counter-increment`, `flight-book`, `crud-create` changing the screen (a re-render through `poll`, the site the chunk edits); `stand_keyboard` built with its triple focus agreement and `disabled` read by presence; the walks and activation rows are the witness the buffered render must leave green.

@history_item n=14 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks entry) — `ENABLED_TAB_ORDER` walks and every interactive-role node named; the `&dyn Document` tree equal to the document's, carrying stable ids only; the refresh's two halves proven headlessly on the stand — the equality the replay must keep.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable()` returns nothing on every stand task (0 · 0 · 0 on the non-lean ones too); the click-without-role count pinned; the acceptance "`unkeyed_actionable` returns nothing on every stand task" rests on this count.

@history_item n=16 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the `stand_*` a11y files run three times in `stand-repeat` under test-plan §10's zero-failure budget; an engine change for every Dioxus document must add no flake there.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled`) — focusability = not inert (self or ancestor) and no disabled form control, `disabled` by presence on form controls; truthy `inert` removes focusability, hits and tree nodes; the replayed attribute writes must reach the mutator paths where cached focusability is recomputed.

@history_item n=18 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` joined `stand-repeat` by its named `--test` list; part of the eight-file repeat witness the extract binds.

@history_item n=19 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg became `cargo test --workspace --locked` over fourteen files, eight also in `stand-repeat`; the leg the acceptance runs with no expectation edited.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages entry) — `unkeyed_actionable` reads empty on all six tasks `escher-session` serves, the load page included; the six-task scope of the actionable-key acceptance.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus fixup built) — `DocumentMutator::flush` calls `BaseDocument::clear_focus` where the focused node reads `is_inert()`, no `blur` / `focusout`; `inert_subtree` 4 → 6 tests; the buffered render must still flush once per applied buffer so the fixup runs — research's open question per the extract.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`; `inert_subtree` asserts it; guards a render-time `set_focus` against naming a node the unapplied buffer has not built.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` hazard in a spawned task) — first recorded the `RefCell already borrowed` panic of a `MountedData` call in a key-handler-spawned task polled inside `render_immediate`; the async form was tried and did not ship, so Cells calls its handles synchronously — the hazard this chunk removes.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — `cells-grid` one Tab stop, arrows bringing the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; the synchronous handler-time call must keep acting at the call once the render-time borrow is lifted.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows) — `LOAD_TAB_ORDER` / `CELLS_TAB_ORDER` walked by `stand_keyboard`, `load-fetch` taking Enter and Space; re-asserted Tab walks the replay must keep unchanged.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally) — leg 14 files · 112 tests; added the line that `inert_subtree` asserts the focus fixup and the window fallback through `&dyn Document`; the CI witness for the fixup under the new flush order.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — the changed set's marking sites widened (a single select's list opening or closing) and its exclusions qualified by measurement; the current `changed_nodes` contract the buffered replay must mark identically.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts) — the leg now 14 files · 114 tests, `stand_keyboard` 23, `stand-repeat` seventeen words; the current tally the chunk's a11y leg run reads against.

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves) — the fixup reads `is_inert()` alone and `DocumentMutator::flush` stays unedited, a focused control that no longer shows keeping focus; the limit, owned by a `CARRY:` on «A11y re-read», must not move under the new flush order.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed after the take) — precedent: dioxus-native-dom's `dioxus_document.rs` is among the cited files whose digits are re-pointed when edited; this chunk edits its two render sites and `mod tests`, so those citations will move.

@history_item n=31 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (IME enabled at a focused input's editor construction) — pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime`, in the same `dioxus_document.rs` `mod tests` the ported test joins; focus at mount goes through the render sites the buffer reshapes, so it must keep passing.

@history_item n=32 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured, its fix routed) — mechanism re-measured at HEAD (`poll` holds `inner.borrow_mut()` across `render_immediate`, the task polled via `handle_task_wakeup`, `set_focus` through `doc_mut` panics), pinned by the two `should_panic` tests with their precondition (handle held, key target focused by Tab), fix routed to «MountedData buffered render» — the record this chunk inverts and rewrites.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — `dioxus_runtime_probe` (10 tests, both layout modes) runs in neither the `a11y` nor `stand-repeat` leg, reached by the workspace `test` leg; the inverted pins keep the count at 10 and the legs unchanged.

@history_item n=34 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the «reacts to a click» reading measured by link 2) — Cells 677 listener-marked elements, `counter-increment` `click`, `crud-list` `keydown`, `NODE_FIELDS` thirteen; one of the probe's other eight tests the buffered render must leave reading the same counts.

@history_item n=35 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report) — a removed listener leaves its event attribute and `data-dioxus-id` standing, `remove_event_listener` lowering only the count, `mounted` writing none; the replay must route create / remove listener calls through `MutationWriter`'s existing paths so this pinned over-report holds.

@history_item n=36 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — no platform adapter in the stand graph, Dioxus at `"0.7.3"`; the chunk keeps Dioxus at 0.7 with no pin bumped, so the adapter-absence reading should stay true and is re-read if the lock moves.
