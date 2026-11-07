@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=37 class=stale
leaf-text: a `MountedData` call in a task spawned from a key handler panicking with `RefCell already borrowed` at `events.rs:162`, in both layout modes, pinned by `dioxus_runtime_probe`'s two `should_panic` tests, its fix routed to the route entry «MountedData buffered render», which ports upstream `286874cc1`'s buffered render onto the 0.7 bridge and inverts the pin
source: fct-kgita7a3 «a `MountedData` call in a task spawned from a key handler acts, with no `RefCell already borrowed` panic»
replace-with: a `MountedData` call in a task spawned from a key handler acting with no `RefCell already borrowed` panic, in both layout modes — `initial_build` and `poll` rendering into the crate-private `MutationBuffer` with the document unborrowed, `poll` borrowing once after `render_immediate` and replaying through one `MutationWriter`, upstream `286874cc1`'s buffered render ported onto the 0.7 bridge, its Dioxus 0.8 move not taken — pinned by `dioxus_runtime_probe`'s two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests, one per layout mode, `should_panic` 0
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=42 class=stale
leaf-text: read now on CI#41333172440 at 2026-11-01-upstream-sync-ui-graph under the same floors escher-driver 98.04 %, dioxus-native-dom 86.12 %, blitz-test-harness 96.06 %, those two inferred from counts, not from the lcov, the 97 % floor holding
source: fct-fx4qekph «escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2937/3387 = 86.71 % (floor 83 %), blitz-test-harness 999/1040 = 96.06 % (as measured at CI#42371051720's `Coverage report` job»
replace-with: read now on CI#42371051720 at 2026-11-07-mounteddata-buffered-render under the same floors escher-driver 98.04 %, dioxus-native-dom 86.71 % (its crate-private `mutation_buffer.rs` 93.97 %, counted with no exclusion), blitz-test-harness 96.06 %, the 97 % floor holding
