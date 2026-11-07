@amendment marker=2026-11-07-mounteddata-buffered-render
title: the bridge's citations re-pointed to their moved lines
section: §Occupied Resources (body: URL schemes and identities); §Conventions (conventions.md: the conventions clause, Tests); §Cross-cutting Patterns (cross-cutting-patterns.md: Service wiring, Event processing, the queued `mounted` skip); §Design Philosophy (design-philosophy.md: `DioxusDocument`); §Established Decisions (established-decisions.md: the IME fact, the removed-node fact, [Dioxus integration], the two probe readings); §Existing Scopes (existing-scopes.md: the module list's `bridge_tests` citation); §Standard Contracts (standard-contracts.md: Document core, the select readers, Dioxus DOM bridge, the listener reading, the select and `multiple` select facts, `flush_queued_mounted_events`)
change:
  - Citations into `dioxus_document.rs`, dioxus-native-dom's `lib.rs` and `dioxus_runtime_probe.rs` re-pointed to the lines they now stand at; no claim around them moved.
why: the report's New text by line: the chunk added lines to `dioxus_document.rs` (the field, the two render sites, the ported test) and one `mod` line to `lib.rs`, and re-wrote the probe's two pins, moving the lines after them.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the bridge's buffered render recorded as a contract
section: §Conventions (conventions.md: Unsafe and borrowing); §Existing Scopes (existing-scopes.md: dioxus-native-dom's module list); §Standard Contracts (standard-contracts.md: a new fact after Dioxus DOM bridge; the listener-callers fact)
change:
  - The borrow-conflict workarounds gain the Dioxus bridge's: the vdom renders into a crate-private `MutationBuffer` with the document's `RefCell` unborrowed, then the document is borrowed once and the buffer replayed through one `MutationWriter`.
  - New: `DioxusDocument` holds `pub(crate) buffered_mutations: MutationBuffer`, set in `DioxusDocument::new`, its only constructor; `initial_build` and `Document::poll`, signatures unmoved, render into it unborrowed (`vdom.rebuild` / `vdom.render_immediate`), take `inner.borrow_mut()` once, build one `MutationWriter`, replay, and drop writer and borrow before `flush_queued_mounted_events`, so user code run during a render reaches the document through `MountedData` with no `RefCell already borrowed` panic.
  - The listener methods: was `create_event_listener` and `remove_event_listener` with no workspace caller, dioxus-core calling them; now dioxus-core calls `MutationBuffer`'s two, which record the call, and `MutationWriter`'s two have one workspace caller, `MutationBuffer::replay`, draining the recorded calls in order.
  - The module list gains `mutation_buffer` after `mutation_writer`: crate-private, outside every `accessibility` gate, no `pub use`.
why: the report's Symbols / APIs (the new crate-private `MutationBuffer`, its `replay` and its `WriteMutations` impl; the field and the two render sites, signatures unmoved) and its Crates / modules (`mod mutation_buffer;`, private and ungated, `keyboard_compat` kept).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the MountedData hazard closed as an engine change
section: §Established Decisions (established-decisions.md: the hazard record under [Dioxus integration]); §Project Intent (project-intent.md: the upstream delta left out)
change:
  - The record: was a measured hazard, no engine change — `DioxusDocument::poll` holding the document's `RefCell` across `render_immediate`, so a key-handler-spawned task that writes a signal and calls `set_focus` panics, pinned by `dioxus_runtime_probe`, its fix owned by the route entry «MountedData buffered render»; now the bridge's buffered render, a ratified direct engine change for every Dioxus document (the stand, the windowed `dioxus-native` launch, any app on the bridge), so such a task reaches the document through `MountedData` and `doc_mut` with no panic.
  - It is a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, flagged not upstreamable (upstream holds it in its 0.8 form), nothing sent.
  - Its witness on Dioxus 0.7 is the probe's two `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}` tests, headless in both layout modes, the windowed shell's lines read and not run.
  - Its merge surface, `dioxus_document.rs`, `mutation_buffer.rs` (replaced whole by upstream's 0.8 file at the 0.8 move) and `lib.rs`, is owned by residual `res-3v42legm`, absorbed into `v030-03`, its re-read owned by the CARRY on «Upstream sync ahead of determinism and exploration».
  - Project Intent: `286874cc1`'s buffered render was to be ported by the route entry; now ported onto the 0.7 bridge, the 0.8 move still waiting.
why: the report's Retired (the panic and the two `should_panic` pins) and Symbols / APIs (the engine change, its flag, its merge surface); the fix the founder chose at the probe chunk, by relayed answer as the record states it; the merge surface's owner per the report's Addendum on overseer1's F424 list.
kept: Cells still calls its handles synchronously, no longer needed for the hazard.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the buffered render's limit and dioxus-core 0.7.10's first-poll paths
section: §Established Decisions (established-decisions.md: two new facts after the buffered-render record)
change:
  - New: the buffered render's limit, by construction — a task polled during render acts on the document as the previous render left it, so a node created in the same render is not yet in it and `set_focus_to` returns `false` for it; the witness and the probe's pins act on nodes an earlier render mounted; owned by an open `residual` row (target: next).
  - New: on dioxus-core 0.7.10 a task spawned from a component body during `render_immediate`'s `Work::RerunScope` arm is first polled by the next `poll`'s `wait_for_work` → `process_events` → `poll_tasks`, before the bridge takes any borrow; a task spawned from an event handler that also dirties a scope is first polled inside `render_immediate`'s `Work::PollTask` arm, the one path on which a borrowed render meets the borrow — read in the registry copy and measured.
why: the report's Symbols / APIs (the limit; the 0.7.10 scheduling, read and measured by the mutation control); the limit recorded as an open residual on the operator's directive at this wrap's resume (item 4).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: non-open residual owner clauses re-said to their rows' status
section: §Established Decisions (established-decisions.md: the seven engine changes, the document's clock, the select engine change); §Standard Contracts (standard-contracts.md: the shell's platform-tree clause)
change:
  - `res-3v42legm`, three sites: was owned by the residual, its owner «Upstream sync ahead of determinism and exploration»; now owned by `res-3v42legm`, absorbed into `v030-03`, its re-read owned by the CARRY on «Upstream sync ahead of determinism and exploration».
  - `res-llkgqygv`, the windowed AT witness: was owned by residual `res-llkgqygv` with no status; now owned by it, re-carried at 0.3.0 (target: next).
why: the operator's directive at this wrap's resume, item 1: bring the master sites naming a non-open residual to their rows' status; the sites from overseer1's F424 leftovers list, each row read in the store (`res-3v42legm` absorbed into `v030-03`, `res-llkgqygv` re-carried, 0.3.0, next).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the buffered render's witness on Dioxus 0.7
section: §Existing Scopes (existing-scopes.md: the `dioxus_runtime_probe` record; a new fact after it)
change:
  - The probe record: was 8 looping tests and the MountedData hazard pinned as two `should_panic` tests, one per mode; now 8 and two, one per mode, `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, through the helper `spawned_focus_acts`, asserting the focus a key-handler-spawned task sets acts (`#presses` reads 1; the engine's focus, the snapshot's one `focused` node and the tree's focus are `focus-target`), the witness of the buffered render; still 10 tests.
  - New: dioxus-native-dom's `DioxusDocument` tests hold upstream's `task_polled_during_render_can_access_document` from `286874cc1` as upstream wrote it, passing; on Dioxus 0.7 it passes with the borrowed render restored too, so it is insensitive to the borrow, and the discriminating witness is the probe's two `…_spawned_acts_*` tests, which fail with `RefCell already borrowed` under that mutation.
why: the report's Spec claims disproved: the mutation control measured the ported test `ok` and the two pins FAILED with the borrowed render restored; the operator's ruling in the implement session, amend the specs.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
