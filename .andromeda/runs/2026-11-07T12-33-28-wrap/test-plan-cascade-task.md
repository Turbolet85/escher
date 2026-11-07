@task_prose part=session
The session ported upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge: dioxus-native-dom gains the crate-private module mutation_buffer (MutationBuffer, recording dioxus-core 0.7.10's 15 WriteMutations calls), and DioxusDocument::initial_build and Document::poll render into it with the document unborrowed, then replay it through one MutationWriter.
It retired the RefCell already borrowed panic of a MountedData call in a task polled inside render_immediate, on every Dioxus document, and turned the probe's two should_panic pins into mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental over the helper spawned_focus_acts, which assert the engine focus, the snapshot's one focused node and the accessibility tree focus all name focus-target.
It ported upstream's task_polled_during_render_can_access_document unit test, and a mutation control measured it insensitive to the borrow on Dioxus 0.7 (ok with the borrowed render restored, while the two pins read FAILED); on the operator's ruling the specs are amended so the probe's two pins are the fix's witness on 0.7.
It measured dioxus-native-dom at 83 unit tests (82 before), its coverage at 2937/3387 = 86.71 % on CI#42371051720 (green 17/17), and re-read the per-OS CI tallies there; no dependency, gate, leg or harness surface moved.
Its wrap addendum re-says the residual owners res-llkgqygv (re-carried at 0.3.0, target next) and res-3v42legm and res-6ssktw6x (absorbed into v030-03, re-read through the CARRY cry-mjwr45xr on Upstream sync ahead of determinism and exploration).
@task_file n=1 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=4 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_item n=1 part=touches
The witness record (fct-e2zfwpza) calls upstream's task_polled_during_render_can_access_document the witness of the route entry MountedData buffered render; measured false on Dioxus 0.7: the test is now ported (dioxus_document.rs 846-885, fn at 850), passes and is insensitive to the borrow on 0.7, guarding the 0.8 scheduling; the fix's discriminating witness on 0.7 is the probe's two spawned_acts pins.
@task_item n=2 part=touches
dioxus-native-dom's unit count (fct-i5a6vv3t): 82 unit tests in nine files becomes 83 in nine files; the new mutation_buffer module holds no test.
@task_item n=3 part=touches
The MountedData hazard records (fct-kgita7a3 in E2E, and any scope or CI row stating the hazard standing, pinned by two should_panic tests, its fix owned by the route entry MountedData buffered render): the hazard is now closed, fixed by the buffered render on the 0.7 bridge.
@task_item n=4 part=touches
The probe's pins (fct-mealwlke, fct-luwk6iun, fct-o7txuegv and every row naming mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental, _incremental or the helper press_with_spawned_focus): renamed and inverted to mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental over spawned_focus_acts, which boots through hazard_ready(hazard_spawned, incremental), presses ArrowDown and asserts presses reads 1 and the engine, snapshot and tree focus name focus-target; the probe holds 0 should_panic and still 10 tests.
@task_item n=5 part=touches
Upstream's mounted-event test element_removed_before_its_mounted_event_is_flushed (fct-ybmlkzsq), recorded as returning with 286874cc1: it stays out on Dioxus 0.7 for 0.7's scheduling, not the borrow, and returns with the Dioxus 0.8 move, owned by residual res-6ssktw6x (absorbed into v030-03) through the CARRY cry-mjwr45xr.
@task_item n=6 part=touches
Rows saying 286874cc1 is to be ported: its buffered render is now ported; the Dioxus 0.8 move (0.8.0-alpha.3) is still not taken, and keyboard_compat.rs's retirement still waits on it.
@task_item n=7 part=touches
The dioxus-native-dom coverage reading (fct-fx4qekph, the line-17 row): 2791/3241 = 86.12 % inferred from counts at CI#41333172440 becomes 2937/3387 = 86.71 % read off CI#42371051720's Coverage report job (floor 83 %); CI#41929821777 read 2797/3241 = 86.30 %; escher-driver 4959/5058 = 98.04 % and blitz-test-harness 999/1040 = 96.06 % unmoved. The dated 2026-10-17 baseline 2472/2934 = 84.25 % stays.
@task_item n=8 part=touches
The per-OS CI tallies record (fct-qjdo4c7k, two rows): re-read on CI#42371051720 at e26f1a207 — linux 220 · 1310 · 0 · 12; macos and coverage 201 · 1307 · 0 · 12; windows 201 · 1259 · 0 · 9; a11y 14 · 114 · 0 · 0; stand repeat 228 · 1155 · 0 · 21; ios and android build only; this chunk adds no result line and +1 passed on each of the four test jobs; the probe chunk's unwritten move CI#41333172440 to CI#41929821777 is +1 result line and +10 passed on each.
@task_item n=9 part=touches
The fct-fx4qekph clause naming res-6ssktw6x as owner: the residual reads absorbed into v030-03, its re-read owned by the CARRY cry-mjwr45xr on Upstream sync ahead of determinism and exploration.
@task_item n=10 part=touches
Owner lines naming residual res-llkgqygv with no status (fct-aitehs62, fct-z2ynupl3): now owned by residual res-llkgqygv, re-carried at 0.3.0 (target next).
@task_item n=11 part=touches
The hazard pages row of Test Data (fct-ay5qvau6, hazard(spawned)): citation-only; the page is unchanged, only probe-file line digits may have moved (the probe gained lines at 12-17 and 574-613).
@task_item n=12 part=touches
Line citations into dioxus_document.rs (the inline test module start 449-450, keyed_nodes_do_not_crash 458-461 and 477-493, the dioxus dev-dependency use 453, the document test 477-487, the keyed-nodes props 462-477, the rustdoc example 60-82): lines were added at 7, 90-91, 155, 166-168, 171, 302-304, 307 and 846-885, so citations past line 7 may have shifted.
@task_item n=13 part=touches
The dioxus-native-dom unit-test listing (bridge, mutation writer, document events): it now holds the ported task_polled_during_render_can_access_document after mounted_data_of_removed_element_reports_error; the crate's module list gains the private mutation_buffer, outside every accessibility feature gate, with keyboard_compat kept.
@task_item n=14 part=touches
Any row stating that Cells calls its MountedData handles synchronously to avoid the hazard: the calls stay synchronous and are kept, no longer needed for the hazard.
@task_fact n=1 slug=test-scope-summary
fact: By construction a task polled during render acts on the document as the previous render left it: a node created in the same render is not yet in it, so set_focus_to returns false for it; the probe's pins act on nodes mounted by an earlier render, and the limit is recorded as an open residual row (target next).
@task_fact n=2 slug=e2e-test-strategy
fact: dioxus-core 0.7.10 scheduling: a task spawned from a component body during render_immediate's RerunScope arm is first polled by the next poll's wait_for_work before the bridge takes any borrow, while a task spawned from an event handler that also dirties a scope is first polled inside render_immediate's PollTask arm; only the second path met the borrow, which is why the key-handler pins discriminate the fix on 0.7 and upstream's test does not.
@task_fact n=3 slug=ci-integration
fact: In a GitHub Actions job log, attributing test result lines to targets by the preceding cargo Running line mis-attributes (stderr Running lines and stdout result lines do not interleave in order; 246 passed fell to url_search_params on CI#42371051720), so a tally move is attributed by grepping the test's own name, extending the ANSI-strip rule.
@task_fact n=4 slug=quality-gates-coverage-targets
fact: dioxus-native-dom's new mutation_buffer.rs reads 116 lines, 7 missed, 93.97 % on CI#42371051720's coverage job.
@task_item n=1 part=untouched
The agent-run contract and its run stand reading (237 · 0 · 7 over 51 files) and run all's 743 · 0 · 12: unchanged; the probe stays out of run stand, and the new unit test is a dioxus-native-dom test, not a blitz-tests target.
@task_item n=2 part=untouched
The CI legs, jobs, caches, stand-repeat's seventeen named targets and the a11y leg: no leg, step or selection changed, and the probe stays out of stand-repeat and the a11y leg.
@task_item n=3 part=untouched
The coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the keyboard_compat.rs coverage exclusion: unchanged.
@task_item n=4 part=untouched
blitz-test-harness's constructors, pump, settle, input and inspection helpers, and Harness::from_vdom's use of DioxusDocument: the public signatures of initial_build and poll are unmoved and callers call as before.
@task_item n=5 part=untouched
Dependencies and third-party test frameworks (test-that 0.6.0, Dioxus 0.7.3, dioxus-core 0.7.10 in the lock, no new crate): no manifest, lock or deny.toml moved.
@task_item n=6 part=untouched
The mutation writer's unit tests and its file: byte-identical, untouched by this session.
@task_item n=7 part=untouched
The mounted-event left-out readings (1 mounted event on Dioxus 0.7 where upstream asserts 0) and the probe's link readings, legs, silence and scrub rows and its 10-test count: still true.
@task_item n=8 part=untouched
The cold-agent pipe, the seven_guis stand, session checks, fixtures, WPT runner and blitz-dom, blitz-paint, escher-driver and apps/browser inventories: not reached.
