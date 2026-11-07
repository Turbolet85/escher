@apply_row n=1 ref_file=fanout-test-plan--cascade-g2.md ref_n=4 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g2.md ref_n=6 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g1.md ref_n=1 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g1.md ref_n=3 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan--cascade-g2.md ref_n=2 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan--cascade-g2.md ref_n=3 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan--cascade-g3.md ref_n=2 v=applied edit=E10
@apply_row n=11 ref_file=fanout-test-plan--cascade-g3.md ref_n=3 v=applied edit=E11
@apply_row n=12 ref_file=fanout-test-plan--cascade-g3.md ref_n=4 v=applied edit=E12
@apply_row n=13 ref_file=fanout-test-plan--cascade-g3.md ref_n=5 v=applied edit=E13
@apply_row n=14 ref_file=fanout-test-plan--cascade-g3.md ref_n=6 v=applied edit=E14
@apply_row n=15 ref_file=fanout-test-plan--cascade-g3.md ref_n=7 v=applied edit=E15
@apply_row n=16 ref_file=fanout-test-plan--cascade-g3.md ref_n=8 v=folded_into to_file=fanout-test-plan--cascade-g3.md to_n=7
@apply_row n=17 ref_file=fanout-test-plan--cascade-g3.md ref_n=9 v=applied edit=E16
@fact id=fct-aitehs62 expect_rev=2
old: owner: residual `res-llkgqygv`
new: owner: residual `res-llkgqygv`, re-carried at 0.3.0 (target: next)

@fact id=fct-jydhmppz expect_rev=1
old: the mutation writer and the document's events;
new: the mutation writer and the document's events — the mutation buffer holding no unit test of its own, read through upstream's ported `task_polled_during_render_can_access_document` and blitz-tests' `dioxus_runtime_probe`;

@fact id=fct-luwk6iun expect_rev=1
old: and the MountedData hazard pinned as two `should_panic` tests, one per layout mode,
new: and the buffered render's fix pinned as two, `mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_acts_incremental`, one per layout mode, each asserting through `spawned_focus_acts` that the task a key handler spawns acts — the engine's focus, the snapshot's one `focused` node and the tree's focus all `focus-target` —, the fix's discriminating witness on Dioxus 0.7,
old: it reads Dioxus 0.7.10 public API alone, with no fork-level change, and measures
new: it reads Dioxus 0.7.10 public API alone, its links needing no fork-level change and its two pins passing on the bridge's buffered render, and measures
old: dioxus_runtime_probe.rs:198-631) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: dioxus_runtime_probe.rs:198-631) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the two pins at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact id=fct-i5a6vv3t expect_rev=1
old: 82 unit tests in nine files (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md) —
new: 83 unit tests in nine files (as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md) — upstream's ported `task_polled_during_render_can_access_document` in dioxus_document.rs's test module among them (below), the crate-private `mutation_buffer.rs` holding none;

@fact after=fct-luwk6iun file=test-scope-summary gid=new:buffered-render-limit kind=procedure subject=scope-entities
- **dioxus-native-dom (Dioxus runtime) — the buffered render's limit:** by construction a task polled during render acts on the document as the previous render left it — a node created in the same render is not yet in it, so `set_focus_to` returns `false` for it; `dioxus_runtime_probe`'s two `…_spawned_acts_*` pins act on nodes mounted by an earlier render — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md; owner: an open `residual` row (origin 2026-11-07-mounteddata-buffered-render, target: next), P5's append

