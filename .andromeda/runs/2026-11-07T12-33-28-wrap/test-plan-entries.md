@amendment marker=2026-11-07-mounteddata-buffered-render
title: dioxus-native-dom's unit tests and the mutation buffer
section: §1 Test Scope Summary (body: the dioxus-native-dom scope line; test-scope-summary.md: the dioxus-native-dom unit-test entry)
change:
  - The dioxus-native-dom scope line names the mutation buffer as holding no unit test of its own, read through upstream's ported `task_polled_during_render_can_access_document` and blitz-tests' `dioxus_runtime_probe`.
  - dioxus-native-dom reads 83 unit tests in nine files (was 82), upstream's ported `task_polled_during_render_can_access_document` in dioxus_document.rs's test module among them, the crate-private `mutation_buffer.rs` holding none.
why: the chunk added the crate-private `mutation_buffer` module with no test and ported upstream's one test; the crate's test run reads 83 passed, at the gate and on the four CI test jobs.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: residual owners re-said to their rows' status
section: §1 Test Scope Summary (body: the tests/blitz-tests scope line; test-scope-summary.md: the accessibility-states gap); §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the `keyboard_compat.rs` exclusion)
change:
  - The adapter-crossing gap's owner reads residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), at both sites (was the residual with no status).
  - The `keyboard_compat.rs` exclusion's owner reads the left-out upstream delta, residual `res-6ssktw6x`, absorbed into `v030-03`, its re-read owned by the open CARRY `cry-mjwr45xr` on «Upstream sync ahead of determinism and exploration» (was owned by residual `res-6ssktw6x`, whose owner is that entry).
why: the operator's directive at this wrap's resume invocation, to bring the master sites naming a non-open residual to their rows' status, the sites listed in overseer1's relay of the F424 pass's leftovers; the residual rows read `res-llkgqygv` re-carried at 0.3.0 and `res-6ssktw6x` absorbed into `v030-03`.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: dioxus_document.rs and dioxus_runtime_probe.rs citations re-pointed
section: §2 Test Strategy, §3 Test Harness Contract, §5 Integration Test Strategy, §7 Test Data & Fixtures (body); integration-test-strategy.md (the listeners and held-signal entries); test-data-fixtures.md (the probe pages row); test-scope-summary.md (the unit-test and IME-on-mount entries); test-strategy.md (the directory pattern); unit-test-strategy.md (the test file location)
change:
  - The line ranges cited in packages/dioxus-native-dom/src/dioxus_document.rs and tests/blitz-tests/tests/dioxus_runtime_probe.rs re-pointed to where the same code now stands; no claim's wording moved.
