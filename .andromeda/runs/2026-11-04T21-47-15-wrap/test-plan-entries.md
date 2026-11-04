@amendment marker=2026-11-04-dioxus-runtime-probe
title: blitz-tests' counts with dioxus_runtime_probe
section: §3 Test Harness Contract (the body: the Proof's newest reading); §2 Test Strategy (test-strategy.md: the blitz-tests manifest's targets)
change:
  - `run all`'s newest reading is passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe` the target with no `stand_` prefix it picked up with no script change, as measured at this chunk's report (was 733 · 0 · 12 with `key_press_after_container_lost_its_box`, at 2026-11-01-upstream-sync-ui-graph); `run stand` unmoved at 237 · 0 · 7 over 51 files.
  - blitz-tests reads 158 `.rs` files, 157 test targets and the unbuilt `all.rs`, `dioxus_runtime_probe.rs` the latest, its own auto-discovered target with no manifest line (was 157 and 156, `key_press_after_container_lost_its_box.rs` the latest, at 2026-11-01-upstream-sync-ui-graph).
why: the chunk adds one blitz-tests target, `dioxus_runtime_probe`, auto-discovered with no `stand_` prefix; `run all` measured 743 · 0 · 12 and `run stand` stayed at 237 over 51 files.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the dioxus_runtime_probe target, its legs and its pages
section: §9 CI Integration (ci-integration.md: Legs reaching `dioxus_runtime_probe`, a new group); §7 Test Data & Fixtures (test-data-fixtures.md: the Dioxus runtime probe pages, a new row); §1 Test Scope Summary (test-scope-summary.md: dioxus-native-dom (Dioxus runtime), a new entry)
change:
  - §9 gains the legs reaching `dioxus_runtime_probe`: the workspace `test` leg, `run all` and `run dioxus_runtime_probe`, each taking it as one more target with no line of its script naming it; it is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and none of the `a11y` leg's fourteen files.
  - §7 gains a row for the probe's own `rsx!` pages: `toggled_listener` (its click listener spread in and out as a `Vec<Attribute>`), `shared_state` (`Writer`, `FirstReader` and `SecondReader` over one `GlobalSignal`) and `hazard(spawned)` (`hazard_spawned`, `hazard_synchronous`).
  - §1 gains the target: 10 tests, 8 looping `for incremental in [false, true]` and the MountedData hazard pinned as two `should_panic` tests, one per layout mode; it reads Dioxus 0.7.10 public API alone, with no fork-level change, and measures link 1 (the component that rendered each node, a deleted CRUD row resolving to nothing), link 2 (each node's handlers), link 3 (a handler's effect as its step's snapshot diff) and link 4 (a held signal's readers).
why: the chunk adds one blitz-tests target reading what Dioxus 0.7 exposes with no fork-level change; auto-discovered with no `stand_` prefix, it joins the workspace `test` leg and `run all` with no script, CI step or leg list moved; its fixture pages are the check's own helpers.
kept: §9's `test`-leg result-line count and its per-OS tallies stand unmoved: the first counts the `test` leg's lines, not the `fast` leg's 221, and the second was not re-read from CI#41914044942.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the red Dependency audit and its green re-read
section: §9 CI Integration (ci-integration.md: Red on `Dependency audit`, a new group)
change:
  - §9 records CI#41911351081 at `53ae48a65` red on `Dependency audit` (advisories failed, licenses ok: RUSTSEC-2026-0433 on pulldown-cmark 0.13.5, RUSTSEC-2026-0429 on zip 2.4.2) and CI#41914044942 at `9b3b29714` green 17/17; why an audit goes red with `Cargo.lock` and `deny.toml` unchanged, and its local reproduction, are pointed to security-plan §Dependency Security, not restated.
why: two advisories published after the base's green audit turned the audit red on an unchanged graph; the operator's fix directive (the operator's own word, in the session) moved pulldown-cmark to 0.13.6 by a semver-compatible update and ignored RUSTSEC-2026-0429 by ID, and the final HEAD's run read green.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the MountedData hazard re-measured, pinned and routed
section: §6 E2E Test Strategy (e2e-test-strategy.md: Cells' handle calls, headless; The MountedData hazard's pins, a new group); §1 Test Scope Summary (test-scope-summary.md: dioxus-native-dom — an upstream test absent)
change:
  - §6's `RefCell already borrowed` panic of a `MountedData` call in a task a key handler spawned now carries its mechanism, re-measured at HEAD: `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, where dioxus-core 0.7.10 first polls the queued task through `handle_task_wakeup`, and the task's `set_focus` borrows through `doc_mut` and panics, in both layout modes, pinned by `dioxus_runtime_probe`; its fix is routed to the route entry «MountedData buffered render», which ports upstream `286874cc1`'s buffered render onto the 0.7 bridge and inverts the pin (was the panic site alone, at 2026-10-31-engine-debts; Cells' synchronous headless calls stand at that reading).
  - §6 gains the pins: `mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_panics_incremental`, two `should_panic` tests, their precondition asserted first by `hazard_ready` (the handle held, the key target focused by Tab); beside them `mounted_data_asked_at_once_in_a_key_handler_acts`, in both layout modes, a `set_focus` with no task moving focus without a panic; `get_scroll_offset` and `get_scroll_size`, a shared borrow through `doc`, and the windowed path read from the code, not run.
  - §1's absent upstream test `task_polled_during_render_can_access_document` (`286874cc1` not taken) is now the witness of that route entry.
why: the chunk's two `should_panic` pins measured the hazard at HEAD, precondition first, the synchronous contrast acting, no fix applied; the founder answered the MountedData fork with a port in a chunk of its own (the founder's word by dialog in the overseer session, relayed by the operator), and the operator's own word approving the P5 card inserts the route entry «MountedData buffered render», that upstream test its witness, the port inverting the pins.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the Dioxus runtime's listeners and a held signal's readers
section: §5 Integration Test Strategy (integration-test-strategy.md: Dioxus VirtualDom ↔ DioxusDocument, a node's listeners; Dioxus VirtualDom ↔ DioxusDocument, a held signal's readers; two new items)
change:
  - a node's listeners: the bridge's listener attributes (named the event, valued `<rust func>`, beside the `data-dioxus-id` mark, written by `create_event_listener`) over-report: dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener leaves an element's attribute list (a spread), and it lowers the per-kind count alone, so a listener removed on a re-render leaves its attribute and the mark standing while a click no longer acts, and a `mounted` listener writes none; the vdom walk's dynamic attributes (`VNode.dynamic_attrs` with `VNode::mounted_dynamic_attribute`) read each element's current listeners exactly, `mounted` included; pinned by `link_2_a_removed_listeners_attribute_stays`, in both layout modes.
  - a held signal's readers: a `GlobalSignal`'s `subscribers()`, visited inside `VirtualDom::in_runtime`, give through `ReactiveContext::origin_scope` exactly its two reader components' scopes, and the writer's click changes exactly the readers' nodes in the step's snapshot diff (`links_3_and_4_a_held_signals_subscribers_are_its_readers` over `shared_state`, both layout modes); hook-local `use_signal` state, where the stand keeps all its state, exposes neither link without a fork-level change, `Scope.hooks` being `pub(crate)`, read from the code, not run.
why: the chunk measured links 2, 3 and 4 through public Dioxus 0.7.10 API: the plan's prediction that a removed listener's attribute stays measured true, and the vdom walk's dynamic attributes, found while drafting the founder's reading, read link 2 exactly.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