@fact id=fct-e2zfwpza expect_rev=2
old: **dioxus-native-dom — an upstream test absent:** upstream's `task_polled_during_render_can_access_document`, its commit `286874cc1` not taken, has no counterpart in the fork's dioxus_document.rs test module; it is the witness of the route entry «MountedData buffered render», which ports `286874cc1`'s buffered render onto the Dioxus 0.7 bridge and inverts `dioxus_runtime_probe`'s hazard pin — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
new: **dioxus-native-dom — upstream's buffered-render test, ported:** upstream's `task_polled_during_render_can_access_document` (`286874cc1`) stands in the fork's dioxus_document.rs test module as upstream wrote it (packages/dioxus-native-dom/src/dioxus_document.rs:846-885), beside `286874cc1`'s buffered render ported onto the Dioxus 0.7 bridge, which renders into a mutation buffer with the document unborrowed; it passes, and is insensitive to the borrow on Dioxus 0.7 — with `poll`'s borrowed render restored it still reads `ok`, dioxus-core 0.7.10 first polling a task a component body spawns in the next `poll`'s `wait_for_work`, before any borrow — so it guards the Dioxus 0.8 scheduling it was written for, and the fix's discriminating witness on Dioxus 0.7 is `dioxus_runtime_probe`'s two `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}` tests, which read FAILED, `RefCell already borrowed`, under the same mutation — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact id=fct-ybmlkzsq expect_rev=1
old: **dioxus-native-dom — a second upstream test absent:**
new: **dioxus-native-dom — an upstream test left out:**
old: where it asserts 0; it returns with `286874cc1`, so
new: where it asserts 0, from Dioxus 0.7's scheduling, not the borrow — the buffered render ported from `286874cc1` moves no scheduling —; it returns with the Dioxus 0.8 move, owned by residual `res-6ssktw6x`, absorbed into `v030-03`, its re-read owned by the open CARRY `cry-mjwr45xr` on «Upstream sync ahead of determinism and exploration», so
old: — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
new: — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md, its cause and its owner at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact id=fct-z2ynupl3 expect_rev=2
old: by residual `res-llkgqygv`, at an active assistive technology
new: by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), at an active assistive technology