why: the chunk added lines to both files (the buffer field, the two render sites, the ported test; the probe's new helper and module text), so the code the citations name moved.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the fix's witness on Dioxus 0.7, the probe's two pins
section: §9 CI Integration (ci-integration.md: the legs reaching `dioxus_runtime_probe`); §6 E2E Test Strategy (e2e-test-strategy.md: the pins entry, and a new entry on dioxus-core 0.7.10's task scheduling); §1 Test Scope Summary (test-scope-summary.md: the `dioxus_runtime_probe` entry and the upstream-test entry)
change:
  - The probe's hazard pins: was two `should_panic` tests `mounted_data_in_a_task_a_key_handler_spawned_panics_{non_incremental,incremental}` pinning the panic; now `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, one per layout mode, both through the `#[track_caller]` helper `spawned_focus_acts`, asserting `#presses` reads `1` and the engine's focus, the snapshot's one `focused` node and the tree's focus are all `focus-target`; `should_panic` 0; the probe still 10 tests, its legs unchanged.
  - The pins named the fix's discriminating witness on Dioxus 0.7: with `poll`'s borrowed render restored both read FAILED, `RefCell already borrowed`.
  - Upstream's `task_polled_during_render_can_access_document`: was absent, the witness of the route entry «MountedData buffered render»; now ported as upstream wrote it, passing and insensitive to the borrow on Dioxus 0.7, guarding the Dioxus 0.8 scheduling it was written for.
  - New entry: dioxus-core 0.7.10 first polls a task a component body spawns in the next `poll`'s `wait_for_work`, before any borrow, and a task an event handler spawns while dirtying a scope inside `render_immediate`'s `Work::PollTask` arm, the one path that met the borrow.
why: the report's disproved claim, measured by a mutation control: under the restored borrow the ported test reads `ok` and the two pins FAILED; amended on the operator's ruling, given directly in the chunk's session, that the witness question goes to the wrap and the specs are amended.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the per-OS CI tallies re-read, and attributing a tally move
section: §9 CI Integration (ci-integration.md: the per-OS tallies record, and a new entry on attributing a tally move)
change:
  - The tallies re-read on CI#42371051720 at `e26f1a207` (green 17/17): linux 220 · 1310 · 0 · 12; macos and coverage 201 · 1307 · 0 · 12; windows 201 · 1259 · 0 · 9; the a11y job and the stand repeat unmoved; ios and android build only (was last read on CI#41333172440).
  - This chunk over CI#41929821777: no new result line and +1 passed on each of the four test jobs, the probe's two pins renamed; CI#41333172440 → CI#41929821777, never written before, +1 result line and +10 passed (`dioxus_runtime_probe`'s 10 tests).
  - New entry: in a GitHub Actions job log the stderr `Running …` lines and the stdout `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name, beside the ANSI strip.
why: the chunk's CI run, its job logs read; attributing result lines by the preceding `Running` line gave 246 passed to `url_search_params`, so the move was attributed by name, a rule for later re-reads.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the MountedData hazard closed by the buffered render
section: §6 E2E Test Strategy (e2e-test-strategy.md: Cells' handle calls, headless)
change:
  - Was: a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed`, `poll` holding `inner.borrow_mut()` across `vdom.render_immediate`, its fix routed to «MountedData buffered render».
  - Now: the call acts, with no panic: `DioxusDocument::initial_build` and `Document::poll` render into the crate-private `MutationBuffer` (`buffered_mutations`) with the document unborrowed, `poll` taking the borrow once, after `render_immediate`, and replaying through one `MutationWriter`, in both layout modes, pinned by `dioxus_runtime_probe`; upstream `286874cc1`'s buffered render ported onto the 0.7 bridge, its Dioxus 0.8 move not taken.
  - Cells' synchronous handle calls kept, though no borrow needs them.
why: the chunk landed the founder's chosen fix, the buffered render ported from upstream onto Dioxus 0.7; the panic is retired on every Dioxus document.
kept: Cells' synchronous `MountedData` calls stand; they are no longer needed for the hazard and are not changed.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: dioxus-native-dom's coverage reading and mutation_buffer.rs
section: §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the per-crate reading, and a new entry on `mutation_buffer.rs`)
change:
  - dioxus-native-dom reads 2937/3387 = 86.71 % (floor 83 %) at CI#42371051720's `Coverage report` job (was 2791/3241 = 86.12 %, inferred from counts at CI#41333172440); blitz-test-harness 999/1040 = 96.06 % unmoved.
  - New entry: `mutation_buffer.rs` reads 116 lines, 7 missed, 93.97 %, counted inside the crate's 83 % floor with no exclusion.
why: the chunk's CI coverage job, read in its job log; the new module joins the crate's reading.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the buffered render's limit
section: §1 Test Scope Summary (test-scope-summary.md: a new dioxus-native-dom (Dioxus runtime) entry)
change:
  - New entry: by construction a task polled during render acts on the document as the previous render left it; a node created in the same render is not yet in it, so `set_focus_to` returns `false` for it; the probe's two `…_spawned_acts_*` pins act on nodes mounted by an earlier render; owner an open `residual` row (target: next).
why: the report's limit, by construction; the operator's directive at this wrap's resume invocation records it as a residual row.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: upstream's mounted-event test stays out
section: §1 Test Scope Summary (test-scope-summary.md: the upstream test left out)
change:
  - `element_removed_before_its_mounted_event_is_flushed` stays out, reading 1 mounted event on Dioxus 0.7 where it asserts 0, from Dioxus 0.7's scheduling, not the borrow; the buffered render moves no scheduling.
  - Its return: was with `286874cc1`; now with the Dioxus 0.8 move, owned by residual `res-6ssktw6x`, absorbed into `v030-03`, its re-read owned by the open CARRY `cry-mjwr45xr` on «Upstream sync ahead of determinism and exploration».
  - Heading: was «a second upstream test absent»; now «an upstream test left out».
why: the buffered render ported from `286874cc1` landed without the test returning; the deferral is on the operator's yes to the P5 card, its owner re-said to the residual's row status on the operator's directive at this wrap's resume.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