@fact id=fct-kgita7a3 expect_rev=3
old: ; a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed` at `events.rs:162`: `DioxusDocument::poll` holds `inner.borrow_mut()` (`dioxus_document.rs:305`) across `vdom.render_immediate` (`:304`), where dioxus-core 0.7.10 first polls the queued task through `handle_task_wakeup` (`virtual_dom.rs:611`), and the task's `set_focus` (`events.rs:295`) borrows through `doc_mut` and panics, in both layout modes, pinned by `dioxus_runtime_probe` (below); its fix is routed to the route entry «MountedData buffered render», which ports upstream `286874cc1`'s buffered render onto the 0.7 bridge and inverts the pin — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: , the calls kept though no borrow needs them; a `MountedData` call in a task spawned from a key handler acts, with no `RefCell already borrowed` panic: `DioxusDocument::initial_build` and `Document::poll` render into the crate-private `MutationBuffer` (`buffered_mutations`) with the document unborrowed, `poll` taking `inner.borrow_mut()` (`dioxus_document.rs:305`) once, after `vdom.render_immediate` (`:304`), and replaying through one `MutationWriter`, so the task dioxus-core 0.7.10 polls inside `render_immediate` reaches the document, in both layout modes, pinned by `dioxus_runtime_probe` (below) — upstream `286874cc1`'s buffered render ported onto the 0.7 bridge, its Dioxus 0.8 move not taken — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact after=fct-kgita7a3 file=e2e-test-strategy gid=new:dioxus-core-scheduling kind=procedure subject=e2e
- **dioxus-core 0.7.10's task scheduling, read and measured:** a task a component body spawns during `render_immediate`'s `Work::RerunScope` arm is first polled by the next `poll`'s `wait_for_work` → `process_events` → `poll_tasks`, before the bridge takes any borrow, no `queue_events` following a `RerunScope`; a task an event handler spawns while it also dirties a scope is first polled inside `render_immediate`'s `Work::PollTask` arm, the one path that meets a borrow held across `render_immediate` — so the probe's key-handler pins discriminate the buffered render on Dioxus 0.7 and upstream's `task_polled_during_render_can_access_document` does not — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/mutation-control.md

@fact id=fct-mealwlke expect_rev=1
old: **The MountedData hazard's pins:** `mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_panics_incremental`, two `should_panic` tests, one per layout mode, pin the panic, their precondition asserted first by `hazard_ready` (the handle held, the key target focused by Tab);
new: **The buffered render's pins:** `mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_acts_incremental`, one per layout mode, both through the `#[track_caller]` helper `spawned_focus_acts`, which boots through `hazard_ready(hazard_spawned, incremental)`, its precondition asserted first (the handle held, the key target focused by Tab), presses ArrowDown and asserts `#presses` reads `1`, `harness.focused()` is `#focus-target`'s node, the snapshot holds exactly one `focused` node, `focus-target`, and `Document::accessibility_tree(&harness.doc).focus` is that node — the fix's discriminating witness on Dioxus 0.7: with `poll`'s borrowed render restored both read FAILED, `RefCell already borrowed`, while upstream's ported `task_polled_during_render_can_access_document` reads `ok`, insensitive to the borrow on 0.7;
old: (tests/blitz-tests/tests/dioxus_runtime_probe.rs:556-572; tests/blitz-tests/tests/dioxus_runtime_probe.rs:606-609;
new: (tests/blitz-tests/tests/dioxus_runtime_probe.rs:556-572; tests/blitz-tests/tests/dioxus_runtime_probe.rs:577-604; tests/blitz-tests/tests/dioxus_runtime_probe.rs:606-609;
old: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the two pins at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact id=fct-o7txuegv expect_rev=1
old: and the MountedData hazard pinned as two `should_panic` tests, one per layout mode;
new: and the buffered render's fix pinned as the two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests, one per layout mode, `should_panic` 0;
old: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the two pins at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact id=fct-qjdo4c7k expect_rev=1
old: the other rows by the same arithmetic, not test by test — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md
new: the other rows by the same arithmetic, not test by test — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md. Re-read on CI#42371051720, the pre-CI commit `e26f1a207` (green 17/17): linux 220 · 1310 · 0 · 12; macos and coverage 201 · 1307 · 0 · 12; windows 201 · 1259 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved; ios and android build only — no new result line and +1 passed on each of the four test jobs over CI#41929821777 (`0022acc21`; upstream's ported `task_polled_during_render_can_access_document`, by name), the probe's two pins renamed; CI#41333172440 → CI#41929821777, never written here, is +1 result line and +10 passed on each of the four test jobs (`dioxus_runtime_probe`'s 10 tests), the a11y job and the stand repeat unmoved — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/ci.md

@fact after=fct-qjdo4c7k file=ci-integration gid=new:tally-attribution kind=procedure subject=ci
- **Attributing a tally move in a job log:** in a GitHub Actions job log the stderr cargo `Running …` lines and the stdout `test result:` lines do not interleave in order, so attributing a result line to the `Running` line before it mis-attributes (246 passed fell to `url_search_params` on CI#42371051720); a tally move is attributed by grepping the test's own name, beside the ANSI strip of the per-OS tallies above — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/ci.md

@fact id=fct-fx4qekph expect_rev=2
old: dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, those two inferred from counts, not from the lcov, `coverage floor: 3 of 3 crates at or above their floors` (as measured at CI#41333172440, read in escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md)
new: dioxus-native-dom 2937/3387 = 86.71 % (floor 83 %), blitz-test-harness 999/1040 = 96.06 % (as measured at CI#42371051720's `Coverage report` job, read in escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/ci.md)
old: the exclusion lifts when escher takes that move, owned by residual `res-6ssktw6x`, the left-out upstream delta, whose owner is «Upstream sync ahead of determinism and exploration»
new: the exclusion lifts when escher takes that move — the left-out upstream delta, residual `res-6ssktw6x`, absorbed into `v030-03`, its re-read owned by the open CARRY `cry-mjwr45xr` on «Upstream sync ahead of determinism and exploration»

@fact after=fct-fx4qekph file=quality-gates-coverage-targets gid=new:mutation-buffer-coverage kind=values subject=gate-coverage-floor
- **`mutation_buffer.rs`'s reading:** dioxus-native-dom's crate-private `mutation_buffer.rs` reads 116 lines, 7 missed, 93.97 %, counted inside the crate's 83 % floor with no exclusion — as measured at CI#42371051720's `Coverage report` job, read in escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/ci.md
@pattern id=tp-01
fixed: pinned as two `should_panic` tests
@pattern id=tp-02
fixed: 82 unit tests in nine files
@pattern id=tp-03
fixed: is the witness of the route entry
@pattern id=tp-04
fixed: it returns with `286874cc1`
@pattern id=tp-05
fixed: borrows through `doc_mut` and panics
@pattern id=tp-06
fixed: two `should_panic` tests, one per layout mode, pin the panic
@pattern id=tp-07
fixed: dioxus-native-dom 2791/3241 = 86.12 %
@pattern id=tp-08
fixed: owned by residual `res-6ssktw6x`
@pattern id=tp-09
regex: residual `res-llkgqygv`(?!, re-carried)
